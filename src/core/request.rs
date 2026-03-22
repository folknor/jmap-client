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

use crate::{
    address_book::AddressBook,
    blob::{
        copy::CopyBlobRequest,
        manage::{BlobGetRequest, BlobLookupRequest, BlobUploadRequest},
    },
    calendar::Calendar,
    calendar_event::{parse::CalendarEventParseRequest, CalendarEvent},
    calendar_event_notification::CalendarEventNotification,
    contact_card::{parse::ContactCardParseRequest, ContactCard},
    client::Client,
    email::{
        import::EmailImportRequest, parse::EmailParseRequest,
        search_snippet::SearchSnippetGetRequest, Email,
    },
    email_submission::EmailSubmission,
    identity::Identity,
    mailbox::Mailbox,
    participant_identity::ParticipantIdentity,
    principal::{availability::PrincipalGetAvailabilityRequest, Principal},
    quota::Quota,
    push_subscription::PushSubscription,
    sieve::{validate::SieveScriptValidateRequest, SieveScript},
    thread::Thread,
    vacation_response::VacationResponse,
    Error, Method, Set, URI,
};
use ahash::AHashMap;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    changes::ChangesRequest,
    copy::CopyRequest,
    get::GetRequest,
    query::QueryRequest,
    query_changes::QueryChangesRequest,
    response::{Response, SingleMethodResponse, TaggedMethodResponse},
    set::SetRequest,
    RequestParams,
};

#[derive(Serialize)]
pub struct Request<'x> {
    #[serde(skip)]
    client: &'x Client,
    #[serde(skip)]
    account_id: String,

    pub using: Vec<URI>,

    #[serde(rename = "methodCalls")]
    pub method_calls: Vec<(Method, Arguments, String)>,

    #[serde(rename = "createdIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_ids: Option<AHashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultReference {
    #[serde(rename = "resultOf")]
    result_of: String,
    name: Method,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Arguments {
    Changes(Box<ChangesRequest>),
    PushGet(Box<GetRequest<PushSubscription<Set>>>),
    PushSet(Box<SetRequest<PushSubscription<Set>>>),
    BlobCopy(Box<CopyBlobRequest>),
    BlobUpload(Box<BlobUploadRequest>),
    BlobGet(Box<BlobGetRequest>),
    BlobLookup(Box<BlobLookupRequest>),
    MailboxGet(Box<GetRequest<Mailbox<Set>>>),
    MailboxQuery(Box<QueryRequest<Mailbox<Set>>>),
    MailboxQueryChanges(Box<QueryChangesRequest<Mailbox<Set>>>),
    MailboxSet(Box<SetRequest<Mailbox<Set>>>),
    ThreadGet(Box<GetRequest<Thread>>),
    EmailGet(Box<GetRequest<Email<Set>>>),
    EmailQuery(Box<QueryRequest<Email<Set>>>),
    EmailQueryChanges(Box<QueryChangesRequest<Email<Set>>>),
    EmailSet(Box<SetRequest<Email<Set>>>),
    EmailCopy(Box<CopyRequest<Email<Set>>>),
    EmailImport(Box<EmailImportRequest>),
    EmailParse(Box<EmailParseRequest>),
    SearchSnippetGet(Box<SearchSnippetGetRequest>),
    IdentityGet(Box<GetRequest<Identity<Set>>>),
    IdentitySet(Box<SetRequest<Identity<Set>>>),
    EmailSubmissionGet(Box<GetRequest<EmailSubmission<Set>>>),
    EmailSubmissionQuery(Box<QueryRequest<EmailSubmission<Set>>>),
    EmailSubmissionQueryChanges(Box<QueryChangesRequest<EmailSubmission<Set>>>),
    EmailSubmissionSet(Box<SetRequest<EmailSubmission<Set>>>),
    VacationResponseGet(Box<GetRequest<VacationResponse<Set>>>),
    VacationResponseSet(Box<SetRequest<VacationResponse<Set>>>),
    SieveScriptGet(Box<GetRequest<SieveScript<Set>>>),
    SieveScriptQuery(Box<QueryRequest<SieveScript<Set>>>),
    SieveScriptValidate(Box<SieveScriptValidateRequest>),
    SieveScriptSet(Box<SetRequest<SieveScript<Set>>>),
    PrincipalGet(Box<GetRequest<Principal<Set>>>),
    PrincipalQuery(Box<QueryRequest<Principal<Set>>>),
    PrincipalQueryChanges(Box<QueryChangesRequest<Principal<Set>>>),
    PrincipalSet(Box<SetRequest<Principal<Set>>>),
    PrincipalGetAvailability(Box<PrincipalGetAvailabilityRequest>),
    QuotaGet(Box<GetRequest<Quota<Set>>>),
    QuotaQuery(Box<QueryRequest<Quota<Set>>>),
    QuotaQueryChanges(Box<QueryChangesRequest<Quota<Set>>>),
    CalendarGet(Box<GetRequest<Calendar<Set>>>),
    CalendarSet(Box<SetRequest<Calendar<Set>>>),
    CalendarEventGet(Box<GetRequest<CalendarEvent<Set>>>),
    CalendarEventQuery(Box<QueryRequest<CalendarEvent<Set>>>),
    CalendarEventQueryChanges(Box<QueryChangesRequest<CalendarEvent<Set>>>),
    CalendarEventSet(Box<SetRequest<CalendarEvent<Set>>>),
    CalendarEventParse(Box<CalendarEventParseRequest>),
    CalendarEventCopy(Box<CopyRequest<CalendarEvent<Set>>>),
    CalendarEventNotificationGet(Box<GetRequest<CalendarEventNotification<Set>>>),
    CalendarEventNotificationQuery(Box<QueryRequest<CalendarEventNotification<Set>>>),
    CalendarEventNotificationQueryChanges(Box<QueryChangesRequest<CalendarEventNotification<Set>>>),
    CalendarEventNotificationSet(Box<SetRequest<CalendarEventNotification<Set>>>),
    ParticipantIdentityGet(Box<GetRequest<ParticipantIdentity<Set>>>),
    ParticipantIdentitySet(Box<SetRequest<ParticipantIdentity<Set>>>),
    AddressBookGet(Box<GetRequest<AddressBook<Set>>>),
    AddressBookSet(Box<SetRequest<AddressBook<Set>>>),
    ContactCardGet(Box<GetRequest<ContactCard<Set>>>),
    ContactCardQuery(Box<QueryRequest<ContactCard<Set>>>),
    ContactCardQueryChanges(Box<QueryChangesRequest<ContactCard<Set>>>),
    ContactCardSet(Box<SetRequest<ContactCard<Set>>>),
    ContactCardParse(Box<ContactCardParseRequest>),
    ContactCardCopy(Box<CopyRequest<ContactCard<Set>>>),
}

