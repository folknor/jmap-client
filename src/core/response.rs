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

// ---------------------------------------------------------------------------
// Macro: Generate public type aliases for response types.
//
// Each invocation `AliasName => ResponseWrapper<InnerType>` expands to:
//   `pub type AliasName = ResponseWrapper<InnerType>;`
// ---------------------------------------------------------------------------
macro_rules! define_response_type_aliases {
    ($($alias:ident => $type:ty);* $(;)?) => {
        $(
            pub type $alias = $type;
        )*
    };
}

// ---------------------------------------------------------------------------
// Macro: Generate `unwrap_*` methods on `TaggedMethodResponse`.
//
// Each entry `method_name, Variant => ReturnType` expands to:
//   pub fn method_name(self) -> crate::Result<ReturnType> { ... }
// ---------------------------------------------------------------------------
macro_rules! impl_unwrap_methods {
    ($($method:ident, $variant:ident => $ret:ty);* $(;)?) => {
        $(
            pub fn $method(self) -> crate::Result<$ret> {
                match self.response {
                    MethodResponse::$variant(response) => Ok(*response),
                    MethodResponse::Error(err) => Err((*err).into()),
                    _ => Err("Response type mismatch".into()),
                }
            }
        )*
    };
}

// ---------------------------------------------------------------------------
// Macro: Generate the `is_type` match arms and the `MethodResponse` enum
// variants, plus the deserializer match arms. This single list drives all
// three expansions so they can never go out of sync.
//
// Each entry is: `Variant, MethodName, BoxedType`
// ---------------------------------------------------------------------------
macro_rules! define_method_response {
    (
        variants { $($variant:ident => $boxed:ty, $method:ident);* $(;)? }
    ) => {
        #[derive(Debug)]
        pub enum MethodResponse {
            $( $variant(Box<$boxed>), )*
            Echo(Box<serde_json::Value>),
            Error(Box<MethodError>),
        }

        impl TaggedMethodResponse {
            /// Returns `true` if this response matches the given [`Method`] type.
            pub fn is_type(&self, type_: Method) -> bool {
                matches!(
                    (&self.response, type_),
                    $( (MethodResponse::$variant(_), Method::$method) )|*
                        | (MethodResponse::Echo(_), Method::Echo)
                        | (MethodResponse::Error(_), Method::Error)
                )
            }
        }

        /// Generates the full deserializer match body for visit_seq.
        macro_rules! deserialize_method_response {
            ($method_val:expr, $seq:expr) => {
                match $method_val {
                    Method::Echo => MethodResponse::Echo(Box::new(
                        $seq.next_element()?
                            .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
                    )),
                    Method::Error => MethodResponse::Error(Box::new(
                        $seq.next_element()?
                            .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
                    )),
                    $(
                        Method::$method => MethodResponse::$variant(Box::new(
                            $seq.next_element()?
                                .ok_or_else(|| serde::de::Error::custom("Expected a method response"))?,
                        )),
                    )*
                }
            };
        }
    };
}

