// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/transport.rs
// Purpose : RPC transport abstraction.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! RPC transport abstraction.

use async_trait::async_trait;

use crate::endpoint::RpcEndpoint;
use crate::error::RpcResult;
use crate::request::RpcRequest;
use crate::response::RpcResponse;

/// Sends JSON-RPC requests through a concrete transport.
#[async_trait]
pub trait RpcTransport: Send + Sync {
    /// Sends one request to one endpoint.
    async fn send(&self, endpoint: &RpcEndpoint, request: &RpcRequest) -> RpcResult<RpcResponse>;
}
