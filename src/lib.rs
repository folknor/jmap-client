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

#[cfg(feature = "contacts")]
pub mod address_book;
pub mod blob;
#[cfg(feature = "calendars")]
pub mod calendar;
#[cfg(feature = "calendars")]
pub mod calendar_event;
#[cfg(feature = "calendars")]
pub mod calendar_event_notification;
pub mod client;
#[cfg(feature = "contacts")]
pub mod contact_card;
pub mod core;
#[cfg(feature = "mail")]
pub mod email;
#[cfg(feature = "mail")]
pub mod email_submission;
pub mod event_source;
#[cfg(feature = "mail")]
pub mod identity;
#[cfg(feature = "mail")]
pub mod mailbox;
#[cfg(feature = "calendars")]
pub mod participant_identity;
pub mod principal;
pub mod push_subscription;
#[cfg(feature = "quota")]
pub mod quota;
#[cfg(feature = "mail")]
pub mod sieve;
#[cfg(feature = "mail")]
pub mod thread;
pub mod transport_reqwest;
#[cfg(feature = "mail")]
pub mod vacation_response;

use crate::core::error::MethodError;
use crate::core::error::ProblemDetails;
use crate::core::set::SetError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg(feature = "websockets")]
pub mod client_ws;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
#[non_exhaustive]
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
#[non_exhaustive]
pub enum PushObject {
    StateChange {
        changed: HashMap<String, HashMap<DataType, String>>,
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
#[non_exhaustive]
pub enum Error {
    Transport(core::transport::TransportError),
    Parse(serde_json::Error),
    Internal(String),
    Problem(Box<ProblemDetails>),
    Method(MethodError),
    Set(SetError<String>),
    #[cfg(feature = "websockets")]
    WebSocket(tokio_tungstenite::tungstenite::error::Error),
}

impl std::error::Error for Error {}

impl From<core::transport::TransportError> for Error {
    fn from(e: core::transport::TransportError) -> Self {
        Error::Transport(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Transport(core::transport::TransportError::with_source("HTTP request failed", e))
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
            Error::Method(e) => write!(f, "Method error: {e}"),
            Error::Set(e) => write!(f, "Set error: {e}"),
            #[cfg(feature = "websockets")]
            Error::WebSocket(e) => write!(f, "WebSocket error: {e}"),
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
