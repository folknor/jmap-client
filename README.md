# jmap-client

[![crates.io](https://img.shields.io/crates/v/jmap-client)](https://crates.io/crates/jmap-client)
[![docs.rs](https://img.shields.io/docsrs/jmap-client)](https://docs.rs/jmap-client)
[![crates.io](https://img.shields.io/crates/l/jmap-client)](http://www.apache.org/licenses/LICENSE-2.0)

_jmap-client_ is a **JSON Meta Application Protocol (JMAP) library** written in Rust. Originally by [Stalwart Labs](https://github.com/stalwartlabs/jmap-client), this fork adds full support for JMAP Calendars, Contacts, Blob Management, and Quotas.

## Supported RFCs

| Spec | Status |
|------|--------|
| [RFC 8620](https://datatracker.ietf.org/doc/html/rfc8620) — JMAP Core | Complete |
| [RFC 8621](https://datatracker.ietf.org/doc/html/rfc8621) — JMAP for Mail | Complete |
| [RFC 8887](https://datatracker.ietf.org/doc/html/rfc8887) — JMAP over WebSocket | Complete |
| [draft-ietf-jmap-calendars-26](https://www.ietf.org/archive/id/draft-ietf-jmap-calendars-26.html) — JMAP for Calendars | Complete |
| [RFC 9610](https://www.rfc-editor.org/rfc/rfc9610) — JMAP for Contacts | Complete |
| [RFC 9404](https://www.rfc-editor.org/rfc/rfc9404) — JMAP Blob Management | Complete |
| [RFC 9425](https://www.rfc-editor.org/rfc/rfc9425) — JMAP Quotas | Complete |
| [draft-ietf-jmap-sieve-14](https://www.ietf.org/archive/id/draft-ietf-jmap-sieve-14.html) — JMAP for Sieve Scripts | Complete |

## Features

- Async-only (powered by `reqwest` + `tokio`).
- WebSocket async streams (`websockets` cargo feature).
- EventSource async streams.
- Typed builders and accessors for all JMAP object types.
- CalendarEvent and ContactCard use JSON map backing for full JSCalendar/JSContact fidelity, including vendor extension properties.
- Session capability introspection for all supported extensions.

## Cargo Features

| Feature | Default | Description |
|---------|---------|-------------|
| `websockets` | Yes | JMAP over WebSocket |

## Usage Example

```rust
// Connect to the JMAP server using Basic authentication.
let client = Client::new()
    .credentials(("john@example.org", "secret"))
    .connect("https://jmap.example.org")
    .await
    .unwrap();

// Create a mailbox.
let mailbox_id = client
    .mailbox_create("My Mailbox", None::<String>, Role::None)
    .await
    .unwrap()
    .take_id();

// Import a message into the mailbox.
client
    .email_import(
        b"From: john@example.org\nSubject: test\n\n test".to_vec(),
        [&mailbox_id],
        ["$draft"].into(),
        None,
    )
    .await
    .unwrap();

// Query emails matching a filter.
let email_id = client
    .email_query(
        Filter::and([
            email::query::Filter::subject("test"),
            email::query::Filter::in_mailbox(&mailbox_id),
        ])
        .into(),
        [email::query::Comparator::from()].into(),
    )
    .await
    .unwrap()
    .take_ids()
    .pop()
    .unwrap();

// Fetch an email.
let email = client
    .email_get(
        &email_id,
        [Property::Subject, Property::Preview, Property::Keywords].into(),
    )
    .await
    .unwrap()
    .unwrap();
assert_eq!(email.preview().unwrap(), "test");

// Account-scoped request batching.
let account = client.account(client.default_account());
let mut request = account.build();
let handle = request.call(quota::QuotaGet::new(account.id_str())).unwrap();
let mut response = request.send().await.unwrap();
let quotas = response.get(&handle).unwrap();

// Typed request batching with result references.
let mut request = client.build();
let account_id = request.default_account_id().to_string();
let mut query = email::EmailQuery::new(&account_id);
query.filter(email::query::Filter::subject("test").into());
let q_handle = request.call(query).unwrap();

let mut get = email::EmailGet::new(&account_id);
get.ids_ref(q_handle.result_reference("/ids"));
let g_handle = request.call(get).unwrap();

let mut response = request.send().await.unwrap();
let emails = response.get(&g_handle).unwrap();
```

## Testing

```bash
cargo test --lib
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Copyright

Copyright (C) 2022, Stalwart Labs LLC
