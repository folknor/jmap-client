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
        response::{ParticipantIdentityGetResponse, ParticipantIdentitySetResponse},
        set::SetRequest,
    },
    Get, Method, Set,
};

use super::{ParticipantIdentity, Property};

impl Client {
    pub async fn participant_identity_get(
        &self,
        id: &str,
        properties: Option<Vec<Property>>,
    ) -> crate::Result<Option<ParticipantIdentity>> {
        let mut request = self.build();
        let get_request = request.get_participant_identity().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<ParticipantIdentityGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    pub async fn participant_identity_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<ParticipantIdentity<Get>>> {
        let mut request = self.build();
        request
            .changes_participant_identity(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_participant_identity(
        &mut self,
    ) -> &mut GetRequest<ParticipantIdentity<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::GetParticipantIdentity,
            Arguments::participant_identity_get(
                self.params(Method::GetParticipantIdentity),
            ),
        )
        .participant_identity_get_mut()
    }

    pub async fn send_get_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentityGetResponse> {
        self.send_single().await
    }

    pub fn set_participant_identity(
        &mut self,
    ) -> &mut SetRequest<ParticipantIdentity<Set>> {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::SetParticipantIdentity,
            Arguments::participant_identity_set(
                self.params(Method::SetParticipantIdentity),
            ),
        )
        .participant_identity_set_mut()
    }

    pub async fn send_set_participant_identity(
        self,
    ) -> crate::Result<ParticipantIdentitySetResponse> {
        self.send_single().await
    }

    pub fn changes_participant_identity(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(crate::URI::Calendars);
        self.add_method_call(
            Method::ChangesParticipantIdentity,
            Arguments::changes(
                self.params(Method::ChangesParticipantIdentity),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    pub async fn send_changes_participant_identity(
        self,
    ) -> crate::Result<ChangesResponse<ParticipantIdentity<Get>>> {
        self.send_single().await
    }
}
