# CLAUDE.md

## Agent rules

- Always launch subagents in the **foreground** (never use `run_in_background`). Background agents cannot get tool approvals.

## Project

Fork of stalwartlabs/jmap-client (Apache-2.0 / MIT), maintained at folknor/jmap-client. Typed Rust bindings for the JMAP protocol.

## Build & test

```bash
cargo build          # default features (async)
cargo test           # default features — 4 tests
# cargo test --all-features  # broken upstream (blocking+async cfg conflict)
```

## Architecture

### Module pattern

Every JMAP object type follows the same structure under `src/<type>/`:
- `mod.rs` — struct with `<State = Get>` phantom type, Property enum, Object/ChangesObject impls
- `get.rs` — getters on `T<Get>`, GetObject impl
- `set.rs` — builder methods on `T<Set>`, SetObject impl
- `query.rs` — Filter enum (untagged), Comparator enum (tag = "property"), QueryObject impl
- `helpers.rs` — `impl Client` convenience methods + `impl Request<'_>` builder methods

### Two data models

**Typed structs** (Mailbox, Calendar, AddressBook, Identity, etc.): fixed fields, serde derive, `Option<T>` for optional. Good for container objects with stable schemas.

**JSON map backing** (CalendarEvent, ContactCard): `serde_json::Map<String, Value>` with custom Serialize/Deserialize. Typed accessor/builder methods wrap the map. Property enum has `Other(String)` with custom serde. Use this for JSCalendar/JSContact objects where the property set is open-ended.

### Wiring a new type

Adding a new JMAP object type requires changes in:
1. `src/lib.rs` — module declaration, Method enum variants, URI if new capability
2. `src/core/request.rs` — Arguments enum variant, constructor, `_mut` accessor
3. `src/core/response.rs` — response type alias, MethodResponse variant, is_type arm, unwrap method, visitor deserialization arm
4. `src/core/session.rs` — Capabilities struct + `deserialize_capabilities_map` dispatch entry + Session accessor (if new capability)

### Capabilities deserialization

`Capabilities` enum uses a custom `deserialize_capabilities_map` that dispatches on the URI key string, NOT `#[serde(untagged)]`. This is critical — untagged would break because several capability structs accept any JSON object. When adding a new capability, add a match arm in the deserializer.

### Nullable field semantics

For properties that are nullable in the spec (e.g. timeZone, color), use `Option<Option<T>>` in typed structs and `Option<Option<&T>>` return types in getters. This distinguishes absent (outer None) from explicitly null (Some(None)) from has-value (Some(Some(v))). For JSON-map-backed types, the getter should check `v.is_null()`.

### PatchObject null semantics

RFC 8620 PatchObject uses `null` (not `false`) to remove map keys. The `patch` field on `Email<Set>` uses `AHashMap<String, serde_json::Value>` and inserts `Value::Null` for removals.

## Code style

- Same license header on all files
- `#[maybe_async::maybe_async]` on all async methods
- Request builder methods call `self.add_capability(URI::X)` for the relevant capability
- Parse methods use separate `URI::XParse` capability (e.g. `URI::CalendarsParse`, `URI::ContactsParse`)
- `#[serde(skip_serializing_if = "...")]` on all optional fields
- SetObject::new() initializes optional fields to None, not empty collections
- Don't commit .md reference docs (CALENDARS.md, CONTACTS.md, etc.)