macro_rules! impl_arguments_constructor {
    // Simple: params only
    ($method_name:ident, $variant:ident, $inner:ty) => {
        pub fn $method_name(params: RequestParams) -> Self {
            Arguments::$variant(Box::new(<$inner>::new(params)))
        }
    };
    // With extra String arg
    ($method_name:ident, $variant:ident, $inner:ty, $arg_name:ident : String) => {
        pub fn $method_name(params: RequestParams, $arg_name: String) -> Self {
            Arguments::$variant(Box::new(<$inner>::new(params, $arg_name)))
        }
    };
}

macro_rules! impl_arguments_accessor {
    ($method_name:ident, $variant:ident, $return_type:ty) => {
        pub fn $method_name(&mut self) -> &mut $return_type {
            match self {
                Arguments::$variant(r) => r.as_mut(),
                _ => unreachable!(),
            }
        }
    };
}

impl Arguments {
    // --- Constructors ---

    // Changes (special: uses ChangesRequest directly)
    impl_arguments_constructor!(changes, Changes, ChangesRequest, since_state: String);

    // Push subscription
    impl_arguments_constructor!(push_get, PushGet, GetRequest<PushSubscription<Set>>);
    impl_arguments_constructor!(push_set, PushSet, SetRequest<PushSubscription<Set>>);

    // Blob
    impl_arguments_constructor!(blob_copy, BlobCopy, CopyBlobRequest, from_account_id: String);
    impl_arguments_constructor!(blob_upload, BlobUpload, BlobUploadRequest);
    impl_arguments_constructor!(blob_get, BlobGet, BlobGetRequest);
    impl_arguments_constructor!(blob_lookup, BlobLookup, BlobLookupRequest);