// ===========================================================================
// Response type aliases
// ===========================================================================
define_response_type_aliases! {
    PushSubscriptionSetResponse     => SetResponse<PushSubscription<Get>>;
    PushSubscriptionGetResponse     => GetResponse<PushSubscription<Get>>;
    MailboxChangesResponse          => ChangesResponse<Mailbox<Get>>;
    MailboxSetResponse              => SetResponse<Mailbox<Get>>;
    MailboxGetResponse              => GetResponse<Mailbox<Get>>;
    ThreadGetResponse               => GetResponse<Thread>;
    ThreadChangesResponse           => ChangesResponse<Thread>;
    EmailGetResponse                => GetResponse<Email<Get>>;
    EmailSetResponse                => SetResponse<Email<Get>>;
    EmailCopyResponse               => CopyResponse<Email<Get>>;
    EmailChangesResponse            => ChangesResponse<Email<Get>>;
    IdentitySetResponse             => SetResponse<Identity<Get>>;
    IdentityGetResponse             => GetResponse<Identity<Get>>;
    IdentityChangesResponse         => ChangesResponse<Identity<Get>>;
    EmailSubmissionSetResponse      => SetResponse<EmailSubmission<Get>>;
    EmailSubmissionGetResponse      => GetResponse<EmailSubmission<Get>>;
    EmailSubmissionChangesResponse  => ChangesResponse<EmailSubmission<Get>>;
    VacationResponseGetResponse     => GetResponse<VacationResponse<Get>>;
    VacationResponseSetResponse     => SetResponse<VacationResponse<Get>>;
    SieveScriptGetResponse          => GetResponse<SieveScript<Get>>;
    SieveScriptSetResponse          => SetResponse<SieveScript<Get>>;
    PrincipalChangesResponse        => ChangesResponse<Principal<Get>>;
    PrincipalSetResponse            => SetResponse<Principal<Get>>;
    PrincipalGetResponse            => GetResponse<Principal<Get>>;
    QuotaGetResponse                => GetResponse<Quota<Get>>;
    QuotaChangesResponse            => ChangesResponse<Quota<Get>>;
    CalendarGetResponse             => GetResponse<Calendar<Get>>;
    CalendarSetResponse             => SetResponse<Calendar<Get>>;
    CalendarChangesResponse         => ChangesResponse<Calendar<Get>>;
    CalendarEventGetResponse        => GetResponse<CalendarEvent<Get>>;
    CalendarEventSetResponse        => SetResponse<CalendarEvent<Get>>;
    CalendarEventChangesResponse    => ChangesResponse<CalendarEvent<Get>>;
    CalendarEventCopyResponse       => CopyResponse<CalendarEvent<Get>>;
    CalendarEventNotificationGetResponse     => GetResponse<CalendarEventNotification<Get>>;
    CalendarEventNotificationSetResponse     => SetResponse<CalendarEventNotification<Get>>;
    CalendarEventNotificationChangesResponse => ChangesResponse<CalendarEventNotification<Get>>;
    ParticipantIdentityGetResponse     => GetResponse<ParticipantIdentity<Get>>;
    ParticipantIdentitySetResponse     => SetResponse<ParticipantIdentity<Get>>;
    ParticipantIdentityChangesResponse => ChangesResponse<ParticipantIdentity<Get>>;
    AddressBookGetResponse          => GetResponse<AddressBook<Get>>;
    AddressBookSetResponse          => SetResponse<AddressBook<Get>>;
    AddressBookChangesResponse      => ChangesResponse<AddressBook<Get>>;
    ContactCardGetResponse          => GetResponse<ContactCard<Get>>;
    ContactCardSetResponse          => SetResponse<ContactCard<Get>>;
    ContactCardChangesResponse      => ChangesResponse<ContactCard<Get>>;
    ContactCardCopyResponse         => CopyResponse<ContactCard<Get>>;
}

// ===========================================================================
// Generic Response struct
// ===========================================================================
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

    pub fn method_response_by_pos(&mut self, index: usize) -> Option<T> {
        if index < self.method_responses.len() {
            Some(self.method_responses.swap_remove(index))
        } else {
            None
        }
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
    // Ok must come before Error — with untagged enums, serde tries
    // variants in order. Ok's first element is a method name String,
    // while Error's first element is the Error enum (which only
    // deserializes from "error"). Placing Ok first ensures normal
    // responses aren't misidentified.
    Ok((String, T, String)),
    Error((Error, MethodError, String)),
}

#[derive(Debug, Deserialize)]
pub enum Error {
    #[serde(rename = "error")]
    Error,
}

// ===========================================================================
// TaggedMethodResponse and MethodResponse
// ===========================================================================
#[derive(Debug)]
pub struct TaggedMethodResponse {
    id: String,
    response: MethodResponse,
}

