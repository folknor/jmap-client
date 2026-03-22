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
    core::request::{Arguments, Request},
    Method, URI,
};

use super::{
    copy::{CopyBlobRequest, CopyBlobResponse},
    manage::{
        BlobGetRequest, BlobGetResponse, BlobLookupRequest, BlobLookupResponse,
        BlobUploadRequest, BlobUploadResponse,
    },
};

impl Client {
    pub async fn blob_copy(
        &self,
        from_account_id: impl Into<String>,
        blob_id: impl Into<String>,
    ) -> crate::Result<String> {
        let blob_id = blob_id.into();
        let mut request = self.build();
        request.copy_blob(from_account_id).blob_id(&blob_id);
        request
            .send_single::<CopyBlobResponse>()
            .await?
            .copied(&blob_id)
    }

    pub async fn blob_upload_text(
        &self,
        text: impl Into<String>,
        type_: Option<impl Into<String>>,
    ) -> crate::Result<String> {
        let mut request = self.build();
        let id = request.upload_blob().create_from_text(text, type_);
        request
            .send_single::<BlobUploadResponse>()
            .await?
            .created(&id)
            .map(|c| c.id)
    }
}

impl Request<'_> {
    pub fn copy_blob(&mut self, from_account_id: impl Into<String>) -> &mut CopyBlobRequest {
        self.add_method_call(
            Method::CopyBlob,
            Arguments::blob_copy(self.params(Method::CopyBlob), from_account_id.into()),
        )
        .blob_copy_mut()
    }

    pub async fn send_copy_blob(self) -> crate::Result<CopyBlobResponse> {
        self.send_single().await
    }

    pub fn upload_blob(&mut self) -> &mut BlobUploadRequest {
        self.add_capability(crate::URI::Blob);
        self.add_method_call(
            Method::UploadBlob,
            Arguments::blob_upload(self.params(Method::UploadBlob)),
        )
        .blob_upload_mut()
    }

    pub async fn send_upload_blob(self) -> crate::Result<BlobUploadResponse> {
        self.send_single().await
    }

    pub fn get_blob(&mut self) -> &mut BlobGetRequest {
        self.add_capability(crate::URI::Blob);
        self.add_method_call(
            Method::GetBlob,
            Arguments::blob_get(self.params(Method::GetBlob)),
        )
        .blob_get_mut()
    }

    pub async fn send_get_blob(self) -> crate::Result<BlobGetResponse> {
        self.send_single().await
    }

    pub fn lookup_blob(&mut self) -> &mut BlobLookupRequest {
        self.add_capability(crate::URI::Blob);
        self.add_method_call(
            Method::LookupBlob,
            Arguments::blob_lookup(self.params(Method::LookupBlob)),
        )
        .blob_lookup_mut()
    }

    /// Create a Blob/lookup request and automatically add the capabilities
    /// required by the given type names. Each type name must have its
    /// defining capability in the `using` array per RFC 9404 §4.3.
    pub fn lookup_blob_with_capabilities(
        &mut self,
        capabilities: impl IntoIterator<Item = URI>,
    ) -> &mut BlobLookupRequest {
        self.add_capability(crate::URI::Blob);
        for cap in capabilities {
            self.add_capability(cap);
        }
        self.add_method_call(
            Method::LookupBlob,
            Arguments::blob_lookup(self.params(Method::LookupBlob)),
        )
        .blob_lookup_mut()
    }

    pub async fn send_lookup_blob(self) -> crate::Result<BlobLookupResponse> {
        self.send_single().await
    }
}