    // Mailbox
    impl_arguments_constructor!(mailbox_get, MailboxGet, GetRequest<Mailbox<Set>>);
    impl_arguments_constructor!(mailbox_query, MailboxQuery, QueryRequest<Mailbox<Set>>);
    impl_arguments_constructor!(mailbox_query_changes, MailboxQueryChanges, QueryChangesRequest<Mailbox<Set>>, since_query_state: String);
    impl_arguments_constructor!(mailbox_set, MailboxSet, SetRequest<Mailbox<Set>>);

    // Thread
    impl_arguments_constructor!(thread_get, ThreadGet, GetRequest<Thread>);

    // Email
    impl_arguments_constructor!(email_get, EmailGet, GetRequest<Email<Set>>);
    impl_arguments_constructor!(email_query, EmailQuery, QueryRequest<Email<Set>>);
    impl_arguments_constructor!(email_query_changes, EmailQueryChanges, QueryChangesRequest<Email<Set>>, since_query_state: String);
    impl_arguments_constructor!(email_set, EmailSet, SetRequest<Email<Set>>);
    impl_arguments_constructor!(email_copy, EmailCopy, CopyRequest<Email<Set>>, from_account_id: String);
    impl_arguments_constructor!(email_import, EmailImport, EmailImportRequest);
    impl_arguments_constructor!(email_parse, EmailParse, EmailParseRequest);
    impl_arguments_constructor!(search_snippet_get, SearchSnippetGet, SearchSnippetGetRequest);

    // Identity
    impl_arguments_constructor!(identity_get, IdentityGet, GetRequest<Identity<Set>>);
    impl_arguments_constructor!(identity_set, IdentitySet, SetRequest<Identity<Set>>);

    // Email submission
    impl_arguments_constructor!(email_submission_get, EmailSubmissionGet, GetRequest<EmailSubmission<Set>>);
    impl_arguments_constructor!(email_submission_query, EmailSubmissionQuery, QueryRequest<EmailSubmission<Set>>);
    impl_arguments_constructor!(email_submission_query_changes, EmailSubmissionQueryChanges, QueryChangesRequest<EmailSubmission<Set>>, since_query_state: String);
    impl_arguments_constructor!(email_submission_set, EmailSubmissionSet, SetRequest<EmailSubmission<Set>>);

    // Vacation response
    impl_arguments_constructor!(vacation_response_get, VacationResponseGet, GetRequest<VacationResponse<Set>>);
    impl_arguments_constructor!(vacation_response_set, VacationResponseSet, SetRequest<VacationResponse<Set>>);

    // Sieve script
    impl_arguments_constructor!(sieve_script_get, SieveScriptGet, GetRequest<SieveScript<Set>>);
    impl_arguments_constructor!(sieve_script_query, SieveScriptQuery, QueryRequest<SieveScript<Set>>);
    impl_arguments_constructor!(sieve_script_set, SieveScriptSet, SetRequest<SieveScript<Set>>);

    // Sieve script validate (special: uses impl Into<String>)
    pub fn sieve_script_validate(params: RequestParams, blob_id: impl Into<String>) -> Self {
        Arguments::SieveScriptValidate(Box::new(SieveScriptValidateRequest::new(params, blob_id)))
    }

    // Principal
    impl_arguments_constructor!(principal_get, PrincipalGet, GetRequest<Principal<Set>>);
    impl_arguments_constructor!(principal_query, PrincipalQuery, QueryRequest<Principal<Set>>);
    impl_arguments_constructor!(principal_query_changes, PrincipalQueryChanges, QueryChangesRequest<Principal<Set>>, since_query_state: String);
    impl_arguments_constructor!(principal_set, PrincipalSet, SetRequest<Principal<Set>>);

    // Principal get availability (special: 4 args with impl Into<String>)
    pub fn principal_get_availability(
        params: RequestParams,
        id: impl Into<String>,
        utc_start: impl Into<String>,
        utc_end: impl Into<String>,
    ) -> Self {
        Arguments::PrincipalGetAvailability(Box::new(PrincipalGetAvailabilityRequest::new(
            params, id, utc_start, utc_end,
        )))
    }

    // Quota
    impl_arguments_constructor!(quota_get, QuotaGet, GetRequest<Quota<Set>>);
    impl_arguments_constructor!(quota_query, QuotaQuery, QueryRequest<Quota<Set>>);
    impl_arguments_constructor!(quota_query_changes, QuotaQueryChanges, QueryChangesRequest<Quota<Set>>, since_query_state: String);

    // Calendar
    impl_arguments_constructor!(calendar_get, CalendarGet, GetRequest<Calendar<Set>>);
    impl_arguments_constructor!(calendar_set, CalendarSet, SetRequest<Calendar<Set>>);

