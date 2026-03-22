jmap-client 0.5.1
================================

### Breaking changes (vs 0.5.0)
- **`SetObject` trait split** — `new()` moved to new `SetObjectCreatable` supertrait. `SetRequest` and `CopyRequest` now bound on `SetObjectCreatable`; `SetResponse` and `CopyResponse` remain bound on `SetObject`. `<Get>` types no longer implement `new()` — eliminates 15 `unimplemented!()` panics. Custom types implementing `SetObject` must add a separate `impl SetObjectCreatable` for the `<Set>` variant.
- **`HttpTransport` returns `bytes::Bytes`** instead of `Vec<u8>`. Custom transport implementations must update return types. `Client::download()` now returns `Bytes`. `TransportError::body` is now `Option<Bytes>`. `bytes::Bytes` is re-exported as `jmap_client::Bytes`.
- **`SseTransport::open_sse`** now takes `last_event_id: Option<&str>` parameter. Custom SSE transport implementations must update their signature.

### Bug fixes
- `session_updated` atomic flag now uses `Acquire`/`Release` ordering instead of `Relaxed`.
- `SseTransport::open_sse` passes `last_event_id` as `Last-Event-ID` header (was ignored).
- `try_cap!` macro no longer clones `serde_json::Value` before deserialization.
- `Client::authorization()` accessor exposed for `websockets` feature.

### Testing
- 66 tests (up from 65): new `SetResponse<TestObj<Get>>` deserialization test confirms trait split correctness.

jmap-client 0.5.0
================================

### Architecture redesign
- **Trait-based method dispatch** — `JmapMethod` trait replaces the `Arguments` (62 variants), `MethodResponse` (76 variants), and `Method` (60+ variants) god enums. Adding a new JMAP method touches only its own module — zero central files.
- **Typed `CallHandle<M>`** — compile-time safe response extraction via `Response::get(&handle)`.
- **`Client<T: HttpTransport>`** — generic over transport. `ReqwestTransport` as default with pooled `reqwest::Client`. `Client::with_transport()` for custom/mock transports.
- **`SseTransport` trait** — EventSource abstracted behind transport boundary.
- **`AccountScope<'a, Tr>`** — account-scoped client view for request batching.
- **`Capability` trait** — typed capability markers with associated `Config` type. `Session::capability_config::<Mail>()`.
- **`Field<T>`** — three-state nullable: `Omitted`/`Null`/`Value(T)`, replacing `Option<Option<T>>`.
- **Typed IDs** — `Id<T>`, `AccountId`, `BlobId`, `State` newtypes available for incremental adoption.
- **Structured errors** — `Error::Internal(String)` eliminated. All variants matchable: `CallNotFound`, `IdNotFound`, `EmptyResponse`, `NotParsable`, `InvalidUrl`, `WebSocketNotConnected`.
- **Feature gating per RFC** — `mail`, `calendars`, `contacts`, `blob`, `quota` features gate modules, `DataType` variants, `Capabilities` variants, session accessors, tests, and examples independently.

### New protocol support
- **JMAP for Calendars** (draft-ietf-jmap-calendars-26): Calendar, CalendarEvent, CalendarEventNotification, ParticipantIdentity.
- **JMAP for Contacts** (RFC 9610): AddressBook, ContactCard with 20 query filters.
- **JMAP Blob Management** (RFC 9404): Blob/upload, Blob/get, Blob/lookup.
- **JMAP Quotas** (RFC 9425): Read-only Quota with get/changes/query/queryChanges.
- **Principal/getAvailability**: Free/busy lookup for scheduling.

### Breaking changes (vs 0.4.0)
- `Method`, `Arguments`, `MethodResponse`, `URI` enums deleted — use `JmapMethod` trait + `Request::call()`.
- `Client` is now `Client<T: HttpTransport = ReqwestTransport>`.
- `Request` is now `Request<'x, T: HttpTransport>`.
- `Response` is a new type with `Response::get<M>(&handle)` — no more `unwrap_get_email()` etc.
- `Error` enum restructured — `Internal(String)` removed, 6 structured variants added.
- `MethodError::error()` renamed to `error_type()`. `SetError::error()` renamed to `error_type()`.
- `Field<T>` replaces `Option<Option<T>>` for nullable properties.
- `Account` renamed to `AccountScope`. `client.account()` renamed to `client.account_scope()`.
- `typed_capability()` renamed to `capability_config()`.
- Blocking support, `maybe_async`, `ahash`, `parking_lot` all removed.
- `base64` 0.13 → 0.22, `reqwest` 0.12 → 0.13, edition 2024.
- All public enums are `#[non_exhaustive]`.
- WebSocket implementation types (`WebSocketResponse`, `WebSocketError`, etc.) now `pub(crate)`.
- `download_url()`, `upload_url()`, `event_source_url()` now `pub(crate)`.
- `ProblemDetails.status`, `MethodError.p_type`, `SetError.type_` now private (getters exist).

### Testing
- 65 tests (up from 4) including orchestration tests for typed handle extraction, method error discrimination, mixed success/error batches, and capability deserialization.

jmap-client 0.4.0
================================
- Calendar Alerts support.

jmap-client 0.3.3
================================
- JMAP for Sharing support.

jmap-client 0.3.2
================================
- Bump to `rustls` 0.22.

jmap-client 0.3.0
================================
- JMAP for Sieve Scripts DRAFT-14 support.
- Set timeouts using `Duration` instead of `u64`.
- SetError handling of unknown properties.
- Support deserializing non-IANA registered roles.

jmap-client 0.2.1
================================
- Using maybe_async to reduce duplicate code.
- Added `accept_invalid_certs` option to builder.

jmap-client 0.2.0
================================
- JMAP for Sieve Scripts support.

jmap-client 0.1.0
================================
- First release.
