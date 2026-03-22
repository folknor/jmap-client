# TODO — jmap-client pre-1.0

## Bugs (remaining)

### LOW

1. **`unimplemented!()` in 14 `SetObject for <Get>::new()` impls** — panic reachable through public trait via nonsensical usage. Would need trait redesign to eliminate. Low risk — type system makes it unlikely to hit.

2. **`session_updated` uses `Relaxed` ordering** — `client.rs:284,297,302`. Race between `send_request` and `refresh_session` in concurrent use. Consequence is a stale "needs refresh" flag, not data corruption. Fix: `Release`/`Acquire` or fold into mutex.

3. **`event_source` ignores `last_event_id` parameter** — `event_source/stream.rs:34`. SSE reconnection with Last-Event-ID is unimplemented. Documented by underscore prefix.

---

## Optimization (remaining)

### MEDIUM

1. **`capability_config()` serialize/deserialize round-trip** — `core/session.rs:280-286`. Accepted as bridge — hand-written accessors are zero-cost primary path. Session parsing is infrequent.

2. **`default_account_id().to_string()` copied ~90 times** — every helpers.rs. One heap allocation per convenience API call. Fix: method constructors accept `&str` internally, or use `Cow<'_, str>`.

3. **`body.to_vec()` copies reqwest's `Bytes` buffer** — `transport_reqwest.rs`. `Bytes` is reference-counted but `.to_vec()` copies. Fix: `HttpTransport` trait returns `Bytes` instead of `Vec<u8>`. Breaking trait change.

### LOW

4. **`CallHandle.call_id` is heap `String` for "s0","s1"** — `core/request.rs:29`. Two allocations per method call. Fix: use `usize` index, format during serialization.

5. **`try_cap!` clones `Value` before deserialization attempt** — `core/session.rs:106-113`. Accepted — session parsing is infrequent.

---

## Duplication (~550 lines reducible)

### HIGH

1. **Parse request/response types (3 identical files, ~120 lines)** — `email/parse.rs`, `calendar_event/parse.rs`, `contact_card/parse.rs`. Fix: generic `ParseResponse<T>` + `define_parse_method!` macro.

### MODERATE

2. **Session capability accessors (10 identical methods, ~90 lines)** — `core/session.rs:288-383`. Fix: `session_cap_accessor!` macro.

3. **Helper method patterns (~200-300 lines)** — 17 helpers.rs files. `*_get`, `*_destroy`, `*_changes`, `*_query` follow identical patterns. Fix: `define_client_helpers!` macro.

4. **Property enum serde boilerplate (~80 lines)** — `calendar_event/mod.rs`, `contact_card/mod.rs`. Fix: `define_property_enum!` macro.

### LOW

5. **GetObject dual impls (~28 lines)** — 14 `get.rs` files. Fix: fold into existing macro.

6. **Mailbox manual Object impls (~10 lines)** — custom ChangesResponse. Fix: extend `impl_jmap_object!`.

7. **json_object_struct common accessors (~20 lines)** — Fix: `JsonObject` trait with default methods.

---

## Remaining specs

See `plans/` directory:
- Sharing (RFC 9670) — `plans/SHARING.md`
- MDN (RFC 9007) — `plans/MDN.md`
- S/MIME (RFC 9219) — `plans/SMIME.md`
