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

use crate::{core::set::SetObject, Get, Set};

use super::{
    Alert, CalendarEvent, EventStatus, FreeBusyStatus, Link, Location, Participant,
    RecurrenceRule, VirtualLocation,
};

impl CalendarEvent<Set> {
    pub fn uid(&mut self, uid: impl Into<String>) -> &mut Self {
        self.uid = Some(uid.into());
        self
    }

    pub fn calendar_ids<U, V>(&mut self, calendar_ids: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.calendar_ids = Some(
            calendar_ids
                .into_iter()
                .map(|id| (id.into(), true))
                .collect(),
        );
        self
    }

    pub fn calendar_id(&mut self, calendar_id: impl Into<String>, set: bool) -> &mut Self {
        self.calendar_ids
            .get_or_insert_with(AHashMap::new)
            .insert(calendar_id.into(), set);
        self
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn description_content_type(
        &mut self,
        content_type: impl Into<String>,
    ) -> &mut Self {
        self.description_content_type = Some(content_type.into());
        self
    }

    pub fn start(&mut self, start: impl Into<String>) -> &mut Self {
        self.start = Some(start.into());
        self
    }

    pub fn duration(&mut self, duration: impl Into<String>) -> &mut Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn time_zone(&mut self, time_zone: Option<impl Into<String>>) -> &mut Self {
        self.time_zone = Some(time_zone.map(|t| t.into()));
        self
    }

    pub fn show_without_time(&mut self, show_without_time: bool) -> &mut Self {
        self.show_without_time = Some(show_without_time);
        self
    }

    pub fn status(&mut self, status: EventStatus) -> &mut Self {
        self.status = Some(status);
        self
    }

    pub fn free_busy_status(&mut self, free_busy_status: FreeBusyStatus) -> &mut Self {
        self.free_busy_status = Some(free_busy_status);
        self
    }

    pub fn recurrence_rules(&mut self, rules: Vec<RecurrenceRule>) -> &mut Self {
        self.recurrence_rules = Some(rules);
        self
    }

    pub fn recurrence_overrides(
        &mut self,
        overrides: AHashMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.recurrence_overrides = Some(overrides);
        self
    }

    pub fn excluded_recurrence_rules(&mut self, rules: Vec<RecurrenceRule>) -> &mut Self {
        self.excluded_recurrence_rules = Some(rules);
        self
    }

    pub fn priority(&mut self, priority: u8) -> &mut Self {
        self.priority = Some(priority);
        self
    }

    pub fn color(&mut self, color: Option<impl Into<String>>) -> &mut Self {
        self.color = Some(color.map(|c| c.into()));
        self
    }

    pub fn locale(&mut self, locale: Option<impl Into<String>>) -> &mut Self {
        self.locale = Some(locale.map(|l| l.into()));
        self
    }

    pub fn keywords(&mut self, keywords: AHashMap<String, bool>) -> &mut Self {
        self.keywords = Some(keywords);
        self
    }

    pub fn categories(&mut self, categories: AHashMap<String, bool>) -> &mut Self {
        self.categories = Some(categories);
        self
    }

    pub fn reply_to(&mut self, reply_to: AHashMap<String, String>) -> &mut Self {
        self.reply_to = Some(reply_to);
        self
    }

    pub fn participants(&mut self, participants: AHashMap<String, Participant>) -> &mut Self {
        self.participants = Some(participants);
        self
    }

    pub fn use_default_alerts(&mut self, use_default_alerts: bool) -> &mut Self {
        self.use_default_alerts = Some(use_default_alerts);
        self
    }

    pub fn alerts(&mut self, alerts: Option<AHashMap<String, Alert>>) -> &mut Self {
        self.alerts = Some(alerts);
        self
    }

    pub fn locations(&mut self, locations: AHashMap<String, Location>) -> &mut Self {
        self.locations = Some(locations);
        self
    }

    pub fn virtual_locations(
        &mut self,
        virtual_locations: AHashMap<String, VirtualLocation>,
    ) -> &mut Self {
        self.virtual_locations = Some(virtual_locations);
        self
    }

    pub fn links(&mut self, links: AHashMap<String, Link>) -> &mut Self {
        self.links = Some(links);
        self
    }
}

impl SetObject for CalendarEvent<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        CalendarEvent {
            _create_id,
            _state: Default::default(),
            id: None,
            uid: None,
            calendar_ids: AHashMap::new().into(),
            title: None,
            description: None,
            description_content_type: None,
            created: None,
            updated: None,
            start: None,
            duration: None,
            time_zone: None,
            show_without_time: None,
            status: None,
            free_busy_status: None,
            recurrence_id: None,
            recurrence_id_time_zone: None,
            recurrence_rules: Vec::with_capacity(0).into(),
            recurrence_overrides: None,
            excluded_recurrence_rules: Vec::with_capacity(0).into(),
            priority: None,
            color: None,
            locale: None,
            keywords: AHashMap::new().into(),
            categories: AHashMap::new().into(),
            prod_id: None,
            reply_to: AHashMap::new().into(),
            participants: AHashMap::new().into(),
            use_default_alerts: None,
            alerts: None,
            locations: AHashMap::new().into(),
            virtual_locations: AHashMap::new().into(),
            links: AHashMap::new().into(),
            method: None,
            sequence: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for CalendarEvent<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
