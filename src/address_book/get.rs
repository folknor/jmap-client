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

use crate::{
    core::field::Field,
    core::get::GetObject,
    Get, Set,
};

use super::{AddressBook, AddressBookRights};

impl AddressBook<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_value().map(String::as_str)
    }

    /// Full three-state access to the description field.
    pub fn description_field(&self) -> &Field<String> {
        &self.description
    }

    pub fn sort_order(&self) -> Option<u32> {
        self.sort_order
    }

    pub fn is_default(&self) -> Option<bool> {
        self.is_default
    }

    pub fn is_subscribed(&self) -> Option<bool> {
        self.is_subscribed
    }

    pub fn share_with(&self) -> Option<&HashMap<String, AddressBookRights>> {
        self.share_with.as_value()
    }

    /// Full three-state access to the share_with field.
    pub fn share_with_field(&self) -> &Field<HashMap<String, AddressBookRights>> {
        &self.share_with
    }

    pub fn my_rights(&self) -> Option<&AddressBookRights> {
        self.my_rights.as_ref()
    }
}

impl GetObject for AddressBook<Set> {
    type GetArguments = ();
}

impl GetObject for AddressBook<Get> {
    type GetArguments = ();
}
