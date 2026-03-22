/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

//! CalendarEvent wraps a JSCalendar Event (RFC 8984) object.
//!
//! Unlike Email or Mailbox, a CalendarEvent IS a JSCalendar object — its
//! property set is open-ended and includes vendor extension properties
//! (e.g. `example.com:custom-field`). The struct therefore stores all
//! properties in a `serde_json::Map` for round-trip fidelity.
//!
//! Typed accessor and builder methods are provided for common properties.
//! For extension or less-common properties, use [`CalendarEvent::property()`]
//! and [`CalendarEvent::set_property()`].
//!
//! For iCalendar ↔ JSCalendar conversion, the `calcard` crate (re-exported
//! from this crate) can parse the serialized JSON.

pub mod get;
pub mod helpers;
pub mod parse;
pub mod query;
pub mod set;

use std::fmt::{self, Display};

use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};

// ---- CalendarEvent ----

/// A calendar event backed by a JSCalendar JSON object.
///
/// All JSCalendar properties (standard and extension) are preserved in the
/// underlying `serde_json::Map`. Typed accessor methods are convenience
/// wrappers that read from / write to this map.
#[derive(Debug, Clone)]
pub struct CalendarEvent<State = Get> {
    _create_id: Option<usize>,
    _state: std::marker::PhantomData<State>,
    /// The raw JSCalendar properties. Every key/value from the server is
    /// preserved here, including vendor extension properties.
    pub properties: serde_json::Map<String, serde_json::Value>,
}

impl<State> Serialize for CalendarEvent<State> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.properties.len()))?;
        for (k, v) in &self.properties {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de, State> Deserialize<'de> for CalendarEvent<State> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CalendarEventVisitor<S>(std::marker::PhantomData<S>);

        impl<'de, S> Visitor<'de> for CalendarEventVisitor<S> {
            type Value = CalendarEvent<S>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a JSCalendar object")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut properties = serde_json::Map::new();
                while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                    properties.insert(key, value);
                }
                Ok(CalendarEvent {
                    _create_id: None,
                    _state: std::marker::PhantomData,
                    properties,
                })
            }
        }

        deserializer.deserialize_map(CalendarEventVisitor(std::marker::PhantomData))
    }
}

// ---- Alert type (used by both CalendarEvent and Calendar default alerts) ----