    // Calendar event
    impl_arguments_constructor!(calendar_event_get, CalendarEventGet, GetRequest<CalendarEvent<Set>>);
    impl_arguments_constructor!(calendar_event_query, CalendarEventQuery, QueryRequest<CalendarEvent<Set>>);
    impl_arguments_constructor!(calendar_event_query_changes, CalendarEventQueryChanges, QueryChangesRequest<CalendarEvent<Set>>, since_query_state: String);
    impl_arguments_constructor!(calendar_event_set, CalendarEventSet, SetRequest<CalendarEvent<Set>>);
    impl_arguments_constructor!(calendar_event_parse, CalendarEventParse, CalendarEventParseRequest);
    impl_arguments_constructor!(calendar_event_copy, CalendarEventCopy, CopyRequest<CalendarEvent<Set>>, from_account_id: String);

    // Calendar event notification
    impl_arguments_constructor!(calendar_event_notification_get, CalendarEventNotificationGet, GetRequest<CalendarEventNotification<Set>>);
    impl_arguments_constructor!(calendar_event_notification_query, CalendarEventNotificationQuery, QueryRequest<CalendarEventNotification<Set>>);
    impl_arguments_constructor!(calendar_event_notification_query_changes, CalendarEventNotificationQueryChanges, QueryChangesRequest<CalendarEventNotification<Set>>, since_query_state: String);
    impl_arguments_constructor!(calendar_event_notification_set, CalendarEventNotificationSet, SetRequest<CalendarEventNotification<Set>>);

    // Participant identity
    impl_arguments_constructor!(participant_identity_get, ParticipantIdentityGet, GetRequest<ParticipantIdentity<Set>>);
    impl_arguments_constructor!(participant_identity_set, ParticipantIdentitySet, SetRequest<ParticipantIdentity<Set>>);

    // Address book
    impl_arguments_constructor!(address_book_get, AddressBookGet, GetRequest<AddressBook<Set>>);
    impl_arguments_constructor!(address_book_set, AddressBookSet, SetRequest<AddressBook<Set>>);

    // Contact card
    impl_arguments_constructor!(contact_card_get, ContactCardGet, GetRequest<ContactCard<Set>>);
    impl_arguments_constructor!(contact_card_query, ContactCardQuery, QueryRequest<ContactCard<Set>>);
    impl_arguments_constructor!(contact_card_query_changes, ContactCardQueryChanges, QueryChangesRequest<ContactCard<Set>>, since_query_state: String);
    impl_arguments_constructor!(contact_card_set, ContactCardSet, SetRequest<ContactCard<Set>>);
    impl_arguments_constructor!(contact_card_parse, ContactCardParse, ContactCardParseRequest);
    impl_arguments_constructor!(contact_card_copy, ContactCardCopy, CopyRequest<ContactCard<Set>>, from_account_id: String);

    // --- Mutable accessors ---

    // Changes
    impl_arguments_accessor!(changes_mut, Changes, ChangesRequest);

    // Push subscription
    impl_arguments_accessor!(push_get_mut, PushGet, GetRequest<PushSubscription<Set>>);
    impl_arguments_accessor!(push_set_mut, PushSet, SetRequest<PushSubscription<Set>>);

    // Blob
    impl_arguments_accessor!(blob_copy_mut, BlobCopy, CopyBlobRequest);
    impl_arguments_accessor!(blob_upload_mut, BlobUpload, BlobUploadRequest);
    impl_arguments_accessor!(blob_get_mut, BlobGet, BlobGetRequest);
    impl_arguments_accessor!(blob_lookup_mut, BlobLookup, BlobLookupRequest);

    // Mailbox
    impl_arguments_accessor!(mailbox_get_mut, MailboxGet, GetRequest<Mailbox<Set>>);
    impl_arguments_accessor!(mailbox_query_mut, MailboxQuery, QueryRequest<Mailbox<Set>>);
    impl_arguments_accessor!(mailbox_query_changes_mut, MailboxQueryChanges, QueryChangesRequest<Mailbox<Set>>);
    impl_arguments_accessor!(mailbox_set_mut, MailboxSet, SetRequest<Mailbox<Set>>);

    // Thread
    impl_arguments_accessor!(thread_get_mut, ThreadGet, GetRequest<Thread>);

