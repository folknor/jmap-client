# Architecture Redesign Plan v2

## Design decisions (settled)

1. **Typed `CallHandle<M>`** — compile-time safe response extraction
2. **Closure-based deserialization** — captured at call-time, downcast at extraction
3. **Method structs compose generic request types** — `EmailGet` wraps `GetRequest<Email<Set>>`, not duplicates fields
4. **Method enum dies completely** — wire names come from `JmapMethod::NAME`
5. **Transport + error abstraction in Phase 1** — not deferred
6. **Capability uses a trait** — not `&'static str`
7. **Get/Set phantom types survive** — `JmapMethod::Response = GetResponse<Email<Get>>`
8. **`call()` returns `Result<CallHandle<M>>`** — no panics
9. **maybe_async already removed** — done in earlier commit

## Core types

### Capability trait

```rust
pub trait Capability {
    /// Wire URI, e.g. "urn:ietf:params:jmap:mail"
    const URI: &'static str;
}

pub struct Mail;
impl Capability for Mail {
    const URI: &'static str = "urn:ietf:params:jmap:mail";
}

pub struct Calendars;
impl Capability for Calendars {
    const URI: &'static str = "urn:ietf:params:jmap:calendars";
}
// etc.
```

### JmapMethod trait

```rust
pub trait JmapMethod: Serialize + Send {
    const NAME: &'static str;
    type Capability: Capability;
    type Response: DeserializeOwned + Send;
}
```

### Method structs compose existing generics

```rust
// EmailGet wraps the existing GetRequest machinery
pub struct EmailGet {
    #[serde(flatten)]
    inner: GetRequest<Email<Set>>,
}

impl JmapMethod for EmailGet {
    const NAME: &'static str = "Email/get";
    type Capability = Mail;
    type Response = GetResponse<Email<Get>>;
}

impl EmailGet {
    pub fn new(account_id: &str) -> Self {
        // Delegates to GetRequest::new() internally
    }

    // Builder methods delegate to inner
    pub fn ids<I, V>(&mut self, ids: I) -> &mut Self { ... }
    pub fn properties(&mut self, props: impl IntoIterator<Item = email::Property>) -> &mut Self { ... }
}
```

This preserves the existing GetRequest/SetRequest/QueryRequest/ChangesRequest generics and their builder methods. Method structs are thin typed wrappers.

### Typed CallHandle

```rust
pub struct CallHandle<M: JmapMethod> {
    call_id: String,
    method_name: &'static str, // M::NAME, for result references
    _phantom: PhantomData<M>,
}

impl<M: JmapMethod> CallHandle<M> {
    /// Create a result reference pointing to this call's response.
    pub fn result_reference(&self, path: impl Into<String>) -> ResultReference {
        ResultReference {
            result_of: self.call_id.clone(),
            name: self.method_name.to_string(),
            path: path.into(),
        }
    }
}
```

### ResultReference (no Method enum)

```rust
pub struct ResultReference {
    #[serde(rename = "resultOf")]
    result_of: String,
    name: String, // was Method enum, now just a string
    path: String,
}
```

### Request builder

```rust
pub struct Request<'x> {
    client: &'x Client,
    account_id: String,
    using: Vec<String>,       // was Vec<URI>, now just strings
    method_calls: Vec<RawMethodCall>,
    deserializers: Vec<DeserializerFn>,
}

// Type-erased storage for a single method call
struct RawMethodCall {
    name: &'static str,
    arguments: serde_json::Value,
    call_id: String,
}

// Type-erased deserializer
type DeserializerFn = Box<dyn FnOnce(serde_json::Value) -> Result<Box<dyn Any + Send>, serde_json::Error> + Send>;

impl<'x> Request<'x> {
    pub fn call<M: JmapMethod>(&mut self, method: M) -> Result<CallHandle<M>> {
        let call_id = format!("s{}", self.method_calls.len());

        // Auto-add capability
        let uri = M::Capability::URI;
        if !self.using.iter().any(|u| u == uri) {
            self.using.push(uri.to_string());
        }

        // Serialize method arguments
        let arguments = serde_json::to_value(&method)?;

        self.method_calls.push(RawMethodCall {
            name: M::NAME,
            arguments,
            call_id: call_id.clone(),
        });

        // Capture type-erased deserializer
        self.deserializers.push(Box::new(|value| {
            Ok(Box::new(serde_json::from_value::<M::Response>(value)?)
                as Box<dyn Any + Send>)
        }));

        Ok(CallHandle {
            call_id,
            method_name: M::NAME,
            _phantom: PhantomData,
        })
    }
}
```

Serialization: `method_calls` serializes as `[["Email/get", {...}, "s0"], ...]` — just arrays of [name_string, arguments_value, call_id_string]. No Method enum needed.

### Response handling with method-error discrimination

