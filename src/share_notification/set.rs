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

//! ShareNotification is destroy-only — no create or update is permitted.

use crate::{core::set::{SetObject, SetObjectCreatable}, Get, Set};

use super::ShareNotification;

impl SetObject for ShareNotification<Set> {
    type SetArguments = ();

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{id}"))
    }
}

impl SetObjectCreatable for ShareNotification<Set> {
    fn new(_create_id: Option<usize>) -> Self {
        ShareNotification {
            _create_id,
            _state: Default::default(),
            id: None,
            created: None,
            changed_by: None,
            object_type: None,
            object_account_id: None,
            object_id: None,
            old_rights: None,
            new_rights: None,
            name: None,
        }
    }
}

impl SetObject for ShareNotification<Get> {
    type SetArguments = ();

    fn create_id(&self) -> Option<String> {
        None
    }
}
