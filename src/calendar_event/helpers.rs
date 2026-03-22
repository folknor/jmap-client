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
        copy::CopyRequest,
        get::GetRequest,
        query::{Comparator, Filter, QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{
            CalendarEventCopyResponse, CalendarEventGetResponse, CalendarEventSetResponse,
        },
        set::SetRequest,
    },
    Get, Method, Set,
};

use super::{
    parse::{CalendarEventParseRequest, CalendarEventParseResponse},
    CalendarEvent, Property,
};

impl Client {
    pub async fn calendar_event_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<CalendarEvent>> {
        let mut request = self.build();
        let get_request = request.get_calendar_event().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties);
        }
        request
            .send_single::<CalendarEventGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    pub async fn calendar_event_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_calendar_event().destroy([id]);
        request
            .send_single::<CalendarEventSetResponse>()
            .await?
            .destroyed(id)
    }

    pub async fn calendar_event_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: Option<usize>,
    ) -> crate::Result<ChangesResponse<CalendarEvent<Get>>> {
        let mut request = self.build();
        let changes_request = request.changes_calendar_event(since_state);
        if let Some(max_changes) = max_changes {
            changes_request.max_changes(max_changes);
        }
        request.send_single().await
    }

    pub async fn calendar_event_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_calendar_event();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort);
        }
        request.send_single::<QueryResponse>().await
    }

    pub async fn calendar_event_parse(
        &self,
        blob_id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Vec<CalendarEvent>> {
        let mut request = self.build();
        let parse_request = request.parse_calendar_event().blob_ids([blob_id]);
        if let Some(properties) = properties {
            parse_request.properties(properties);
        }
        request
            .send_single::<CalendarEventParseResponse>()
            .await
            .and_then(|mut r| r.parsed(blob_id))
    }
}

impl Request<'_> {
    pub fn copy_calendar_event(
        &mut self,
        from_account_id: impl Into<String>,
    ) -> &mut CopyRequest<CalendarEvent<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::CopyCalendarEvent,
            Arguments::calendar_event_copy(
                self.params(Method::CopyCalendarEvent),
                from_account_id.into(),
            ),
        )
        .calendar_event_copy_mut()
    }

    pub async fn send_copy_calendar_event(self) -> crate::Result<CalendarEventCopyResponse> {
        self.send_single().await
    }

    pub fn get_calendar_event(&mut self) -> &mut GetRequest<CalendarEvent<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::GetCalendarEvent,
            Arguments::calendar_event_get(self.params(Method::GetCalendarEvent)),
        )
        .calendar_event_get_mut()
    }

    pub async fn send_get_calendar_event(self) -> crate::Result<CalendarEventGetResponse> {
        self.send_single().await
    }

    pub fn set_calendar_event(&mut self) -> &mut SetRequest<CalendarEvent<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::SetCalendarEvent,
            Arguments::calendar_event_set(self.params(Method::SetCalendarEvent)),
        )
        .calendar_event_set_mut()
    }

    pub async fn send_set_calendar_event(self) -> crate::Result<CalendarEventSetResponse> {
        self.send_single().await
    }

    pub fn changes_calendar_event(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::ChangesCalendarEvent,
            Arguments::changes(
                self.params(Method::ChangesCalendarEvent),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    pub async fn send_changes_calendar_event(
        self,
    ) -> crate::Result<ChangesResponse<CalendarEvent<Get>>> {
        self.send_single().await
    }

    pub fn query_calendar_event(&mut self) -> &mut QueryRequest<CalendarEvent<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::QueryCalendarEvent,
            Arguments::calendar_event_query(self.params(Method::QueryCalendarEvent)),
        )
        .calendar_event_query_mut()
    }

    pub async fn send_query_calendar_event(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_calendar_event_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<CalendarEvent<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::QueryChangesCalendarEvent,
            Arguments::calendar_event_query_changes(
                self.params(Method::QueryChangesCalendarEvent),
                since_query_state.into(),
            ),
        )
        .calendar_event_query_changes_mut()
    }

    pub async fn send_query_calendar_event_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn parse_calendar_event(&mut self) -> &mut CalendarEventParseRequest {
        self.add_capability(crate::URI::CalendarsParse);
        self.add_method_call(
            Method::ParseCalendarEvent,
            Arguments::calendar_event_parse(self.params(Method::ParseCalendarEvent)),
        )
        .calendar_event_parse_mut()
    }

    pub async fn send_parse_calendar_event(self) -> crate::Result<CalendarEventParseResponse> {
        self.send_single().await
    }
}
