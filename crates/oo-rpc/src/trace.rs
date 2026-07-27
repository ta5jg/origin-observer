// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/trace.rs
// Purpose : RPC observation trace.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! RPC observation trace.

use oo_core::{Digest, ProviderId};

use crate::endpoint::RpcEndpoint;
use crate::request::RpcRequest;
use crate::response::RpcResponse;

/// Attributable RPC observation.
#[derive(Debug, Clone, PartialEq)]
pub struct RpcTrace {
    provider_id: ProviderId,
    endpoint: RpcEndpoint,
    request: RpcRequest,
    response: RpcResponse,
    attempt: u32,
    digest: Digest,
}

impl RpcTrace {
    /// Creates an RPC trace.
    #[must_use]
    pub const fn new(
        provider_id: ProviderId,
        endpoint: RpcEndpoint,
        request: RpcRequest,
        response: RpcResponse,
        attempt: u32,
        digest: Digest,
    ) -> Self {
        Self {
            provider_id,
            endpoint,
            request,
            response,
            attempt,
            digest,
        }
    }

    /// Returns the provider that produced the observation.
    #[must_use]
    pub const fn provider_id(&self) -> ProviderId {
        self.provider_id
    }

    /// Returns the endpoint used for the observation.
    #[must_use]
    pub const fn endpoint(&self) -> &RpcEndpoint {
        &self.endpoint
    }

    /// Returns the request.
    #[must_use]
    pub const fn request(&self) -> &RpcRequest {
        &self.request
    }

    /// Returns the response.
    #[must_use]
    pub const fn response(&self) -> &RpcResponse {
        &self.response
    }

    /// Returns the attempt number.
    #[must_use]
    pub const fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Returns the integrity digest.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.digest
    }
}
