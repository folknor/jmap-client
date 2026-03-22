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

#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod address_book;
pub mod blob;
pub mod calendar;
pub mod calendar_event;
pub mod calendar_event_notification;
pub mod client;
pub mod contact_card;
pub mod core;
pub mod email;
pub mod email_submission;
pub mod event_source;
pub mod identity;
pub mod mailbox;
pub mod participant_identity;
pub mod principal;
pub mod push_subscription;
pub mod quota;
pub mod sieve;
pub mod thread;
pub mod vacation_response;

use crate::core::error::MethodError;
use crate::core::error::ProblemDetails;
use crate::core::set::SetError;
use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg(feature = "websockets")]
pub mod client_ws;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum URI {
    #[serde(rename = "urn:ietf:params:jmap:core")]
    Core,
    #[serde(rename = "urn:ietf:params:jmap:mail")]
    Mail,
    #[serde(rename = "urn:ietf:params:jmap:submission")]
    Submission,
    #[serde(rename = "urn:ietf:params:jmap:vacationresponse")]
    VacationResponse,
    #[serde(rename = "urn:ietf:params:jmap:contacts")]
    Contacts,
    #[serde(rename = "urn:ietf:params:jmap:calendars")]
    Calendars,
    #[serde(rename = "urn:ietf:params:jmap:calendars:parse")]
    CalendarsParse,
    #[serde(rename = "urn:ietf:params:jmap:contacts:parse")]
    ContactsParse,
    #[serde(rename = "urn:ietf:params:jmap:blob")]
    Blob,
    #[serde(rename = "urn:ietf:params:jmap:quota")]
    Quota,
    #[serde(rename = "urn:ietf:params:jmap:websocket")]
    WebSocket,
    #[serde(rename = "urn:ietf:params:jmap:sieve")]
    Sieve,
    #[serde(rename = "urn:ietf:params:jmap:principals")]
    Principals,
    #[serde(rename = "urn:ietf:params:jmap:principals:owner")]
    PrincipalsOwner,
}

