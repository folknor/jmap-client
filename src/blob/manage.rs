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

//! RFC 9404 JMAP Blob Management — Blob/upload, Blob/get, Blob/lookup.
//!
//! These are JMAP method calls (not HTTP endpoint operations).

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::{core::RequestParams, Error};

// ---- Blob/upload (RFC 9404 Section 3) ----

/// Request for `Blob/upload` — create blobs via JMAP method call.
#[derive(Debug, Clone, Serialize)]
pub struct BlobUploadRequest {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "create")]
    create: AHashMap<String, BlobUploadCreate>,
}

/// A single blob creation entry.
#[derive(Debug, Clone, Serialize)]
pub struct BlobUploadCreate {
    #[serde(rename = "data")]
    pub data: Vec<DataSource>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Data source for blob assembly.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DataSource {
    Blob(DataSourceBlob),
    String(DataSourceString),
}

/// Reference another blob (with optional range) as a data source.
#[derive(Debug, Clone, Serialize)]
pub struct DataSourceBlob {
    #[serde(rename = "data:asBlob")]
    pub blob_id: String,

    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,

    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
}

/// A literal string value as a data source.
#[derive(Debug, Clone, Serialize)]
pub struct DataSourceString {
    #[serde(rename = "data:asText")]
    pub value: String,
}

/// Response for `Blob/upload`.
#[derive(Debug, Clone, Deserialize)]
pub struct BlobUploadResponse {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "created")]
    created: Option<AHashMap<String, BlobUploadCreated>>,

    #[serde(rename = "notCreated")]
    not_created: Option<AHashMap<String, serde_json::Value>>,
}

/// Result of a successfully created blob.
#[derive(Debug, Clone, Deserialize)]
pub struct BlobUploadCreated {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub type_: Option<String>,

    #[serde(rename = "size")]
    pub size: Option<u64>,
}

impl BlobUploadRequest {
    pub fn new(params: RequestParams) -> Self {
        BlobUploadRequest {
            account_id: params.account_id,
            create: AHashMap::new(),
        }
    }

    /// Add a blob creation entry. Returns the create id for referencing.
    pub fn create_from_text(
        &mut self,
        text: impl Into<String>,
        type_: Option<impl Into<String>>,
    ) -> String {
        let create_id = format!("b{}", self.create.len());
        self.create.insert(
            create_id.clone(),
            BlobUploadCreate {
                data: vec![DataSource::String(DataSourceString {
                    value: text.into(),
                })],
                type_: type_.map(|t| t.into()),
            },
        );
        create_id
    }

    /// Add a blob creation entry from a blob reference.
    pub fn create_from_blob(
        &mut self,
        blob_id: impl Into<String>,
        offset: Option<u64>,
        length: Option<u64>,
        type_: Option<impl Into<String>>,
    ) -> String {
        let create_id = format!("b{}", self.create.len());
        self.create.insert(
            create_id.clone(),
            BlobUploadCreate {
                data: vec![DataSource::Blob(DataSourceBlob {
                    blob_id: blob_id.into(),
                    offset,
                    length,
                })],
                type_: type_.map(|t| t.into()),
            },
        );
        create_id
    }

    /// Add a blob creation entry with arbitrary data sources.
    pub fn create_with_sources(
        &mut self,
        data: Vec<DataSource>,
        type_: Option<impl Into<String>>,
    ) -> String {
        let create_id = format!("b{}", self.create.len());
        self.create.insert(
            create_id.clone(),
            BlobUploadCreate {
                data,
                type_: type_.map(|t| t.into()),
            },
        );
        create_id
    }
}

impl BlobUploadResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn created(&mut self, id: &str) -> crate::Result<BlobUploadCreated> {
        if let Some(result) = self.created.as_mut().and_then(|r| r.remove(id)) {
            Ok(result)
        } else if let Some(error) = self.not_created.as_ref().and_then(|r| r.get(id)) {
            Err(Error::Internal(format!("Blob {} not created: {:?}", id, error)))
        } else {
            Err(Error::Internal(format!("Id {} not found.", id)))
        }
    }

    pub fn created_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.created.as_ref().map(|map| map.keys())
    }
}

// ---- Blob/get (RFC 9404 Section 4) ----