/// JSCalendar Alert object (RFC 8984 Section 4.5).
///
/// Used directly in Calendar's `defaultAlertsWithTime` /
/// `defaultAlertsWithoutTime`, and also representable in CalendarEvent's
/// `alerts` map (though CalendarEvent stores all properties as JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "trigger")]
    pub trigger: AlertTrigger,

    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AlertAction>,

    #[serde(rename = "acknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<String>,

    #[serde(rename = "relatedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_to: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum AlertTrigger {
    #[serde(rename = "OffsetTrigger")]
    OffsetTrigger {
        #[serde(rename = "offset")]
        offset: String,

        #[serde(rename = "relativeTo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        relative_to: Option<RelativeTo>,
    },
    #[serde(rename = "AbsoluteTrigger")]
    AbsoluteTrigger {
        #[serde(rename = "when")]
        when: String,
    },
    #[serde(rename = "UnknownTrigger")]
    UnknownTrigger {},
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertAction {
    #[serde(rename = "display")]
    Display,
    #[serde(rename = "email")]
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelativeTo {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
}

// ---- CalendarEvent/get arguments ----

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetArguments {
    #[serde(rename = "recurrenceOverridesBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_overrides_before: Option<String>,

    #[serde(rename = "recurrenceOverridesAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_overrides_after: Option<String>,

    #[serde(rename = "reduceParticipants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_participants: Option<bool>,

    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl GetArguments {
    pub fn recurrence_overrides_before(
        &mut self,
        before: impl Into<String>,
    ) -> &mut Self {
        self.recurrence_overrides_before = Some(before.into());
        self
    }

    pub fn recurrence_overrides_after(
        &mut self,
        after: impl Into<String>,
    ) -> &mut Self {
        self.recurrence_overrides_after = Some(after.into());
        self
    }

    pub fn reduce_participants(&mut self, reduce: bool) -> &mut Self {
        self.reduce_participants = Some(reduce);
        self
    }

    pub fn time_zone(&mut self, tz: impl Into<String>) -> &mut Self {
        self.time_zone = Some(tz.into());
        self
    }
}

// ---- CalendarEvent/set arguments ----

#[derive(Debug, Clone, Default, Serialize)]
pub struct SetArguments {
    #[serde(rename = "sendSchedulingMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_scheduling_messages: Option<bool>,
}

impl SetArguments {
    pub fn send_scheduling_messages(&mut self, send: bool) -> &mut Self {
        self.send_scheduling_messages = Some(send);
        self
    }
}

// ---- CalendarEvent/query arguments ----

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryArguments {
    #[serde(rename = "expandRecurrences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_recurrences: Option<bool>,

    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl QueryArguments {
    pub fn expand_recurrences(&mut self, expand: bool) -> &mut Self {
        self.expand_recurrences = Some(expand);
        self
    }

    pub fn time_zone(&mut self, tz: impl Into<String>) -> &mut Self {
        self.time_zone = Some(tz.into());
        self
    }
}

// ---- Property enum ----

/// Property names for CalendarEvent/get `properties` lists.
///
/// Common JSCalendar properties have typed variants. Extension or
/// less-common properties use `Other(String)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Property {
    Id,
    Uid,
    CalendarIds,
    IsDraft,
    Title,
    Description,
    DescriptionContentType,
    Created,
    Updated,
    Start,
    Duration,
    TimeZone,
    ShowWithoutTime,
    Status,
    FreeBusyStatus,
    RecurrenceId,
    RecurrenceIdTimeZone,
    RecurrenceRules,
    RecurrenceOverrides,
    ExcludedRecurrenceRules,
    Priority,
    Color,
    Locale,
    Keywords,
    Categories,
    ProdId,
    ReplyTo,
    Participants,
    UseDefaultAlerts,
    Alerts,
    Locations,
    VirtualLocations,
    Links,
    RelatedTo,
    ExcludedDates,
    Localizations,
    Method,
    Sequence,
    /// Any JSCalendar property not covered by the typed variants,
    /// including vendor extension properties.
    Other(String),
}

impl Property {
    fn as_str(&self) -> &str {
        match self {
            Property::Id => "id",
            Property::Uid => "uid",
            Property::CalendarIds => "calendarIds",
            Property::IsDraft => "isDraft",
            Property::Title => "title",
            Property::Description => "description",
            Property::DescriptionContentType => "descriptionContentType",
            Property::Created => "created",
            Property::Updated => "updated",
            Property::Start => "start",
            Property::Duration => "duration",
            Property::TimeZone => "timeZone",
            Property::ShowWithoutTime => "showWithoutTime",
            Property::Status => "status",
            Property::FreeBusyStatus => "freeBusyStatus",
            Property::RecurrenceId => "recurrenceId",
            Property::RecurrenceIdTimeZone => "recurrenceIdTimeZone",
            Property::RecurrenceRules => "recurrenceRules",
            Property::RecurrenceOverrides => "recurrenceOverrides",
            Property::ExcludedRecurrenceRules => "excludedRecurrenceRules",
            Property::Priority => "priority",
            Property::Color => "color",
            Property::Locale => "locale",
            Property::Keywords => "keywords",
            Property::Categories => "categories",
            Property::ProdId => "prodId",
            Property::ReplyTo => "replyTo",
            Property::Participants => "participants",
            Property::UseDefaultAlerts => "useDefaultAlerts",
            Property::Alerts => "alerts",
            Property::Locations => "locations",
            Property::VirtualLocations => "virtualLocations",
            Property::Links => "links",
            Property::RelatedTo => "relatedTo",
            Property::ExcludedDates => "excludedDates",
            Property::Localizations => "localizations",
            Property::Method => "method",
            Property::Sequence => "sequence",
            Property::Other(s) => s.as_str(),
        }
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for Property {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Property {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PropertyVisitor;

        impl<'de> Visitor<'de> for PropertyVisitor {
            type Value = Property;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a JSCalendar property name")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Property, E> {
                Ok(Property::from(v))
            }
        }

        deserializer.deserialize_str(PropertyVisitor)
    }
}

impl From<&str> for Property {
    fn from(s: &str) -> Self {
        match s {
            "id" => Property::Id,
            "uid" => Property::Uid,
            "calendarIds" => Property::CalendarIds,
            "isDraft" => Property::IsDraft,
            "title" => Property::Title,
            "description" => Property::Description,
            "descriptionContentType" => Property::DescriptionContentType,
            "created" => Property::Created,
            "updated" => Property::Updated,
            "start" => Property::Start,
            "duration" => Property::Duration,
            "timeZone" => Property::TimeZone,
            "showWithoutTime" => Property::ShowWithoutTime,
            "status" => Property::Status,
            "freeBusyStatus" => Property::FreeBusyStatus,
            "recurrenceId" => Property::RecurrenceId,
            "recurrenceIdTimeZone" => Property::RecurrenceIdTimeZone,
            "recurrenceRules" => Property::RecurrenceRules,
            "recurrenceOverrides" => Property::RecurrenceOverrides,
            "excludedRecurrenceRules" => Property::ExcludedRecurrenceRules,
            "priority" => Property::Priority,
            "color" => Property::Color,
            "locale" => Property::Locale,
            "keywords" => Property::Keywords,
            "categories" => Property::Categories,
            "prodId" => Property::ProdId,
            "replyTo" => Property::ReplyTo,
            "participants" => Property::Participants,
            "useDefaultAlerts" => Property::UseDefaultAlerts,
            "alerts" => Property::Alerts,
            "locations" => Property::Locations,
            "virtualLocations" => Property::VirtualLocations,
            "links" => Property::Links,
            "relatedTo" => Property::RelatedTo,
            "excludedDates" => Property::ExcludedDates,
            "localizations" => Property::Localizations,
            "method" => Property::Method,
            "sequence" => Property::Sequence,
            other => Property::Other(other.to_string()),
        }
    }
}

impl Object for CalendarEvent<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for CalendarEvent<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for CalendarEvent<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for CalendarEvent<Get> {
    type ChangesResponse = ();
}
