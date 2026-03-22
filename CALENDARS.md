# JMAP for Calendars — Implementation Reference

Spec: [draft-ietf-jmap-calendars-26](https://www.ietf.org/archive/id/draft-ietf-jmap-calendars-26.html) (in RFC Editor queue, effectively frozen)
Data format: [RFC 8984 JSCalendar](https://www.rfc-editor.org/rfc/rfc8984)

## Capabilities

```
urn:ietf:params:jmap:calendars
urn:ietf:params:jmap:calendars:parse
```

Session properties: `mayCreateCalendar`, `maxCalendarsPerEvent`.

## Objects and Methods

### Calendar

Container for events. Analogous to Mailbox for Email.

**Methods:** Calendar/get, Calendar/changes, Calendar/set

**Properties:**
- `id` — Server-assigned ID
- `name` — Display name (String)
- `description` — Optional description (String|null)
- `color` — CSS color value (String|null)
- `sortOrder` — Unsigned integer for ordering
- `isSubscribed` — Boolean
- `isVisible` — Boolean
- `isDefault` — Boolean
- `includeInAvailability` — `"all"` | `"attending"` | `"none"`
- `defaultAlertsWithTime` — Map of Alert objects for timed events
- `defaultAlertsWithoutTime` — Map of Alert objects for all-day events
- `timeZone` — IANA timezone string (String|null)
- `shareWith` — Map of principal ID → rights object
- `myRights` — Rights object for current user

**Rights model (CalendarRight):**
- `mayReadFreeBusy`
- `mayReadItems`
- `mayWriteAll`
- `mayWriteOwn`
- `mayUpdatePrivate`
- `mayRSVP`
- `mayShare`
- `mayDelete`

**Calendar/set arguments:**
- `onDestroyRemoveEvents` — Boolean, remove events when calendar deleted
- `onSuccessSetIsDefault` — Id reference, set as default after creation

### CalendarEvent

An event in JSCalendar format. The property set IS the JSCalendar object — not a wrapper around it.

**Methods:** CalendarEvent/get, CalendarEvent/changes, CalendarEvent/set, CalendarEvent/copy, CalendarEvent/query, CalendarEvent/queryChanges, CalendarEvent/parse

**CalendarEvent/get arguments:**
- `recurrenceOverridesBefore` — UTC datetime, limit recurrence expansion
- `recurrenceOverridesAfter` — UTC datetime, limit recurrence expansion
- `reduceParticipants` — Boolean, strip participants for bandwidth
- `timeZone` — IANA timezone, convert all times

**CalendarEvent/set arguments:**
- `sendSchedulingMessages` — Boolean, send iTIP scheduling messages

**CalendarEvent/query arguments:**
- `expandRecurrences` — Boolean, expand recurring events into instances
- `timeZone` — IANA timezone for date comparisons

**Filter properties:**
- `inCalendar` — ID of calendar to filter by
- `after` — UTC datetime (events ending after this)
- `before` — UTC datetime (events starting before this)
- `text` — Free-text search across title/description/location/etc.
- `title` — Match in event title
- `description` — Match in event description
- `location` — Match in event locations
- `owner` — Match owner
- `attendee` — Match attendee
- `uid` — Exact UID match

**Comparator properties:** `start`, `uid`, `recurrenceId`, `created`, `updated`

### CalendarEventNotification

Tracks changes to calendar events for multi-user notification.

**Methods:** CalendarEventNotification/get, CalendarEventNotification/changes, CalendarEventNotification/set, CalendarEventNotification/query, CalendarEventNotification/queryChanges

### ParticipantIdentity

User identity for calendar event participation (analogous to Identity for email).

**Methods:** ParticipantIdentity/get, ParticipantIdentity/changes, ParticipantIdentity/set

## JSCalendar Key Types (RFC 8984)

These are the value types used in CalendarEvent properties:

- **RecurrenceRule** — frequency, interval, rscale, skip, firstDayOfWeek, byDay, byMonthDay, byMonth, bySetPosition, byHour, byMinute, bySecond, byWeekNo, byYearDay, count, until
- **Alert** — trigger (OffsetTrigger | AbsoluteTrigger | UnknownTrigger), action (`"display"` | `"email"`)
- **OffsetTrigger** — offset (SignedDuration), relativeTo (`"start"` | `"end"`)
- **AbsoluteTrigger** — when (UTCDateTime)
- **Participant** — name, email, kind, roles (Map), participationStatus, expectReply, scheduleAgent, scheduleSequence, scheduleForceSend, delegatedTo, delegatedFrom, memberOf, links, sendTo, invitedBy, participationComment
- **Location** — name, description, coordinates, timeZone, links
- **VirtualLocation** — name, description, uri, features
- **Link** — href, cid, contentType, size, rel, display, title

## Design Decision: Use `calcard` for the Property Model

**Do NOT create a large typed Rust struct for CalendarEvent properties.**

Stalwart's server uses the `calcard` crate (Apache-2.0 / MIT, `calcard = "0.3"`) as the property/value model. CalendarEvent is NOT a typed struct — it's a generic tree:

```rust
// In Stalwart's jmap-proto:
impl JmapObject for CalendarEvent {
    type Property = JSCalendarProperty<Id>;    // ~100-variant enum from calcard
    type Element = JSCalendarValue<Id, BlobId>; // typed value enum from calcard
}
```

`JSCalendarProperty` is a ~100-variant enum covering ALL JSCalendar property names (title, start, duration, recurrenceRules, participants, alerts, locations, etc. plus sub-properties like byDay, offset, roles). `JSCalendarValue` carries typed values (DateTime, Duration, FreeBusyStatus, ParticipantKind, etc.).

The `JSCalendar<'x, I, B>` type wraps `Value<JSCalendarProperty<I>, JSCalendarValue<I, B>>` which is a generic JSON-like tree (Null/Bool/Number/Str/Array/Object) keyed by the property enum.

**For jmap-client, add `calcard = "0.3"` as a dependency and use:**
- `JSCalendarProperty<String>` as CalendarEvent's property type (jmap-client uses String IDs)
- `JSCalendarValue<String, String>` as the element type
- The `Calendar` container object gets a small typed struct (like Mailbox)
- CalendarEvent get/set pass JSCalendar objects through directly

This avoids building and maintaining a 100+ field typed struct, stays compatible with the server's wire format, gets parsing/validation from calcard for free, and handles extension properties naturally.

**Same approach for contacts:** use `calcard`'s `JSContactProperty<String>` / `JSContactValue<String, String>`.

## Notes from Server Implementation

Filter values are case-insensitive (server lowercases text/title/description/location/owner/attendee). Datetime filters use RFC 3339 format parsed as local time.

The server's CalendarEvent/get arguments use custom types:
- `recurrenceOverridesBefore/After` — parsed as RFC 3339 local time
- `timeZone` — IANA timezone via `Tz::from_str()`
- `reduceParticipants` — boolean
- `expandRecurrences` — boolean (query argument)
- `sendSchedulingMessages` — boolean (set argument)
