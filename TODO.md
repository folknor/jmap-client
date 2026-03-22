# TODO — jmap-client audit findings

Results from four parallel Opus agent reviews: code quality/bugs, duplicated logic, optimization, and missing test coverage.

---

## Bugs & Code Quality

### HIGH

1. **`#[forbid(unsafe_code)]` scoped wrong** — `lib.rs:183`: attribute only applies to `address_book` module, not the crate. Should be `#![forbid(unsafe_code)]` inner attribute at top of lib.rs.

2. **Principal methods missing `URI::Principals` capability** — `principal/helpers.rs:297-361`: `get_principal`, `changes_principal`, `query_principal`, `query_principal_changes`, `set_principal` don't call `add_capability(URI::Principals)`. Only `get_availability_principal` does. Spec-compliant servers will reject these requests.

3. **`SubmissionCapabilities::submission_extensions` wrong type** — `email/mod.rs:742-749`: typed as `Vec<String>` but RFC 8620/8621 specifies `String[String[]]` (a map of extension name to list of arguments). Should be `AHashMap<String, Vec<String>>`. Deserialization fails with spec-compliant servers.

4. **`HeaderValue` untagged enum variant ordering** — `email/mod.rs:329-341`: serde tries variants in declaration order. `AsText(String)` matches before `AsTextAll(Vec<String>)` for single strings; `AsAddressesAll(Vec<Vec<EmailAddress>>)` matches before `AsAddresses(Vec<EmailAddress>)` for single-element arrays. Can cause incorrect header value type inference.

### MEDIUM

5. **`copy_blob` helper has misleading `#[maybe_async::maybe_async]`** — `blob/helpers.rs:59-60`: annotated but not async, just returns `&mut CopyBlobRequest`. Harmless but confusing.

6. **62 `unreachable!()` panics in public `*_mut()` accessors** — `core/request.rs:422-875`: every `*_mut()` method panics with unhelpful message if called on wrong variant. Should be descriptive errors.

7. **`string_not_set`/`list_not_set`/`map_not_set` naming is confusing** — `core/set.rs:444-458`: returns `true` for `Some(empty)` (skip) and `false` for `None` (serialize as null). Correct for JMAP semantics but naming implies the opposite.

8. **`method_response_by_pos` panics on out-of-bounds** — `core/response.rs:85-87`: calls `Vec::remove(index)` which panics. Public API should return `Option<T>`.

### LOW

9. **`Request::new` always includes `URI::Mail`** — `core/request.rs:878-886`: calendar/contacts/blob-only requests unnecessarily include `urn:ietf:params:jmap:mail`.

10. **`SingleMethodResponse` untagged enum `Error` before `Ok`** — `core/response.rs:114-119`: fragile ordering with untagged enums, though unlikely to cause issues in practice.

11. **No `#[serde(default)]` on `CoreCapabilities`/`MailCapabilities` fields** — `core/session.rs:137-162`, `email/mod.rs:721-740`: if server omits any field, entire session deserialization fails. Fragile against non-fully-compliant servers.

12. **`SieveCapabilities.extensions` missing `#[serde(default)]`** — `core/session.rs:184-185`: `Vec<String>` without default; omission by server fails deserialization.

---

## Duplicated Logic (~2,100 reducible lines)

### Highest impact — macro candidates

1. **`response.rs` unwrap_* methods (77 methods, ~450 lines)** — identical 3-line match pattern. Single `macro_rules!` invocation per method.

2. **`request.rs` Arguments constructors + `_mut` accessors (99 methods, ~550 lines)** — mechanical boilerplate. Two macros could generate all of them.

3. **`response.rs` visitor `visit_seq` match (75 arms, ~260 lines)** — every arm is `Method::X => MethodResponse::X(seq.next_element()?.ok_or_else(...))`.

4. **`response.rs` `is_type` match (75 arms, ~150 lines)** — maps `(MethodResponse::X, Method::X)` pairs. Same data as visitor.

5. **`response.rs` type aliases (47 aliases, ~30 lines)** — `type XGetResponse = GetResponse<X<Get>>` etc.

6. **`*/helpers.rs` Request impl methods (15 modules, ~400 lines)** — near-identical `get_*`, `set_*`, `changes_*`, `query_*` methods differing only in type names and capability.

### Medium impact

7. **CalendarEvent/ContactCard identical Serialize/Deserialize impls (~70 lines)** — same struct shape, same custom serde. Macro `json_backed_entity!`.

8. **CalendarEvent/ContactCard identical parse.rs files (~100 lines)** — same request/response pattern. Generic `ParseRequest<T>` or macro.

9. **CalendarEvent/ContactCard identical SetObject impls (~20 lines)**.