impl AsRef<str> for URI {
    fn as_ref(&self) -> &str {
        match self {
            URI::Core => "urn:ietf:params:jmap:core",
            URI::Mail => "urn:ietf:params:jmap:mail",
            URI::Submission => "urn:ietf:params:jmap:submission",
            URI::VacationResponse => "urn:ietf:params:jmap:vacationresponse",
            URI::Contacts => "urn:ietf:params:jmap:contacts",
            URI::Quota => "urn:ietf:params:jmap:quota",
            URI::Blob => "urn:ietf:params:jmap:blob",
            URI::Calendars => "urn:ietf:params:jmap:calendars",
            URI::CalendarsParse => "urn:ietf:params:jmap:calendars:parse",
            URI::ContactsParse => "urn:ietf:params:jmap:contacts:parse",
            URI::WebSocket => "urn:ietf:params:jmap:websocket",
            URI::Sieve => "urn:ietf:params:jmap:sieve",
            URI::Principals => "urn:ietf:params:jmap:principals",
            URI::PrincipalsOwner => "urn:ietf:params:jmap:principals:owner",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "Core/echo")]
    Echo,
    #[serde(rename = "Blob/copy")]
    CopyBlob,
    #[serde(rename = "Blob/upload")]
    UploadBlob,
    #[serde(rename = "Blob/get")]
    GetBlob,
    #[serde(rename = "Blob/lookup")]
    LookupBlob,
    #[serde(rename = "PushSubscription/get")]
    GetPushSubscription,
    #[serde(rename = "PushSubscription/set")]
    SetPushSubscription,
    #[serde(rename = "Mailbox/get")]
    GetMailbox,
    #[serde(rename = "Mailbox/changes")]
    ChangesMailbox,
    #[serde(rename = "Mailbox/query")]
    QueryMailbox,
    #[serde(rename = "Mailbox/queryChanges")]
    QueryChangesMailbox,
    #[serde(rename = "Mailbox/set")]
    SetMailbox,
    #[serde(rename = "Thread/get")]
    GetThread,
    #[serde(rename = "Thread/changes")]
    ChangesThread,
    #[serde(rename = "Email/get")]
    GetEmail,
    #[serde(rename = "Email/changes")]
    ChangesEmail,
    #[serde(rename = "Email/query")]
    QueryEmail,
    #[serde(rename = "Email/queryChanges")]
    QueryChangesEmail,
    #[serde(rename = "Email/set")]
    SetEmail,
    #[serde(rename = "Email/copy")]
    CopyEmail,
    #[serde(rename = "Email/import")]
    ImportEmail,
    #[serde(rename = "Email/parse")]
    ParseEmail,
    #[serde(rename = "SearchSnippet/get")]
    GetSearchSnippet,
    #[serde(rename = "Identity/get")]
    GetIdentity,
    #[serde(rename = "Identity/changes")]
    ChangesIdentity,
    #[serde(rename = "Identity/set")]
    SetIdentity,
    #[serde(rename = "EmailSubmission/get")]
    GetEmailSubmission,
    #[serde(rename = "EmailSubmission/changes")]
    ChangesEmailSubmission,
    #[serde(rename = "EmailSubmission/query")]
    QueryEmailSubmission,
    #[serde(rename = "EmailSubmission/queryChanges")]
    QueryChangesEmailSubmission,
    #[serde(rename = "EmailSubmission/set")]
    SetEmailSubmission,
    #[serde(rename = "VacationResponse/get")]
    GetVacationResponse,
    #[serde(rename = "VacationResponse/set")]
    SetVacationResponse,
    #[serde(rename = "SieveScript/get")]
    GetSieveScript,
    #[serde(rename = "SieveScript/set")]
    SetSieveScript,
    #[serde(rename = "SieveScript/query")]
    QuerySieveScript,
    #[serde(rename = "SieveScript/validate")]
    ValidateSieveScript,
    #[serde(rename = "Principal/get")]
    GetPrincipal,
    #[serde(rename = "Principal/changes")]
    ChangesPrincipal,
    #[serde(rename = "Principal/query")]
    QueryPrincipal,
    #[serde(rename = "Principal/queryChanges")]
    QueryChangesPrincipal,
    #[serde(rename = "Principal/set")]
    SetPrincipal,
    #[serde(rename = "Principal/getAvailability")]
    GetAvailabilityPrincipal,
    #[serde(rename = "Quota/get")]
    GetQuota,
    #[serde(rename = "Quota/changes")]
    ChangesQuota,
    #[serde(rename = "Quota/query")]
    QueryQuota,
    #[serde(rename = "Quota/queryChanges")]
    QueryChangesQuota,
    #[serde(rename = "Calendar/get")]
    GetCalendar,
    #[serde(rename = "Calendar/changes")]
    ChangesCalendar,
    #[serde(rename = "Calendar/set")]
    SetCalendar,
    #[serde(rename = "CalendarEvent/get")]
    GetCalendarEvent,
    #[serde(rename = "CalendarEvent/changes")]
    ChangesCalendarEvent,
    #[serde(rename = "CalendarEvent/query")]
    QueryCalendarEvent,
    #[serde(rename = "CalendarEvent/queryChanges")]
    QueryChangesCalendarEvent,
    #[serde(rename = "CalendarEvent/set")]
    SetCalendarEvent,
    #[serde(rename = "CalendarEvent/parse")]
    ParseCalendarEvent,
    #[serde(rename = "CalendarEvent/copy")]
    CopyCalendarEvent,
    #[serde(rename = "CalendarEventNotification/get")]
    GetCalendarEventNotification,
    #[serde(rename = "CalendarEventNotification/changes")]
    ChangesCalendarEventNotification,
    #[serde(rename = "CalendarEventNotification/query")]
    QueryCalendarEventNotification,
    #[serde(rename = "CalendarEventNotification/queryChanges")]
    QueryChangesCalendarEventNotification,
    #[serde(rename = "CalendarEventNotification/set")]
    SetCalendarEventNotification,
    #[serde(rename = "ParticipantIdentity/get")]
    GetParticipantIdentity,
    #[serde(rename = "ParticipantIdentity/changes")]
    ChangesParticipantIdentity,
    #[serde(rename = "ParticipantIdentity/set")]
    SetParticipantIdentity,
    #[serde(rename = "AddressBook/get")]
    GetAddressBook,
    #[serde(rename = "AddressBook/changes")]
    ChangesAddressBook,
    #[serde(rename = "AddressBook/set")]
    SetAddressBook,
    #[serde(rename = "ContactCard/get")]
    GetContactCard,
    #[serde(rename = "ContactCard/changes")]
    ChangesContactCard,
    #[serde(rename = "ContactCard/query")]
    QueryContactCard,
    #[serde(rename = "ContactCard/queryChanges")]
    QueryChangesContactCard,
    #[serde(rename = "ContactCard/set")]
    SetContactCard,
    #[serde(rename = "ContactCard/parse")]
    ParseContactCard,
    #[serde(rename = "ContactCard/copy")]
    CopyContactCard,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum DataType {
    #[serde(rename = "Email")]
    Email = 0,
    #[serde(rename = "EmailDelivery")]
    EmailDelivery = 1,
    #[serde(rename = "EmailSubmission")]
    EmailSubmission = 2,
    #[serde(rename = "Mailbox")]
    Mailbox = 3,
    #[serde(rename = "Thread")]
    Thread = 4,
    #[serde(rename = "Identity")]
    Identity = 5,
    #[serde(rename = "Core")]
    Core = 6,
    #[serde(rename = "PushSubscription")]
    PushSubscription = 7,
    #[serde(rename = "SearchSnippet")]
    SearchSnippet = 8,
    #[serde(rename = "VacationResponse")]
    VacationResponse = 9,
    #[serde(rename = "MDN")]
    Mdn = 10,
    #[serde(rename = "Quota")]
    Quota = 11,
    #[serde(rename = "SieveScript")]
    SieveScript = 12,
    #[serde(rename = "Calendar")]
    Calendar = 13,
    #[serde(rename = "CalendarEvent")]
    CalendarEvent = 14,
    #[serde(rename = "CalendarEventNotification")]
    CalendarEventNotification = 15,
    #[serde(rename = "AddressBook")]
    AddressBook = 16,
    #[serde(rename = "ContactCard")]
    ContactCard = 17,
    #[serde(rename = "FileNode")]
    FileNode = 18,
    #[serde(rename = "Principal")]
    Principal = 19,
    #[serde(rename = "ShareNotification")]
    ShareNotification = 20,
    #[serde(rename = "ParticipantIdentity")]
    ParticipantIdentity = 21,
    #[serde(rename = "CalendarAlert")]
    CalendarAlert = 22,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
pub enum PushObject {
    StateChange {
        changed: AHashMap<String, AHashMap<DataType, String>>,
    },
    EmailPush {
        #[serde(rename = "accountId")]
        account_id: String,
        email: serde_json::Value,
    },
    CalendarAlert(CalendarAlert),
    Group {
        entries: Vec<PushObject>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarAlert {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "calendarEventId")]
    pub calendar_event_id: String,
    pub uid: String,
    #[serde(rename = "recurrenceId")]
    pub recurrence_id: Option<String>,
    #[serde(rename = "alertId")]
    pub alert_id: String,
}

#[derive(Debug, Clone)]
pub struct Get;
#[derive(Debug, Clone)]
pub struct Set;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Transport(reqwest::Error),
    Parse(serde_json::Error),
    Internal(String),
    Problem(Box<ProblemDetails>),
    Server(String),
    Method(MethodError),
    Set(SetError<String>),
    #[cfg(feature = "websockets")]
    WebSocket(tokio_tungstenite::tungstenite::error::Error),
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Transport(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parse(e)
    }
}

impl From<MethodError> for Error {
    fn from(e: MethodError) -> Self {
        Error::Method(e)
    }
}

impl From<ProblemDetails> for Error {
    fn from(e: ProblemDetails) -> Self {
        Error::Problem(Box::new(e))
    }
}

impl From<SetError<String>> for Error {
    fn from(e: SetError<String>) -> Self {
        Error::Set(e)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Internal(s.to_string())
    }
}

#[cfg(feature = "websockets")]
impl From<tokio_tungstenite::tungstenite::error::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::error::Error) -> Self {
        Error::WebSocket(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Transport(e) => write!(f, "Transport error: {e}"),
            Error::Parse(e) => write!(f, "Parse error: {e}"),
            Error::Internal(e) => write!(f, "Internal error: {e}"),
            Error::Problem(e) => write!(f, "Request failed: {e}"),
            Error::Server(e) => write!(f, "Server failed: {e}"),
            Error::Method(e) => write!(f, "Request failed: {e}"),
            Error::Set(e) => write!(f, "Set failed: {e}"),
            #[cfg(feature = "websockets")]
            Error::WebSocket(e) => write!(f, "WebSockets error: {e}"),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Mailbox => write!(f, "Mailbox"),
            DataType::Thread => write!(f, "Thread"),
            DataType::Email => write!(f, "Email"),
            DataType::EmailDelivery => write!(f, "EmailDelivery"),
            DataType::Identity => write!(f, "Identity"),
            DataType::EmailSubmission => write!(f, "EmailSubmission"),
            DataType::CalendarAlert => write!(f, "CalendarAlert"),
            DataType::Core => write!(f, "Core"),
            DataType::PushSubscription => write!(f, "PushSubscription"),
            DataType::SearchSnippet => write!(f, "SearchSnippet"),
            DataType::VacationResponse => write!(f, "VacationResponse"),
            DataType::Mdn => write!(f, "MDN"),
            DataType::Quota => write!(f, "Quota"),
            DataType::SieveScript => write!(f, "SieveScript"),
            DataType::Calendar => write!(f, "Calendar"),
            DataType::CalendarEvent => write!(f, "CalendarEvent"),
            DataType::CalendarEventNotification => write!(f, "CalendarEventNotification"),
            DataType::AddressBook => write!(f, "AddressBook"),
            DataType::ContactCard => write!(f, "ContactCard"),
            DataType::FileNode => write!(f, "FileNode"),
            DataType::Principal => write!(f, "Principal"),
            DataType::ShareNotification => write!(f, "ShareNotification"),
            DataType::ParticipantIdentity => write!(f, "ParticipantIdentity"),
        }
    }
}

#[cfg(test)]
mod tests;
