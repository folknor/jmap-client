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

use ahash::AHashMap;
use serde::{de::Visitor, Deserialize};
use std::fmt;

use crate::{
    address_book::AddressBook,
    blob::{
        copy::CopyBlobResponse,
        manage::{BlobGetResponse, BlobLookupResponse, BlobUploadResponse},
    },
    calendar::Calendar,
    calendar_event::{parse::CalendarEventParseResponse, CalendarEvent},
    calendar_event_notification::CalendarEventNotification,
    contact_card::{parse::ContactCardParseResponse, ContactCard},
    email::{
        import::EmailImportResponse, parse::EmailParseResponse,
        search_snippet::SearchSnippetGetResponse, Email,
    },
    email_submission::EmailSubmission,
    identity::Identity,
    mailbox::Mailbox,
    participant_identity::ParticipantIdentity,
    principal::{availability::PrincipalGetAvailabilityResponse, Principal},
    quota::Quota,
    push_subscription::PushSubscription,
    sieve::{validate::SieveScriptValidateResponse, SieveScript},
    thread::Thread,
    vacation_response::VacationResponse,
    Get, Method,
};

use super::{
    changes::ChangesResponse, copy::CopyResponse, error::MethodError, get::GetResponse,
    query::QueryResponse, query_changes::QueryChangesResponse, set::SetResponse,
};

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "methodResponses")]
    method_responses: Vec<T>,

    #[serde(rename = "createdIds")]
    created_ids: Option<AHashMap<String, String>>,

    #[serde(rename = "sessionState")]
    session_state: String,

    request_id: Option<String>,
}

impl<T> Response<T> {
    pub fn new(
        method_responses: Vec<T>,
        created_ids: Option<AHashMap<String, String>>,
        session_state: String,
        request_id: Option<String>,
    ) -> Self {
        Response {
            method_responses,
            created_ids,
            session_state,
            request_id,
        }
    }

    pub fn method_responses(&self) -> &[T] {
        self.method_responses.as_ref()
    }

    pub fn unwrap_method_responses(self) -> Vec<T> {
        self.method_responses
    }

    pub fn method_response_by_pos(&mut self, index: usize) -> T {
        self.method_responses.remove(index)
    }

    pub fn pop_method_response(&mut self) -> Option<T> {
        self.method_responses.pop()
    }

    pub fn created_ids(&self) -> Option<impl Iterator<Item = (&String, &String)>> {
        self.created_ids.as_ref().map(|map| map.iter())
    }

