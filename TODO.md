# TODO — jmap-client

Items below have been evaluated and consciously deferred. Each includes the rationale for deferral and the conditions under which it should be revisited.

---

## Bugs (low severity — not blocking 1.0)

### 1. `unimplemented!()` in `SetObject for <Get>::new()` (14 sites)

Every JMAP object type implements `SetObject` for both `Type<Set>` and `Type<Get>`. The `<Get>` impl's `new()` method contains `unimplemented!()` because Get-state objects should never be constructed through the set path — they come from server responses.

**Risk:** A downstream consumer who implements a custom type and incorrectly calls `SetObject::new()` on a `<Get>` variant will panic. The type system makes this unlikely in normal usage — `SetRequest` is generic over `<Set>` variants, not `<Get>`.

**Why deferred:** Fixing this requires redesigning the `SetObject` trait to either remove `new()` from `<Get>` impls (impossible without splitting the trait or adding negative bounds) or returning `Result` from `new()` (breaks every existing impl). The `json_object_struct!` and `impl_jmap_object!` macros generate these impls, so the surface is controlled.

**Revisit when:** If the trait system is redesigned, or if a user reports hitting this panic.

---

### 2. `session_updated` uses `Relaxed` atomic ordering

`Client` tracks whether the session needs refreshing via an `AtomicBool` with `Relaxed` ordering at `client.rs:284,297,302`. Both `send_request()` and `refresh_session()` take `&self` and can be called concurrently.

**Risk:** If thread A calls `send_request()` (which sets the flag to `false` when session state diverges) and thread B calls `refresh_session()` (which sets it to `true`) concurrently, the flag could end up in the wrong state. The consequence is a stale "needs refresh" indicator — the client might think the session is current when it isn't, or vice versa.

**Why deferred:** The flag is advisory. No data corruption or incorrect JMAP behavior results from a stale value — the worst case is an unnecessary `refresh_session()` call or a missed refresh that self-corrects on the next request. Fixing properly would require `Acquire`/`Release` ordering or folding the flag into the session mutex, both of which add complexity for a marginal correctness improvement.

**Revisit when:** If concurrent client usage becomes a primary use case, or if users report stale session state.

---

### 3. EventSource ignores `last_event_id` parameter

`event_source/stream.rs:34` accepts `_last_event_id: Option<&str>` but never uses it. Per the W3C Server-Sent Events specification, when reconnecting to an SSE endpoint, the client should send the `Last-Event-ID` HTTP header with the value of the last received event ID, allowing the server to replay missed events.

**Risk:** If the EventSource connection drops and the caller reconnects, events between the disconnect and reconnect are lost. The caller has no way to request replay from the server.

**Why deferred:** JMAP's EventSource is primarily used for state change notifications, not for reliable event delivery. Missed state changes are detected on the next successful request (the session state comparison in `send_request()` catches divergence). Full reconnection support would also need retry logic, backoff, and connection lifecycle management — a significant feature, not a bug fix.

**Revisit when:** Implementing robust EventSource reconnection, or if a user needs reliable event replay.

---

## Optimization (accepted trade-offs)

### 1. `capability_config()` serde round-trip

`Session::capability_config::<C>()` at `core/session.rs` serializes the `Capabilities` enum variant to `serde_json::Value`, then deserializes it into `C::Config`. This is two serde passes to extract a typed config that's already stored in the enum.

**Why accepted:** Session parsing happens once per connection (at connect) and capability configs are accessed infrequently. The hand-written accessors (`core_capabilities()`, `mail_capabilities()`, etc.) return `&T` with zero overhead and are the primary path. `capability_config()` is a convenience bridge for generic code. The round-trip cost is ~microseconds on a structure with a handful of fields.

**Revisit when:** If `capability_config()` is called in a hot loop, or if the hand-written accessors are removed in favor of the generic path (which would require storing raw JSON alongside the enum for direct deserialization).

---

### 2. `default_account_id().to_string()` copied in ~90 helper methods

Every convenience helper method (e.g., `email_get`, `mailbox_create`, `calendar_event_query`) calls `request.default_account_id().to_string()` to pass the account ID to method struct constructors. This allocates one `String` per helper call.

**Why accepted:** The allocation is small (account IDs are typically 10-30 bytes) and happens once per JMAP operation — dwarfed by the HTTP request cost. Fixing would require changing all method constructors to accept `&str` and store `Cow<'_, str>`, or passing the account ID by reference through the entire builder chain. That's ~90 call sites of mechanical churn for a sub-microsecond improvement.

**Revisit when:** If profiling shows helper method overhead is significant relative to network I/O, or if the `AccountScope` type grows helper methods that can pass the account ID by reference internally.

---

### 3. `body.to_vec()` copies reqwest's `Bytes` buffer

`transport_reqwest.rs` calls `.to_vec()` on the `bytes::Bytes` response from reqwest. `Bytes` is reference-counted and zero-copy from the network buffer, but `.to_vec()` copies the entire body into a new `Vec<u8>`.

**Why accepted:** Changing this requires modifying the `HttpTransport` trait to return `bytes::Bytes` instead of `Vec<u8>`, which would add `bytes` as a public API dependency of the trait (not just an internal dependency). Every custom transport implementor would need to depend on the `bytes` crate. The copy cost is O(n) in response size but is a single memcpy — fast relative to network latency and JSON parsing.

**Revisit when:** If a second transport implementation is written that also uses `Bytes` natively, making the trait change justified. Or if profiling shows the copy is significant for large blob downloads.

---

### 4. `CallHandle.call_id` is heap-allocated `String`

`CallHandle` stores `call_id: String` which is always a short string like `"s0"`, `"s1"`. This allocates 2-3 bytes on the heap per method call (plus the `String` header overhead). The call_id is also cloned into `RawMethodCall`.

**Why accepted:** Two tiny allocations per method call. A typical JMAP request has 1-5 method calls. The total overhead is ~100 bytes of heap allocation per request — noise compared to the JSON serialization and HTTP transfer. Fixing would require using `usize` internally and formatting to string only during serialization, which adds complexity to `CallHandle`, `ResultReference`, and the response lookup path.

**Revisit when:** If the crate is used in an extremely high-throughput scenario where per-request overhead matters at the sub-microsecond level.

---

### 5. `try_cap!` macro clones `serde_json::Value` before deserialization

The session capability deserializer at `core/session.rs` clones each capability's JSON `Value` before attempting `serde_json::from_value()`, because `from_value` takes ownership and the original is needed for the `Capabilities::Other(value)` fallback on parse failure.

**Why accepted:** Session parsing happens once per connection. The clone is only wasteful when deserialization succeeds (the original is discarded), and capability objects are small (typically 2-5 fields). The key-based dispatch means the typed parse almost always succeeds, so the fallback path that needs the original value is rarely taken.

**Revisit when:** Never, unless session parsing becomes a hot path (it won't — sessions are cached).

---

## Remaining specs

Implementation plans for additional JMAP specifications are in the `plans/` directory:

- **Sharing (RFC 9670)** — `plans/SHARING.md` — ShareNotification object, shared data framework
- **MDN (RFC 9007)** — `plans/MDN.md` — Read receipts (MDN/send, MDN/parse)
- **S/MIME (RFC 9219)** — `plans/SMIME.md` — Email signature verification properties and filters
