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
        query::{Comparator, Filter, QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{
            CalendarEventNotificationGetResponse, CalendarEventNotificationSetResponse,
        },
        set::SetRequest,
    },
    Get, Method, Set,
};

use super::{CalendarEventNotification, Property};

impl Client {
    pub async fn calendar_event_notification_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<CalendarEventNotification>> {
        let mut request = self.build();
        let get_request = request.get_calendar_event_notification().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties);
        }
        request
            .send_single::<CalendarEventNotificationGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    pub async fn calendar_event_notification_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_calendar_event_notification().destroy([id]);
        request
            .send_single::<CalendarEventNotificationSetResponse>()
            .await?
            .destroyed(id)
    }

    pub async fn calendar_event_notification_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<CalendarEventNotification<Get>>> {
        let mut request = self.build();
        request
            .changes_calendar_event_notification(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }

    pub async fn calendar_event_notification_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<
            impl IntoIterator<Item = Comparator<super::query::Comparator>>,
        >,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_calendar_event_notification();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort);
        }
        request.send_single::<QueryResponse>().await
    }
}

impl Request<'_> {
    pub fn get_calendar_event_notification(
        &mut self,
    ) -> &mut GetRequest<CalendarEventNotification<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::GetCalendarEventNotification,
            Arguments::calendar_event_notification_get(
                self.params(Method::GetCalendarEventNotification),
            ),
        )
        .calendar_event_notification_get_mut()
    }

    pub async fn send_get_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationGetResponse> {
        self.send_single().await
    }

    pub fn set_calendar_event_notification(
        &mut self,
    ) -> &mut SetRequest<CalendarEventNotification<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::SetCalendarEventNotification,
            Arguments::calendar_event_notification_set(
                self.params(Method::SetCalendarEventNotification),
            ),
        )
        .calendar_event_notification_set_mut()
    }

    pub async fn send_set_calendar_event_notification(
        self,
    ) -> crate::Result<CalendarEventNotificationSetResponse> {
        self.send_single().await
    }

    pub fn changes_calendar_event_notification(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::ChangesCalendarEventNotification,
            Arguments::changes(
                self.params(Method::ChangesCalendarEventNotification),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    pub async fn send_changes_calendar_event_notification(
        self,
    ) -> crate::Result<ChangesResponse<CalendarEventNotification<Get>>> {
        self.send_single().await
    }

    pub fn query_calendar_event_notification(
        &mut self,
    ) -> &mut QueryRequest<CalendarEventNotification<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::QueryCalendarEventNotification,
            Arguments::calendar_event_notification_query(
                self.params(Method::QueryCalendarEventNotification),
            ),
        )
        .calendar_event_notification_query_mut()
    }

    pub async fn send_query_calendar_event_notification(
        self,
    ) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_calendar_event_notification_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<CalendarEventNotification<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::QueryChangesCalendarEventNotification,
            Arguments::calendar_event_notification_query_changes(
                self.params(Method::QueryChangesCalendarEventNotification),
                since_query_state.into(),
            ),
        )
        .calendar_event_notification_query_changes_mut()
    }

    pub async fn send_query_calendar_event_notification_changes(
        self,
    ) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }
}
