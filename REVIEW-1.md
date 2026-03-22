# API Redesign Notes

These notes assume:

- We are pre-1.0 and can break the API freely.
- We target Rust 1.92+.
- We care more about long-term library design than short-term migration cost.
- The API must conform to the relevant JMAP RFCs and drafts.

## High-level direction

I would do a real redesign, not an incremental cleanup.

The current crate already has enough public coupling that the cleanest path is to separate:

- protocol and data model
- request orchestration
- transport implementation
- streaming transports

Today, `reqwest` is not just an implementation detail. It materially shapes the public surface. For example:

- `Error::Transport(reqwest::Error)` is part of the public error model in `src/lib.rs`
- session discovery and connection are hardwired to `reqwest` in `src/client.rs`
- blob upload/download and streaming paths also depend directly on `reqwest`

I do favor decoupling `reqwest` from the core crate. Not because `reqwest` is bad, but because JMAP is an application protocol and `reqwest` is just one HTTP client. The protocol crate should own the JMAP model and orchestration layer, while `reqwest` should be one supported transport adapter.

That does not mean removing `reqwest` support. I would keep it as:

- the default transport adapter
- the primary documented on-ramp
- probably the easiest way to get started

But I would not keep it as a mandatory direct dependency of the protocol core.

## Crate split

I would split the project into a small set of focused crates:

- `jmap-types`
  - Pure data model
  - Request and response types
  - IDs
  - method definitions
  - capabilities
  - RFC-specific object types
- `jmap-client-core`
  - session discovery
  - request batching
  - result references
  - capability checks
  - state tracking
  - transport traits
- `jmap-client-reqwest`
  - default HTTP transport implementation
- `jmap-client-ws`
  - WebSocket transport and subscription support
- optionally `jmap-client-sse`
  - EventSource support

This gives us:

- a protocol layer usable without `reqwest`
- cleaner testing and mocking
- easier support for other runtimes and environments
- a smaller and more stable core API

## Client construction

I would replace the current pattern:

```rust
let client = Client::new()
    .credentials(("john@example.org", "secret"))
    .connect("https://jmap.example.org")
    .await?;
```

with a transport-agnostic construction model:

```rust
let transport = ReqwestTransport::builder()
    .bearer_auth(token)
    .build()?;

let client = JmapClient::connect(transport, session_url).await?;
```

If the caller already has a session object:

```rust
let client = JmapClient::from_session(transport, session);
```

The client should not know whether the underlying implementation is:

- `reqwest`
- `hyper`
- browser `fetch`
- a mock transport
- an internal gateway client

## Transport design

The core crate should define transport traits and normalized transport-facing types.

Example shape:

```rust
pub trait HttpTransport: Send + Sync + 'static {
    async fn send(
        &self,
        request: HttpRequest<Bytes>,
    ) -> Result<HttpResponse<Bytes>, TransportError>;
}

pub trait StreamingTransport: Send + Sync + 'static {
    type EventStream: Stream<Item = Result<ServerEvent, TransportError>> + Send;

    async fn open_event_source(
        &self,
        request: HttpRequest<Bytes>,
    ) -> Result<Self::EventStream, TransportError>;
}
```

The important point is not the exact signature. The important point is that core should not expose `reqwest` types or `reqwest::Error`.

## Error model

I would normalize the public error surface into crate-owned error types, for example:

- `TransportError`
- `HttpError`
- `ProtocolError`
- `DecodeError`
- `CapabilityError`
- `MethodError`

This matters because downstream users should not have their own public APIs contaminated by whichever transport adapter we happen to use internally.

`reqwest` errors can still exist inside `jmap-client-reqwest`, but should be mapped into the core error model before crossing crate boundaries.

## Public API shape

I would move away from making one helper method per JMAP method the primary abstraction.

Those helpers are useful, but JMAP is fundamentally:

- batched
- account-scoped
- capability-driven
- reference-oriented

The first-class API should reflect that.

Example:

```rust
let response = client
    .request(account_id)
    .using::<Mail>()
    .using::<Submission>()
    .call(EmailQuery::new().filter(...).sort(...))
    .call(EmailGet::new().ids(refs.path("/ids/*")).properties([...]))
    .send()
    .await?;
```

Then convenience helpers can exist as sugar:

```rust
client.email_get(id).await?;
client.mailbox_create("Work").await?;
```

But those helpers should be layered on top of the batch API, not define the architecture.

## Typed capabilities

I would make capabilities strongly typed:

```rust
pub trait Capability {
    const URI: &'static str;
    type AccountCaps;
    type SessionCaps;
}
```

And expose capability-driven operations like:

```rust
client.capability::<Mail>()?;
client.account_capability::<Calendars>(account_id)?;
client.supports_method::<EmailQuery>(account_id)?;
```

Requests should also be able to declare `using` in a typed way:

```rust
request.using::<Mail>().using::<Quota>();
```

This would move capability handling from stringly protocol plumbing toward a real type-driven API.

## Better account modeling

JMAP is account-centric. The API should make that obvious.

I would expose first-class session and account types:

- `JmapSession`
- `JmapAccount`
- `AccountCapabilities<C>`

And likely provide account-scoped client views:

```rust
let mail = client.account(primary_mail_account)?;
mail.email_query(...).await?;
```

That is a better fit than passing raw account IDs as strings throughout the API.

## IDs and protocol scalar types

I would introduce newtypes for common protocol identifiers:

- `AccountId`
- `EmailId`
- `MailboxId`
- `BlobId`
- `State`
- `CallId`
- `RequestId`

This provides:

