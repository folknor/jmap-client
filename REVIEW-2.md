# Pre-1.0 Architecture Improvements

Architectural improvements for jmap-client, assuming unlimited engineering resources, Rust 1.92+, and no backwards-compatibility constraints. Roughly ordered by impact.

---

## 1. Kill the God Enums — trait-based method dispatch

The single biggest structural problem. `Arguments` has ~60 variants, `MethodResponse` has ~60, `Method` has ~60. Every new JMAP type requires touching `request.rs`, `response.rs`, and `lib.rs`. This is the bottleneck for extensibility and compile times.

Replace with a trait-based approach:

```rust
// Each JMAP method becomes a self-describing type
trait JmapMethod: Serialize {
    const NAME: &'static str;          // "Email/get"
    const CAPABILITY: URI;             // URI::Mail
    type Response: DeserializeOwned;   // GetResponse<Email<Get>>
}

struct EmailGet { /* fields */ }
impl JmapMethod for EmailGet {
    const NAME: &'static str = "Email/get";
    const CAPABILITY: URI = URI::Mail;
    type Response = GetResponse<Email<Get>>;
}
```

`Request::add_call<M: JmapMethod>(method: M)` replaces the massive enum match. Response deserialization dispatches on the method name string via a type-erased registry built at request time (you know what you asked for, so you know how to deserialize each call ID). This eliminates the central wiring bottleneck entirely — new types are self-contained.

## 2. Feature-gate per RFC

If you only need JMAP Mail, you shouldn't compile Calendar, Contacts, Quotas, Sieve, Blob, Principals. This is trivial once the God Enums are gone (no central enum to keep in sync):

```toml
[features]
default = ["mail"]
mail = []
calendars = []
contacts = []
quotas = []
sieve = []
blob = []
principals = []
websockets = ["dep:tokio", "dep:tokio-tungstenite"]
```

Each feature gates its `src/<type>/` module. Cuts compile time and binary size proportionally.

## 3. Decouple transport via a trait

Not because reqwest is bad, but because it unlocks testing, WASM, and alternative runtimes:

```rust
trait Transport {
    async fn send(&self, url: &str, headers: HeaderMap, body: Vec<u8>) -> Result<TransportResponse>;
    // Optional: streaming upload, WebSocket upgrade
}

struct TransportResponse {
    status: u16,
    headers: HeaderMap,
    body: Vec<u8>,
}
```

Provide `ReqwestTransport` as the default (behind a `reqwest` feature). `Client` becomes `Client<T: Transport>` with `Client<ReqwestTransport>` as the ergonomic default. This also makes unit testing trivial — inject a mock transport that returns canned JSON.

## 4. Drop `maybe_async`

It's a proc macro that rewrites the entire async surface. On Rust 1.92+ this adds complexity for a use case (blocking) that's trivially solved by the caller:

```rust
let rt = tokio::runtime::Runtime::new().unwrap();
let result = rt.block_on(client.email_get(...));
```

Remove the `maybe_async` attribute from every method. One less proc macro dependency, cleaner generated docs, simpler debugging.

## 5. Drop `ahash` and `parking_lot`

- **AHashMap to std HashMap**: Since Rust 1.36, `std::collections::HashMap` uses `hashbrown` internally. The performance difference is negligible for the map sizes in JMAP (dozens of entries, not millions). One less dependency.
- **parking_lot to std::Mutex**: Modern std Mutex is competitive. The session mutex is not contended. One less dependency.

## 6. Structured errors

`Error::Internal(String)` and `Error::Server(String)` are opaque. Replace with specific, matchable variants:

```rust
enum Error {
    Transport(Box<dyn std::error::Error + Send + Sync>), // generic over transport
    Deserialize { source: serde_json::Error, context: &'static str },
    Problem(ProblemDetails),
    Method(MethodError),
    Set(SetError),
    SessionExpired,
    NoAccountForCapability(URI),
    // ... specific variants for each failure mode
}
```

Every variant is matchable. No more parsing error strings.

## 7. Proc macro derive for JMAP objects

Replace the declarative macros (`impl_jmap_object!`, `json_object_struct!`) with a derive macro in a `jmap-client-derive` subcrate:

```rust
#[derive(JmapObject)]
#[jmap(capability = "urn:ietf:params:jmap:mail", account = true)]
pub struct Mailbox<State = Get> {
    #[jmap(id)]
    id: Option<String>,
    #[jmap(property)]
    name: Option<String>,
    #[jmap(nullable)]  // generates Option<Option<T>> semantics
    description: Option<Option<String>>,
    // ...
}
```

Benefits: better error messages, IDE support, documentation on the struct itself instead of hidden in macro invocations. The derive generates `Object`, `GetObject`, `SetObject`, `ChangesObject` impls, the Property enum, and the getter/setter methods.

## 8. Streaming pagination

JMAP supports `position`, `anchor`, `limit` for pagination. The current API returns one page. Add an async stream iterator:

```rust
let mut events = client.calendar_event_query_stream(filter, sort);
while let Some(batch) = events.next().await? {
    for event in batch { /* ... */ }
}
```

Handles `position` advancement, `queryState` tracking, and back-pressure automatically.

## 9. Typed capability introspection

Currently capabilities are an enum you match on. Make them first-class:

```rust
if let Some(mail_cap) = session.capability::<MailCapabilities>() {
    println!("max size: {}", mail_cap.max_size_attachments_per_email());
}
```

Each capability type registers itself (similar to the method trait approach). No central enum to maintain.

## 10. Modernize dependencies

| Current | Target | Why |
|---------|--------|-----|
| `base64` 0.13 | 0.22 | 0.13 is years old, API changed significantly |
| `chrono` 0.4 | `jiff` 1.x or `time` 0.3 | `jiff` is the modern choice for calendar-heavy work, better timezone handling |
| `reqwest` 0.13 | keep (behind feature) | still the standard |

## 11. Builder validation at type level

Currently builders let you forget required fields and you get a runtime error or silent null. Use typestate for required fields:

```rust
let event = CalendarEvent::builder()
    .calendar_ids(["cal1"])   // required — returns Builder<HasCalendar>
    .title("Meeting")         // optional
    .start("2025-06-15T10:00:00")
    .build();                 // only available when all required fields set
```

Particularly valuable for Set operations where the spec mandates certain fields on create.

## 12. `#[non_exhaustive]` on all public enums

Every public enum (Property, Filter, Comparator, Error, Capabilities, Method, etc.) should be `#[non_exhaustive]`. This lets us add variants without breaking downstream post-1.0. Cheap to do now, painful to retrofit later.

---

## Out of scope

- **GraphQL-style query building** — JMAP's method call model is already well-defined; another abstraction layer would fight the protocol.
- **Connection pooling** — reqwest already handles this.
- **Caching layer** — application-level concern, not protocol-level.
- **Sync wrapper** — caller's problem (see item 4).
