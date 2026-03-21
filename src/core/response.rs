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
    blob::copy::CopyBlobResponse,
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
    principal::Principal,
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
    CopyBlob(CopyBlobResponse),
    GetPushSubscription(PushSubscriptionGetResponse),
    SetPushSubscription(PushSubscriptionSetResponse),
    GetMailbox(MailboxGetResponse),
    ChangesMailbox(MailboxChangesResponse),
    QueryMailbox(QueryResponse),
    QueryChangesMailbox(QueryChangesResponse),
    SetMailbox(MailboxSetResponse),
    GetThread(ThreadGetResponse),
    ChangesThread(ThreadChangesResponse),
    GetEmail(EmailGetResponse),
    ChangesEmail(EmailChangesResponse),
    QueryEmail(QueryResponse),
    QueryChangesEmail(QueryChangesResponse),
    SetEmail(EmailSetResponse),
    CopyEmail(EmailCopyResponse),
    ImportEmail(EmailImportResponse),
    ParseEmail(EmailParseResponse),
    GetSearchSnippet(SearchSnippetGetResponse),
    GetIdentity(IdentityGetResponse),
    ChangesIdentity(IdentityChangesResponse),
    SetIdentity(IdentitySetResponse),
    GetEmailSubmission(EmailSubmissionGetResponse),
    ChangesEmailSubmission(EmailSubmissionChangesResponse),
    QueryEmailSubmission(QueryResponse),
    QueryChangesEmailSubmission(QueryChangesResponse),
    SetEmailSubmission(EmailSubmissionSetResponse),
    GetVacationResponse(VacationResponseGetResponse),
    SetVacationResponse(VacationResponseSetResponse),
    GetSieveScript(SieveScriptGetResponse),
    QuerySieveScript(QueryResponse),
    SetSieveScript(SieveScriptSetResponse),
    ValidateSieveScript(SieveScriptValidateResponse),

    GetPrincipal(PrincipalGetResponse),
    ChangesPrincipal(PrincipalChangesResponse),
    QueryPrincipal(QueryResponse),
    QueryChangesPrincipal(QueryChangesResponse),
    SetPrincipal(PrincipalSetResponse),

    GetCalendar(CalendarGetResponse),
    ChangesCalendar(CalendarChangesResponse),
    SetCalendar(CalendarSetResponse),
    GetCalendarEvent(CalendarEventGetResponse),
    ChangesCalendarEvent(CalendarEventChangesResponse),
    QueryCalendarEvent(QueryResponse),
    QueryChangesCalendarEvent(QueryChangesResponse),
    SetCalendarEvent(CalendarEventSetResponse),
    ParseCalendarEvent(CalendarEventParseResponse),
    CopyCalendarEvent(CalendarEventCopyResponse),
    GetCalendarEventNotification(CalendarEventNotificationGetResponse),
    ChangesCalendarEventNotification(CalendarEventNotificationChangesResponse),
    QueryCalendarEventNotification(QueryResponse),
    QueryChangesCalendarEventNotification(QueryChangesResponse),
    SetCalendarEventNotification(CalendarEventNotificationSetResponse),
    GetParticipantIdentity(ParticipantIdentityGetResponse),
    ChangesParticipantIdentity(ParticipantIdentityChangesResponse),
    SetParticipantIdentity(ParticipantIdentitySetResponse),

    GetAddressBook(AddressBookGetResponse),
    ChangesAddressBook(AddressBookChangesResponse),
    SetAddressBook(AddressBookSetResponse),
    GetContactCard(ContactCardGetResponse),
    ChangesContactCard(ContactCardChangesResponse),
    QueryContactCard(QueryResponse),
    QueryChangesContactCard(QueryChangesResponse),
    SetContactCard(ContactCardSetResponse),
    ParseContactCard(ContactCardParseResponse),
    CopyContactCard(ContactCardCopyResponse),

    Echo(serde_json::Value),
    Error(MethodError),
}

