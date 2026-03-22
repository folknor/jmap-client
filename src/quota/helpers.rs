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
        response::QuotaGetResponse,
    },
    Get, Method, Set,
};

use super::{Property, Quota};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn quota_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<Quota>> {
        let mut request = self.build();
        let get_request = request.get_quota().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<QuotaGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn quota_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<Quota<Get>>> {
        let mut request = self.build();
        request
            .changes_quota(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }

    #[maybe_async::maybe_async]
    pub async fn quota_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_quota();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }
}

impl Request<'_> {
    pub fn get_quota(&mut self) -> &mut GetRequest<Quota<Set>> {
        self.add_capability(crate::URI::Quota);
        self.add_method_call(
            Method::GetQuota,
            Arguments::quota_get(self.params(Method::GetQuota)),
        )
        .quota_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_quota(self) -> crate::Result<QuotaGetResponse> {
        self.send_single().await
    }

    pub fn changes_quota(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(crate::URI::Quota);
        self.add_method_call(
            Method::ChangesQuota,
            Arguments::changes(self.params(Method::ChangesQuota), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_quota(self) -> crate::Result<ChangesResponse<Quota<Get>>> {
        self.send_single().await
    }

    pub fn query_quota(&mut self) -> &mut QueryRequest<Quota<Set>> {
        self.add_capability(crate::URI::Quota);
        self.add_method_call(
            Method::QueryQuota,
            Arguments::quota_query(self.params(Method::QueryQuota)),
        )
        .quota_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_quota(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_quota_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<Quota<Set>> {
        self.add_capability(crate::URI::Quota);
        self.add_method_call(
            Method::QueryChangesQuota,
            Arguments::quota_query_changes(
                self.params(Method::QueryChangesQuota),
                since_query_state.into(),
            ),
        )
        .quota_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_quota_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }
}