    pub fn session_state(&self) -> &str {
        &self.session_state
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl Response<TaggedMethodResponse> {
    pub fn method_response_by_id(&self, id: &str) -> Option<&TaggedMethodResponse> {
        self.method_responses
            .iter()
            .find(|response| response.call_id() == id)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SingleMethodResponse<T> {
    Error((Error, MethodError, String)),
    Ok((String, T, String)),
}

#[derive(Debug, Deserialize)]
pub enum Error {
    #[serde(rename = "error")]
    Error,
}

pub type PushSubscriptionSetResponse = SetResponse<PushSubscription<Get>>;
pub type PushSubscriptionGetResponse = GetResponse<PushSubscription<Get>>;
pub type MailboxChangesResponse = ChangesResponse<Mailbox<Get>>;
pub type MailboxSetResponse = SetResponse<Mailbox<Get>>;
pub type MailboxGetResponse = GetResponse<Mailbox<Get>>;
pub type ThreadGetResponse = GetResponse<Thread>;
pub type ThreadChangesResponse = ChangesResponse<Thread>;
pub type EmailGetResponse = GetResponse<Email<Get>>;
pub type EmailSetResponse = SetResponse<Email<Get>>;
pub type EmailCopyResponse = CopyResponse<Email<Get>>;
pub type EmailChangesResponse = ChangesResponse<Email<Get>>;
pub type IdentitySetResponse = SetResponse<Identity<Get>>;
pub type IdentityGetResponse = GetResponse<Identity<Get>>;
pub type IdentityChangesResponse = ChangesResponse<Identity<Get>>;
pub type EmailSubmissionSetResponse = SetResponse<EmailSubmission<Get>>;
pub type EmailSubmissionGetResponse = GetResponse<EmailSubmission<Get>>;
pub type EmailSubmissionChangesResponse = ChangesResponse<EmailSubmission<Get>>;
pub type VacationResponseGetResponse = GetResponse<VacationResponse<Get>>;
pub type VacationResponseSetResponse = SetResponse<VacationResponse<Get>>;
pub type SieveScriptGetResponse = GetResponse<SieveScript<Get>>;
pub type SieveScriptSetResponse = SetResponse<SieveScript<Get>>;
pub type PrincipalChangesResponse = ChangesResponse<Principal<Get>>;
pub type PrincipalSetResponse = SetResponse<Principal<Get>>;
pub type PrincipalGetResponse = GetResponse<Principal<Get>>;
pub type QuotaGetResponse = GetResponse<Quota<Get>>;
pub type QuotaChangesResponse = ChangesResponse<Quota<Get>>;
pub type CalendarGetResponse = GetResponse<Calendar<Get>>;
pub type CalendarSetResponse = SetResponse<Calendar<Get>>;
pub type CalendarChangesResponse = ChangesResponse<Calendar<Get>>;
pub type CalendarEventGetResponse = GetResponse<CalendarEvent<Get>>;
pub type CalendarEventSetResponse = SetResponse<CalendarEvent<Get>>;
pub type CalendarEventChangesResponse = ChangesResponse<CalendarEvent<Get>>;
pub type CalendarEventCopyResponse = CopyResponse<CalendarEvent<Get>>;
pub type CalendarEventNotificationGetResponse = GetResponse<CalendarEventNotification<Get>>;
pub type CalendarEventNotificationSetResponse = SetResponse<CalendarEventNotification<Get>>;
pub type CalendarEventNotificationChangesResponse =
    ChangesResponse<CalendarEventNotification<Get>>;
pub type ParticipantIdentityGetResponse = GetResponse<ParticipantIdentity<Get>>;
pub type ParticipantIdentitySetResponse = SetResponse<ParticipantIdentity<Get>>;
pub type ParticipantIdentityChangesResponse = ChangesResponse<ParticipantIdentity<Get>>;
pub type AddressBookGetResponse = GetResponse<AddressBook<Get>>;
pub type AddressBookSetResponse = SetResponse<AddressBook<Get>>;
pub type AddressBookChangesResponse = ChangesResponse<AddressBook<Get>>;
pub type ContactCardGetResponse = GetResponse<ContactCard<Get>>;
pub type ContactCardSetResponse = SetResponse<ContactCard<Get>>;
pub type ContactCardChangesResponse = ChangesResponse<ContactCard<Get>>;
pub type ContactCardCopyResponse = CopyResponse<ContactCard<Get>>;

#[derive(Debug)]
pub struct TaggedMethodResponse {
    id: String,
    response: MethodResponse,
}

#[derive(Debug)]
pub enum MethodResponse {
    CopyBlob(Box<CopyBlobResponse>),
    UploadBlob(Box<BlobUploadResponse>),
    GetBlob(Box<BlobGetResponse>),
    LookupBlob(Box<BlobLookupResponse>),
    GetPushSubscription(Box<PushSubscriptionGetResponse>),
    SetPushSubscription(Box<PushSubscriptionSetResponse>),
    GetMailbox(Box<MailboxGetResponse>),
    ChangesMailbox(Box<MailboxChangesResponse>),
    QueryMailbox(Box<QueryResponse>),
    QueryChangesMailbox(Box<QueryChangesResponse>),
    SetMailbox(Box<MailboxSetResponse>),
    GetThread(Box<ThreadGetResponse>),
    ChangesThread(Box<ThreadChangesResponse>),
    GetEmail(Box<EmailGetResponse>),
    ChangesEmail(Box<EmailChangesResponse>),
    QueryEmail(Box<QueryResponse>),
    QueryChangesEmail(Box<QueryChangesResponse>),
    SetEmail(Box<EmailSetResponse>),
    CopyEmail(Box<EmailCopyResponse>),
    ImportEmail(Box<EmailImportResponse>),
    ParseEmail(Box<EmailParseResponse>),
    GetSearchSnippet(Box<SearchSnippetGetResponse>),
    GetIdentity(Box<IdentityGetResponse>),
    ChangesIdentity(Box<IdentityChangesResponse>),
    SetIdentity(Box<IdentitySetResponse>),
    GetEmailSubmission(Box<EmailSubmissionGetResponse>),
    ChangesEmailSubmission(Box<EmailSubmissionChangesResponse>),
    QueryEmailSubmission(Box<QueryResponse>),
    QueryChangesEmailSubmission(Box<QueryChangesResponse>),
    SetEmailSubmission(Box<EmailSubmissionSetResponse>),
    GetVacationResponse(Box<VacationResponseGetResponse>),
    SetVacationResponse(Box<VacationResponseSetResponse>),
    GetSieveScript(Box<SieveScriptGetResponse>),
    QuerySieveScript(Box<QueryResponse>),
    SetSieveScript(Box<SieveScriptSetResponse>),
    ValidateSieveScript(Box<SieveScriptValidateResponse>),

    GetPrincipal(Box<PrincipalGetResponse>),
    ChangesPrincipal(Box<PrincipalChangesResponse>),
    QueryPrincipal(Box<QueryResponse>),
    QueryChangesPrincipal(Box<QueryChangesResponse>),
    SetPrincipal(Box<PrincipalSetResponse>),
    GetAvailabilityPrincipal(Box<PrincipalGetAvailabilityResponse>),

    GetQuota(Box<QuotaGetResponse>),
    ChangesQuota(Box<QuotaChangesResponse>),
    QueryQuota(Box<QueryResponse>),
    QueryChangesQuota(Box<QueryChangesResponse>),

    GetCalendar(Box<CalendarGetResponse>),
    ChangesCalendar(Box<CalendarChangesResponse>),
    SetCalendar(Box<CalendarSetResponse>),
    GetCalendarEvent(Box<CalendarEventGetResponse>),
    ChangesCalendarEvent(Box<CalendarEventChangesResponse>),
    QueryCalendarEvent(Box<QueryResponse>),
    QueryChangesCalendarEvent(Box<QueryChangesResponse>),
    SetCalendarEvent(Box<CalendarEventSetResponse>),
    ParseCalendarEvent(Box<CalendarEventParseResponse>),
    CopyCalendarEvent(Box<CalendarEventCopyResponse>),
    GetCalendarEventNotification(Box<CalendarEventNotificationGetResponse>),
    ChangesCalendarEventNotification(Box<CalendarEventNotificationChangesResponse>),
    QueryCalendarEventNotification(Box<QueryResponse>),
    QueryChangesCalendarEventNotification(Box<QueryChangesResponse>),
    SetCalendarEventNotification(Box<CalendarEventNotificationSetResponse>),
    GetParticipantIdentity(Box<ParticipantIdentityGetResponse>),
    ChangesParticipantIdentity(Box<ParticipantIdentityChangesResponse>),
    SetParticipantIdentity(Box<ParticipantIdentitySetResponse>),

    GetAddressBook(Box<AddressBookGetResponse>),
    ChangesAddressBook(Box<AddressBookChangesResponse>),
    SetAddressBook(Box<AddressBookSetResponse>),
    GetContactCard(Box<ContactCardGetResponse>),
    ChangesContactCard(Box<ContactCardChangesResponse>),
    QueryContactCard(Box<QueryResponse>),
    QueryChangesContactCard(Box<QueryChangesResponse>),
    SetContactCard(Box<ContactCardSetResponse>),
    ParseContactCard(Box<ContactCardParseResponse>),
    CopyContactCard(Box<ContactCardCopyResponse>),

    Echo(Box<serde_json::Value>),
    Error(Box<MethodError>),
}

impl TaggedMethodResponse {
    pub fn call_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn is_type(&self, type_: Method) -> bool {
        matches!(
            (&self.response, type_),
            (MethodResponse::CopyBlob(_), Method::CopyBlob)
                | (MethodResponse::UploadBlob(_), Method::UploadBlob)
                | (MethodResponse::GetBlob(_), Method::GetBlob)
                | (MethodResponse::LookupBlob(_), Method::LookupBlob)
                | (
                    MethodResponse::GetPushSubscription(_),
                    Method::GetPushSubscription
                )
                | (
                    MethodResponse::SetPushSubscription(_),
                    Method::SetPushSubscription
                )
                | (MethodResponse::GetMailbox(_), Method::GetMailbox)
                | (MethodResponse::ChangesMailbox(_), Method::ChangesMailbox)
                | (MethodResponse::QueryMailbox(_), Method::QueryMailbox)
                | (
                    MethodResponse::QueryChangesMailbox(_),
                    Method::QueryChangesMailbox
                )
                | (MethodResponse::SetMailbox(_), Method::SetMailbox)
                | (MethodResponse::GetThread(_), Method::GetThread)
                | (MethodResponse::ChangesThread(_), Method::ChangesThread)
                | (MethodResponse::GetEmail(_), Method::GetEmail)
                | (MethodResponse::ChangesEmail(_), Method::ChangesEmail)
                | (MethodResponse::QueryEmail(_), Method::QueryEmail)
                | (
                    MethodResponse::QueryChangesEmail(_),
                    Method::QueryChangesEmail
                )
                | (MethodResponse::SetEmail(_), Method::SetEmail)
                | (MethodResponse::CopyEmail(_), Method::CopyEmail)
                | (MethodResponse::ImportEmail(_), Method::ImportEmail)
                | (MethodResponse::ParseEmail(_), Method::ParseEmail)
                | (
                    MethodResponse::GetSearchSnippet(_),
                    Method::GetSearchSnippet
                )
                | (MethodResponse::GetIdentity(_), Method::GetIdentity)
                | (MethodResponse::ChangesIdentity(_), Method::ChangesIdentity)
                | (MethodResponse::SetIdentity(_), Method::SetIdentity)
                | (
                    MethodResponse::GetEmailSubmission(_),
                    Method::GetEmailSubmission
                )
                | (
                    MethodResponse::ChangesEmailSubmission(_),
                    Method::ChangesEmailSubmission
                )
                | (
                    MethodResponse::QueryEmailSubmission(_),
                    Method::QueryEmailSubmission
                )
                | (
                    MethodResponse::QueryChangesEmailSubmission(_),
                    Method::QueryChangesEmailSubmission
                )
                | (
                    MethodResponse::SetEmailSubmission(_),
                    Method::SetEmailSubmission
                )
                | (
                    MethodResponse::GetVacationResponse(_),
                    Method::GetVacationResponse
                )
                | (
                    MethodResponse::SetVacationResponse(_),
                    Method::SetVacationResponse
                )
                | (MethodResponse::GetSieveScript(_), Method::GetSieveScript)
                | (
                    MethodResponse::ValidateSieveScript(_),
                    Method::ValidateSieveScript
                )
                | (
                    MethodResponse::QuerySieveScript(_),
                    Method::QuerySieveScript
                )
                | (MethodResponse::SetSieveScript(_), Method::SetSieveScript)
                | (MethodResponse::GetPrincipal(_), Method::GetPrincipal)
                | (
                    MethodResponse::ChangesPrincipal(_),
                    Method::ChangesPrincipal
                )
                | (MethodResponse::QueryPrincipal(_), Method::QueryPrincipal)
                | (
                    MethodResponse::QueryChangesPrincipal(_),
                    Method::QueryChangesPrincipal
                )
                | (MethodResponse::SetPrincipal(_), Method::SetPrincipal)
                | (
                    MethodResponse::GetAvailabilityPrincipal(_),
                    Method::GetAvailabilityPrincipal
                )
                | (MethodResponse::GetQuota(_), Method::GetQuota)
                | (MethodResponse::ChangesQuota(_), Method::ChangesQuota)
                | (MethodResponse::QueryQuota(_), Method::QueryQuota)
                | (
                    MethodResponse::QueryChangesQuota(_),
                    Method::QueryChangesQuota
                )
                | (MethodResponse::GetCalendar(_), Method::GetCalendar)
                | (MethodResponse::ChangesCalendar(_), Method::ChangesCalendar)
                | (MethodResponse::SetCalendar(_), Method::SetCalendar)
                | (MethodResponse::GetCalendarEvent(_), Method::GetCalendarEvent)
                | (
                    MethodResponse::ChangesCalendarEvent(_),
                    Method::ChangesCalendarEvent
                )
                | (
                    MethodResponse::QueryCalendarEvent(_),
                    Method::QueryCalendarEvent
                )
                | (
                    MethodResponse::QueryChangesCalendarEvent(_),
                    Method::QueryChangesCalendarEvent
                )
                | (MethodResponse::SetCalendarEvent(_), Method::SetCalendarEvent)
                | (
                    MethodResponse::ParseCalendarEvent(_),
                    Method::ParseCalendarEvent
                )
                | (
                    MethodResponse::CopyCalendarEvent(_),
                    Method::CopyCalendarEvent
                )
                | (
                    MethodResponse::GetCalendarEventNotification(_),
                    Method::GetCalendarEventNotification
                )
                | (
                    MethodResponse::ChangesCalendarEventNotification(_),
                    Method::ChangesCalendarEventNotification
                )
                | (
                    MethodResponse::QueryCalendarEventNotification(_),
                    Method::QueryCalendarEventNotification
                )
                | (
                    MethodResponse::QueryChangesCalendarEventNotification(_),
                    Method::QueryChangesCalendarEventNotification
                )
                | (
                    MethodResponse::SetCalendarEventNotification(_),
                    Method::SetCalendarEventNotification
                )
                | (
                    MethodResponse::GetParticipantIdentity(_),
                    Method::GetParticipantIdentity
                )
                | (
                    MethodResponse::ChangesParticipantIdentity(_),
                    Method::ChangesParticipantIdentity
                )
                | (
                    MethodResponse::SetParticipantIdentity(_),
                    Method::SetParticipantIdentity
                )
                | (MethodResponse::GetAddressBook(_), Method::GetAddressBook)
                | (
                    MethodResponse::ChangesAddressBook(_),
                    Method::ChangesAddressBook
                )
                | (MethodResponse::SetAddressBook(_), Method::SetAddressBook)
                | (MethodResponse::GetContactCard(_), Method::GetContactCard)
                | (
                    MethodResponse::ChangesContactCard(_),
                    Method::ChangesContactCard
                )
                | (
                    MethodResponse::QueryContactCard(_),
                    Method::QueryContactCard
                )
                | (
                    MethodResponse::QueryChangesContactCard(_),
                    Method::QueryChangesContactCard
                )
                | (MethodResponse::SetContactCard(_), Method::SetContactCard)
                | (
                    MethodResponse::ParseContactCard(_),
                    Method::ParseContactCard
                )
                | (
                    MethodResponse::CopyContactCard(_),
                    Method::CopyContactCard
                )
                | (MethodResponse::Echo(_), Method::Echo)
                | (MethodResponse::Error(_), Method::Error)
        )
    }

    pub fn unwrap_method_response(self) -> MethodResponse {
        self.response
    }

    pub fn unwrap_upload_blob(self) -> crate::Result<BlobUploadResponse> {
        match self.response {
            MethodResponse::UploadBlob(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_blob(self) -> crate::Result<BlobGetResponse> {
        match self.response {
            MethodResponse::GetBlob(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_lookup_blob(self) -> crate::Result<BlobLookupResponse> {
        match self.response {
            MethodResponse::LookupBlob(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_blob(self) -> crate::Result<CopyBlobResponse> {
        match self.response {
            MethodResponse::CopyBlob(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_push_subscription(self) -> crate::Result<PushSubscriptionGetResponse> {
        match self.response {
            MethodResponse::GetPushSubscription(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_push_subscription(self) -> crate::Result<PushSubscriptionSetResponse> {
        match self.response {
            MethodResponse::SetPushSubscription(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_mailbox(self) -> crate::Result<MailboxGetResponse> {
        match self.response {
            MethodResponse::GetMailbox(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_mailbox(self) -> crate::Result<MailboxChangesResponse> {
        match self.response {
            MethodResponse::ChangesMailbox(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_mailbox(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryMailbox(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_mailbox(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesMailbox(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_mailbox(self) -> crate::Result<MailboxSetResponse> {
        match self.response {
            MethodResponse::SetMailbox(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_thread(self) -> crate::Result<ThreadGetResponse> {
        match self.response {
            MethodResponse::GetThread(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_thread(self) -> crate::Result<ThreadChangesResponse> {
        match self.response {
            MethodResponse::ChangesThread(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_email(self) -> crate::Result<EmailGetResponse> {
        match self.response {
            MethodResponse::GetEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_email(self) -> crate::Result<EmailChangesResponse> {
        match self.response {
            MethodResponse::ChangesEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_email(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_email(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_email(self) -> crate::Result<EmailSetResponse> {
        match self.response {
            MethodResponse::SetEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_email(self) -> crate::Result<EmailCopyResponse> {
        match self.response {
            MethodResponse::CopyEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_import_email(self) -> crate::Result<EmailImportResponse> {
        match self.response {
            MethodResponse::ImportEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_parse_email(self) -> crate::Result<EmailParseResponse> {
        match self.response {
            MethodResponse::ParseEmail(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_search_snippet(self) -> crate::Result<SearchSnippetGetResponse> {
        match self.response {
            MethodResponse::GetSearchSnippet(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_identity(self) -> crate::Result<IdentityGetResponse> {
        match self.response {
            MethodResponse::GetIdentity(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_identity(self) -> crate::Result<IdentityChangesResponse> {
        match self.response {
            MethodResponse::ChangesIdentity(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_identity(self) -> crate::Result<IdentitySetResponse> {
        match self.response {
            MethodResponse::SetIdentity(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_email_submission(self) -> crate::Result<EmailSubmissionGetResponse> {
        match self.response {
            MethodResponse::GetEmailSubmission(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_email_submission(self) -> crate::Result<EmailSubmissionChangesResponse> {
        match self.response {
            MethodResponse::ChangesEmailSubmission(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_email_submission(self) -> crate::Result<EmailSubmissionSetResponse> {
        match self.response {
            MethodResponse::SetEmailSubmission(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_email_submission(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryEmailSubmission(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_email_submission(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesEmailSubmission(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_vacation_response(self) -> crate::Result<VacationResponseGetResponse> {
        match self.response {
            MethodResponse::GetVacationResponse(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_vacation_response(self) -> crate::Result<VacationResponseSetResponse> {
        match self.response {
            MethodResponse::SetVacationResponse(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_sieve_script(self) -> crate::Result<SieveScriptGetResponse> {
        match self.response {
            MethodResponse::GetSieveScript(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_validate_sieve_script(self) -> crate::Result<SieveScriptValidateResponse> {
        match self.response {
            MethodResponse::ValidateSieveScript(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_sieve_script(self) -> crate::Result<SieveScriptSetResponse> {
        match self.response {
            MethodResponse::SetSieveScript(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_sieve_script(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QuerySieveScript(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_principal(self) -> crate::Result<PrincipalGetResponse> {
        match self.response {
            MethodResponse::GetPrincipal(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_principal(self) -> crate::Result<PrincipalChangesResponse> {
        match self.response {
            MethodResponse::ChangesPrincipal(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_principal(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryPrincipal(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_principal(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesPrincipal(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_principal(self) -> crate::Result<PrincipalSetResponse> {
        match self.response {
            MethodResponse::SetPrincipal(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_availability_principal(
        self,
    ) -> crate::Result<PrincipalGetAvailabilityResponse> {
        match self.response {
            MethodResponse::GetAvailabilityPrincipal(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_quota(self) -> crate::Result<QuotaGetResponse> {
        match self.response {
            MethodResponse::GetQuota(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_quota(self) -> crate::Result<QuotaChangesResponse> {
        match self.response {
            MethodResponse::ChangesQuota(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_quota(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryQuota(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_quota(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesQuota(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_calendar(self) -> crate::Result<CalendarGetResponse> {
        match self.response {
            MethodResponse::GetCalendar(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_calendar(self) -> crate::Result<CalendarChangesResponse> {
        match self.response {
            MethodResponse::ChangesCalendar(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_calendar(self) -> crate::Result<CalendarSetResponse> {
        match self.response {
            MethodResponse::SetCalendar(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_calendar_event(self) -> crate::Result<CalendarEventGetResponse> {
        match self.response {
            MethodResponse::GetCalendarEvent(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_calendar_event(self) -> crate::Result<CalendarEventChangesResponse> {
        match self.response {
            MethodResponse::ChangesCalendarEvent(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_calendar_event(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryCalendarEvent(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_calendar_event(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesCalendarEvent(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_calendar_event(self) -> crate::Result<CalendarEventSetResponse> {
        match self.response {
            MethodResponse::SetCalendarEvent(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_parse_calendar_event(self) -> crate::Result<CalendarEventParseResponse> {
        match self.response {
            MethodResponse::ParseCalendarEvent(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_calendar_event(self) -> crate::Result<CalendarEventCopyResponse> {
        match self.response {
            MethodResponse::CopyCalendarEvent(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationGetResponse> {
        match self.response {
            MethodResponse::GetCalendarEventNotification(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationChangesResponse> {
        match self.response {
            MethodResponse::ChangesCalendarEventNotification(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_calendar_event_notification(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryCalendarEventNotification(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_calendar_event_notification(
        self,
    ) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesCalendarEventNotification(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationSetResponse> {
        match self.response {
            MethodResponse::SetCalendarEventNotification(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentityGetResponse> {
        match self.response {
            MethodResponse::GetParticipantIdentity(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentityChangesResponse> {
        match self.response {
            MethodResponse::ChangesParticipantIdentity(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentitySetResponse> {
        match self.response {
            MethodResponse::SetParticipantIdentity(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_address_book(self) -> crate::Result<AddressBookGetResponse> {
        match self.response {
            MethodResponse::GetAddressBook(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_address_book(self) -> crate::Result<AddressBookChangesResponse> {
        match self.response {
            MethodResponse::ChangesAddressBook(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_address_book(self) -> crate::Result<AddressBookSetResponse> {
        match self.response {
            MethodResponse::SetAddressBook(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_contact_card(self) -> crate::Result<ContactCardGetResponse> {
        match self.response {
            MethodResponse::GetContactCard(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_contact_card(self) -> crate::Result<ContactCardChangesResponse> {
        match self.response {
            MethodResponse::ChangesContactCard(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_contact_card(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryContactCard(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_contact_card(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesContactCard(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_contact_card(self) -> crate::Result<ContactCardSetResponse> {
        match self.response {
            MethodResponse::SetContactCard(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_parse_contact_card(self) -> crate::Result<ContactCardParseResponse> {
        match self.response {
            MethodResponse::ParseContactCard(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_contact_card(self) -> crate::Result<ContactCardCopyResponse> {
        match self.response {
            MethodResponse::CopyContactCard(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_echo(self) -> crate::Result<serde_json::Value> {
        match self.response {
            MethodResponse::Echo(response) => Ok(*response),
            MethodResponse::Error(err) => Err((*err).into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.response, MethodResponse::Error(_))
    }
}

impl<'de> Deserialize<'de> for TaggedMethodResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(TaggedMethodResponseVisitor)
    }
}

struct TaggedMethodResponseVisitor;

impl<'de> Visitor<'de> for TaggedMethodResponseVisitor {
    type Value = TaggedMethodResponse;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid JMAP method response")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let response = match seq
            .next_element::<Method>()?
            .ok_or_else(|| serde::de::Error::custom("Expected a method name"))?
        {
            Method::Echo => MethodResponse::Echo(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::UploadBlob => MethodResponse::UploadBlob(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetBlob => MethodResponse::GetBlob(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::LookupBlob => MethodResponse::LookupBlob(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::CopyBlob => MethodResponse::CopyBlob(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetPushSubscription => MethodResponse::GetPushSubscription(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetPushSubscription => MethodResponse::SetPushSubscription(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetMailbox => MethodResponse::GetMailbox(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesMailbox => MethodResponse::ChangesMailbox(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryMailbox => MethodResponse::QueryMailbox(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryChangesMailbox => MethodResponse::QueryChangesMailbox(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetMailbox => MethodResponse::SetMailbox(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetThread => MethodResponse::GetThread(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesThread => MethodResponse::ChangesThread(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetEmail => MethodResponse::GetEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesEmail => MethodResponse::ChangesEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryEmail => MethodResponse::QueryEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryChangesEmail => MethodResponse::QueryChangesEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetEmail => MethodResponse::SetEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::CopyEmail => MethodResponse::CopyEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ImportEmail => MethodResponse::ImportEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ParseEmail => MethodResponse::ParseEmail(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetSearchSnippet => MethodResponse::GetSearchSnippet(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetIdentity => MethodResponse::GetIdentity(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesIdentity => MethodResponse::ChangesIdentity(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetIdentity => MethodResponse::SetIdentity(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetEmailSubmission => MethodResponse::GetEmailSubmission(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesEmailSubmission => MethodResponse::ChangesEmailSubmission(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryEmailSubmission => MethodResponse::QueryEmailSubmission(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryChangesEmailSubmission => MethodResponse::QueryChangesEmailSubmission(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetEmailSubmission => MethodResponse::SetEmailSubmission(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetVacationResponse => MethodResponse::GetVacationResponse(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetVacationResponse => MethodResponse::SetVacationResponse(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetSieveScript => MethodResponse::GetSieveScript(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetSieveScript => MethodResponse::SetSieveScript(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QuerySieveScript => MethodResponse::QuerySieveScript(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ValidateSieveScript => MethodResponse::ValidateSieveScript(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetPrincipal => MethodResponse::GetPrincipal(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesPrincipal => MethodResponse::ChangesPrincipal(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryPrincipal => MethodResponse::QueryPrincipal(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryChangesPrincipal => MethodResponse::QueryChangesPrincipal(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetPrincipal => MethodResponse::SetPrincipal(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetAvailabilityPrincipal => MethodResponse::GetAvailabilityPrincipal(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetQuota => MethodResponse::GetQuota(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesQuota => MethodResponse::ChangesQuota(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryQuota => MethodResponse::QueryQuota(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryChangesQuota => MethodResponse::QueryChangesQuota(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetCalendar => MethodResponse::GetCalendar(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesCalendar => MethodResponse::ChangesCalendar(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetCalendar => MethodResponse::SetCalendar(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetCalendarEvent => MethodResponse::GetCalendarEvent(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesCalendarEvent => MethodResponse::ChangesCalendarEvent(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryCalendarEvent => MethodResponse::QueryCalendarEvent(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryChangesCalendarEvent => MethodResponse::QueryChangesCalendarEvent(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetCalendarEvent => MethodResponse::SetCalendarEvent(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ParseCalendarEvent => MethodResponse::ParseCalendarEvent(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::CopyCalendarEvent => MethodResponse::CopyCalendarEvent(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetCalendarEventNotification => {
                MethodResponse::GetCalendarEventNotification(Box::new(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                ))
            }
            Method::ChangesCalendarEventNotification => {
                MethodResponse::ChangesCalendarEventNotification(Box::new(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                ))
            }
            Method::QueryCalendarEventNotification => {
                MethodResponse::QueryCalendarEventNotification(Box::new(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                ))
            }
            Method::QueryChangesCalendarEventNotification => {
                MethodResponse::QueryChangesCalendarEventNotification(Box::new(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                ))
            }
            Method::SetCalendarEventNotification => {
                MethodResponse::SetCalendarEventNotification(Box::new(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                ))
            }
            Method::GetParticipantIdentity => MethodResponse::GetParticipantIdentity(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesParticipantIdentity => MethodResponse::ChangesParticipantIdentity(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetParticipantIdentity => MethodResponse::SetParticipantIdentity(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetAddressBook => MethodResponse::GetAddressBook(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesAddressBook => MethodResponse::ChangesAddressBook(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetAddressBook => MethodResponse::SetAddressBook(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::GetContactCard => MethodResponse::GetContactCard(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ChangesContactCard => MethodResponse::ChangesContactCard(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryContactCard => MethodResponse::QueryContactCard(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::QueryChangesContactCard => MethodResponse::QueryChangesContactCard(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::SetContactCard => MethodResponse::SetContactCard(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::ParseContactCard => MethodResponse::ParseContactCard(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::CopyContactCard => MethodResponse::CopyContactCard(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
            Method::Error => MethodResponse::Error(Box::new(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            )),
        };

        let id = seq
            .next_element::<String>()?
            .ok_or_else(|| serde::de::Error::custom("Expected method call id"))?;

        Ok(TaggedMethodResponse { response, id })
    }
}