/// Request for `Blob/get` — retrieve blob content with optional range.
#[derive(Debug, Clone, Serialize)]
pub struct BlobGetRequest {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "ids")]
    ids: Vec<BlobGetItem>,
}

/// A single blob get entry with optional range and encoding.
#[derive(Debug, Clone, Serialize)]
pub struct BlobGetItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,

    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,

    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(rename = "digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<Vec<String>>,
}

/// Response for `Blob/get`.
#[derive(Debug, Clone, Deserialize)]
pub struct BlobGetResponse {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "list")]
    list: Vec<BlobGetResult>,

    #[serde(rename = "notFound")]
    not_found: Option<Vec<String>>,
}

/// Result for a single blob get entry.
#[derive(Debug, Clone, Deserialize)]
pub struct BlobGetResult {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "data")]
    pub data: Option<String>,

    #[serde(rename = "size")]
    pub size: Option<u64>,

    #[serde(rename = "digest")]
    pub digest: Option<AHashMap<String, String>>,

    #[serde(rename = "isEncodingProblem")]
    pub is_encoding_problem: Option<bool>,

    #[serde(rename = "isTruncated")]
    pub is_truncated: Option<bool>,
}

impl BlobGetRequest {
    pub fn new(params: RequestParams) -> Self {
        BlobGetRequest {
            account_id: params.account_id,
            ids: Vec::new(),
        }
    }

    /// Add a blob to retrieve by ID.
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.push(BlobGetItem {
            id: id.into(),
            offset: None,
            length: None,
            encoding: None,
            digest: None,
        });
        self
    }

    /// Add a blob to retrieve with range and options.
    pub fn id_with_options(&mut self, item: BlobGetItem) -> &mut Self {
        self.ids.push(item);
        self
    }
}

impl BlobGetResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn list(&self) -> &[BlobGetResult] {
        &self.list
    }

    pub fn take_list(&mut self) -> Vec<BlobGetResult> {
        std::mem::take(&mut self.list)
    }

    pub fn not_found(&self) -> Option<&[String]> {
        self.not_found.as_deref()
    }
}

impl BlobGetItem {
    pub fn new(id: impl Into<String>) -> Self {
        BlobGetItem {
            id: id.into(),
            offset: None,
            length: None,
            encoding: None,
            digest: None,
        }
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn length(mut self, length: u64) -> Self {
        self.length = Some(length);
        self
    }

    pub fn encoding(mut self, encoding: impl Into<String>) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    pub fn digest<U, V>(mut self, algorithms: U) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.digest = Some(algorithms.into_iter().map(|a| a.into()).collect());
        self
    }
}

// ---- Blob/lookup (RFC 9404 Section 5) ----

/// Request for `Blob/lookup` — reverse lookup which objects reference a blob.
#[derive(Debug, Clone, Serialize)]
pub struct BlobLookupRequest {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "typeNames")]
    type_names: Vec<String>,

    #[serde(rename = "ids")]
    ids: Vec<String>,
}

/// Response for `Blob/lookup`.
#[derive(Debug, Clone, Deserialize)]
pub struct BlobLookupResponse {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "list")]
    list: Vec<BlobLookupResult>,

    #[serde(rename = "notFound")]
    not_found: Option<Vec<String>>,
}

/// Result for a single blob lookup entry.
#[derive(Debug, Clone, Deserialize)]
pub struct BlobLookupResult {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "matchedIds")]
    pub matched_ids: AHashMap<String, Vec<String>>,
}

impl BlobLookupRequest {
    pub fn new(params: RequestParams) -> Self {
        BlobLookupRequest {
            account_id: params.account_id,
            type_names: Vec::new(),
            ids: Vec::new(),
        }
    }

    pub fn type_names<U, V>(&mut self, type_names: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.type_names = type_names.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn ids<U, V>(&mut self, ids: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.ids = ids.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl BlobLookupResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn list(&self) -> &[BlobLookupResult] {
        &self.list
    }

    pub fn take_list(&mut self) -> Vec<BlobLookupResult> {
        std::mem::take(&mut self.list)
    }

    pub fn not_found(&self) -> Option<&[String]> {
        self.not_found.as_deref()
    }
}
