# TODO — jmap-client remaining work

## Bugs & Code Quality

### MEDIUM

1. **`SubmissionCapabilities` wrong type (upstream)** — `email/mod.rs`: `submission_extensions` is now `AHashMap<String, Vec<String>>` (fixed), but `MailCapabilities` fields still lack `#[serde(default)]` for robustness. Consider adding defaults to all mail capability fields.

2. **`unreachable!()` panics in `*_mut()` accessors** — `core/request.rs`: macro-generated accessor methods panic with unhelpful message if called on wrong variant. Could provide descriptive error, but would need an error type change.

### LOW

3. **`string_not_set`/`list_not_set`/`map_not_set` naming** — `core/set.rs`: function names suggest the opposite of what they do. Documentation or rename would help.

---

## Dedup (remaining)

1. **`*/helpers.rs` Request impl methods (~15 modules)** — near-identical `get_*`, `set_*`, `changes_*`, `query_*` methods. Significant variation between modules makes macroization complex but possible.

2. **Object/ChangesObject trait impls** — repeated in all typed-struct modules (calendar, address_book, mailbox, identity, etc.). A macro could eliminate ~10 lines per entity.

---

## Optimization (remaining)

1. **`account_id.clone()` on every `params()` call** — `core/request.rs`: clones String per method call. Could use `Cow<'a, str>` or borrow, but needs lifetime changes across the Request API.

2. **Feature-gating `#[cfg]`** — features are defined (mail, calendars, contacts, blob, quota) but not yet wired into conditional compilation. The macro-generated request/response plumbing needs per-variant `#[cfg]` support. Significant refactor.

---

## Remaining specs

### Sharing (RFC 9670) — see SHARING.md
- ShareNotification/get, /changes, /set (destroy only), /query, /queryChanges
- Shared data framework (isSubscribed, myRights, shareWith)

### MDN (RFC 9007) — see MDN.md
- MDN/send, MDN/parse
- Read receipts over JMAP

### S/MIME (RFC 9219) — see SMIME.md
- Additional Email/get properties and query filters
- Server-side verification, low client effort

---

## Test coverage gaps

1. **AlertTrigger::UnknownTrigger is not a catch-all** — serde `#[serde(tag = "@type")]` only matches literal `"UnknownTrigger"`, not arbitrary types. Future trigger types will fail deserialization. Consider using `#[serde(other)]` or a fallback mechanism.
