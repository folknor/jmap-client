# JMAP Quotas — Implementation Reference

Spec: [RFC 9425](https://www.rfc-editor.org/rfc/rfc9425) (published October 2023)

## Capability

```
urn:ietf:params:jmap:quota
```

Empty capability object in session and account.

## Object: Quota

Represents a storage or count limit on an account.

**Methods:** Quota/get, Quota/changes, Quota/query, Quota/queryChanges

**Properties:**
- `id` (Id) — Server-assigned identifier
- `resourceType` (String) — `"count"` or `"octets"`
- `used` (UnsignedInt) — Current usage
- `hardLimit` (UnsignedInt) — Maximum allowed; blocks creation/updates when reached
- `scope` (String) — `"account"`, `"domain"`, or `"global"`
- `name` (String) — Quota identifier
- `types` (String[]) — JMAP type names this quota applies to (e.g., `["Mail"]`, `["Calendar", "Contact"]`)
- `warnLimit` (UnsignedInt|null) — Warning threshold
- `softLimit` (UnsignedInt|null) — Soft limit allowing selective operations
- `description` (String|null) — Human-readable explanation

**Filter properties:**
- `name` — Substring match
- `scope` — Exact match
- `resourceType` — Exact match
- `type` — Match if types array contains value

**Comparator properties:** `name`, `used`

## Relevance to Ratatoskr

- Show "X GB of Y GB used" in account settings or status bar
- Warn users approaching quota before send/upload fails
- Per-type quota awareness (mail vs calendar vs contacts storage)
- Small spec — straightforward to implement
