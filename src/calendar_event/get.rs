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

use crate::{core::get::GetObject, Get, Set};

use super::{CalendarEvent, GetArguments};

impl CalendarEvent<Get> {
    pub fn id(&self) -> Option<&str> {
        self.properties.get("id")?.as_str()
    }

    pub fn take_id(&mut self) -> String {
        self.properties
            .remove("id")
            .and_then(|v| match v {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn uid(&self) -> Option<&str> {
        self.properties.get("uid")?.as_str()
    }

    pub fn calendar_ids(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("calendarIds")?.as_object()
    }

    pub fn is_draft(&self) -> Option<bool> {
        self.properties.get("isDraft")?.as_bool()
    }

    pub fn title(&self) -> Option<&str> {
        self.properties.get("title")?.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.properties.get("description")?.as_str()
    }

    pub fn description_content_type(&self) -> Option<&str> {
        self.properties.get("descriptionContentType")?.as_str()
    }

    pub fn created(&self) -> Option<&str> {
        self.properties.get("created")?.as_str()
    }

    pub fn updated(&self) -> Option<&str> {
        self.properties.get("updated")?.as_str()
    }

    pub fn start(&self) -> Option<&str> {
        self.properties.get("start")?.as_str()
    }

    pub fn duration(&self) -> Option<&str> {
        self.properties.get("duration")?.as_str()
    }

    pub fn time_zone(&self) -> Option<&str> {
        self.properties.get("timeZone")?.as_str()
    }

    pub fn show_without_time(&self) -> Option<bool> {
        self.properties.get("showWithoutTime")?.as_bool()
    }

    pub fn status(&self) -> Option<&str> {
        self.properties.get("status")?.as_str()
    }

    pub fn free_busy_status(&self) -> Option<&str> {
        self.properties.get("freeBusyStatus")?.as_str()
    }

    pub fn recurrence_id(&self) -> Option<&str> {
        self.properties.get("recurrenceId")?.as_str()
    }

    pub fn recurrence_rules(&self) -> Option<&Vec<serde_json::Value>> {
        self.properties.get("recurrenceRules")?.as_array()
    }

    pub fn recurrence_overrides(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("recurrenceOverrides")?.as_object()
    }

    pub fn excluded_recurrence_rules(&self) -> Option<&Vec<serde_json::Value>> {
        self.properties.get("excludedRecurrenceRules")?.as_array()
    }

    pub fn excluded_dates(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("excludedDates")?.as_object()
    }

    pub fn priority(&self) -> Option<u64> {
        self.properties.get("priority")?.as_u64()
    }

    pub fn color(&self) -> Option<&str> {
        self.properties.get("color")?.as_str()
    }

    pub fn locale(&self) -> Option<&str> {
        self.properties.get("locale")?.as_str()
    }

    pub fn keywords(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("keywords")?.as_object()
    }

    pub fn categories(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("categories")?.as_object()
    }

    pub fn prod_id(&self) -> Option<&str> {
        self.properties.get("prodId")?.as_str()
    }

    pub fn reply_to(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("replyTo")?.as_object()
    }

    pub fn participants(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("participants")?.as_object()
    }

    pub fn use_default_alerts(&self) -> Option<bool> {
        self.properties.get("useDefaultAlerts")?.as_bool()
    }

    pub fn alerts(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("alerts")?.as_object()
    }

    pub fn locations(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("locations")?.as_object()
    }

    pub fn virtual_locations(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("virtualLocations")?.as_object()
    }

    pub fn links(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("links")?.as_object()
    }

    pub fn related_to(
        &self,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.properties.get("relatedTo")?.as_object()
    }

    pub fn method(&self) -> Option<&str> {
        self.properties.get("method")?.as_str()
    }

    pub fn sequence(&self) -> Option<u64> {
        self.properties.get("sequence")?.as_u64()
    }

    /// Access any property as a raw JSON value, including extension
    /// properties not covered by the typed accessors.
    pub fn property(&self, name: &str) -> Option<&serde_json::Value> {
        self.properties.get(name)
    }

    /// Access the full underlying JSCalendar properties map.
    pub fn as_properties(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.properties
    }
}

impl GetObject for CalendarEvent<Set> {
    type GetArguments = GetArguments;
}

impl GetObject for CalendarEvent<Get> {
    type GetArguments = GetArguments;
}
