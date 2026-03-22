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

use serde::{de::DeserializeOwned, Serialize};

use super::capability::Capability;

/// A self-describing JMAP method call.
///
/// Each JMAP method (Email/get, CalendarEvent/query, etc.) is a concrete
/// type implementing this trait. The trait carries the wire name,
/// required capability, and associated response type.
pub trait JmapMethod: Serialize + Send {
    /// Wire method name, e.g. `"Email/get"`.
    const NAME: &'static str;

    /// The capability required for this method.
    type Cap: Capability;

    /// The deserialized response type.
    type Response: DeserializeOwned + Send;
}

/// Generates a JMAP /get method struct that wraps `GetRequest<O>`.
#[macro_export]
macro_rules! define_get_method {
    ($name:ident, $obj:ty, $method_name:expr, $cap:ty, $response:ty) => {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct $name {
            #[serde(flatten)]
            inner: $crate::core::get::GetRequest<$obj>,
        }

        impl $crate::core::method::JmapMethod for $name {
            const NAME: &'static str = $method_name;
            type Cap = $cap;
            type Response = $response;
        }

        impl $name {
            pub fn new(account_id: impl Into<String>) -> Self {
                Self {
                    inner: $crate::core::get::GetRequest::new(account_id),
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::core::get::GetRequest<$obj>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}

/// Generates a JMAP /set method struct that wraps `SetRequest<O>`.
#[macro_export]
macro_rules! define_set_method {
    ($name:ident, $obj:ty, $method_name:expr, $cap:ty, $response:ty) => {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct $name {
            #[serde(flatten)]
            inner: $crate::core::set::SetRequest<$obj>,
        }

        impl $crate::core::method::JmapMethod for $name {
            const NAME: &'static str = $method_name;
            type Cap = $cap;
            type Response = $response;
        }

        impl $name {
            pub fn new(account_id: impl Into<String>) -> Self {
                Self {
                    inner: $crate::core::set::SetRequest::new(account_id),
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::core::set::SetRequest<$obj>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}

/// Generates a JMAP /changes method struct that wraps `ChangesRequest`.
#[macro_export]
macro_rules! define_changes_method {
    ($name:ident, $method_name:expr, $cap:ty, $response:ty) => {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct $name {
            #[serde(flatten)]
            inner: $crate::core::changes::ChangesRequest,
        }

        impl $crate::core::method::JmapMethod for $name {
            const NAME: &'static str = $method_name;
            type Cap = $cap;
            type Response = $response;
        }

        impl $name {
            pub fn new(
                account_id: impl Into<String>,
                since_state: impl Into<String>,
            ) -> Self {
                Self {
                    inner: $crate::core::changes::ChangesRequest::new(
                        account_id,
                        since_state,
                    ),
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::core::changes::ChangesRequest;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}

/// Generates a JMAP /query method struct that wraps `QueryRequest<O>`.
#[macro_export]
macro_rules! define_query_method {
    ($name:ident, $obj:ty, $method_name:expr, $cap:ty) => {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct $name {
            #[serde(flatten)]
            inner: $crate::core::query::QueryRequest<$obj>,
        }

        impl $crate::core::method::JmapMethod for $name {
            const NAME: &'static str = $method_name;
            type Cap = $cap;
            type Response = $crate::core::query::QueryResponse;
        }

        impl $name {
            pub fn new(account_id: impl Into<String>) -> Self {
                Self {
                    inner: $crate::core::query::QueryRequest::new(account_id),
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::core::query::QueryRequest<$obj>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}

/// Generates a JMAP /queryChanges method struct that wraps `QueryChangesRequest<O>`.
#[macro_export]
macro_rules! define_query_changes_method {
    ($name:ident, $obj:ty, $method_name:expr, $cap:ty) => {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct $name {
            #[serde(flatten)]
            inner: $crate::core::query_changes::QueryChangesRequest<$obj>,
        }

        impl $crate::core::method::JmapMethod for $name {
            const NAME: &'static str = $method_name;
            type Cap = $cap;
            type Response = $crate::core::query_changes::QueryChangesResponse;
        }

        impl $name {
            pub fn new(
                account_id: impl Into<String>,
                since_query_state: impl Into<String>,
            ) -> Self {
                Self {
                    inner: $crate::core::query_changes::QueryChangesRequest::new(
                        account_id,
                        since_query_state,
                    ),
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::core::query_changes::QueryChangesRequest<$obj>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}

/// Generates a JMAP /copy method struct that wraps `CopyRequest<O>`.
#[macro_export]
macro_rules! define_copy_method {
    ($name:ident, $obj:ty, $method_name:expr, $cap:ty, $response:ty) => {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct $name {
            #[serde(flatten)]
            inner: $crate::core::copy::CopyRequest<$obj>,
        }

        impl $crate::core::method::JmapMethod for $name {
            const NAME: &'static str = $method_name;
            type Cap = $cap;
            type Response = $response;
        }

        impl $name {
            pub fn new(
                account_id: impl Into<String>,
                from_account_id: impl Into<String>,
            ) -> Self {
                Self {
                    inner: $crate::core::copy::CopyRequest::new(
                        account_id,
                        from_account_id,
                    ),
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::core::copy::CopyRequest<$obj>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}
