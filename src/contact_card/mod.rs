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

//! ContactCard wraps a JSContact Card (RFC 9553) object.
//!
//! Unlike Email or Mailbox, a ContactCard IS a JSContact object — its
//! property set is open-ended and includes vendor extension properties.
//! The struct therefore stores all properties in a `serde_json::Map`
//! for round-trip fidelity.
//!
//! Typed accessor and builder methods are provided for common properties.
//! For extension or less-common properties, use [`ContactCard::property()`]
//! and [`ContactCard::set_property()`].
//!
//! For vCard ↔ JSContact conversion, the `calcard` crate (re-exported
//! from this crate) can parse the serialized JSON.

pub mod get;
pub mod helpers;
pub mod parse;
pub mod query;
pub mod set;

use std::fmt::{self, Display};

use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};

// Re-export calcard for users who want vCard conversion or deep
// JSContact type access.
pub use calcard;

// ---- ContactCard ----

/// A contact card backed by a JSContact JSON object.
///
/// All JSContact properties (standard and extension) are preserved in the
/// underlying `serde_json::Map`. Typed accessor methods are convenience
/// wrappers that read from / write to this map.
#[derive(Debug, Clone)]
pub struct ContactCard<State = Get> {
    _create_id: Option<usize>,
    _state: std::marker::PhantomData<State>,
    /// The raw JSContact properties. Every key/value from the server is
    /// preserved here, including vendor extension properties.
    pub properties: serde_json::Map<String, serde_json::Value>,
}

impl<State> Serialize for ContactCard<State> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.properties.len()))?;
        for (k, v) in &self.properties {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de, State> Deserialize<'de> for ContactCard<State> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ContactCardVisitor<S>(std::marker::PhantomData<S>);

        impl<'de, S> Visitor<'de> for ContactCardVisitor<S> {
            type Value = ContactCard<S>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a JSContact object")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut properties = serde_json::Map::new();
                while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                    properties.insert(key, value);
                }
                Ok(ContactCard {
                    _create_id: None,
                    _state: std::marker::PhantomData,
                    properties,
                })
            }
        }

        deserializer.deserialize_map(ContactCardVisitor(std::marker::PhantomData))
    }
}

// ---- Property enum ----

/// Property names for ContactCard/get `properties` lists.
///
/// Common JSContact properties have typed variants. Extension or
/// less-common properties use `Other(String)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Property {
    Id,
    Uid,
    AddressBookIds,
    Kind,
    FullName,
    Name,
    Nicknames,
    Emails,
    Phones,
    Addresses,
    Organizations,
    OnlineServices,
    Notes,
    Media,
    Created,
    Updated,
    /// Any JSContact property not covered by the typed variants,
    /// including vendor extension properties.
    Other(String),
}

impl Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Property::Id => "id",
            Property::Uid => "uid",
            Property::AddressBookIds => "addressBookIds",
            Property::Kind => "kind",
            Property::FullName => "fullName",
            Property::Name => "name",
            Property::Nicknames => "nicknames",
            Property::Emails => "emails",
            Property::Phones => "phones",
            Property::Addresses => "addresses",
            Property::Organizations => "organizations",
            Property::OnlineServices => "onlineServices",
            Property::Notes => "notes",
            Property::Media => "media",
            Property::Created => "created",
            Property::Updated => "updated",
            Property::Other(s) => s.as_str(),
        })
    }
}

impl Serialize for Property {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Property {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PropertyVisitor;

        impl<'de> Visitor<'de> for PropertyVisitor {
            type Value = Property;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a JSContact property name")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Property, E> {
                Ok(Property::from(v))
            }
        }

        deserializer.deserialize_str(PropertyVisitor)
    }
}

impl From<&str> for Property {
    fn from(s: &str) -> Self {
        match s {
            "id" => Property::Id,
            "uid" => Property::Uid,
            "addressBookIds" => Property::AddressBookIds,
            "kind" => Property::Kind,
            "fullName" => Property::FullName,
            "name" => Property::Name,
            "nicknames" => Property::Nicknames,
            "emails" => Property::Emails,
            "phones" => Property::Phones,
            "addresses" => Property::Addresses,
            "organizations" => Property::Organizations,
            "onlineServices" => Property::OnlineServices,
            "notes" => Property::Notes,
            "media" => Property::Media,
            "created" => Property::Created,
            "updated" => Property::Updated,
            other => Property::Other(other.to_string()),
        }
    }
}

impl Object for ContactCard<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for ContactCard<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for ContactCard<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for ContactCard<Get> {
    type ChangesResponse = ();
}
