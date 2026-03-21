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

pub mod get;
pub mod helpers;
pub mod parse;
pub mod query;
pub mod set;

use std::fmt::Display;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::core::changes::ChangesObject;
use crate::core::set::{list_not_set, map_not_set, string_not_set};
use crate::core::Object;
use crate::{Get, Set};

// ---- CalendarEvent ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "string_not_set")]
    pub uid: Option<String>,

    #[serde(rename = "calendarIds")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub calendar_ids: Option<AHashMap<String, bool>>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "string_not_set")]
    pub title: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "string_not_set")]
    pub description: Option<String>,

    #[serde(rename = "descriptionContentType")]
    #[serde(skip_serializing_if = "string_not_set")]
    pub description_content_type: Option<String>,

    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "string_not_set")]
    pub start: Option<String>,

    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "string_not_set")]
    pub duration: Option<String>,

    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Option<String>>,

    #[serde(rename = "showWithoutTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_without_time: Option<bool>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>,

    #[serde(rename = "freeBusyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_busy_status: Option<FreeBusyStatus>,

    #[serde(rename = "recurrenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_id: Option<String>,

    #[serde(rename = "recurrenceIdTimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_id_time_zone: Option<Option<String>>,

    #[serde(rename = "recurrenceRules")]
    #[serde(skip_serializing_if = "list_not_set")]
    pub recurrence_rules: Option<Vec<RecurrenceRule>>,

    #[serde(rename = "recurrenceOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_overrides: Option<AHashMap<String, serde_json::Value>>,

    #[serde(rename = "excludedRecurrenceRules")]
    #[serde(skip_serializing_if = "list_not_set")]
    pub excluded_recurrence_rules: Option<Vec<RecurrenceRule>>,

    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,

    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Option<String>>,

    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub keywords: Option<AHashMap<String, bool>>,

    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub categories: Option<AHashMap<String, bool>>,

    #[serde(rename = "prodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_id: Option<String>,

    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub reply_to: Option<AHashMap<String, String>>,

    #[serde(rename = "participants")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub participants: Option<AHashMap<String, Participant>>,

    #[serde(rename = "useDefaultAlerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_alerts: Option<bool>,

    #[serde(rename = "alerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Option<AHashMap<String, Alert>>>,

    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub locations: Option<AHashMap<String, Location>>,

    #[serde(rename = "virtualLocations")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub virtual_locations: Option<AHashMap<String, VirtualLocation>>,

    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub links: Option<AHashMap<String, Link>>,

    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    #[serde(rename = "sequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u32>,
}

// ---- JSCalendar types (RFC 8984) ----

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventStatus {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "tentative")]
    Tentative,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FreeBusyStatus {
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "free")]
    Free,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrenceRule {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "frequency")]
    pub frequency: Frequency,

    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,

    #[serde(rename = "rscale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rscale: Option<String>,

    #[serde(rename = "skip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Skip>,

    #[serde(rename = "firstDayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_day_of_week: Option<Day>,

    #[serde(rename = "byDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_day: Option<Vec<NDay>>,

    #[serde(rename = "byMonthDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month_day: Option<Vec<i32>>,

    #[serde(rename = "byMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month: Option<Vec<String>>,

    #[serde(rename = "byYearDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_year_day: Option<Vec<i32>>,

    #[serde(rename = "byWeekNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_week_no: Option<Vec<i32>>,

    #[serde(rename = "byHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_hour: Option<Vec<u32>>,

    #[serde(rename = "byMinute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_minute: Option<Vec<u32>>,

    #[serde(rename = "bySecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_second: Option<Vec<u32>>,

    #[serde(rename = "bySetPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_set_position: Option<Vec<i32>>,

    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    #[serde(rename = "until")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Frequency {
    #[serde(rename = "yearly")]
    Yearly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "minutely")]
    Minutely,
    #[serde(rename = "secondly")]
    Secondly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Skip {
    #[serde(rename = "omit")]
    Omit,
    #[serde(rename = "backward")]
    Backward,
    #[serde(rename = "forward")]
    Forward,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Day {
    #[serde(rename = "mo")]
    Monday,
    #[serde(rename = "tu")]
    Tuesday,
    #[serde(rename = "we")]
    Wednesday,
    #[serde(rename = "th")]
    Thursday,
    #[serde(rename = "fr")]
    Friday,
    #[serde(rename = "sa")]
    Saturday,
    #[serde(rename = "su")]
    Sunday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDay {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "day")]
    pub day: Day,

    #[serde(rename = "nthOfPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nth_of_period: Option<i32>,
}

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
    pub related_to: Option<AHashMap<String, Relation>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "relation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<AHashMap<String, bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "sendTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to: Option<AHashMap<String, String>>,

    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ParticipantKind>,

    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AHashMap<String, bool>>,

    #[serde(rename = "participationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation_status: Option<ParticipationStatus>,

    #[serde(rename = "participationComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation_comment: Option<String>,

    #[serde(rename = "expectReply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_reply: Option<bool>,

    #[serde(rename = "scheduleAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_agent: Option<ScheduleAgent>,

    #[serde(rename = "scheduleForceSend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_force_send: Option<bool>,

    #[serde(rename = "scheduleSequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_sequence: Option<u32>,

    #[serde(rename = "scheduleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_status: Option<Vec<String>>,

    #[serde(rename = "scheduleUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_updated: Option<String>,

    #[serde(rename = "invitedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,

    #[serde(rename = "delegatedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_to: Option<AHashMap<String, bool>>,

    #[serde(rename = "delegatedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_from: Option<AHashMap<String, bool>>,

    #[serde(rename = "memberOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of: Option<AHashMap<String, bool>>,

    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<AHashMap<String, Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticipantKind {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "location")]
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticipationStatus {
    #[serde(rename = "needs-action")]
    NeedsAction,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "tentative")]
    Tentative,
    #[serde(rename = "delegated")]
    Delegated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScheduleAgent {
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "relativeTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_to: Option<RelativeTo>,

    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    #[serde(rename = "coordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<String>,

    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<AHashMap<String, Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualLocation {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<AHashMap<String, bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "href")]
    pub href: String,

    #[serde(rename = "cid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,

    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    #[serde(rename = "rel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,

    #[serde(rename = "display")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

