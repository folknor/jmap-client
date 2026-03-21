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

use serde_json::json;

use crate::{core::set::SetObject, Get, Set};

use super::ContactCard;

impl ContactCard<Set> {
    pub fn uid(&mut self, uid: impl Into<String>) -> &mut Self {
        self.properties
            .insert("uid".into(), serde_json::Value::String(uid.into()));
        self
    }

    pub fn address_book_ids<U, V>(&mut self, address_book_ids: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        let map: serde_json::Map<String, serde_json::Value> = address_book_ids
            .into_iter()
            .map(|id| (id.into(), json!(true)))
            .collect();
        self.properties
            .insert("addressBookIds".into(), serde_json::Value::Object(map));
        self
    }

    pub fn address_book_id(
        &mut self,
        address_book_id: impl Into<String>,
        set: bool,
    ) -> &mut Self {
        let entry = self
            .properties
            .entry("addressBookIds")
            .or_insert_with(|| json!({}));
        if let Some(map) = entry.as_object_mut() {
            map.insert(address_book_id.into(), json!(set));
        }
        self
    }

    pub fn kind(&mut self, kind: impl Into<String>) -> &mut Self {
        self.properties
            .insert("kind".into(), serde_json::Value::String(kind.into()));
        self
    }

    pub fn full_name(&mut self, full_name: impl Into<String>) -> &mut Self {
        self.properties
            .insert("fullName".into(), serde_json::Value::String(full_name.into()));
        self
    }

    pub fn name(
        &mut self,
        name: serde_json::Map<String, serde_json::Value>,
    ) -> &mut Self {
        self.properties
            .insert("name".into(), serde_json::Value::Object(name));
        self
    }

    pub fn emails(
        &mut self,
        emails: serde_json::Map<String, serde_json::Value>,
    ) -> &mut Self {
        self.properties
            .insert("emails".into(), serde_json::Value::Object(emails));
        self
    }

    pub fn phones(
        &mut self,
        phones: serde_json::Map<String, serde_json::Value>,
    ) -> &mut Self {
        self.properties
            .insert("phones".into(), serde_json::Value::Object(phones));
        self
    }

    pub fn addresses(
        &mut self,
        addresses: serde_json::Map<String, serde_json::Value>,
    ) -> &mut Self {
        self.properties
            .insert("addresses".into(), serde_json::Value::Object(addresses));
        self
    }

    pub fn organizations(
        &mut self,
        organizations: serde_json::Map<String, serde_json::Value>,
    ) -> &mut Self {
        self.properties.insert(
            "organizations".into(),
            serde_json::Value::Object(organizations),
        );
        self
    }

    pub fn online_services(
        &mut self,
        online_services: serde_json::Map<String, serde_json::Value>,
    ) -> &mut Self {
        self.properties.insert(
            "onlineServices".into(),
            serde_json::Value::Object(online_services),
        );
        self
    }

    pub fn notes(
        &mut self,
        notes: serde_json::Map<String, serde_json::Value>,
    ) -> &mut Self {
        self.properties
            .insert("notes".into(), serde_json::Value::Object(notes));
        self
    }

    /// Set any property by name. Use this for extension properties or
    /// less-common JSContact properties not covered by typed methods.
    pub fn set_property(
        &mut self,
        name: impl Into<String>,
        value: serde_json::Value,
    ) -> &mut Self {
        self.properties.insert(name.into(), value);
        self
    }
}

impl SetObject for ContactCard<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        ContactCard {
            _create_id,
            _state: Default::default(),
            properties: serde_json::Map::new(),
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for ContactCard<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
