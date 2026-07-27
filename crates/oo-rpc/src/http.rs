// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/http.rs
// Purpose : HTTP JSON-RPC transport.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! HTTP JSON-RPC transport.

use async_trait::async_trait;

use crate::endpoint::RpcEndpoint;
use crate::error::{RpcError, RpcResult};
use crate::request::RpcRequest;
use crate::response::RpcResponse;
use crate::transport::RpcTransport;

/// Reqwest-backed JSON-RPC transport.
#[derive(Debug, Clone)]
pub struct HttpTransport {
    client: reqwest::Client,
}

impl Default for HttpTransport {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl HttpTransport {
    /// Creates an HTTP transport.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl RpcTransport for HttpTransport {
    async fn send(&self, endpoint: &RpcEndpoint, request: &RpcRequest) -> RpcResult<RpcResponse> {
        let response = self
            .client
            .post(endpoint.url().clone())
            .json(request)
            .send()
            .await
            .map_err(|error| RpcError::Transport(error.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(RpcError::Transport(format!("HTTP status {status}")));
        }

        response
            .json::<RpcResponse>()
            .await
            .map_err(|error| RpcError::Transport(error.to_string()))
    }
}
