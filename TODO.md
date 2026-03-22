# TODO — jmap-client

Items below have been evaluated and consciously deferred. Each includes the rationale for deferral and the conditions under which it should be revisited.

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

### 3. `CallHandle.call_id` is heap-allocated `String`

`CallHandle` stores `call_id: String` which is always a short string like `"s0"`, `"s1"`. This allocates 2-3 bytes on the heap per method call (plus the `String` header overhead). The call_id is also cloned into `RawMethodCall`.

**Why accepted:** Two tiny allocations per method call. A typical JMAP request has 1-5 method calls. The total overhead is ~100 bytes of heap allocation per request — noise compared to the JSON serialization and HTTP transfer. Fixing would require using `usize` internally and formatting to string only during serialization, which adds complexity to `CallHandle`, `ResultReference`, and the response lookup path.

**Revisit when:** If the crate is used in an extremely high-throughput scenario where per-request overhead matters at the sub-microsecond level.

---

### 4. SSE stream copies `Bytes` to `Vec<u8>` at parser boundary

`SseTransport::ByteStream` yields `Vec<u8>` chunks. `ReqwestByteStream` converts reqwest's `bytes::Bytes` to `Vec<u8>` via `.to_vec()`, and the SSE parser (`event_source/parser.rs`) stores owned `Vec<u8>` internally. While `HttpTransport` now returns `Bytes` directly, the SSE path still copies because the parser iterates byte-by-byte over an owned buffer.

**Why accepted:** True zero-copy SSE would require rewriting the parser to use `Bytes` with an offset cursor instead of consuming `Vec<u8>`. The copy cost per SSE chunk is small (chunks are typically short JSON state-change notifications, not large payloads), and SSE is a low-throughput notification channel, not a bulk data path.

**Revisit when:** If SSE is used for high-volume event delivery, or if the parser is rewritten for other reasons.

---

## Remaining specs

Implementation plans for additional JMAP specifications are in the `plans/` directory:

- **MDN (RFC 9007)** — `plans/MDN.md` — Read receipts (MDN/send, MDN/parse)
- **S/MIME (RFC 9219)** — `plans/SMIME.md` — Email signature verification properties and filters