- type safety
- clearer method signatures
- less accidental mixing of unrelated strings

Example:

```rust
fn email_get(&self, id: &EmailId) -> ...;
fn mailbox_query(&self, account: &AccountId, ...) -> ...;
```

I would avoid premature micro-optimization in string storage until profiling justifies it.

## `AHashMap` in the public API

I would stop exposing `AHashMap` publicly.

My default rule would be:

- use `std::collections::HashMap` unless ordering matters
- use `IndexMap` where stable iteration order is valuable to users
- reserve specialized hashers for internal-only use

In a protocol library like this, deterministic ordering is often useful for:

- debugging
- snapshot tests
- documentation examples
- predictable serialization behavior in practice

So for many public protocol dictionaries I would likely prefer `IndexMap<String, T>` over `HashMap<String, T>`.

For large internal lookup tables, standard `HashMap` is enough.

Pre-1.0 is the right time to remove implementation-specific map types from the public surface.

## Async model

I would lean fully into async-only.

The README already presents the crate as async-only, so I would simplify around that assumption and remove compatibility layers that dilute the design.

The core crates should depend only on lightweight async-facing building blocks, such as:

- `futures-core`
- `bytes`
- `http`
- `serde`

Runtime-heavy integration should live in adapters.

That makes the protocol layer more portable across:

- Tokio
- other async runtimes
- WASM/browser environments
- custom executors

## Result reference ergonomics

JMAP result references are one of the protocol’s most powerful features and deserve a real API.

Users should not need to construct path strings manually.

Instead:

```rust
let q = request.call(EmailQuery::new().filter(...));
request.call(EmailGet::new().ids(q.ref_ids()));
```

or:

```rust
let ids = query.ids_ref();
request.call(EmailGet::new().ids(ids));
```

This should serialize into the RFC-compliant result reference shape while preserving type information in user code.

## Nullability and patch semantics

The current `Option<Option<T>>` approach matches the spec, but it is not a great user-facing API.

I would preserve the semantics while improving the representation.

For example:

```rust
pub enum Field<T> {
    Omitted,
    Null,
    Value(T),
}
```

Then setters become more explicit:

```rust
calendar.time_zone(Field::Null);
calendar.color(Field::Value("#ff0000".into()));
```

This is clearer than forcing users to reason about `Some(None)`.

For patch APIs, I would avoid making users encode removals via raw JSON nulls in maps. Instead:

```rust
patch.set("keywords.$seen", true);
patch.remove("keywords.$draft");
```

The implementation can still serialize removals as `null` where the RFC requires that behavior.

## Object model strategy

The current split between typed structs and JSON-map-backed objects is directionally correct. I would formalize it.

Two main categories:

- `TypedObject`
  - for RFC-stable schemas with well-defined fields
- `OpenObject`
  - for JSCalendar/JSContact-style extensible objects backed by a property map

For open-ended objects, I would preserve unknown fields by default and add:

- typed property keys
- extension namespace support
- strongly typed convenience accessors for common fields

This gives us high fidelity without giving up ergonomics.

## Method typing

I would move away from giant centralized enums like `Arguments` and `MethodResponse` as the primary extensibility mechanism.

Instead, each method type should carry its own response association:

```rust
pub trait MethodCall {
    const NAME: &'static str;
    type Response: DeserializeOwned;
    type Capability: Capability;
}
```

Then `EmailGet`, `MailboxQuery`, `CalendarEventSet`, and so on each implement `MethodCall`.

The batch builder can still type-erase internally if needed, but the maintenance burden is much lower because adding a new JMAP method no longer requires touching a giant central registry of variants in several files.

## Streaming, WebSocket, and EventSource

I would not model WebSocket and EventSource as special client modes.

Instead, I would expose event services off the main client/session model:

```rust
let mut stream = client.events().event_source(account).open().await?;
let mut ws = client.events().websocket().open().await?;
```

Both should yield a unified high-level event model, such as:

- state changed
- push verification
- keepalive
- reconnect required
- protocol error

That would make recovery policy and stream lifecycle more explicit and more testable.

## Testing investment

With unlimited engineering resources, I would invest heavily in test structure:

- RFC fixture tests for canonical request and response payloads
- transport contract tests shared across transport adapters
- capability matrix tests
- property preservation tests for open objects
- snapshot tests for serialized JMAP requests
- integration tests against at least one real JMAP server
- fuzzing for parsing and result-reference resolution

This is what makes a large pre-1.0 redesign sustainable.

## Documentation strategy

I would document the library in three layers:

- simple tasks
  - mailbox creation
  - email get
  - calendar event create
- real JMAP usage
  - batching
  - result references
  - call IDs
  - capability negotiation
- bring-your-own-transport
  - `reqwest` adapter
  - custom transport
  - mocking and tests

The project should read as a JMAP toolkit with a strong default client, not as a pile of convenience methods over a hidden protocol core.

## Concrete end state

If I were defining the target architecture, it would be:

- async-only
- reqwest-free core
- `reqwest` kept as the default adapter, not the mandatory foundation
- no `AHashMap` in the public API
- typed IDs, state tokens, capabilities, and result references
- first-class batch API
- convenience helpers layered on top
- unified streaming/event surface
- crate-owned public error types
- adding a new method should be local to that method module plus capability registration

## Bottom line on `reqwest`

I do genuinely favor removing `reqwest` as a direct dependency of the protocol core.

I do not favor removing `reqwest` support.

My actual preference is:

- keep `reqwest`
- support it well
- probably make it the default documented transport
- stop letting it define the architecture of the crate
