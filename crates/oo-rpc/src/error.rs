// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/error.rs
// Purpose : RPC error types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! RPC error types.

use thiserror::Error;

/// RPC crate result type.
pub type RpcResult<T> = Result<T, RpcError>;

/// Errors produced while preparing, sending or replaying RPC observations.
#[derive(Debug, Error)]
pub enum RpcError {
    /// Endpoint URL is not usable for JSON-RPC transport.
    #[error("invalid RPC endpoint: {0}")]
    InvalidEndpoint(String),

    /// Request method is empty or malformed.
    #[error("invalid RPC method: {0}")]
    InvalidMethod(String),

    /// A matching replay fixture was not found.
    #[error("RPC fixture not found for request {0}")]
    MissingFixture(u64),

    /// Transport failed before a JSON-RPC response was available.
    #[error("RPC transport failed: {0}")]
    Transport(String),

    /// The JSON-RPC response itself contains an error object.
    #[error("RPC response error {code}: {message}")]
    Response {
        /// JSON-RPC error code.
        code: i64,
        /// JSON-RPC error message.
        message: String,
    },
}
