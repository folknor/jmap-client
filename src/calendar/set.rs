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

use crate::{calendar_event::Alert, core::set::SetObject, Get, Set};

use super::{Calendar, CalendarRights, CalendarSetArguments, IncludeInAvailability};

impl Calendar<Set> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(&mut self, description: Option<impl Into<String>>) -> &mut Self {
        self.description = Some(description.map(std::convert::Into::into));
        self
    }

    pub fn color(&mut self, color: Option<impl Into<String>>) -> &mut Self {
        self.color = Some(color.map(std::convert::Into::into));
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

    pub fn is_visible(&mut self, is_visible: bool) -> &mut Self {
        self.is_visible = Some(is_visible);
        self
    }

    pub fn include_in_availability(
        &mut self,
        include: IncludeInAvailability,
    ) -> &mut Self {
        self.include_in_availability = Some(include);
        self
    }

    pub fn default_alerts_with_time(
        &mut self,
        alerts: Option<HashMap<String, Alert>>,
    ) -> &mut Self {
        self.default_alerts_with_time = Some(alerts);
        self
    }

    pub fn default_alerts_without_time(
        &mut self,
        alerts: Option<HashMap<String, Alert>>,
    ) -> &mut Self {
        self.default_alerts_without_time = Some(alerts);
        self
    }

    pub fn time_zone(&mut self, time_zone: Option<impl Into<String>>) -> &mut Self {
        self.time_zone = Some(time_zone.map(std::convert::Into::into));
        self
    }

    pub fn share_with(
        &mut self,
        share_with: Option<HashMap<String, CalendarRights>>,
    ) -> &mut Self {
        self.share_with = Some(share_with);
        self
    }
}

impl SetObject for Calendar<Set> {
    type SetArguments = CalendarSetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        Calendar {
            _create_id,
            _state: Default::default(),
            id: None,
            name: None,
            description: None,
            color: None,
            sort_order: None,
            is_subscribed: None,
            is_visible: None,
            is_default: None,
            include_in_availability: None,
            default_alerts_with_time: None,
            default_alerts_without_time: None,
            time_zone: None,
            share_with: None,
            my_rights: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{id}"))
    }
}

impl SetObject for Calendar<Get> {
    type SetArguments = CalendarSetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
