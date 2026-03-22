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

use crate::client::Client;

use super::{Thread, ThreadGet};

impl<Tr: crate::core::transport::HttpTransport> Client<Tr> {
    pub async fn thread_get(&self, id: &str) -> crate::Result<Option<Thread>> {
        let mut request = self.build();
        let account_id = request.default_account_id().to_string();
        let mut get = ThreadGet::new(&account_id);
        get.ids([id]);
        let handle = request.call(get)?;
        let mut response = request.send().await?;
        response.get(&handle).map(|mut r| r.take_list().pop())
    }
}
