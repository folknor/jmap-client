# CLAUDE.md

## Agent rules

- Always launch subagents in the **foreground** (never use `run_in_background`). Background agents cannot get tool approvals.

## Project

Fork of stalwartlabs/jmap-client (Apache-2.0 / MIT), maintained at folknor/jmap-client. Typed Rust bindings for the JMAP protocol. Pre-1.0 — API stabilization phase.

## Build & test

```bash
cargo build                                          # default features (tls-rustls)
cargo test --lib                                     # 74 tests with all features
cargo test --lib --no-default-features -F tls-rustls # 22 core-only tests
cargo build --no-default-features -F tls-rustls      # minimal build (core + principal + share_notification + push)
cargo clippy --lib                                   # zero warnings expected
```

Lints are in `[lints.clippy]` in Cargo.toml, not inline in source.

## Architecture

### Trait-based method dispatch (no central enums)

Every JMAP method is a self-describing struct implementing `JmapMethod`:
```rust
pub trait JmapMethod: Serialize + Send {
    const NAME: &'static str;       // "Email/get"
    type Cap: Capability;           // capability::Mail
    type Response: DeserializeOwned; // GetResponse<Email<Get>>
}
```

Adding a new method: define a struct, use `define_get_method!` / `define_set_method!` etc., done. **Zero central files touched.**

### Request/Response flow

```rust
let mut request = client.build();
let handle = request.call(EmailGet::new(&account_id))?;  // typed CallHandle<EmailGet>
let mut response = request.send().await?;
let result = response.get(&handle)?;  // compile-time safe extraction
```

`CallHandle<M>` validates call_id and method name. `Response::get()` handles method errors (returns `Error::Method` for JMAP error responses).

### Transport abstraction

`Client<T: HttpTransport = ReqwestTransport>` — generic over transport.
- `HttpTransport` — api_request, upload, download, get_session (returns `Bytes`)
- `SseTransport` — open_sse (EventSource, with `last_event_id` support)
- `ReqwestTransport` — default implementation with pooled reqwest::Client
- `Client::with_transport(transport, session)` — custom transport injection
- WebSocket remains reqwest-specific (documented)

All convenience helpers are `impl<Tr: HttpTransport> Client<Tr>` — custom transports get the full API.

### Module pattern

Every JMAP object type under `src/<type>/`:
- `mod.rs` — struct with `<State = Get>` phantom, Property enum, method struct definitions via `define_*_method!` macros
- `get.rs` — getters on `T<Get>`, GetObject impl
- `set.rs` — builder methods on `T<Set>`, SetObject + SetObjectCreatable impls
- `query.rs` — Filter/Comparator enums, QueryObject impl
- `helpers.rs` — `impl<Tr: HttpTransport> Client<Tr>` convenience methods

### Two data models

**Typed structs** (Mailbox, Calendar, AddressBook, etc.): serde derive, `Field<T>` for nullable properties.

**JSON map backing** (CalendarEvent, ContactCard): `serde_json::Map` via `json_object_struct!` macro. Property enum has `Other(String)`. Extension properties preserved on round-trip.

### Key types

- `Field<T>` — three-state nullable: `Omitted` / `Null` / `Value(T)`. Use instead of `Option<Option<T>>`.
- `Id<T>` — phantom-typed string ID: `AccountId`, `BlobId`, `State`. Available for incremental adoption.
- `Account<'a, Tr>` — account-scoped view of Client. Use `account.build()` for scoped requests.
- `Capability` trait — typed URIs with associated `Config` type.
- `TransportError` — crate-owned, `#[non_exhaustive]`, carries response body (`Bytes`) for ProblemDetails parsing.

### Capabilities

`Capabilities` enum in session.rs uses `deserialize_capabilities_map` to dispatch on URI key string. When adding a new capability:
1. Add struct in session.rs
2. Add variant to `Capabilities` enum (with `#[cfg]` if feature-gated)
3. Add match arm in deserializer
4. Add `Capability` impl in capability.rs with `type Config`
5. Add session accessor method

`Session::typed_capability::<C>()` is a convenience bridge (serde round-trip). Hand-written accessors are zero-cost and primary.

### Feature gates

Per-RFC features: `mail`, `calendars`, `contacts`, `blob`, `quota`. Each gates:
- Module declarations in lib.rs
- DataType enum variants (with `#[serde(other)]` catch-all)
- Capabilities enum variants + session accessors + deserializer arms
- PushObject/PushNotification variants
- Test modules

### Error model

Structured variants — no `Error::Internal(String)`:
- `CallNotFound`, `IdNotFound`, `EmptyResponse`, `NotParsable`, `InvalidUrl`, `WebSocketNotConnected`
- `Transport(TransportError)` — wraps transport errors, auto-parses ProblemDetails from body
- `Method(MethodError)` — JMAP method-level errors
- No `From<reqwest::Error>` — reqwest errors converted to TransportError at point of use

### PatchObject null semantics

RFC 8620: `null` removes map keys, not `false`. Email `patch` field uses `HashMap<String, serde_json::Value>` with `Value::Null` for removals.

## Code style

- License header on all files
- Async-only (no maybe_async, no blocking)
- `#[non_exhaustive]` on all public enums and TransportError
- Clippy lints in Cargo.toml `[lints.clippy]`
- `#[serde(skip_serializing_if = "...")]` on optional fields
- `Field::is_omitted` for skip_serializing_if on Field<T> fields (with `#[serde(default)]`)
- SetObjectCreatable::new() initializes optional fields to None/Omitted, not empty collections
- Don't commit .md reference docs (CALENDARS.md, etc.)
- Helper impl blocks use `impl<Tr: HttpTransport> Client<Tr>` (not bare `impl Client`)