    // Email
    impl_arguments_accessor!(email_get_mut, EmailGet, GetRequest<Email<Set>>);
    impl_arguments_accessor!(email_query_mut, EmailQuery, QueryRequest<Email<Set>>);
    impl_arguments_accessor!(email_query_changes_mut, EmailQueryChanges, QueryChangesRequest<Email<Set>>);
    impl_arguments_accessor!(email_set_mut, EmailSet, SetRequest<Email<Set>>);
    impl_arguments_accessor!(email_copy_mut, EmailCopy, CopyRequest<Email<Set>>);
    impl_arguments_accessor!(email_import_mut, EmailImport, EmailImportRequest);
    impl_arguments_accessor!(email_parse_mut, EmailParse, EmailParseRequest);
    impl_arguments_accessor!(search_snippet_get_mut, SearchSnippetGet, SearchSnippetGetRequest);

    // Identity
    impl_arguments_accessor!(identity_get_mut, IdentityGet, GetRequest<Identity<Set>>);
    impl_arguments_accessor!(identity_set_mut, IdentitySet, SetRequest<Identity<Set>>);

    // Email submission
    impl_arguments_accessor!(email_submission_get_mut, EmailSubmissionGet, GetRequest<EmailSubmission<Set>>);
    impl_arguments_accessor!(email_submission_query_mut, EmailSubmissionQuery, QueryRequest<EmailSubmission<Set>>);
    impl_arguments_accessor!(email_submission_query_changes_mut, EmailSubmissionQueryChanges, QueryChangesRequest<EmailSubmission<Set>>);
    impl_arguments_accessor!(email_submission_set_mut, EmailSubmissionSet, SetRequest<EmailSubmission<Set>>);

    // Vacation response
    impl_arguments_accessor!(vacation_response_get_mut, VacationResponseGet, GetRequest<VacationResponse<Set>>);
    impl_arguments_accessor!(vacation_response_set_mut, VacationResponseSet, SetRequest<VacationResponse<Set>>);

    // Sieve script
    impl_arguments_accessor!(sieve_script_get_mut, SieveScriptGet, GetRequest<SieveScript<Set>>);
    impl_arguments_accessor!(sieve_script_query_mut, SieveScriptQuery, QueryRequest<SieveScript<Set>>);
    impl_arguments_accessor!(sieve_script_validate_mut, SieveScriptValidate, SieveScriptValidateRequest);
    impl_arguments_accessor!(sieve_script_set_mut, SieveScriptSet, SetRequest<SieveScript<Set>>);

    // Principal
    impl_arguments_accessor!(principal_get_mut, PrincipalGet, GetRequest<Principal<Set>>);
    impl_arguments_accessor!(principal_query_mut, PrincipalQuery, QueryRequest<Principal<Set>>);
    impl_arguments_accessor!(principal_query_changes_mut, PrincipalQueryChanges, QueryChangesRequest<Principal<Set>>);
    impl_arguments_accessor!(principal_set_mut, PrincipalSet, SetRequest<Principal<Set>>);
    impl_arguments_accessor!(principal_get_availability_mut, PrincipalGetAvailability, PrincipalGetAvailabilityRequest);

    // Quota
    impl_arguments_accessor!(quota_get_mut, QuotaGet, GetRequest<Quota<Set>>);
    impl_arguments_accessor!(quota_query_mut, QuotaQuery, QueryRequest<Quota<Set>>);
    impl_arguments_accessor!(quota_query_changes_mut, QuotaQueryChanges, QueryChangesRequest<Quota<Set>>);

    // Calendar
    impl_arguments_accessor!(calendar_get_mut, CalendarGet, GetRequest<Calendar<Set>>);
    impl_arguments_accessor!(calendar_set_mut, CalendarSet, SetRequest<Calendar<Set>>);

    // Calendar event
    impl_arguments_accessor!(calendar_event_get_mut, CalendarEventGet, GetRequest<CalendarEvent<Set>>);
    impl_arguments_accessor!(calendar_event_query_mut, CalendarEventQuery, QueryRequest<CalendarEvent<Set>>);
    impl_arguments_accessor!(calendar_event_query_changes_mut, CalendarEventQueryChanges, QueryChangesRequest<CalendarEvent<Set>>);
    impl_arguments_accessor!(calendar_event_set_mut, CalendarEventSet, SetRequest<CalendarEvent<Set>>);
    impl_arguments_accessor!(calendar_event_parse_mut, CalendarEventParse, CalendarEventParseRequest);
    impl_arguments_accessor!(calendar_event_copy_mut, CalendarEventCopy, CopyRequest<CalendarEvent<Set>>);

