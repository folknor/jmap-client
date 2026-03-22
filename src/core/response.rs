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

use std::collections::HashMap;

use serde::de;
use serde::Deserialize;

use super::error::MethodError;
use super::method::JmapMethod;
use super::request::CallHandle;

/// Raw deserialized JMAP response envelope (used internally).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RawResponse<T> {
    #[serde(rename = "methodResponses")]
    method_responses: Vec<T>,

    #[serde(rename = "createdIds")]
    created_ids: Option<HashMap<String, String>>,

    #[serde(rename = "sessionState")]
    session_state: String,
}

impl<T> RawResponse<T> {
    pub fn unwrap_method_responses(self) -> Vec<T> {
        self.method_responses
    }

    pub fn session_state(&self) -> &str {
        &self.session_state
    }
}

/// A parsed JMAP response with typed method result extraction.
#[derive(Debug)]
pub struct Response {
    raw: Vec<(String, RawCallResult, String)>,
    session_state: String,
    created_ids: Option<HashMap<String, String>>,
}

/// A single method call result — either success data or a method error.
#[derive(Debug)]
enum RawCallResult {
    Success(serde_json::Value),
    Error(MethodError),
}

impl Response {
    /// Extract a typed response by its call handle.
    ///
    /// Compile-time safe: the handle's type parameter ensures the response
    /// is deserialized into the correct type.
    pub fn get<M: JmapMethod>(
        &mut self,
        handle: &CallHandle<M>,
    ) -> crate::Result<M::Response> {
        let pos = self
            .raw
            .iter()
            .position(|(_, _, id)| id == &handle.call_id)
            .ok_or_else(|| {
                crate::Error::Internal(format!("Call {} not found in response", handle.call_id))
            })?;

        let (_, result, _) = self.raw.remove(pos);

        match result {
            RawCallResult::Success(value) => {
                serde_json::from_value(value).map_err(crate::Error::from)
            }
            RawCallResult::Error(e) => Err(e.into()),
        }
    }

    pub fn session_state(&self) -> &str {
        &self.session_state
    }

    pub fn created_ids(&self) -> Option<&HashMap<String, String>> {
        self.created_ids.as_ref()
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawEnvelope {
            #[serde(rename = "methodResponses")]
            method_responses: Vec<serde_json::Value>,
            #[serde(rename = "createdIds")]
            created_ids: Option<HashMap<String, String>>,
            #[serde(rename = "sessionState")]
            session_state: String,
        }

        let envelope = RawEnvelope::deserialize(deserializer)?;
        let mut raw = Vec::with_capacity(envelope.method_responses.len());

        for entry in envelope.method_responses {
            let arr = entry
                .as_array()
                .ok_or_else(|| de::Error::custom("method response must be an array"))?;

            if arr.len() != 3 {
                return Err(de::Error::custom("method response must have 3 elements"));
            }

            let method_name = arr[0]
                .as_str()
                .ok_or_else(|| de::Error::custom("method name must be a string"))?
                .to_string();

            let call_id = arr[2]
                .as_str()
                .ok_or_else(|| de::Error::custom("call id must be a string"))?
                .to_string();

            let result = if method_name == "error" {
                let error: MethodError =
                    serde_json::from_value(arr[1].clone()).map_err(de::Error::custom)?;
                RawCallResult::Error(error)
            } else {
                RawCallResult::Success(arr[1].clone())
            };

            raw.push((method_name, result, call_id));
        }

        Ok(Response {
            raw,
            session_state: envelope.session_state,
            created_ids: envelope.created_ids,
        })
    }
}
