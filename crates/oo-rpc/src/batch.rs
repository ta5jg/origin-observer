// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/batch.rs
// Purpose : JSON-RPC batch model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! JSON-RPC batch model.

use crate::request::RpcRequest;

/// Ordered JSON-RPC request batch.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RpcBatch {
    requests: Vec<RpcRequest>,
}

impl RpcBatch {
    /// Creates an empty batch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a request to the batch.
    pub fn push(&mut self, request: RpcRequest) {
        self.requests.push(request);
    }

    /// Returns batch requests in deterministic order.
    #[must_use]
    pub fn requests(&self) -> &[RpcRequest] {
        &self.requests
    }

    /// Returns true when no requests are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }
}
