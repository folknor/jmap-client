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
    calendar_event::Alert,
    core::get::GetObject,
    Get, Set,
};

use super::{Calendar, CalendarRights, IncludeInAvailability};

impl Calendar<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<Option<&str>> {
        self.description
            .as_ref()
            .map(|d| d.as_deref())
    }

    pub fn color(&self) -> Option<Option<&str>> {
        self.color.as_ref().map(|c| c.as_deref())
    }

    pub fn sort_order(&self) -> Option<u32> {
        self.sort_order
    }

    pub fn is_subscribed(&self) -> Option<bool> {
        self.is_subscribed
    }

    pub fn is_visible(&self) -> Option<bool> {
        self.is_visible
    }

    pub fn is_default(&self) -> Option<bool> {
        self.is_default
    }

    pub fn include_in_availability(&self) -> Option<&IncludeInAvailability> {
        self.include_in_availability.as_ref()
    }

    pub fn default_alerts_with_time(&self) -> Option<Option<&HashMap<String, Alert>>> {
        self.default_alerts_with_time
            .as_ref()
            .map(|a| a.as_ref())
    }

    pub fn default_alerts_without_time(&self) -> Option<Option<&HashMap<String, Alert>>> {
        self.default_alerts_without_time
            .as_ref()
            .map(|a| a.as_ref())
    }

    pub fn time_zone(&self) -> Option<Option<&str>> {
        self.time_zone.as_ref().map(|t| t.as_deref())
    }

    pub fn share_with(&self) -> Option<Option<&HashMap<String, CalendarRights>>> {
        self.share_with.as_ref().map(|s| s.as_ref())
    }

    pub fn my_rights(&self) -> Option<&CalendarRights> {
        self.my_rights.as_ref()
    }
}

impl GetObject for Calendar<Set> {
    type GetArguments = ();
}

impl GetObject for Calendar<Get> {
    type GetArguments = ();
}