10. **Object/ChangesObject trait impls repeated in all modules (~100+ lines)** — boilerplate `requires_account_id() -> true` etc.

### Suggested macro structure

- `jmap_entity!` — generates findings 1-6 and 10 from entity name + capability + operations
- `json_backed_entity!` — generates findings 7-9 for CalendarEvent/ContactCard family

---

## Optimization Opportunities

### HIGH

1. **Box `Arguments` enum variants** — `request.rs:78-143`: 48 variants unboxed, every value sized to largest (hundreds of bytes of padding). Box all or large variants.

2. **Box `MethodResponse` enum variants** — `response.rs:182-264`: 65+ variants, same problem. Every `TaggedMethodResponse` in response vec wastes space.

### MEDIUM

3. **`account_id.clone()` on every `params()` call** — `request.rs:921`: clones String per method call. Use `Cow<'a, str>` or borrow.

4. **`Property::Serialize` allocates via `to_string()`** — `calendar_event/mod.rs:349-353`, `contact_card/mod.rs:147-151`: `Display` formatting allocates a temporary String. Direct `&str` match in serialize would be zero-alloc.

5. **CalendarEvent/ContactCard double-serialization through `serde_json::Value`** — every setter creates Value objects, then custom Serialize iterates the map again. Somewhat unavoidable for extension properties.

6. **Compile time: untagged 48-variant enum + monomorphization** — large generated code from `#[serde(untagged)]` on Arguments. Consider feature-gating calendar/contacts.

### LOW

7. **`SetRequest::create/update` clone+re-lookup** — `core/set.rs:179-214`: should use entry API.

8. **`Vec<URI>` linear scan for capabilities** — `request.rs:933-937`: O(n) contains check per builder call. Could be `u16` bitfield.

9. **`format!("s{}"/"c{}")` for short IDs** — minor allocations, could use stack-allocated formatter.

10. **`Vec::remove` vs `swap_remove`** — `response.rs:85-87`: O(n) shift vs O(1) if order doesn't matter.

---

## Missing Test Coverage (prioritized by bug risk)

### CRITICAL

1. **`calendar_id(_, false)` and `address_book_id(_, false)` send `false` not `null`** — `calendar_event/set.rs:49`, `contact_card/set.rs:49`: same PatchObject bug that was fixed for Email in PR #18. `json!(set)` produces `false` instead of `Value::Null`. Servers will reject or silently ignore.

### HIGH

2. **CalendarEvent nullable getter semantics** — `calendar_event/get.rs:75-187`: `time_zone()`, `color()`, `locale()`, `alerts()` return `Option<Option<&T>>` but non-null non-matching types (e.g. number for string field) silently collapse to `Some(None)`, indistinguishable from explicit null. Test with absent/null/present/wrong-type JSON.

3. **Capabilities deserializer fallback destroys data** — `core/session.rs:100-101`: `unwrap_or_else(|_| Capabilities::Other(JsonValue::Null))` — the original `value` was moved into `serde_json::from_value()`, so fallback loses it. Should preserve original value on parse failure.

4. **Query filter serialization** — all `*/query.rs`: zero tests for any filter. Especially `name/given`, `name/surname`, `name/surname2` slash-containing ContactCard filters need verification.

5. **CalendarEvent/ContactCard round-trip with extension properties** — `calendar_event/mod.rs:58-94`, `contact_card/mod.rs:58-94`: no test that vendor extensions survive deserialize → reserialize.

### MEDIUM

6. **Property enum round-trip** — all known variants of CalendarEvent::Property and ContactCard::Property. A typo in Display/From match would silently drop properties.

7. **Calendar `Option<Option<T>>` serialization** — `calendar/mod.rs:44-82`: verify `Some(None)` serializes as `null` (not absent). Critical for clearing nullable fields.

8. **Blob DataSource untagged enum** — `blob/manage.rs:48-57`: colon-containing field names (`data:asText`, `data:asBase64`) need wire format verification.

9. **`AlertTrigger::UnknownTrigger` is NOT a catch-all** — `calendar_event/mod.rs:125-144`: serde `#[serde(tag = "@type")]` only matches literal `"UnknownTrigger"`, not arbitrary types. Future/unknown trigger types will fail deserialization entirely.

10. **Session deserialization with all new capability types** — existing `test_deserialize` only covers Email/Thread responses. No test for Quota, Blob, Calendars, Contacts, Principals capability parsing.

---

## Remaining spec work (from ROADMAP.md and other .md files)

- SHARING.md — not yet read/implemented
- MDN.md — not yet read/implemented
- SMIME.md — not yet read/implemented
- ROADMAP.md — not yet read