// This invocation defines the MethodResponse enum, the is_type() method,
// and the deserialize_method_arms!() helper macro used below.
define_method_response! {
    variants {
        CopyBlob                            => CopyBlobResponse,                            CopyBlob;
        UploadBlob                          => BlobUploadResponse,                          UploadBlob;
        GetBlob                             => BlobGetResponse,                             GetBlob;
        LookupBlob                          => BlobLookupResponse,                          LookupBlob;
        GetPushSubscription                 => PushSubscriptionGetResponse,                 GetPushSubscription;
        SetPushSubscription                 => PushSubscriptionSetResponse,                 SetPushSubscription;
        GetMailbox                          => MailboxGetResponse,                           GetMailbox;
        ChangesMailbox                      => MailboxChangesResponse,                       ChangesMailbox;
        QueryMailbox                        => QueryResponse,                                QueryMailbox;
        QueryChangesMailbox                 => QueryChangesResponse,                         QueryChangesMailbox;
        SetMailbox                          => MailboxSetResponse,                           SetMailbox;
        GetThread                           => ThreadGetResponse,                            GetThread;
        ChangesThread                       => ThreadChangesResponse,                        ChangesThread;
        GetEmail                            => EmailGetResponse,                             GetEmail;
        ChangesEmail                        => EmailChangesResponse,                         ChangesEmail;
        QueryEmail                          => QueryResponse,                                QueryEmail;
        QueryChangesEmail                   => QueryChangesResponse,                         QueryChangesEmail;
        SetEmail                            => EmailSetResponse,                             SetEmail;
        CopyEmail                           => EmailCopyResponse,                            CopyEmail;
        ImportEmail                         => EmailImportResponse,                          ImportEmail;
        ParseEmail                          => EmailParseResponse,                           ParseEmail;
        GetSearchSnippet                    => SearchSnippetGetResponse,                     GetSearchSnippet;
        GetIdentity                         => IdentityGetResponse,                          GetIdentity;
        ChangesIdentity                     => IdentityChangesResponse,                      ChangesIdentity;
        SetIdentity                         => IdentitySetResponse,                          SetIdentity;
        GetEmailSubmission                  => EmailSubmissionGetResponse,                   GetEmailSubmission;
        ChangesEmailSubmission              => EmailSubmissionChangesResponse,               ChangesEmailSubmission;
        QueryEmailSubmission                => QueryResponse,                                QueryEmailSubmission;
        QueryChangesEmailSubmission         => QueryChangesResponse,                         QueryChangesEmailSubmission;
        SetEmailSubmission                  => EmailSubmissionSetResponse,                   SetEmailSubmission;
        GetVacationResponse                 => VacationResponseGetResponse,                  GetVacationResponse;
        SetVacationResponse                 => VacationResponseSetResponse,                  SetVacationResponse;
        GetSieveScript                      => SieveScriptGetResponse,                       GetSieveScript;
        QuerySieveScript                    => QueryResponse,                                QuerySieveScript;
        SetSieveScript                      => SieveScriptSetResponse,                       SetSieveScript;
        ValidateSieveScript                 => SieveScriptValidateResponse,                  ValidateSieveScript;
        GetPrincipal                        => PrincipalGetResponse,                         GetPrincipal;
        ChangesPrincipal                    => PrincipalChangesResponse,                     ChangesPrincipal;
        QueryPrincipal                      => QueryResponse,                                QueryPrincipal;
        QueryChangesPrincipal               => QueryChangesResponse,                         QueryChangesPrincipal;
        SetPrincipal                        => PrincipalSetResponse,                         SetPrincipal;
        GetAvailabilityPrincipal            => PrincipalGetAvailabilityResponse,              GetAvailabilityPrincipal;
        GetQuota                            => QuotaGetResponse,                             GetQuota;
        ChangesQuota                        => QuotaChangesResponse,                         ChangesQuota;
        QueryQuota                          => QueryResponse,                                QueryQuota;
        QueryChangesQuota                   => QueryChangesResponse,                         QueryChangesQuota;
        GetCalendar                         => CalendarGetResponse,                          GetCalendar;
        ChangesCalendar                     => CalendarChangesResponse,                      ChangesCalendar;
        SetCalendar                         => CalendarSetResponse,                          SetCalendar;
        GetCalendarEvent                    => CalendarEventGetResponse,                     GetCalendarEvent;
        ChangesCalendarEvent                => CalendarEventChangesResponse,                 ChangesCalendarEvent;
        QueryCalendarEvent                  => QueryResponse,                                QueryCalendarEvent;
        QueryChangesCalendarEvent           => QueryChangesResponse,                         QueryChangesCalendarEvent;
        SetCalendarEvent                    => CalendarEventSetResponse,                     SetCalendarEvent;
        ParseCalendarEvent                  => CalendarEventParseResponse,                   ParseCalendarEvent;
        CopyCalendarEvent                   => CalendarEventCopyResponse,                    CopyCalendarEvent;
        GetCalendarEventNotification        => CalendarEventNotificationGetResponse,         GetCalendarEventNotification;
        ChangesCalendarEventNotification    => CalendarEventNotificationChangesResponse,     ChangesCalendarEventNotification;
        QueryCalendarEventNotification      => QueryResponse,                                QueryCalendarEventNotification;
        QueryChangesCalendarEventNotification => QueryChangesResponse,                       QueryChangesCalendarEventNotification;
        SetCalendarEventNotification        => CalendarEventNotificationSetResponse,         SetCalendarEventNotification;
        GetParticipantIdentity              => ParticipantIdentityGetResponse,               GetParticipantIdentity;
        ChangesParticipantIdentity          => ParticipantIdentityChangesResponse,           ChangesParticipantIdentity;
        SetParticipantIdentity              => ParticipantIdentitySetResponse,               SetParticipantIdentity;
        GetAddressBook                      => AddressBookGetResponse,                       GetAddressBook;
        ChangesAddressBook                  => AddressBookChangesResponse,                   ChangesAddressBook;
        SetAddressBook                      => AddressBookSetResponse,                       SetAddressBook;
        GetContactCard                      => ContactCardGetResponse,                       GetContactCard;
        ChangesContactCard                  => ContactCardChangesResponse,                   ChangesContactCard;
        QueryContactCard                    => QueryResponse,                                QueryContactCard;
        QueryChangesContactCard             => QueryChangesResponse,                         QueryChangesContactCard;
        SetContactCard                      => ContactCardSetResponse,                       SetContactCard;
        ParseContactCard                    => ContactCardParseResponse,                     ParseContactCard;
        CopyContactCard                     => ContactCardCopyResponse,                      CopyContactCard;
    }
}

