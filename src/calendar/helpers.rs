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

use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        request::{Arguments, Request},
        response::{CalendarGetResponse, CalendarSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set,
};

use super::{Calendar, Property};

impl Client {
    pub async fn calendar_create(
        &self,
        name: impl Into<String>,
    ) -> crate::Result<Calendar> {
        let mut request = self.build();
        let id = request
            .set_calendar()
            .create()
            .name(name)
            .is_subscribed(true)
            .is_visible(true)
            .create_id()
            .unwrap();
        request
            .send_single::<CalendarSetResponse>()
            .await?
            .created(&id)
    }

    pub async fn calendar_destroy(
        &self,
        id: &str,
        remove_events: bool,
    ) -> crate::Result<()> {
        let mut request = self.build();
        request
            .set_calendar()
            .destroy([id])
            .arguments()
            .on_destroy_remove_events(remove_events);
        request
            .send_single::<CalendarSetResponse>()
            .await?
            .destroyed(id)
    }

    pub async fn calendar_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<Calendar>> {
        let mut request = self.build();
        let get_request = request.get_calendar().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties);
        }
        request
            .send_single::<CalendarGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    pub async fn calendar_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<Calendar<Get>>> {
        let mut request = self.build();
        request
            .changes_calendar(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_calendar(&mut self) -> &mut GetRequest<Calendar<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::GetCalendar,
            Arguments::calendar_get(self.params(Method::GetCalendar)),
        )
        .calendar_get_mut()
    }

    pub async fn send_get_calendar(self) -> crate::Result<CalendarGetResponse> {
        self.send_single().await
    }

    pub fn set_calendar(&mut self) -> &mut SetRequest<Calendar<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::SetCalendar,
            Arguments::calendar_set(self.params(Method::SetCalendar)),
        )
        .calendar_set_mut()
    }

    pub async fn send_set_calendar(self) -> crate::Result<CalendarSetResponse> {
        self.send_single().await
    }

    pub fn changes_calendar(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::ChangesCalendar,
            Arguments::changes(self.params(Method::ChangesCalendar), since_state.into()),
        )
        .changes_mut()
    }

    pub async fn send_changes_calendar(self) -> crate::Result<ChangesResponse<Calendar<Get>>> {
        self.send_single().await
    }
}
