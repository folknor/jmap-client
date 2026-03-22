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

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use reqwest::header;
use reqwest::redirect;
use reqwest::Client as HttpClient;

use crate::core::transport::{HttpTransport, TransportError};

static USER_AGENT: &str = concat!("jmap-client/", env!("CARGO_PKG_VERSION"));

/// Default HTTP transport implementation using `reqwest`.
pub struct ReqwestTransport {
    headers: header::HeaderMap,
    timeout: Duration,
    accept_invalid_certs: bool,
    trusted_hosts: Arc<HashSet<String>>,
}

impl ReqwestTransport {
    pub fn new(
        headers: header::HeaderMap,
        timeout: Duration,
        accept_invalid_certs: bool,
        trusted_hosts: Arc<HashSet<String>>,
    ) -> Self {
        Self {
            headers,
            timeout,
            accept_invalid_certs,
            trusted_hosts,
        }
    }

    pub(crate) fn headers(&self) -> &header::HeaderMap {
        &self.headers
    }

    pub(crate) fn redirect_policy(&self) -> redirect::Policy {
        let trusted_hosts = Arc::clone(&self.trusted_hosts);
        redirect::Policy::custom(move |attempt| {
            if attempt.previous().len() > 5 {
                attempt.error("Too many redirects.")
            } else if matches!(attempt.url().host_str(), Some(host) if trusted_hosts.contains(host))
            {
                attempt.follow()
            } else {
                let message = format!(
                    "Aborting redirect request to unknown host '{}'.",
                    attempt.url().host_str().unwrap_or("")
                );
                attempt.error(message)
            }
        })
    }

    fn build_client(&self) -> Result<HttpClient, TransportError> {
        HttpClient::builder()
            .redirect(self.redirect_policy())
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .timeout(self.timeout)
            .default_headers(self.headers.clone())
            .build()
            .map_err(|e| TransportError::with_source("Failed to build HTTP client", e))
    }

    async fn handle_response(
        response: reqwest::Response,
    ) -> Result<Vec<u8>, TransportError> {
        if response.status().is_success() {
            response
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| TransportError::with_source("Failed to read response body", e))
        } else if response
            .headers()
            .get(header::CONTENT_TYPE)
            .is_some_and(|ct| ct.as_bytes().starts_with(b"application/problem+json"))
        {
            let body = response.bytes().await.map_err(|e| {
                TransportError::with_source("Failed to read error response", e)
            })?;
            // Return the problem details body so the caller can parse it
            Err(TransportError::new(
                String::from_utf8_lossy(&body).to_string(),
            ))
        } else {
            Err(TransportError::new(format!(
                "HTTP {}",
                response.status()
            )))
        }
    }
}

impl HttpTransport for ReqwestTransport {
    async fn api_request(
        &self,
        url: &str,
        body: Vec<u8>,
    ) -> Result<Vec<u8>, TransportError> {
        let client = self.build_client()?;
        let response = client
            .post(url)
            .body(body)
            .send()
            .await
            .map_err(|e| TransportError::with_source("API request failed", e))?;
        Self::handle_response(response).await
    }

    async fn upload(
        &self,
        url: &str,
        body: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<Vec<u8>, TransportError> {
        let client = self.build_client()?;
        let mut req = client.post(url);
        if let Some(ct) = content_type {
            req = req.header(header::CONTENT_TYPE, ct);
        }
        let response = req
            .body(body)
            .send()
            .await
            .map_err(|e| TransportError::with_source("Upload failed", e))?;
        Self::handle_response(response).await
    }

    async fn download(&self, url: &str) -> Result<Vec<u8>, TransportError> {
        let client = self.build_client()?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| TransportError::with_source("Download failed", e))?;
        Self::handle_response(response).await
    }

    async fn get_session(&self, url: &str) -> Result<Vec<u8>, TransportError> {
        let client = self.build_client()?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| TransportError::with_source("Session fetch failed", e))?;
        Self::handle_response(response).await
    }
}