impl TaggedMethodResponse {
    pub fn call_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn is_type(&self, type_: Method) -> bool {
        matches!(
            (&self.response, type_),
            (MethodResponse::CopyBlob(_), Method::CopyBlob)
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

    pub fn unwrap_copy_blob(self) -> crate::Result<CopyBlobResponse> {
        match self.response {
            MethodResponse::CopyBlob(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_push_subscription(self) -> crate::Result<PushSubscriptionGetResponse> {
        match self.response {
            MethodResponse::GetPushSubscription(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_push_subscription(self) -> crate::Result<PushSubscriptionSetResponse> {
        match self.response {
            MethodResponse::SetPushSubscription(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_mailbox(self) -> crate::Result<MailboxGetResponse> {
        match self.response {
            MethodResponse::GetMailbox(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_mailbox(self) -> crate::Result<MailboxChangesResponse> {
        match self.response {
            MethodResponse::ChangesMailbox(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_mailbox(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryMailbox(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_mailbox(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesMailbox(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_mailbox(self) -> crate::Result<MailboxSetResponse> {
        match self.response {
            MethodResponse::SetMailbox(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_thread(self) -> crate::Result<ThreadGetResponse> {
        match self.response {
            MethodResponse::GetThread(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_thread(self) -> crate::Result<ThreadChangesResponse> {
        match self.response {
            MethodResponse::ChangesThread(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_email(self) -> crate::Result<EmailGetResponse> {
        match self.response {
            MethodResponse::GetEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_email(self) -> crate::Result<EmailChangesResponse> {
        match self.response {
            MethodResponse::ChangesEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_email(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_email(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_email(self) -> crate::Result<EmailSetResponse> {
        match self.response {
            MethodResponse::SetEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_email(self) -> crate::Result<EmailCopyResponse> {
        match self.response {
            MethodResponse::CopyEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_import_email(self) -> crate::Result<EmailImportResponse> {
        match self.response {
            MethodResponse::ImportEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_parse_email(self) -> crate::Result<EmailParseResponse> {
        match self.response {
            MethodResponse::ParseEmail(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_search_snippet(self) -> crate::Result<SearchSnippetGetResponse> {
        match self.response {
            MethodResponse::GetSearchSnippet(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_identity(self) -> crate::Result<IdentityGetResponse> {
        match self.response {
            MethodResponse::GetIdentity(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_identity(self) -> crate::Result<IdentityChangesResponse> {
        match self.response {
            MethodResponse::ChangesIdentity(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_identity(self) -> crate::Result<IdentitySetResponse> {
        match self.response {
            MethodResponse::SetIdentity(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_email_submission(self) -> crate::Result<EmailSubmissionGetResponse> {
        match self.response {
            MethodResponse::GetEmailSubmission(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_email_submission(self) -> crate::Result<EmailSubmissionChangesResponse> {
        match self.response {
            MethodResponse::ChangesEmailSubmission(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_email_submission(self) -> crate::Result<EmailSubmissionSetResponse> {
        match self.response {
            MethodResponse::SetEmailSubmission(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_email_submission(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryEmailSubmission(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_email_submission(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesEmailSubmission(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_vacation_response(self) -> crate::Result<VacationResponseGetResponse> {
        match self.response {
            MethodResponse::GetVacationResponse(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_vacation_response(self) -> crate::Result<VacationResponseSetResponse> {
        match self.response {
            MethodResponse::SetVacationResponse(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_sieve_script(self) -> crate::Result<SieveScriptGetResponse> {
        match self.response {
            MethodResponse::GetSieveScript(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_validate_sieve_script(self) -> crate::Result<SieveScriptValidateResponse> {
        match self.response {
            MethodResponse::ValidateSieveScript(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_sieve_script(self) -> crate::Result<SieveScriptSetResponse> {
        match self.response {
            MethodResponse::SetSieveScript(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_sieve_script(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QuerySieveScript(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_principal(self) -> crate::Result<PrincipalGetResponse> {
        match self.response {
            MethodResponse::GetPrincipal(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_principal(self) -> crate::Result<PrincipalChangesResponse> {
        match self.response {
            MethodResponse::ChangesPrincipal(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_principal(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryPrincipal(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_principal(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesPrincipal(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_principal(self) -> crate::Result<PrincipalSetResponse> {
        match self.response {
            MethodResponse::SetPrincipal(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_calendar(self) -> crate::Result<CalendarGetResponse> {
        match self.response {
            MethodResponse::GetCalendar(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_calendar(self) -> crate::Result<CalendarChangesResponse> {
        match self.response {
            MethodResponse::ChangesCalendar(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_calendar(self) -> crate::Result<CalendarSetResponse> {
        match self.response {
            MethodResponse::SetCalendar(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_calendar_event(self) -> crate::Result<CalendarEventGetResponse> {
        match self.response {
            MethodResponse::GetCalendarEvent(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_calendar_event(self) -> crate::Result<CalendarEventChangesResponse> {
        match self.response {
            MethodResponse::ChangesCalendarEvent(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_calendar_event(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryCalendarEvent(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_calendar_event(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesCalendarEvent(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_calendar_event(self) -> crate::Result<CalendarEventSetResponse> {
        match self.response {
            MethodResponse::SetCalendarEvent(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_parse_calendar_event(self) -> crate::Result<CalendarEventParseResponse> {
        match self.response {
            MethodResponse::ParseCalendarEvent(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_calendar_event(self) -> crate::Result<CalendarEventCopyResponse> {
        match self.response {
            MethodResponse::CopyCalendarEvent(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationGetResponse> {
        match self.response {
            MethodResponse::GetCalendarEventNotification(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationChangesResponse> {
        match self.response {
            MethodResponse::ChangesCalendarEventNotification(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_calendar_event_notification(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryCalendarEventNotification(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_calendar_event_notification(
        self,
    ) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesCalendarEventNotification(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationSetResponse> {
        match self.response {
            MethodResponse::SetCalendarEventNotification(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentityGetResponse> {
        match self.response {
            MethodResponse::GetParticipantIdentity(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentityChangesResponse> {
        match self.response {
            MethodResponse::ChangesParticipantIdentity(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentitySetResponse> {
        match self.response {
            MethodResponse::SetParticipantIdentity(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_address_book(self) -> crate::Result<AddressBookGetResponse> {
        match self.response {
            MethodResponse::GetAddressBook(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_address_book(self) -> crate::Result<AddressBookChangesResponse> {
        match self.response {
            MethodResponse::ChangesAddressBook(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_address_book(self) -> crate::Result<AddressBookSetResponse> {
        match self.response {
            MethodResponse::SetAddressBook(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_contact_card(self) -> crate::Result<ContactCardGetResponse> {
        match self.response {
            MethodResponse::GetContactCard(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_contact_card(self) -> crate::Result<ContactCardChangesResponse> {
        match self.response {
            MethodResponse::ChangesContactCard(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_contact_card(self) -> crate::Result<QueryResponse> {
        match self.response {
            MethodResponse::QueryContactCard(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_contact_card(self) -> crate::Result<QueryChangesResponse> {
        match self.response {
            MethodResponse::QueryChangesContactCard(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_contact_card(self) -> crate::Result<ContactCardSetResponse> {
        match self.response {
            MethodResponse::SetContactCard(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_parse_contact_card(self) -> crate::Result<ContactCardParseResponse> {
        match self.response {
            MethodResponse::ParseContactCard(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_contact_card(self) -> crate::Result<ContactCardCopyResponse> {
        match self.response {
            MethodResponse::CopyContactCard(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_echo(self) -> crate::Result<serde_json::Value> {
        match self.response {
            MethodResponse::Echo(response) => Ok(response),
            MethodResponse::Error(err) => Err(err.into()),
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
            Method::Echo => MethodResponse::Echo(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::CopyBlob => MethodResponse::CopyBlob(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetPushSubscription => MethodResponse::GetPushSubscription(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetPushSubscription => MethodResponse::SetPushSubscription(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetMailbox => MethodResponse::GetMailbox(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesMailbox => MethodResponse::ChangesMailbox(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryMailbox => MethodResponse::QueryMailbox(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryChangesMailbox => MethodResponse::QueryChangesMailbox(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetMailbox => MethodResponse::SetMailbox(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetThread => MethodResponse::GetThread(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesThread => MethodResponse::ChangesThread(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetEmail => MethodResponse::GetEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesEmail => MethodResponse::ChangesEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryEmail => MethodResponse::QueryEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryChangesEmail => MethodResponse::QueryChangesEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetEmail => MethodResponse::SetEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::CopyEmail => MethodResponse::CopyEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ImportEmail => MethodResponse::ImportEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ParseEmail => MethodResponse::ParseEmail(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetSearchSnippet => MethodResponse::GetSearchSnippet(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetIdentity => MethodResponse::GetIdentity(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesIdentity => MethodResponse::ChangesIdentity(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetIdentity => MethodResponse::SetIdentity(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetEmailSubmission => MethodResponse::GetEmailSubmission(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesEmailSubmission => MethodResponse::ChangesEmailSubmission(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryEmailSubmission => MethodResponse::QueryEmailSubmission(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryChangesEmailSubmission => MethodResponse::QueryChangesEmailSubmission(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetEmailSubmission => MethodResponse::SetEmailSubmission(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetVacationResponse => MethodResponse::GetVacationResponse(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetVacationResponse => MethodResponse::SetVacationResponse(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetSieveScript => MethodResponse::GetSieveScript(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetSieveScript => MethodResponse::SetSieveScript(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QuerySieveScript => MethodResponse::QuerySieveScript(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ValidateSieveScript => MethodResponse::ValidateSieveScript(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetPrincipal => MethodResponse::GetPrincipal(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesPrincipal => MethodResponse::ChangesPrincipal(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryPrincipal => MethodResponse::QueryPrincipal(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryChangesPrincipal => MethodResponse::QueryChangesPrincipal(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetPrincipal => MethodResponse::SetPrincipal(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetCalendar => MethodResponse::GetCalendar(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesCalendar => MethodResponse::ChangesCalendar(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetCalendar => MethodResponse::SetCalendar(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetCalendarEvent => MethodResponse::GetCalendarEvent(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesCalendarEvent => MethodResponse::ChangesCalendarEvent(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryCalendarEvent => MethodResponse::QueryCalendarEvent(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryChangesCalendarEvent => MethodResponse::QueryChangesCalendarEvent(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetCalendarEvent => MethodResponse::SetCalendarEvent(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ParseCalendarEvent => MethodResponse::ParseCalendarEvent(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::CopyCalendarEvent => MethodResponse::CopyCalendarEvent(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetCalendarEventNotification => {
                MethodResponse::GetCalendarEventNotification(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                )
            }
            Method::ChangesCalendarEventNotification => {
                MethodResponse::ChangesCalendarEventNotification(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                )
            }
            Method::QueryCalendarEventNotification => {
                MethodResponse::QueryCalendarEventNotification(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                )
            }
            Method::QueryChangesCalendarEventNotification => {
                MethodResponse::QueryChangesCalendarEventNotification(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                )
            }
            Method::SetCalendarEventNotification => {
                MethodResponse::SetCalendarEventNotification(
                    seq.next_element()?
                        .ok_or_else(|| {
                            serde::de::Error::custom("Expected a method response")
                        })?,
                )
            }
            Method::GetParticipantIdentity => MethodResponse::GetParticipantIdentity(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesParticipantIdentity => MethodResponse::ChangesParticipantIdentity(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetParticipantIdentity => MethodResponse::SetParticipantIdentity(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetAddressBook => MethodResponse::GetAddressBook(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesAddressBook => MethodResponse::ChangesAddressBook(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetAddressBook => MethodResponse::SetAddressBook(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::GetContactCard => MethodResponse::GetContactCard(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ChangesContactCard => MethodResponse::ChangesContactCard(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryContactCard => MethodResponse::QueryContactCard(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::QueryChangesContactCard => MethodResponse::QueryChangesContactCard(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::SetContactCard => MethodResponse::SetContactCard(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::ParseContactCard => MethodResponse::ParseContactCard(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::CopyContactCard => MethodResponse::CopyContactCard(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
            Method::Error => MethodResponse::Error(
                seq.next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
            ),
        };

        let id = seq
            .next_element::<String>()?
            .ok_or_else(|| serde::de::Error::custom("Expected method call id"))?;

        Ok(TaggedMethodResponse { response, id })
    }
}