```rust
enum RawCallResult {
    Success(serde_json::Value),
    Error(MethodError),
}

pub struct Response {
    results: Vec<(String, RawCallResult)>,   // (call_id, result)
    deserialized: HashMap<String, Box<dyn Any + Send>>,
    session_state: String,
}

impl Response {
    /// Extract a typed response. Compile-time safe via CallHandle<M>.
    pub fn get<M: JmapMethod>(&mut self, handle: &CallHandle<M>) -> Result<M::Response> {
        // Check if already deserialized (from the stored closure)
        if let Some(boxed) = self.deserialized.remove(&handle.call_id) {
            return boxed.downcast::<M::Response>()
                .map(|b| *b)
                .map_err(|_| Error::TypeMismatch);
        }

        // Find raw result
        let (_, result) = self.results.iter()
            .find(|(id, _)| id == &handle.call_id)
            .ok_or(Error::CallNotFound)?;

        match result {
            RawCallResult::Success(value) => {
                Ok(serde_json::from_value(value.clone())?)
            }
            RawCallResult::Error(e) => {
                Err(Error::Method(e.clone()))
            }
        }
    }
}
```

### Transport trait

```rust
pub trait Transport: Send + Sync {
    /// Send a JMAP API request (method calls).
    fn api_request(
        &self,
        url: &str,
        body: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;

    /// Upload a blob (multipart).
    fn upload(
        &self,
        url: &str,
        body: Vec<u8>,
        content_type: Option<&str>,
    ) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;

    /// Download a blob.
    fn download(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;
}

pub struct TransportError {
    pub status: Option<u16>,
    pub message: String,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}
```

WebSocket and EventSource are separate optional traits, not part of the core Transport.

### Structured errors (crate-owned)

```rust
pub enum Error {
    /// Transport-level failure (network, TLS, timeout).
    Transport(TransportError),
    /// JSON deserialization failure.
    Deserialize(serde_json::Error),
    /// Server returned a problem details response (RFC 7807).
    Problem(ProblemDetails),
    /// A JMAP method call returned an error response.
    Method(MethodError),
    /// A JMAP set operation returned per-object errors.
    Set(SetError<String>),
    /// The requested call ID was not found in the response.
    CallNotFound,
    /// Response type did not match the expected type.
    TypeMismatch,
}
```

No `reqwest::Error` in the public API. `TransportError` wraps it internally.

### Client construction

```rust
pub struct Client<T: Transport = ReqwestTransport> {
    transport: T,
    session: std::sync::Mutex<Arc<Session>>,
    // ...
}

// Default ergonomic path
let client = Client::connect("https://jmap.example.org")
    .credentials(("user", "pass"))
    .await?;

// Custom transport
let client = Client::with_transport(my_transport, session);
```

## User-facing API (end state)

### Batch API (first-class)

```rust
let mut request = client.request();
let query = request.call(EmailQuery::new(&account_id)
    .filter(Filter::and([
        email::query::Filter::subject("test"),
        email::query::Filter::in_mailbox(&mailbox_id),
    ]))
    .sort([email::query::Comparator::received_at()]))?;

let get = request.call(EmailGet::new(&account_id)
    .ids_ref(query.result_reference("/ids"))
    .properties([email::Property::Subject, email::Property::Preview]))?;

let mut response = request.send().await?;

let query_result = response.get(&query)?;
let emails = response.get(&get)?;
```

### Convenience helpers (sugar on top)

```rust
// These still exist, layered on the batch API
let email = client.email_get(&account_id, &id, None).await?;
let quotas = client.quota_get_all(&account_id).await?;
```

## Phases

### Phase 1: Foundation
- `Capability` trait + implementations for all URIs
- `JmapMethod` trait
- `Transport` trait + `ReqwestTransport`
- `TransportError` + crate-owned `Error` enum
- New `Request` with `call<M>()` + typed `CallHandle<M>`
- New `Response` with `get<M>()` + method-error handling
- New `ResultReference` (string-based, no Method enum)
- Wire serialization as `[[name, args, id], ...]`

### Phase 2: Migrate method types
- Each module gets method structs (EmailGet, EmailSet, EmailQuery, etc.)
- Each struct wraps existing GetRequest/SetRequest/QueryRequest generics via `#[serde(flatten)]`
- Old helpers rewritten to use `request.call()`
- Old Arguments/MethodResponse/Method enums deleted

### Phase 3: Cleanup
- Remove old macro infrastructure
- Feature-gate per RFC (trivial now — no central enums)
- Update tests and examples

## What survives from current codebase

- All object type definitions (Email, Mailbox, Calendar, CalendarEvent, ContactCard, etc.)
- Get/Set phantom type pattern
- GetRequest/SetRequest/QueryRequest/ChangesRequest generics and their builder methods
- Filter/Comparator types
- Property enums
- Session and capability structs
- All serde annotations on data types
- The json_object_struct! macro for CalendarEvent/ContactCard
- Tests (updated to use new API)
