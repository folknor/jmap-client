# JMAP for Contacts — Implementation Reference

Spec: [RFC 9553 JSContact](https://www.rfc-editor.org/rfc/rfc9553) (published May 2024)
JMAP binding: [RFC 9610 JMAP for Contacts](https://www.rfc-editor.org/rfc/rfc9610)

## Capabilities

```
urn:ietf:params:jmap:contacts
urn:ietf:params:jmap:contacts:parse
urn:ietf:params:jmap:principals
urn:ietf:params:jmap:principals:owner
urn:ietf:params:jmap:principals:availability
```

Session properties: `mayCreateAddressBook`, `maxAddressBooksPerCard`, `accountIdForPrincipal`, `currentUserPrincipalId`.

## Objects and Methods

### AddressBook

Container for contacts. Analogous to Mailbox for Email, Calendar for CalendarEvent.

**Methods:** AddressBook/get, AddressBook/changes, AddressBook/set

**Properties:**
- `id` — Server-assigned ID
- `name` — Display name (String)
- `description` — Optional description (String|null)
- `sortOrder` — Unsigned integer for ordering
- `isDefault` — Boolean
- `isSubscribed` — Boolean
- `shareWith` — Map of principal ID → rights object
- `myRights` — Rights object for current user

**Rights model (AddressBookRight):**
- `mayRead`
- `mayWrite`
- `mayShare`
- `mayDelete`

**AddressBook/set arguments:**
- `onDestroyRemoveContents` — Boolean, remove contacts when address book deleted
- `onSuccessSetIsDefault` — Id reference, set as default after creation

### ContactCard

A contact in JSContact format (RFC 9553). The property set IS the JSContact Card object.

**Methods:** ContactCard/get, ContactCard/changes, ContactCard/set, ContactCard/copy, ContactCard/query, ContactCard/queryChanges, ContactCard/parse

**Filter properties:**
- `inAddressBook` — ID of address book to filter by
- `uid` — Exact UID match
- `hasMember` — Filter groups containing this UID
- `kind` — Contact kind (e.g., `"individual"`, `"group"`, `"org"`)
- `createdBefore` / `createdAfter` — UTC datetime
- `updatedBefore` / `updatedAfter` — UTC datetime
- `text` — Free-text search across all string properties
- `name` — Match in full name
- `name/given` — Match in given name
- `name/surname` — Match in surname
- `name/surname2` — Match in second surname
- `nickname` — Match in nicknames
- `organization` — Match in organizations
- `email` — Match in email addresses
- `phone` — Match in phone numbers
- `onlineService` — Match in online services (social media, etc.)
- `address` — Match in postal addresses
- `note` — Match in notes

**Comparator properties:** `created`, `updated`, `name/given`, `name/surname`, `name/surname2`

### Principal

User or resource identity for sharing and scheduling.

**Methods:** Principal/get, Principal/changes, Principal/set, Principal/query, Principal/queryChanges, Principal/getAvailability

`Principal/getAvailability` is used for free/busy lookups — given a principal and time range, returns availability.

## JSContact Key Types (RFC 9553)

ContactCard properties use JSContact types. The server uses `JSContactProperty` as the ContactCard property type directly (same pattern as CalendarEvent using JSCalendarProperty).

Key JSContact structures:
- **Name** — components (given, surname, surname2, etc.), full, phoneticSystem
- **EmailAddress** — address, contexts, pref, label
- **Phone** — number, features (voice, fax, cell, etc.), contexts, pref
- **Address** — street, locality, region, postcode, country, coordinates, timeZone, contexts
- **Organization** — name, units
- **OnlineService** — service, uri, user, contexts
- **Note** — note, created, author
- **Media** — kind (photo, sound, logo), uri, mediaType

## Design Decision: Use `calcard` for the Property Model

Same as CalendarEvent — do NOT build a large typed struct. Use `calcard`'s types:

```rust
// In Stalwart's jmap-proto:
impl JmapObject for ContactCard {
    type Property = JSContactProperty<Id>;
    type Element = JSContactValue<Id, BlobId>;
}
```

**For jmap-client:** use `JSContactProperty<String>` and `JSContactValue<String, String>`.

The `AddressBook` container gets a small typed struct (like Calendar/Mailbox).

## Notes from Server Implementation

Filter values are case-insensitive (server lowercases text fields). Email filter is exact match (not lowercased). Date filters use UTC date format.

The `hasMember` filter searches for groups containing a specific member UID. The `kind` filter matches the JSContact `kind` property (`"individual"`, `"group"`, `"org"`, `"location"`, `"device"`).
