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
        id::AccountId,
        transport::HttpTransport,
    },
};

/// An account-scoped view of a [`Client`].
///
/// Pairs a client reference with a specific account ID. Use
/// [`Account::build()`] to create request batches pre-scoped to
/// this account, avoiding the need to thread account IDs manually.
///
/// ```ignore
/// let account = client.account(client.default_account());
/// let mut request = account.build();
/// let handle = request.call(EmailGet::new(account.id_str()))?;
/// let mut response = request.send().await?;
/// let emails = response.get(&handle)?;
/// ```
///
/// This is an unchecked view — the account ID is not validated
/// against the session. Use [`Client::session()`] to check
/// available accounts.
pub struct Account<'a, Tr: HttpTransport> {
    client: &'a Client<Tr>,
    account_id: AccountId,
}

impl<'a, Tr: HttpTransport> Account<'a, Tr> {
    pub fn new(client: &'a Client<Tr>, account_id: impl Into<AccountId>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    /// The account ID.
    pub fn id(&self) -> &AccountId {
        &self.account_id
    }

    /// The account ID as a string slice.
    pub fn id_str(&self) -> &str {
        self.account_id.as_str()
    }

    /// Access the underlying client.
    pub fn client(&self) -> &'a Client<Tr> {
        self.client
    }

    /// Build a request batch scoped to this account.
    pub fn build(&self) -> crate::core::request::Request<'_, Tr> {
        self.client
            .build()
            .account_id(self.account_id.as_str().to_string())
    }
}

impl<Tr: HttpTransport> Client<Tr> {
    /// Create an account-scoped view of this client.
    ///
    /// ```ignore
    /// let account = client.account(client.default_account());
    /// ```
    pub fn account(&self, account_id: impl Into<AccountId>) -> Account<'_, Tr> {
        Account::new(self, account_id)
    }
}
