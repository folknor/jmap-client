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

use crate::core::capability::Capability;
use crate::email::{MailCapabilities, SubmissionCapabilities};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "capabilities")]
    #[serde(deserialize_with = "deserialize_capabilities_map")]
    capabilities: HashMap<String, Capabilities>,

    #[serde(rename = "accounts")]
    accounts: HashMap<String, Account>,

    #[serde(rename = "primaryAccounts")]
    primary_accounts: HashMap<String, String>,

    #[serde(rename = "username")]
    username: String,

    #[serde(rename = "apiUrl")]
    api_url: String,

    #[serde(rename = "downloadUrl")]
    download_url: String,

    #[serde(rename = "uploadUrl")]
    upload_url: String,

    #[serde(rename = "eventSourceUrl")]
    event_source_url: String,

    #[serde(rename = "state")]
    state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "isPersonal")]
    is_personal: bool,

    #[serde(rename = "isReadOnly")]
    is_read_only: bool,

    #[serde(rename = "accountCapabilities")]
    #[serde(deserialize_with = "deserialize_capabilities_map")]
    account_capabilities: HashMap<String, Capabilities>,
}

/// Session/account capability value. The correct variant is selected by
/// the map key (capability URI), not by the value shape.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Capabilities {
    Core(CoreCapabilities),
    Mail(MailCapabilities),
    Submission(SubmissionCapabilities),
    WebSocket(WebSocketCapabilities),
    Sieve(SieveCapabilities),
    Quota(QuotaCapabilities),
    Blob(BlobCapabilities),
    Calendars(CalendarsCapabilities),
    Contacts(ContactsCapabilities),
    Principals(PrincipalsCapabilities),
    Other(serde_json::Value),
}

/// Custom deserializer for `HashMap<String, Capabilities>` that
/// dispatches to the correct `Capabilities` variant based on the URI
/// key, rather than relying on `#[serde(untagged)]` trial-and-error.
fn deserialize_capabilities_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Capabilities>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: HashMap<String, JsonValue> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::with_capacity(raw.len());

    /// Try to deserialize a capability value as a typed struct,
    /// falling back to `Other(original_value)` on parse failure.
    macro_rules! try_cap {
        ($value:expr, $variant:ident) => {
            match serde_json::from_value::<_>($value.clone()) {
                Ok(v) => Capabilities::$variant(v),
                Err(_) => Capabilities::Other($value),
            }
        };
    }

    for (key, value) in raw {
        let cap = match key.as_str() {
            "urn:ietf:params:jmap:core" => try_cap!(value, Core),
            "urn:ietf:params:jmap:mail" => try_cap!(value, Mail),
            "urn:ietf:params:jmap:submission" => try_cap!(value, Submission),
            "urn:ietf:params:jmap:websocket" => try_cap!(value, WebSocket),
            "urn:ietf:params:jmap:sieve" => try_cap!(value, Sieve),
            "urn:ietf:params:jmap:quota" => try_cap!(value, Quota),
            "urn:ietf:params:jmap:blob" => try_cap!(value, Blob),
            "urn:ietf:params:jmap:calendars" => try_cap!(value, Calendars),
            "urn:ietf:params:jmap:contacts" => try_cap!(value, Contacts),
            "urn:ietf:params:jmap:principals" => try_cap!(value, Principals),
            _ => Capabilities::Other(value),
        };
        result.insert(key, cap);
    }

    Ok(result)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CoreCapabilities {
    #[serde(rename = "maxSizeUpload")]
    max_size_upload: usize,

    #[serde(rename = "maxConcurrentUpload")]
    max_concurrent_upload: usize,

    #[serde(rename = "maxSizeRequest")]
    max_size_request: usize,

    #[serde(rename = "maxConcurrentRequests")]
    max_concurrent_requests: usize,

    #[serde(rename = "maxCallsInRequest")]
    max_calls_in_request: usize,

    #[serde(rename = "maxObjectsInGet")]
    max_objects_in_get: usize,

    #[serde(rename = "maxObjectsInSet")]
    max_objects_in_set: usize,

    #[serde(rename = "collationAlgorithms")]
    collation_algorithms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketCapabilities {
    #[serde(rename = "url")]
    url: String,
    #[serde(rename = "supportsPush")]
    supports_push: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SieveCapabilities {
    #[serde(rename = "implementation")]
    implementation: Option<String>,
    #[serde(rename = "maxSizeScriptName")]
    max_script_name: Option<usize>,
    #[serde(rename = "maxSizeScript")]
    max_script_size: Option<usize>,
    #[serde(rename = "maxNumberScripts")]
    max_scripts: Option<usize>,
    #[serde(rename = "maxNumberRedirects")]
    max_redirects: Option<usize>,
    #[serde(rename = "sieveExtensions")]
    extensions: Vec<String>,
    #[serde(rename = "notificationMethods")]
    notification_methods: Option<Vec<String>>,
    #[serde(rename = "externalLists")]
    ext_lists: Option<Vec<String>>,
}

/// Capabilities for `urn:ietf:params:jmap:quota` (RFC 9425).
///
/// Empty capability object per spec — presence indicates quota support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaCapabilities {}

/// Capabilities for `urn:ietf:params:jmap:blob` (RFC 9404).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobCapabilities {
    #[serde(rename = "maxSizeBlobSet")]
    #[serde(default)]
    max_size_blob_set: Option<u64>,

    #[serde(rename = "supportedDigestAlgorithms")]
    #[serde(default)]
    supported_digest_algorithms: Vec<String>,

    #[serde(rename = "supportedTypeNames")]
    #[serde(default)]
    supported_type_names: Vec<String>,
}

/// Capabilities for `urn:ietf:params:jmap:calendars`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarsCapabilities {
    #[serde(rename = "mayCreateCalendar")]
    #[serde(default)]
    may_create_calendar: bool,

    #[serde(rename = "maxCalendarsPerEvent")]
    #[serde(default)]
    max_calendars_per_event: Option<usize>,
}

