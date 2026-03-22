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
        response::{AddressBookGetResponse, AddressBookSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set,
};

use super::{AddressBook, Property};

impl Client {
    pub async fn address_book_create(
        &self,
        name: impl Into<String>,
    ) -> crate::Result<AddressBook> {
        let mut request = self.build();
        let id = request
            .set_address_book()
            .create()
            .name(name)
            .is_subscribed(true)
            .create_id()
            .unwrap();
        request
            .send_single::<AddressBookSetResponse>()
            .await?
            .created(&id)
    }

    pub async fn address_book_destroy(
        &self,
        id: &str,
        remove_contents: bool,
    ) -> crate::Result<()> {
        let mut request = self.build();
        request
            .set_address_book()
            .destroy([id])
            .arguments()
            .on_destroy_remove_contents(remove_contents);
        request
            .send_single::<AddressBookSetResponse>()
            .await?
            .destroyed(id)
    }

    pub async fn address_book_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<AddressBook>> {
        let mut request = self.build();
        let get_request = request.get_address_book().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<AddressBookGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    pub async fn address_book_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<AddressBook<Get>>> {
        let mut request = self.build();
        request
            .changes_address_book(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_address_book(&mut self) -> &mut GetRequest<AddressBook<Set>> {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::GetAddressBook,
            Arguments::address_book_get(self.params(Method::GetAddressBook)),
        )
        .address_book_get_mut()
    }

    pub async fn send_get_address_book(self) -> crate::Result<AddressBookGetResponse> {
        self.send_single().await
    }

    pub fn set_address_book(&mut self) -> &mut SetRequest<AddressBook<Set>> {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::SetAddressBook,
            Arguments::address_book_set(self.params(Method::SetAddressBook)),
        )
        .address_book_set_mut()
    }

    pub async fn send_set_address_book(self) -> crate::Result<AddressBookSetResponse> {
        self.send_single().await
    }

    pub fn changes_address_book(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(crate::URI::Contacts);
        self.add_method_call(
            Method::ChangesAddressBook,
            Arguments::changes(self.params(Method::ChangesAddressBook), since_state.into()),
        )
        .changes_mut()
    }

    pub async fn send_changes_address_book(self) -> crate::Result<ChangesResponse<AddressBook<Get>>> {
        self.send_single().await
    }
}