// ===========================================================================
// TaggedMethodResponse impl: unwrap helpers
// ===========================================================================
impl TaggedMethodResponse {
    pub fn call_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn unwrap_method_response(self) -> MethodResponse {
        self.response
    }

    impl_unwrap_methods! {
        unwrap_upload_blob,                              UploadBlob                          => BlobUploadResponse;
        unwrap_get_blob,                                 GetBlob                             => BlobGetResponse;
        unwrap_lookup_blob,                              LookupBlob                          => BlobLookupResponse;
        unwrap_copy_blob,                                CopyBlob                            => CopyBlobResponse;
        unwrap_get_push_subscription,                    GetPushSubscription                 => PushSubscriptionGetResponse;
        unwrap_set_push_subscription,                    SetPushSubscription                 => PushSubscriptionSetResponse;
        unwrap_get_mailbox,                              GetMailbox                          => MailboxGetResponse;
        unwrap_changes_mailbox,                          ChangesMailbox                      => MailboxChangesResponse;
        unwrap_query_mailbox,                             QueryMailbox                       => QueryResponse;
        unwrap_query_changes_mailbox,                     QueryChangesMailbox                => QueryChangesResponse;
        unwrap_set_mailbox,                              SetMailbox                          => MailboxSetResponse;
        unwrap_get_thread,                               GetThread                           => ThreadGetResponse;
        unwrap_changes_thread,                           ChangesThread                       => ThreadChangesResponse;
        unwrap_get_email,                                GetEmail                            => EmailGetResponse;
        unwrap_changes_email,                            ChangesEmail                        => EmailChangesResponse;
        unwrap_query_email,                              QueryEmail                          => QueryResponse;
        unwrap_query_changes_email,                      QueryChangesEmail                   => QueryChangesResponse;
        unwrap_set_email,                                SetEmail                            => EmailSetResponse;
        unwrap_copy_email,                               CopyEmail                           => EmailCopyResponse;
        unwrap_import_email,                             ImportEmail                         => EmailImportResponse;
        unwrap_parse_email,                              ParseEmail                          => EmailParseResponse;
        unwrap_get_search_snippet,                       GetSearchSnippet                    => SearchSnippetGetResponse;
        unwrap_get_identity,                             GetIdentity                         => IdentityGetResponse;
        unwrap_changes_identity,                         ChangesIdentity                     => IdentityChangesResponse;
        unwrap_set_identity,                             SetIdentity                         => IdentitySetResponse;
        unwrap_get_email_submission,                     GetEmailSubmission                  => EmailSubmissionGetResponse;
        unwrap_changes_email_submission,                 ChangesEmailSubmission              => EmailSubmissionChangesResponse;
        unwrap_set_email_submission,                     SetEmailSubmission                  => EmailSubmissionSetResponse;
        unwrap_query_email_submission,                   QueryEmailSubmission                => QueryResponse;
        unwrap_query_changes_email_submission,            QueryChangesEmailSubmission        => QueryChangesResponse;
        unwrap_get_vacation_response,                    GetVacationResponse                 => VacationResponseGetResponse;
        unwrap_set_vacation_response,                    SetVacationResponse                 => VacationResponseSetResponse;
        unwrap_get_sieve_script,                         GetSieveScript                      => SieveScriptGetResponse;
        unwrap_validate_sieve_script,                    ValidateSieveScript                 => SieveScriptValidateResponse;
        unwrap_set_sieve_script,                         SetSieveScript                      => SieveScriptSetResponse;
        unwrap_query_sieve_script,                       QuerySieveScript                    => QueryResponse;
        unwrap_get_principal,                            GetPrincipal                        => PrincipalGetResponse;
        unwrap_changes_principal,                        ChangesPrincipal                    => PrincipalChangesResponse;
        unwrap_query_principal,                          QueryPrincipal                      => QueryResponse;
        unwrap_query_changes_principal,                  QueryChangesPrincipal               => QueryChangesResponse;
        unwrap_set_principal,                            SetPrincipal                        => PrincipalSetResponse;
        unwrap_get_availability_principal,               GetAvailabilityPrincipal            => PrincipalGetAvailabilityResponse;
        unwrap_get_quota,                                GetQuota                            => QuotaGetResponse;
        unwrap_changes_quota,                            ChangesQuota                        => QuotaChangesResponse;
        unwrap_query_quota,                              QueryQuota                          => QueryResponse;
        unwrap_query_changes_quota,                      QueryChangesQuota                   => QueryChangesResponse;
        unwrap_get_calendar,                             GetCalendar                         => CalendarGetResponse;
        unwrap_changes_calendar,                         ChangesCalendar                     => CalendarChangesResponse;
        unwrap_set_calendar,                             SetCalendar                         => CalendarSetResponse;
        unwrap_get_calendar_event,                       GetCalendarEvent                    => CalendarEventGetResponse;
        unwrap_changes_calendar_event,                   ChangesCalendarEvent                => CalendarEventChangesResponse;
        unwrap_query_calendar_event,                     QueryCalendarEvent                  => QueryResponse;
        unwrap_query_changes_calendar_event,             QueryChangesCalendarEvent           => QueryChangesResponse;
        unwrap_set_calendar_event,                       SetCalendarEvent                    => CalendarEventSetResponse;
        unwrap_parse_calendar_event,                     ParseCalendarEvent                  => CalendarEventParseResponse;
        unwrap_copy_calendar_event,                      CopyCalendarEvent                   => CalendarEventCopyResponse;
        unwrap_get_calendar_event_notification,          GetCalendarEventNotification        => CalendarEventNotificationGetResponse;
        unwrap_changes_calendar_event_notification,      ChangesCalendarEventNotification    => CalendarEventNotificationChangesResponse;
        unwrap_query_calendar_event_notification,        QueryCalendarEventNotification      => QueryResponse;
        unwrap_query_changes_calendar_event_notification, QueryChangesCalendarEventNotification => QueryChangesResponse;
        unwrap_set_calendar_event_notification,          SetCalendarEventNotification        => CalendarEventNotificationSetResponse;
        unwrap_get_participant_identity,                 GetParticipantIdentity              => ParticipantIdentityGetResponse;
        unwrap_changes_participant_identity,             ChangesParticipantIdentity          => ParticipantIdentityChangesResponse;
        unwrap_set_participant_identity,                 SetParticipantIdentity              => ParticipantIdentitySetResponse;
        unwrap_get_address_book,                         GetAddressBook                      => AddressBookGetResponse;
        unwrap_changes_address_book,                     ChangesAddressBook                  => AddressBookChangesResponse;
        unwrap_set_address_book,                         SetAddressBook                      => AddressBookSetResponse;
        unwrap_get_contact_card,                         GetContactCard                      => ContactCardGetResponse;
        unwrap_changes_contact_card,                     ChangesContactCard                  => ContactCardChangesResponse;
        unwrap_query_contact_card,                       QueryContactCard                    => QueryResponse;
        unwrap_query_changes_contact_card,               QueryChangesContactCard             => QueryChangesResponse;
        unwrap_set_contact_card,                         SetContactCard                      => ContactCardSetResponse;
        unwrap_parse_contact_card,                       ParseContactCard                    => ContactCardParseResponse;
        unwrap_copy_contact_card,                        CopyContactCard                     => ContactCardCopyResponse;
        unwrap_echo,                                     Echo                                => serde_json::Value;
    }

    pub fn is_error(&self) -> bool {
        matches!(self.response, MethodResponse::Error(_))
    }
}

// ===========================================================================
// Deserialization
// ===========================================================================
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
        let method = seq
            .next_element::<Method>()?
            .ok_or_else(|| serde::de::Error::custom("Expected a method name"))?;
        let response = deserialize_method_response!(method, seq);

        let id = seq
            .next_element::<String>()?
            .ok_or_else(|| serde::de::Error::custom("Expected method call id"))?;

        Ok(TaggedMethodResponse { response, id })
    }
}
