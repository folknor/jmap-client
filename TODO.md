# TODO — jmap-client pre-1.0

Findings from three parallel Opus agent reviews: code quality/bugs, optimization, and duplicated logic.

---

## Bugs & Code Quality

### HIGH

1. **`Field::Omitted` serialize panics via `unreachable!()`** — `core/field.rs:78`. If any struct field lacks `#[serde(skip_serializing_if = "Field::is_omitted")]`, serialization panics. Should return `serde::ser::Error::custom(...)` instead.

2. **`ClientBuilder::connect()` panics on missing credentials** — `client.rs:123`. `.expect("Missing credentials")` panics instead of returning `Err`. Should return a structured error.

### MEDIUM

3. **`SetErrorType`/`MethodErrorType` missing `#[serde(other)]`** — `core/set.rs:97-150`, `core/error.rs:56-99`. Unknown server error types (vendor extensions, new RFCs) cause entire response deserialization to fail. Add `Other` variant with `#[serde(other)]`.

4. **`CalendarEventCopy` response type uses `<Set>` instead of `<Get>`** — `calendar_event/mod.rs:386`. Inconsistent with `EmailCopy` which uses `<Get>`. Could cause panics via `unimplemented!()` in `SetObject::new()` for `<Get>`.

5. **`QueryResponse::id()` panics on out-of-bounds** — `core/query.rs:191`. Direct indexing `self.ids[pos]` panics. Should return `Option<&str>`.

6. **SSE parser missing newline between multi-line `data:` fields** — `event_source/parser.rs:122-124`. Per W3C SSE spec, successive `data:` lines should be joined with `\n`. Current code concatenates without separator. Not a problem for JMAP's single-line JSON events but violates the spec.

7. **`unimplemented!()` in 14 `SetObject for <Get>::new()` impls** — panic reachable through the public trait, though only via nonsensical usage. `core/mod.rs:196` and all `set.rs` files.

### LOW

8. **`unwrap()` in header construction** — `client.rs:137,142` and `client_ws.rs:219,222`. Authorization and forwarded-for headers panic if values contain non-ASCII. Practically safe but incorrect for a library.

9. **`session_updated` uses `Relaxed` ordering** — `client.rs:284,297,302`. Race between `send_request` and `refresh_session` in concurrent use. Should use `Release`/`Acquire` or fold into the mutex.

10. **Bitwise `&` instead of `&&`** — `client_ws.rs:224`. `self.accept_invalid_certs & capabilities.url().starts_with("wss")` — functionally correct for bool but looks like a typo.

11. **`event_source` ignores `last_event_id` parameter** — `event_source/stream.rs:34`. Accepted but silently unused. SSE reconnection with Last-Event-ID is unimplemented.

12. **`push_subscription_create` unwraps `create_id()`** — `push_subscription/helpers.rs:39`. Safe by construction but panics in library code.

---

## Optimization

### HIGH

1. **Double serialization in `Request::call()`** — `core/request.rs:132`. Every method is serialized to `serde_json::Value` (allocating the entire Value tree), then serialized again to JSON bytes in `send_request()`. Fix: use `serde_json::value::RawValue` or `Box<dyn erased_serde::Serialize>` to serialize once.

2. **Double deserialization + clone in Response** — `core/response.rs:116-141`. Response body parsed into `Vec<serde_json::Value>`, individual responses cloned (line 140), then deserialized again into typed structs in `Response::get()`. Fix: use `Box<RawValue>` to defer deserialization — raw JSON bytes → typed struct directly, no Value intermediary.

### MEDIUM

3. **`capability_config()` serialize/deserialize round-trip** — `core/session.rs:280-286`. Serializes `Capabilities` enum variant to Value, then deserializes into `C::Config`. Two serde passes. Hand-written accessors are zero-cost. Fix: match on enum variant directly and return reference or clone.

4. **`default_account_id().to_string()` copied ~90 times** — every helpers.rs file. One heap allocation per convenience API call. Fix: method constructors accept `&str` internally, or use `Cow<'_, str>`.

5. **`using` field allocates `String` for static URIs** — `core/request.rs:78,99,127-129`. All capability URIs are `&'static str` but stored as `String`. Fix: `Vec<&'static str>`.

6. **`body.to_vec()` copies reqwest's `Bytes` buffer** — `transport_reqwest.rs:96,101,203`. `Bytes` is reference-counted but `.to_vec()` copies. Fix: `HttpTransport` trait returns `Bytes` instead of `Vec<u8>`.

### LOW

7. **`Vec::remove()` instead of `swap_remove()`** — `core/response.rs:79`. O(n) shift vs O(1). Negligible for typical 2-5 method batches.

8. **`CallHandle.call_id` is heap `String` for "s0","s1"** — `core/request.rs:29`. Two allocations per method call. Fix: use `usize` index, format during serialization.

9. **`ResultReference.name` owns `String` for static method names** — `core/request.rs:42-43`. Fix: `&'static str`.

10. **`try_cap!` clones `Value` before deserialization attempt** — `core/session.rs:106-113`. Fix: restructure to pass by ownership, clone only for fallback.

11. **`URLPart::parse()` clones `buf` unnecessarily** — `core/session.rs:582,608`. Fix: `std::mem::take(&mut buf)`.

---

## Duplicated Logic (~550-650 lines reducible)

### HIGH

1. **Parse request/response types (3 identical files, ~120 lines)** — `email/parse.rs`, `calendar_event/parse.rs`, `contact_card/parse.rs`. Identical ParseResponse struct, identical `parsed()`/`not_parsable()`/`not_found()` methods, identical builder. Fix: generic `ParseResponse<T>` + `define_parse_method!` macro.

### MODERATE

2. **Session capability accessors (10 identical methods, ~90 lines)** — `core/session.rs:288-383`. Every accessor follows the same 5-line pattern. Fix: `session_cap_accessor!` macro, or improve `capability_config()` to avoid serde round-trip.

3. **Helper method patterns (~200-300 lines)** — 17 helpers.rs files. `*_get`, `*_destroy`, `*_changes`, `*_query` follow identical patterns. Principal has 7 "update single field" methods with identical 6-line bodies. Fix: `define_client_helpers!` macro or generic helper function.

4. **Property enum serde boilerplate (~80 lines)** — `calendar_event/mod.rs:252-374`, `contact_card/mod.rs:46-144`. CalendarEvent::Property and ContactCard::Property have identical 6-component serde pattern (as_str, Display, Serialize, Visitor, Deserialize, From). Fix: `define_property_enum!` macro.

### LOW

5. **GetObject dual impls (~28 lines)** — 14 `get.rs` files each impl GetObject for both `<Set>` and `<Get>` identically. Fix: fold into existing macro.

6. **Mailbox manual Object impls (~10 lines)** — `mailbox/mod.rs:235-251`. Kept manual because of custom ChangesResponse. Fix: extend `impl_jmap_object!` with optional ChangesResponse parameter.

7. **json_object_struct common accessors (~20 lines)** — `calendar_event/get.rs:17-29`, `contact_card/get.rs:17-29`. Identical `id()`, `take_id()`, `uid()`, `property()`, `as_properties()`. Fix: `JsonObject` trait with default methods.

---

## Remaining specs

### Sharing (RFC 9670) — see SHARING.md
- ShareNotification/get, /changes, /set (destroy only), /query, /queryChanges

### MDN (RFC 9007) — see MDN.md
- MDN/send, MDN/parse

### S/MIME (RFC 9219) — see SMIME.md
- Additional Email/get properties and query filters
