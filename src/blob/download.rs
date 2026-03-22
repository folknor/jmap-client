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

use crate::{client::Client, core::session::URLPart};

impl<Tr: crate::core::transport::HttpTransport> Client<Tr> {
    pub async fn download(&self, blob_id: &str) -> crate::Result<Vec<u8>> {
        let account_id = self.default_account_id();
        let mut download_url = String::with_capacity(
            self.session().download_url().len() + account_id.len() + blob_id.len(),
        );

        for part in self.download_url() {
            match part {
                URLPart::Value(value) => download_url.push_str(value),
                URLPart::Parameter(param) => match param {
                    super::URLParameter::AccountId => download_url.push_str(account_id),
                    super::URLParameter::BlobId => download_url.push_str(blob_id),
                    super::URLParameter::Name => download_url.push_str("none"),
                    super::URLParameter::Type => {
                        download_url.push_str("application/octet-stream");
                    }
                },
            }
        }

        self.transport()
            .download(&download_url)
            .await
            .map_err(crate::Error::from)
    }
}
