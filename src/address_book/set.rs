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

use std::collections::HashMap;

use crate::{core::field::Field, core::set::SetObject, Get, Set};

use super::{AddressBook, AddressBookRights, AddressBookSetArguments};

impl AddressBook<Set> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(&mut self, description: Option<impl Into<String>>) -> &mut Self {
        self.description = match description {
            Some(d) => Field::Value(d.into()),
            None => Field::Null,
        };
        self
    }

    pub fn sort_order(&mut self, sort_order: u32) -> &mut Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn is_subscribed(&mut self, is_subscribed: bool) -> &mut Self {
        self.is_subscribed = Some(is_subscribed);
        self
    }

    pub fn share_with(
        &mut self,
        share_with: Option<HashMap<String, AddressBookRights>>,
    ) -> &mut Self {
        self.share_with = match share_with {
            Some(sw) => Field::Value(sw),
            None => Field::Null,
        };
        self
    }
}

impl SetObject for AddressBook<Set> {
    type SetArguments = AddressBookSetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        AddressBook {
            _create_id,
            _state: Default::default(),
            id: None,
            name: None,
            description: Field::Omitted,
            sort_order: None,
            is_default: None,
            is_subscribed: None,
            share_with: Field::Omitted,
            my_rights: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{id}"))
    }
}

impl SetObject for AddressBook<Get> {
    type SetArguments = AddressBookSetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
