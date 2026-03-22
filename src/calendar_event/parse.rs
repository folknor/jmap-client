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

use serde::{Deserialize, Serialize};

use super::{CalendarEvent, Property};
use crate::{core::RequestParams, Error};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct CalendarEventParseRequest {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "blobIds")]
    blob_ids: Vec<String>,

    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<Property>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalendarEventParseResponse {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "parsed")]
    parsed: Option<HashMap<String, Vec<CalendarEvent>>>,

    #[serde(rename = "notParsable")]
    not_parsable: Option<Vec<String>>,

    #[serde(rename = "notFound")]
    not_found: Option<Vec<String>>,
}

impl CalendarEventParseRequest {
    pub fn new(params: RequestParams<'_>) -> Self {
        CalendarEventParseRequest {
            account_id: params.account_id.to_string(),
            blob_ids: Vec::new(),
            properties: None,
        }
    }

    pub fn blob_ids<U, V>(&mut self, blob_ids: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.blob_ids = blob_ids.into_iter().map(std::convert::Into::into).collect();
        self
    }

    pub fn properties(
        &mut self,
        properties: impl IntoIterator<Item = Property>,
    ) -> &mut Self {
        self.properties = Some(properties.into_iter().collect());
        self
    }
}

impl CalendarEventParseResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn parsed(&mut self, blob_id: &str) -> crate::Result<Vec<CalendarEvent>> {
        if let Some(result) = self.parsed.as_mut().and_then(|r| r.remove(blob_id)) {
            Ok(result)
        } else if self
            .not_parsable
            .as_ref()
            .map(|np| np.iter().any(|id| id == blob_id))
            .unwrap_or(false)
        {
            Err(Error::Internal(format!(
                "blobId {blob_id} is not parsable."
            )))
        } else {
            Err(Error::Internal(format!("blobId {blob_id} not found.")))
        }
    }

    pub fn parsed_list(
        &self,
    ) -> Option<impl Iterator<Item = (&String, &Vec<CalendarEvent>)>> {
        self.parsed.as_ref().map(|map| map.iter())
    }

    pub fn not_parsable(&self) -> Option<&[String]> {
        self.not_parsable.as_deref()
    }

    pub fn not_found(&self) -> Option<&[String]> {
        self.not_found.as_deref()
    }
}
