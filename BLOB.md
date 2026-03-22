# JMAP Blob Management — Implementation Reference

Spec: [RFC 9404](https://www.rfc-editor.org/rfc/rfc9404) (published August 2023)

## Capability

```
urn:ietf:params:jmap:blob
```

Session properties:
- `maxSizeBlobSet` — Maximum size of blob that can be created via Blob/upload
- `supportedDigestAlgorithms` — List of supported hash algorithms (e.g., `"sha"`, `"sha-256"`, `"sha-512"`)
- `supportedTypeNames` — List of type names that support Blob/lookup

## Methods

### Blob/upload

Create blobs through JMAP method calls (inline, not the separate upload endpoint). Accepts data from multiple sources.

Only `create` is allowed — blobs cannot be updated or deleted.

Each create entry specifies:
- `data` — Array of data sources, concatenated in order:
  - `DataSourceBlob` — reference another blob by ID, with optional offset/length
  - `DataSourceString` — literal string value
- `type` — Optional MIME type hint

### Blob/get

Retrieve blob content with optional range.

Properties:
- `offset` — Starting byte offset (UnsignedInt)
- `length` — Number of bytes to return (UnsignedInt)
- `encoding` — Return format: not specified (raw), `"base64"`, or specific encoding
- `digest` — Array of digest algorithm names to compute on the blob

Returns:
- `data` — Blob content (as text or base64)
- `digest` — Map of algorithm → hex digest values
- `size` — Total blob size
- `isEncodingProblem` — Boolean
- `isTruncated` — Boolean

### Blob/lookup

Reverse lookup — find which objects reference a given blob.

Request:
- `typeNames` — List of JMAP type names to search (e.g., `"Email"`, `"CalendarEvent"`)
- `ids` — List of blob IDs to look up

Returns map of blob ID → list of `{id, type, name}` references.

## Use Cases for Ratatoskr

- **Attachment uploads** — Create blobs inline with compose, reference in Email/set
- **Calendar event attachments** — Binary attachments on CalendarEvent via blob references
- **Contact photos** — Avatar/photo blobs for ContactCard
- **Efficient large uploads** — Blob/upload supports chunked/referenced assembly

## Notes from Server Implementation

Stalwart declares `urn:ietf:params:jmap:blob` in capabilities. The `supportedTypeNames` likely includes `"Email"`, `"CalendarEvent"`, `"ContactCard"`.
