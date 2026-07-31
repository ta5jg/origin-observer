// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/response.rs
// Purpose : JSON-RPC response model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! JSON-RPC response model.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{RpcError, RpcResult};
use crate::request::JSON_RPC_VERSION;

/// JSON-RPC error object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcResponseError {
    /// JSON-RPC error code.
    pub code: i64,
    /// JSON-RPC error message.
    pub message: String,
}

/// JSON-RPC response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcResponse {
    jsonrpc: String,
    id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    error: Option<RpcResponseError>,
}

impl RpcResponse {
    /// Creates a successful JSON-RPC response.
    #[must_use]
    pub fn success(id: u64, result: Value) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_owned(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Creates an error JSON-RPC response.
    #[must_use]
    pub fn failure(id: u64, code: i64, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_owned(),
            id,
            result: None,
            error: Some(RpcResponseError {
                code,
                message: message.into(),
            }),
        }
    }

    /// Returns the response id.
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Returns the same response carrying a different request id.
    ///
    /// A replayed fixture answers the caller's request, so it adopts that
    /// request's id while leaving the recorded result untouched.
    #[must_use]
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    /// Returns the result when the response carries one.
    #[must_use]
    pub const fn result(&self) -> Option<&Value> {
        self.result.as_ref()
    }

    /// Returns the error object when the response carries one.
    #[must_use]
    pub const fn error(&self) -> Option<&RpcResponseError> {
        self.error.as_ref()
    }

    /// Returns whether the node answered with an error object.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// Returns the raw result or an explicit response error.
    pub fn into_result(self) -> RpcResult<Value> {
        if let Some(error) = self.error {
            return Err(RpcError::Response {
                code: error.code,
                message: error.message,
            });
        }

        Ok(self.result.unwrap_or(Value::Null))
    }

    /// Serializes the full JSON-RPC response as a JSON value.
    #[must_use]
    pub fn to_json_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}
