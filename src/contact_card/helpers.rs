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
        response::{ContactCardGetResponse, ContactCardSetResponse},
        set::SetRequest,
    },
    Get, Method, Set,
};

use super::{
    parse::{ContactCardParseRequest, ContactCardParseResponse},
    ContactCard, Property,
};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn contact_card_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<ContactCard>> {
        let mut request = self.build();
        let get_request = request.get_contact_card().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<ContactCardGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn contact_card_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_contact_card().destroy([id]);
        request
            .send_single::<ContactCardSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn contact_card_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: Option<usize>,
    ) -> crate::Result<ChangesResponse<ContactCard<Get>>> {
        let mut request = self.build();
        let changes_request = request.changes_contact_card(since_state);
        if let Some(max_changes) = max_changes {
            changes_request.max_changes(max_changes);
        }
        request.send_single().await
    }

    #[maybe_async::maybe_async]
    pub async fn contact_card_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_contact_card();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    #[maybe_async::maybe_async]
    pub async fn contact_card_parse(
        &self,
        blob_id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Vec<ContactCard>> {
        let mut request = self.build();
        let parse_request = request.parse_contact_card().blob_ids([blob_id]);
        if let Some(properties) = properties {
            parse_request.properties(properties);
        }
        request
            .send_single::<ContactCardParseResponse>()
            .await
            .and_then(|mut r| r.parsed(blob_id))
    }
}

impl Request<'_> {
    pub fn get_contact_card(&mut self) -> &mut GetRequest<ContactCard<Set>> {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::GetContactCard,
            Arguments::contact_card_get(self.params(Method::GetContactCard)),
        )
        .contact_card_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_contact_card(self) -> crate::Result<ContactCardGetResponse> {
        self.send_single().await
    }

    pub fn set_contact_card(&mut self) -> &mut SetRequest<ContactCard<Set>> {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::SetContactCard,
            Arguments::contact_card_set(self.params(Method::SetContactCard)),
        )
        .contact_card_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_contact_card(self) -> crate::Result<ContactCardSetResponse> {
        self.send_single().await
    }

    pub fn changes_contact_card(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::ChangesContactCard,
            Arguments::changes(
                self.params(Method::ChangesContactCard),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_contact_card(
        self,
    ) -> crate::Result<ChangesResponse<ContactCard<Get>>> {
        self.send_single().await
    }

    pub fn query_contact_card(&mut self) -> &mut QueryRequest<ContactCard<Set>> {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::QueryContactCard,
            Arguments::contact_card_query(self.params(Method::QueryContactCard)),
        )
        .contact_card_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_contact_card(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_contact_card_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<ContactCard<Set>> {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::QueryChangesContactCard,
            Arguments::contact_card_query_changes(
                self.params(Method::QueryChangesContactCard),
                since_query_state.into(),
            ),
        )
        .contact_card_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_contact_card_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn parse_contact_card(&mut self) -> &mut ContactCardParseRequest {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::ParseContactCard,
            Arguments::contact_card_parse(self.params(Method::ParseContactCard)),
        )
        .contact_card_parse_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_parse_contact_card(self) -> crate::Result<ContactCardParseResponse> {
        self.send_single().await
    }
}
