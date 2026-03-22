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

use std::future::Future;

/// Transport-level error. Wraps the underlying HTTP client error
/// without leaking it into the crate's public error model.
#[derive(Debug)]
pub struct TransportError {
    pub message: String,
    /// HTTP response body, if available (for parsing ProblemDetails).
    pub body: Option<Vec<u8>>,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl TransportError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            body: None,
            source: None,
        }
    }

    pub fn with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            body: None,
            source: Some(Box::new(source)),
        }
    }

    pub fn with_body(message: impl Into<String>, body: Vec<u8>) -> Self {
        Self {
            message: message.into(),
            body: Some(body),
            source: None,
        }
    }
}

/// HTTP transport abstraction.
///
/// Implement this trait to use a custom HTTP client. The default
/// implementation uses `reqwest`.
pub trait HttpTransport: Send + Sync + 'static {
    /// Send a JMAP API request (POST with JSON body).
    fn api_request(
        &self,
        url: &str,
        body: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;

    /// Upload a blob (POST with binary body).
    fn upload(
        &self,
        url: &str,
        body: Vec<u8>,
        content_type: Option<&str>,
    ) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;

    /// Download a blob (GET, returns raw bytes).
    fn download(&self, url: &str)
        -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;

    /// Fetch the session resource (GET, returns JSON).
    fn get_session(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;
}

/// Streaming transport for Server-Sent Events (EventSource).
///
/// Implement this to provide EventSource support with a custom HTTP client.
/// The default implementation uses reqwest's byte streaming.
pub trait SseTransport: Send + Sync + 'static {
    /// The byte stream type returned by the SSE connection.
    type ByteStream: futures_util::Stream<Item = Result<Vec<u8>, TransportError>>
        + Send
        + Unpin;

    /// Open an SSE connection to the given URL.
    fn open_sse(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Self::ByteStream, TransportError>> + Send;
}
