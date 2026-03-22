jmap-client 0.5.0
================================

### New protocol support
- **JMAP for Calendars** (draft-ietf-jmap-calendars-26): Calendar, CalendarEvent, CalendarEventNotification, ParticipantIdentity with full get/changes/set/query/queryChanges/copy/parse methods.
- **JMAP for Contacts** (RFC 9610): AddressBook, ContactCard with full get/changes/set/query/queryChanges/copy/parse methods. 20 query filters including `name/given`, `name/surname`, `name/surname2`.
- **JMAP Blob Management** (RFC 9404): Blob/upload (inline creation from text/base64/blob references), Blob/get (ranged retrieval with digest computation), Blob/lookup (reverse object reference lookup).
- **JMAP Quotas** (RFC 9425): Read-only Quota object with get/changes/query/queryChanges. 4 filters, 2 comparators.
- **Principal/getAvailability**: Free/busy lookup for scheduling.

### New session capabilities
- `CalendarsCapabilities` (mayCreateCalendar, maxCalendarsPerEvent)
- `ContactsCapabilities` (mayCreateAddressBook, maxAddressBooksPerCard)
- `PrincipalsCapabilities` (currentUserPrincipalId, accountIdForPrincipal)
- `BlobCapabilities` (maxSizeBlobSet, supportedDigestAlgorithms, supportedTypeNames)
- `QuotaCapabilities`
- Capabilities deserialization now dispatches on URI key (not value shape), fixing silent variant misidentification.

### Design
- CalendarEvent and ContactCard use JSON map backing (`serde_json::Map`) for round-trip fidelity with JSCalendar/JSContact extension properties. Typed accessor/builder methods wrap the map; `set_property()`/`property()` escape hatches for arbitrary properties. `Property::Other(String)` for extension property names.
- Typed container objects (Calendar, AddressBook) use traditional serde-derived structs.
- `json_object_struct!` macro generates shared boilerplate for JSON-backed types.

### Breaking changes
- `SubmissionCapabilities::submission_extensions` changed from `Vec<String>` to `AHashMap<String, Vec<String>>` (RFC 8620/8621 compliance).
- `HeaderValue` enum variant ordering changed (most-specific first) for correct untagged deserialization.
- `SingleMethodResponse` variant ordering changed (`Ok` before `Error`).
- `Response::method_response_by_pos` now returns `Option<T>` instead of panicking.
- `Request::new` no longer includes `URI::Mail` by default; mail-related helpers add it explicitly.
- `Calendar::calendar_destroy` now takes a `remove_events: bool` parameter.
- Minimum Rust edition: 2024.

### Bug fixes
- **PatchObject null semantics**: `mailbox_id(_, false)`, `keyword(_, false)`, `calendar_id(_, false)`, `address_book_id(_, false)` now serialize as `null` (not `false`) per RFC 8620. Fixes Fastmail compatibility. (stalwartlabs#18)
- **Identity capability**: Identity request builders now add `URI::Submission` to `using`.
- **Principal capability**: Principal request builders now add `URI::Principals` to `using`.
- **Parse capabilities**: CalendarEvent/parse and ContactCard/parse use separate `URI::CalendarsParse` / `URI::ContactsParse` capabilities.
- **Nullable field semantics**: CalendarEvent getters for `timeZone`, `color`, `locale`, `alerts` return `Option<Option<&T>>` distinguishing absent from null.
- **`#![forbid(unsafe_code)]`** now applies crate-wide (was scoped to one module).
- `CoreCapabilities` and `SieveCapabilities` use `#[serde(default)]` for robustness against non-compliant servers.
- `SetRequest::create/update` uses entry API (avoids clone+re-lookup).
- `ParticipantIdentity::new()` initializes `send_to` to `None` (was empty map).

### Performance
- All `Arguments` (62 variants) and `MethodResponse` (76 variants) enum payloads are now `Box`ed, reducing stack usage from hundreds of bytes to pointer-sized.
- `Property::Serialize` is zero-allocation (direct `&str` match instead of `to_string()`).
- Macros deduplicate ~1,500 lines of boilerplate in request.rs and response.rs.

### Removed
- **Blocking support removed** — the crate is now async-only. The `blocking` feature, `maybe-async` dependency, and all 185 `#[maybe_async::maybe_async]` annotations are gone. Fixes the long-standing `--all-features` compilation failure.
- **`async` feature removed** — async is no longer optional, `futures-util` and `async-stream` are unconditional dependencies.
- **`ring`/`aws_lc_rs` feature selectors removed** — `rustls` is an implementation detail of the `websockets` feature, not a user-facing TLS backend choice.

### Dependencies
- `reqwest` 0.12 -> 0.13 (feature `rustls-tls-webpki-roots` renamed to `rustls`).
- `tokio-tungstenite` 0.28 -> 0.29.
- `maybe-async` removed.

### Testing
- 57 tests (up from 4) covering serialization, deserialization, nullable semantics, property round-trips, query filters, blob wire format, session capabilities, and alert triggers.

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
