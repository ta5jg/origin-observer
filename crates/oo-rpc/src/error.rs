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
    ///
    /// The key is the canonical form of the request, not its id: a fixture set
    /// answers a question, and the question is the method with its parameters.
    #[error("RPC fixture not found for {method} with params {params}")]
    MissingFixture {
        /// Method that was requested.
        method: String,
        /// Canonical JSON parameters that were requested.
        params: String,
    },

    /// A fixture file could not be read or written.
    #[error("RPC fixture store failed for {path}: {message}")]
    FixtureStore {
        /// Path involved in the failure.
        path: String,
        /// Description of the failure.
        message: String,
    },

    /// The configured rate limit for an endpoint was reached.
    #[error(
        "RPC rate limit reached for {endpoint}: {permitted} request(s) per {window_ms}ms; retry in {retry_after_ms}ms"
    )]
    RateLimited {
        /// Endpoint that refused.
        endpoint: String,
        /// Permitted requests per window.
        permitted: u32,
        /// Window length in milliseconds.
        window_ms: u64,
        /// Time until a slot frees.
        retry_after_ms: u64,
    },

    /// A request would read unpinned chain state.
    ///
    /// An observation against `latest` cannot be reproduced, because the state
    /// it read has moved on by the time anyone repeats it.
    #[error(
        "RPC request {method} reads unpinned state ({block}); pin a block or allow unpinned reads explicitly"
    )]
    UnpinnedBlock {
        /// Method that carried the unpinned reference.
        method: String,
        /// Block reference that was rejected.
        block: String,
    },

    /// The endpoint does not serve the chain the observation expected.
    #[error("RPC endpoint {endpoint} serves chain {observed}, not the expected chain {expected}")]
    ChainMismatch {
        /// Endpoint that was queried.
        endpoint: String,
        /// Chain id the endpoint reported.
        observed: u64,
        /// Chain id the configuration expected.
        expected: u64,
    },

    /// The endpoint returned a chain id that could not be interpreted.
    #[error("RPC endpoint {endpoint} returned an unusable chain id: {value}")]
    UnreadableChainId {
        /// Endpoint that was queried.
        endpoint: String,
        /// Raw value that could not be interpreted.
        value: String,
    },

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

impl RpcError {
    /// Returns whether repeating the request could plausibly succeed.
    ///
    /// Only transport failures and rate limits are retryable. A malformed
    /// request, a missing fixture, an unpinned read, a chain mismatch or an
    /// error the node itself returned will fail identically on every attempt,
    /// and retrying them would turn one wrong observation into several.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        matches!(self, Self::Transport(_) | Self::RateLimited { .. })
    }
}
