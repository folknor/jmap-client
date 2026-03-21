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

use ahash::AHashMap;

use crate::{core::get::GetObject, Get, Set};

use super::{
    Alert, CalendarEvent, EventStatus, FreeBusyStatus, Link, Location, Participant,
    RecurrenceRule, VirtualLocation,
};

impl CalendarEvent<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn uid(&self) -> Option<&str> {
        self.uid.as_deref()
    }

    pub fn calendar_ids(&self) -> Option<&AHashMap<String, bool>> {
        self.calendar_ids.as_ref()
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn description_content_type(&self) -> Option<&str> {
        self.description_content_type.as_deref()
    }

    pub fn created(&self) -> Option<&str> {
        self.created.as_deref()
    }

    pub fn updated(&self) -> Option<&str> {
        self.updated.as_deref()
    }

    pub fn start(&self) -> Option<&str> {
        self.start.as_deref()
    }

    pub fn duration(&self) -> Option<&str> {
        self.duration.as_deref()
    }

    pub fn time_zone(&self) -> Option<Option<&str>> {
        self.time_zone.as_ref().map(|t| t.as_deref())
    }

    pub fn show_without_time(&self) -> Option<bool> {
        self.show_without_time
    }

    pub fn status(&self) -> Option<&EventStatus> {
        self.status.as_ref()
    }

    pub fn free_busy_status(&self) -> Option<&FreeBusyStatus> {
        self.free_busy_status.as_ref()
    }

    pub fn recurrence_id(&self) -> Option<&str> {
        self.recurrence_id.as_deref()
    }

    pub fn recurrence_rules(&self) -> Option<&[RecurrenceRule]> {
        self.recurrence_rules.as_deref()
    }

    pub fn recurrence_overrides(&self) -> Option<&AHashMap<String, serde_json::Value>> {
        self.recurrence_overrides.as_ref()
    }

    pub fn excluded_recurrence_rules(&self) -> Option<&[RecurrenceRule]> {
        self.excluded_recurrence_rules.as_deref()
    }

    pub fn priority(&self) -> Option<u8> {
        self.priority
    }

    pub fn color(&self) -> Option<Option<&str>> {
        self.color.as_ref().map(|c| c.as_deref())
    }

    pub fn locale(&self) -> Option<Option<&str>> {
        self.locale.as_ref().map(|l| l.as_deref())
    }

    pub fn keywords(&self) -> Option<&AHashMap<String, bool>> {
        self.keywords.as_ref()
    }

    pub fn categories(&self) -> Option<&AHashMap<String, bool>> {
        self.categories.as_ref()
    }

    pub fn prod_id(&self) -> Option<&str> {
        self.prod_id.as_deref()
    }

    pub fn reply_to(&self) -> Option<&AHashMap<String, String>> {
        self.reply_to.as_ref()
    }

    pub fn participants(&self) -> Option<&AHashMap<String, Participant>> {
        self.participants.as_ref()
    }

    pub fn use_default_alerts(&self) -> Option<bool> {
        self.use_default_alerts
    }

    pub fn alerts(&self) -> Option<Option<&AHashMap<String, Alert>>> {
        self.alerts.as_ref().map(|a| a.as_ref())
    }

    pub fn locations(&self) -> Option<&AHashMap<String, Location>> {
        self.locations.as_ref()
    }

    pub fn virtual_locations(&self) -> Option<&AHashMap<String, VirtualLocation>> {
        self.virtual_locations.as_ref()
    }

    pub fn links(&self) -> Option<&AHashMap<String, Link>> {
        self.links.as_ref()
    }

    pub fn method(&self) -> Option<&str> {
        self.method.as_deref()
    }

    pub fn sequence(&self) -> Option<u32> {
        self.sequence
    }
}

impl GetObject for CalendarEvent<Set> {
    type GetArguments = ();
}

impl GetObject for CalendarEvent<Get> {
    type GetArguments = ();
}
