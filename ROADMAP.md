# jmap-client fork roadmap

Fork: `folknor/jmap-client` (Apache-2.0 / MIT)
Upstream: `stalwartlabs/jmap-client` (unmaintained — maintainer confirmed in discussion #15)

## Why this fork exists

The upstream jmap-client is the most complete JMAP client library in Rust, but it only covers email-related specs. Stalwart Labs is focused on their AGPL server and has deprioritized the client library. Multiple consumers need calendar, contacts, and blob support:

- **Ratatoskr** — desktop email client, needs full JMAP support including calendars and contacts
- **Thunderbird** — evaluating jmap-client for their JMAP implementation (bugzilla #1322991)
- **Other Rust JMAP consumers** — the ecosystem has no alternative

## Implementation priority

### 1. Calendars (draft-ietf-jmap-calendars-26, in RFC Editor queue)
See `CALENDARS.md` for full reference.
- Calendar/get, Calendar/changes, Calendar/set
- CalendarEvent/get, /changes, /set, /copy, /query, /queryChanges, /parse
- CalendarEventNotification/get, /changes, /set, /query, /queryChanges
- ParticipantIdentity/get, /changes, /set
- Uses `calcard` crate for JSCalendar property model (NOT a large typed struct)

### 2. Contacts (RFC 9553 JSContact + RFC 9610 JMAP for Contacts)
See `CONTACTS.md` for full reference.
- AddressBook/get, /changes, /set
- ContactCard/get, /changes, /set, /copy, /query, /queryChanges, /parse
- Principal/get, /changes, /set, /query, /queryChanges, /getAvailability
- Uses `calcard` crate for JSContact property model

### 3. Blob management (RFC 9404)
See `BLOB.md` for full reference.
- Blob/upload, Blob/get, Blob/lookup
- Needed for attachment handling across email, calendar, and contacts

### 4. Quotas (RFC 9425)
See `QUOTA.md` for full reference.
- Quota/get, /changes, /query, /queryChanges
- Storage and count limits per account
- Small spec, straightforward

### 5. Sharing (RFC 9670)
See `SHARING.md` for full reference.
- Principal/get, /changes, /set, /query, /queryChanges (extends existing principal module)
- ShareNotification/get, /changes, /set (destroy only), /query, /queryChanges
- Shared data framework (isSubscribed, myRights, shareWith on Calendar/AddressBook/Mailbox)
- Required for shared calendars, shared address books, delegated mailboxes

### 6. MDN Handling (RFC 9007)
See `MDN.md` for full reference.
- MDN/send, MDN/parse
- Read receipts over JMAP
- $mdnsent keyword tracking

### 7. S/MIME Verification (RFC 9219)
See `SMIME.md` for full reference.
- Additional Email/get properties (smimeStatus, smimeErrors, smimeVerifiedAt)
- Additional Email/query filters (hasSmime, hasVerifiedSmime)
- Server-side verification — low client effort

### 8. Cherry-pick upstream fixes
- PR #18: Email/set null vs false for mailboxIds/keywords patch removal
- PR #18: Include submission capability for Identity/* requests

### 9. Future: Generic HTTP client (discussion #15)
Make the HTTP client generic instead of hardcoding reqwest. Lower priority but would make the library usable by more consumers (Thunderbird, projects using hyper directly).

## Status

| Spec | RFC | Status |
|---|---|---|
| Core | 8620 | ✅ Implemented |
| Mail | 8621 | ✅ Implemented |
| WebSocket | 8887 | ✅ Implemented |
| Sieve | 9661 | ✅ Implemented |
| Blob | 9404 | ✅ Implemented |
| Calendars | draft-26 | ✅ Implemented |
| Contacts | 9610 | ✅ Implemented |
| Quotas | 9425 | ✅ Implemented |
| Principals | 9670 | ✅ Partially (get/set/query/getAvailability done, ShareNotification missing) |
| Sharing | 9670 | ❌ ShareNotification missing |
| MDN | 9007 | ❌ Not implemented |
| S/MIME | 9219 | ❌ Not implemented |

## Design principles

- **Trait-based method dispatch** — JmapMethod trait, no central enums. Adding a method touches only its own module.
- **Transport-generic** — Client<T: HttpTransport>, SseTransport for EventSource. ReqwestTransport as default.
- **JSON map backing for JSCalendar/JSContact** — CalendarEvent and ContactCard use `serde_json::Map` for extension property round-trip fidelity.
- **Feature-gated per RFC** — mail, calendars, contacts, blob, quota features gate modules independently.
- **Implement from the RFCs** — do not copy from Stalwart's AGPL server code.
- **Apache-2.0 / MIT dual license**
- **Async-only**

Unimplemented spec plans are in `plans/` (SHARING.md, MDN.md, SMIME.md, ARCHITECTURE.md).