/// Capabilities for `urn:ietf:params:jmap:contacts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactsCapabilities {
    #[serde(rename = "mayCreateAddressBook")]
    #[serde(default)]
    may_create_address_book: bool,

    #[serde(rename = "maxAddressBooksPerCard")]
    #[serde(default)]
    max_address_books_per_card: Option<usize>,
}

/// Capabilities for `urn:ietf:params:jmap:principals`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipalsCapabilities {
    #[serde(rename = "currentUserPrincipalId")]
    #[serde(default)]
    current_user_principal_id: Option<String>,

    #[serde(rename = "accountIdForPrincipal")]
    #[serde(default)]
    account_id_for_principal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyCapabilities {}

impl Session {
    pub fn capabilities(&self) -> impl Iterator<Item = &String> {
        self.capabilities.keys()
    }

    pub fn capability(&self, capability: impl AsRef<str>) -> Option<&Capabilities> {
        self.capabilities.get(capability.as_ref())
    }

    pub fn has_capability(&self, capability: impl AsRef<str>) -> bool {
        self.capabilities.contains_key(capability.as_ref())
    }

    pub fn websocket_capabilities(&self) -> Option<&WebSocketCapabilities> {
        self.capabilities
            .get(crate::core::capability::WebSocket::URI)
            .and_then(|v| match v {
                Capabilities::WebSocket(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn core_capabilities(&self) -> Option<&CoreCapabilities> {
        self.capabilities
            .get(crate::core::capability::Core::URI)
            .and_then(|v| match v {
                Capabilities::Core(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn mail_capabilities(&self) -> Option<&MailCapabilities> {
        self.capabilities
            .get(crate::core::capability::Mail::URI)
            .and_then(|v| match v {
                Capabilities::Mail(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn submission_capabilities(&self) -> Option<&SubmissionCapabilities> {
        self.capabilities
            .get(crate::core::capability::Submission::URI)
            .and_then(|v| match v {
                Capabilities::Submission(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn sieve_capabilities(&self) -> Option<&SieveCapabilities> {
        self.capabilities
            .get(crate::core::capability::Sieve::URI)
            .and_then(|v| match v {
                Capabilities::Sieve(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn quota_capabilities(&self) -> Option<&QuotaCapabilities> {
        self.capabilities
            .get(crate::core::capability::Quota::URI)
            .and_then(|v| match v {
                Capabilities::Quota(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn blob_capabilities(&self) -> Option<&BlobCapabilities> {
        self.capabilities
            .get(crate::core::capability::Blob::URI)
            .and_then(|v| match v {
                Capabilities::Blob(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn calendars_capabilities(&self) -> Option<&CalendarsCapabilities> {
        self.capabilities
            .get(crate::core::capability::Calendars::URI)
            .and_then(|v| match v {
                Capabilities::Calendars(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn contacts_capabilities(&self) -> Option<&ContactsCapabilities> {
        self.capabilities
            .get(crate::core::capability::Contacts::URI)
            .and_then(|v| match v {
                Capabilities::Contacts(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn principals_capabilities(&self) -> Option<&PrincipalsCapabilities> {
        self.capabilities
            .get(crate::core::capability::Principals::URI)
            .and_then(|v| match v {
                Capabilities::Principals(capabilities) => Some(capabilities),
                _ => None,
            })
    }

    pub fn accounts(&self) -> impl Iterator<Item = &String> {
        self.accounts.keys()
    }

    pub fn account(&self, account: &str) -> Option<&Account> {
        self.accounts.get(account)
    }

    pub fn primary_accounts(&self) -> impl Iterator<Item = (&String, &String)> {
        self.primary_accounts.iter()
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    pub fn download_url(&self) -> &str {
        &self.download_url
    }

    pub fn upload_url(&self) -> &str {
        &self.upload_url
    }

    pub fn event_source_url(&self) -> &str {
        &self.event_source_url
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

impl Account {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }

    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    pub fn capabilities(&self) -> impl Iterator<Item = &String> {
        self.account_capabilities.keys()
    }

    pub fn capability(&self, capability: &str) -> Option<&Capabilities> {
        self.account_capabilities.get(capability)
    }
}

impl CoreCapabilities {
    pub fn max_size_upload(&self) -> usize {
        self.max_size_upload
    }

    pub fn max_concurrent_upload(&self) -> usize {
        self.max_concurrent_upload
    }

    pub fn max_size_request(&self) -> usize {
        self.max_size_request
    }

    pub fn max_concurrent_requests(&self) -> usize {
        self.max_concurrent_requests
    }

    pub fn max_calls_in_request(&self) -> usize {
        self.max_calls_in_request
    }

    pub fn max_objects_in_get(&self) -> usize {
        self.max_objects_in_get
    }

    pub fn max_objects_in_set(&self) -> usize {
        self.max_objects_in_set
    }

    pub fn collation_algorithms(&self) -> &[String] {
        &self.collation_algorithms
    }
}

impl WebSocketCapabilities {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn supports_push(&self) -> bool {
        self.supports_push
    }
}

impl SieveCapabilities {
    pub fn max_script_name_size(&self) -> usize {
        self.max_script_name.unwrap_or(512)
    }

    pub fn max_script_size(&self) -> Option<usize> {
        self.max_script_size
    }

    pub fn max_number_scripts(&self) -> Option<usize> {
        self.max_scripts
    }

    pub fn max_number_redirects(&self) -> Option<usize> {
        self.max_redirects
    }

    pub fn sieve_extensions(&self) -> &[String] {
        &self.extensions
    }

    pub fn notification_methods(&self) -> Option<&[String]> {
        self.notification_methods.as_deref()
    }

    pub fn external_lists(&self) -> Option<&[String]> {
        self.ext_lists.as_deref()
    }
}

impl BlobCapabilities {
    pub fn max_size_blob_set(&self) -> Option<u64> {
        self.max_size_blob_set
    }

    pub fn supported_digest_algorithms(&self) -> &[String] {
        &self.supported_digest_algorithms
    }

    pub fn supported_type_names(&self) -> &[String] {
        &self.supported_type_names
    }
}

impl CalendarsCapabilities {
    pub fn may_create_calendar(&self) -> bool {
        self.may_create_calendar
    }

    pub fn max_calendars_per_event(&self) -> Option<usize> {
        self.max_calendars_per_event
    }
}

impl ContactsCapabilities {
    pub fn may_create_address_book(&self) -> bool {
        self.may_create_address_book
    }

    pub fn max_address_books_per_card(&self) -> Option<usize> {
        self.max_address_books_per_card
    }
}

impl PrincipalsCapabilities {
    pub fn current_user_principal_id(&self) -> Option<&str> {
        self.current_user_principal_id.as_deref()
    }

    pub fn account_id_for_principal(&self) -> Option<&str> {
        self.account_id_for_principal.as_deref()
    }
}

pub trait URLParser: Sized {
    fn parse(value: &str) -> Option<Self>;
}

#[non_exhaustive]
pub enum URLPart<T: URLParser> {
    Value(String),
    Parameter(T),
}

impl<T: URLParser> URLPart<T> {
    pub fn parse(url: &str) -> crate::Result<Vec<URLPart<T>>> {
        let mut parts = Vec::new();
        let mut buf = String::with_capacity(url.len());
        let mut in_parameter = false;

        for ch in url.chars() {
            match ch {
                '{' => {
                    if !buf.is_empty() {
                        parts.push(URLPart::Value(buf.clone()));
                        buf.clear();
                    }
                    in_parameter = true;
                }
                '}' => {
                    if in_parameter && !buf.is_empty() {
                        parts.push(URLPart::Parameter(T::parse(&buf).ok_or_else(|| {
                            crate::Error::Internal(format!(
                                "Invalid parameter '{buf}' in URL: {url}"
                            ))
                        })?));
                        buf.clear();
                    } else {
                        return Err(crate::Error::Internal(format!("Invalid URL: {url}")));
                    }
                    in_parameter = false;
                }
                _ => {
                    buf.push(ch);
                }
            }
        }

        if !buf.is_empty() {
            if !in_parameter {
                parts.push(URLPart::Value(buf.clone()));
            } else {
                return Err(crate::Error::Internal(format!("Invalid URL: {url}")));
            }
        }

        Ok(parts)
    }
}
