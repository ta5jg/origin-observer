// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/request.rs
// Purpose : JSON-RPC request model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! JSON-RPC request model.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{RpcError, RpcResult};

/// JSON-RPC protocol version used by Origin Observer.
pub const JSON_RPC_VERSION: &str = "2.0";

/// Deterministic JSON-RPC request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Value,
}

impl RpcRequest {
    /// Creates a JSON-RPC request.
    pub fn new(id: u64, method: impl Into<String>, params: Value) -> RpcResult<Self> {
        let method = method.into();

        if method.trim().is_empty() {
            return Err(RpcError::InvalidMethod(
                "method must not be empty".to_owned(),
            ));
        }

        Ok(Self {
            jsonrpc: JSON_RPC_VERSION.to_owned(),
            id,
            method,
            params,
        })
    }

    /// Returns the request id.
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Returns the JSON-RPC method.
    #[must_use]
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Returns the JSON-RPC params.
    #[must_use]
    pub const fn params(&self) -> &Value {
        &self.params
    }
}