    // Calendar event notification
    impl_arguments_accessor!(calendar_event_notification_get_mut, CalendarEventNotificationGet, GetRequest<CalendarEventNotification<Set>>);
    impl_arguments_accessor!(calendar_event_notification_query_mut, CalendarEventNotificationQuery, QueryRequest<CalendarEventNotification<Set>>);
    impl_arguments_accessor!(calendar_event_notification_query_changes_mut, CalendarEventNotificationQueryChanges, QueryChangesRequest<CalendarEventNotification<Set>>);
    impl_arguments_accessor!(calendar_event_notification_set_mut, CalendarEventNotificationSet, SetRequest<CalendarEventNotification<Set>>);

    // Participant identity
    impl_arguments_accessor!(participant_identity_get_mut, ParticipantIdentityGet, GetRequest<ParticipantIdentity<Set>>);
    impl_arguments_accessor!(participant_identity_set_mut, ParticipantIdentitySet, SetRequest<ParticipantIdentity<Set>>);

    // Address book
    impl_arguments_accessor!(address_book_get_mut, AddressBookGet, GetRequest<AddressBook<Set>>);
    impl_arguments_accessor!(address_book_set_mut, AddressBookSet, SetRequest<AddressBook<Set>>);

    // Contact card
    impl_arguments_accessor!(contact_card_get_mut, ContactCardGet, GetRequest<ContactCard<Set>>);
    impl_arguments_accessor!(contact_card_query_mut, ContactCardQuery, QueryRequest<ContactCard<Set>>);
    impl_arguments_accessor!(contact_card_query_changes_mut, ContactCardQueryChanges, QueryChangesRequest<ContactCard<Set>>);
    impl_arguments_accessor!(contact_card_set_mut, ContactCardSet, SetRequest<ContactCard<Set>>);
    impl_arguments_accessor!(contact_card_parse_mut, ContactCardParse, ContactCardParseRequest);
    impl_arguments_accessor!(contact_card_copy_mut, ContactCardCopy, CopyRequest<ContactCard<Set>>);
}

impl<'x> Request<'x> {
    pub fn new(client: &'x Client) -> Self {
        Request {
            using: vec![URI::Core],
            method_calls: vec![],
            created_ids: None,
            account_id: client.default_account_id().to_string(),
            client,
        }
    }

    pub fn account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = account_id.into();
        self
    }

    #[maybe_async::maybe_async]
    pub async fn send(self) -> crate::Result<Response<TaggedMethodResponse>> {
        self.client.send(&self).await
    }

    #[cfg(feature = "websockets")]
    pub async fn send_ws(self) -> crate::Result<String> {
        self.client.send_ws(self).await
    }

    #[maybe_async::maybe_async]
    pub async fn send_single<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let response: Response<SingleMethodResponse<T>> = self.client.send(&self).await?;
        match response
            .unwrap_method_responses()
            .pop()
            .ok_or_else(|| Error::Internal("Server returned no results".to_string()))?
        {
            SingleMethodResponse::Ok((_, response, _)) => Ok(response),
            SingleMethodResponse::Error((_, err, _)) => Err(err.into()),
        }
    }

    pub fn params(&self, method: Method) -> RequestParams {
        RequestParams {
            account_id: self.account_id.clone(),
            method,
            call_id: self.method_calls.len(),
        }
    }

    pub fn add_method_call(&mut self, method: Method, arguments: Arguments) -> &mut Arguments {
        let call_id = format!("s{}", self.method_calls.len());
        self.method_calls.push((method, arguments, call_id));
        &mut self.method_calls.last_mut().unwrap().1
    }

    pub fn add_capability(&mut self, uri: URI) {
        if !self.using.contains(&uri) {
            self.using.push(uri);
        }
    }

    pub fn last_result_reference(&self, path: impl Into<String>) -> ResultReference {
        let last_method = self.method_calls.last().unwrap();
        ResultReference {
            result_of: last_method.2.clone(),
            name: last_method.0,
            path: path.into(),
        }
    }
}

impl ResultReference {
    pub fn new(method: Method, call_id: usize, path: impl Into<String>) -> Self {
        ResultReference {
            result_of: format!("s{}", call_id),
            name: method,
            path: path.into(),
        }
    }
}