// ---- Property enum ----

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "uid")]
    Uid,
    #[serde(rename = "calendarIds")]
    CalendarIds,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "descriptionContentType")]
    DescriptionContentType,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "duration")]
    Duration,
    #[serde(rename = "timeZone")]
    TimeZone,
    #[serde(rename = "showWithoutTime")]
    ShowWithoutTime,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "freeBusyStatus")]
    FreeBusyStatus,
    #[serde(rename = "recurrenceId")]
    RecurrenceId,
    #[serde(rename = "recurrenceIdTimeZone")]
    RecurrenceIdTimeZone,
    #[serde(rename = "recurrenceRules")]
    RecurrenceRules,
    #[serde(rename = "recurrenceOverrides")]
    RecurrenceOverrides,
    #[serde(rename = "excludedRecurrenceRules")]
    ExcludedRecurrenceRules,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "locale")]
    Locale,
    #[serde(rename = "keywords")]
    Keywords,
    #[serde(rename = "categories")]
    Categories,
    #[serde(rename = "prodId")]
    ProdId,
    #[serde(rename = "replyTo")]
    ReplyTo,
    #[serde(rename = "participants")]
    Participants,
    #[serde(rename = "useDefaultAlerts")]
    UseDefaultAlerts,
    #[serde(rename = "alerts")]
    Alerts,
    #[serde(rename = "locations")]
    Locations,
    #[serde(rename = "virtualLocations")]
    VirtualLocations,
    #[serde(rename = "links")]
    Links,
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "sequence")]
    Sequence,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Uid => write!(f, "uid"),
            Property::CalendarIds => write!(f, "calendarIds"),
            Property::Title => write!(f, "title"),
            Property::Description => write!(f, "description"),
            Property::DescriptionContentType => write!(f, "descriptionContentType"),
            Property::Created => write!(f, "created"),
            Property::Updated => write!(f, "updated"),
            Property::Start => write!(f, "start"),
            Property::Duration => write!(f, "duration"),
            Property::TimeZone => write!(f, "timeZone"),
            Property::ShowWithoutTime => write!(f, "showWithoutTime"),
            Property::Status => write!(f, "status"),
            Property::FreeBusyStatus => write!(f, "freeBusyStatus"),
            Property::RecurrenceId => write!(f, "recurrenceId"),
            Property::RecurrenceIdTimeZone => write!(f, "recurrenceIdTimeZone"),
            Property::RecurrenceRules => write!(f, "recurrenceRules"),
            Property::RecurrenceOverrides => write!(f, "recurrenceOverrides"),
            Property::ExcludedRecurrenceRules => write!(f, "excludedRecurrenceRules"),
            Property::Priority => write!(f, "priority"),
            Property::Color => write!(f, "color"),
            Property::Locale => write!(f, "locale"),
            Property::Keywords => write!(f, "keywords"),
            Property::Categories => write!(f, "categories"),
            Property::ProdId => write!(f, "prodId"),
            Property::ReplyTo => write!(f, "replyTo"),
            Property::Participants => write!(f, "participants"),
            Property::UseDefaultAlerts => write!(f, "useDefaultAlerts"),
            Property::Alerts => write!(f, "alerts"),
            Property::Locations => write!(f, "locations"),
            Property::VirtualLocations => write!(f, "virtualLocations"),
            Property::Links => write!(f, "links"),
            Property::Method => write!(f, "method"),
            Property::Sequence => write!(f, "sequence"),
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
