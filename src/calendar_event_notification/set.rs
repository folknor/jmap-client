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

use crate::{core::set::SetObject, Get, Set};

use super::CalendarEventNotification;

impl SetObject for CalendarEventNotification<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        CalendarEventNotification {
            _create_id,
            _state: Default::default(),
            id: None,
            created: None,
            changed_by: None,
            calendar_event_id: None,
            is_draft: None,
            type_: None,
            event: None,
            event_patch: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for CalendarEventNotification<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
