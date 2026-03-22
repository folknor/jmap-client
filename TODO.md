# TODO — jmap-client remaining work

## Dedup

1. **`*/helpers.rs` Request impl methods (~15 modules)** — near-identical `get_*`, `set_*`, `changes_*`, `query_*` methods. Significant variation between modules makes macroization complex but possible.

## Optimization

1. **Feature-gating `#[cfg]`** — features are defined (mail, calendars, contacts, blob, quota) but not yet wired into conditional compilation. The macro-generated request/response plumbing needs per-variant `#[cfg]` support.

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
