// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/fixture.rs
// Purpose : Deterministic RPC replay fixtures.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Deterministic RPC replay fixtures.

use std::collections::BTreeMap;

use async_trait::async_trait;

use crate::endpoint::RpcEndpoint;
use crate::error::{RpcError, RpcResult};
use crate::request::RpcRequest;
use crate::response::RpcResponse;
use crate::transport::RpcTransport;

/// In-memory deterministic replay transport.
#[derive(Debug, Clone, Default)]
pub struct FixtureTransport {
    responses: BTreeMap<u64, RpcResponse>,
}

impl FixtureTransport {
    /// Creates an empty fixture transport.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a response by request id.
    pub fn insert(&mut self, response: RpcResponse) {
        self.responses.insert(response.id(), response);
    }
}

#[async_trait]
impl RpcTransport for FixtureTransport {
    async fn send(&self, _endpoint: &RpcEndpoint, request: &RpcRequest) -> RpcResult<RpcResponse> {
        self.responses
            .get(&request.id())
            .cloned()
            .ok_or(RpcError::MissingFixture(request.id()))
    }
}
