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

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Method;

pub mod changes;
pub mod copy;
pub mod error;
pub mod get;
pub mod query;
pub mod query_changes;
pub mod request;
pub mod response;
pub mod session;
pub mod set;

pub struct RequestParams {
    pub account_id: String,
    pub method: Method,
    pub call_id: usize,
}

impl RequestParams {
    pub fn new(account_id: impl Into<String>, method: Method, call_id: usize) -> Self {
        Self {
            account_id: account_id.into(),
            method,
            call_id,
        }
    }
}

pub trait Object: Sized {
    type Property: Display + Serialize + for<'de> Deserialize<'de>;
    fn requires_account_id() -> bool;
}

/// Generates a JSON-map-backed JMAP object type with custom
/// Serialize/Deserialize, Object, ChangesObject, and SetObject impls.
///
/// Used for types like CalendarEvent and ContactCard where the property
/// set is open-ended (JSCalendar/JSContact).
#[macro_export]
macro_rules! json_object_struct {
    ($name:ident, $expecting:expr, $property:ty, $set_args:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name<State = $crate::Get> {
            _create_id: Option<usize>,
            _state: std::marker::PhantomData<State>,
            /// The raw properties map. Every key/value from the server is
            /// preserved, including vendor extension properties.
            pub properties: serde_json::Map<String, serde_json::Value>,
        }

        impl<State> serde::Serialize for $name<State> {
            fn serialize<S: serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(self.properties.len()))?;
                for (k, v) in &self.properties {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }

        impl<'de, State> serde::Deserialize<'de> for $name<State> {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                struct JsonObjectVisitor<S>(std::marker::PhantomData<S>);

                impl<'de, S> serde::de::Visitor<'de> for JsonObjectVisitor<S> {
                    type Value = $name<S>;

                    fn expecting(
                        &self,
                        f: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        f.write_str($expecting)
                    }

                    fn visit_map<M: serde::de::MapAccess<'de>>(
                        self,
                        mut map: M,
                    ) -> Result<Self::Value, M::Error> {
                        let mut properties = serde_json::Map::new();
                        while let Some((key, value)) =
                            map.next_entry::<String, serde_json::Value>()?
                        {
                            properties.insert(key, value);
                        }
                        Ok($name {
                            _create_id: None,
                            _state: std::marker::PhantomData,
                            properties,
                        })
                    }
                }

                deserializer
                    .deserialize_map(JsonObjectVisitor(std::marker::PhantomData))
            }
        }

        impl $crate::core::Object for $name<$crate::Set> {
            type Property = $property;
            fn requires_account_id() -> bool {
                true
            }
        }

        impl $crate::core::Object for $name<$crate::Get> {
            type Property = $property;
            fn requires_account_id() -> bool {
                true
            }
        }

        impl $crate::core::changes::ChangesObject for $name<$crate::Set> {
            type ChangesResponse = ();
        }

        impl $crate::core::changes::ChangesObject for $name<$crate::Get> {
            type ChangesResponse = ();
        }

        impl $crate::core::set::SetObject for $name<$crate::Set> {
            type SetArguments = $set_args;

            fn new(_create_id: Option<usize>) -> Self {
                $name {
                    _create_id,
                    _state: Default::default(),
                    properties: serde_json::Map::new(),
                }
            }

            fn create_id(&self) -> Option<String> {
                self._create_id.map(|id| format!("c{}", id))
            }
        }

        impl $crate::core::set::SetObject for $name<$crate::Get> {
            type SetArguments = $set_args;

            fn new(_create_id: Option<usize>) -> Self {
                unimplemented!()
            }

            fn create_id(&self) -> Option<String> {
                None
            }
        }
    };
}
