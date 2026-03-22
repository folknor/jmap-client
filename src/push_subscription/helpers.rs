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
    core::set::SetObject,
    DataType,
};

use super::{Keys, PushSubscription, PushSubscriptionSet};

impl Client {
    pub async fn push_subscription_create(
        &self,
        device_client_id: impl Into<String>,
        url: impl Into<String>,
        keys: Option<Keys>,
    ) -> crate::Result<PushSubscription> {
        let mut request = self.build();
        let account_id = request.default_account_id().to_string();
        let mut set = PushSubscriptionSet::new(&account_id);
        let create_req = set
            .create()
            .device_client_id(device_client_id)
            .url(url);

        if let Some(keys) = keys {
            create_req.keys(keys);
        }

        let id = create_req.create_id().unwrap();
        let handle = request.call(set)?;
        let mut response = request.send().await?;
        response.get(&handle)?.created(&id)
    }

    pub async fn push_subscription_verify(
        &self,
        id: &str,
        verification_code: impl Into<String>,
    ) -> crate::Result<Option<PushSubscription>> {
        let mut request = self.build();
        let account_id = request.default_account_id().to_string();
        let mut set = PushSubscriptionSet::new(&account_id);
        set.update(id).verification_code(verification_code);
        let handle = request.call(set)?;
        let mut response = request.send().await?;
        response.get(&handle)?.updated(id)
    }

    pub async fn push_subscription_update_types(
        &self,
        id: &str,
        types: Option<impl IntoIterator<Item = DataType>>,
    ) -> crate::Result<Option<PushSubscription>> {
        let mut request = self.build();
        let account_id = request.default_account_id().to_string();
        let mut set = PushSubscriptionSet::new(&account_id);
        set.update(id).types(types);
        let handle = request.call(set)?;
        let mut response = request.send().await?;
        response.get(&handle)?.updated(id)
    }

    pub async fn push_subscription_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        let account_id = request.default_account_id().to_string();
        let mut set = PushSubscriptionSet::new(&account_id);
        set.destroy([id]);
        let handle = request.call(set)?;
        let mut response = request.send().await?;
        response.get(&handle)?.destroyed(id)
    }
}
