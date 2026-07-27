// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/client.rs
// Purpose : Attributable RPC client.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Attributable RPC client.

use oo_core::{Digest, ProviderId};

use crate::endpoint::RpcEndpoint;
use crate::error::RpcResult;
use crate::request::RpcRequest;
use crate::retry::RetryPolicy;
use crate::trace::RpcTrace;
use crate::transport::RpcTransport;

/// RPC client that records attribution and integrity trace data.
pub struct RpcClient<T> {
    provider_id: ProviderId,
    endpoint: RpcEndpoint,
    transport: T,
    retry: RetryPolicy,
}

impl<T> RpcClient<T>
where
    T: RpcTransport,
{
    /// Creates an RPC client.
    #[must_use]
    pub const fn new(provider_id: ProviderId, endpoint: RpcEndpoint, transport: T) -> Self {
        Self {
            provider_id,
            endpoint,
            transport,
            retry: RetryPolicy::new(1, 0),
        }
    }

    /// Sets the retry policy.
    #[must_use]
    pub const fn with_retry(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    /// Sends a request and returns an attributable trace.
    pub async fn observe(&self, request: RpcRequest) -> RpcResult<RpcTrace> {
        let mut attempt = 1;

        loop {
            match self.transport.send(&self.endpoint, &request).await {
                Ok(response) => {
                    let digest = trace_digest(&request, &response);

                    return Ok(RpcTrace::new(
                        self.provider_id,
                        self.endpoint.clone(),
                        request,
                        response,
                        attempt,
                        digest,
                    ));
                }
                Err(error) if attempt < self.retry.max_attempts() => {
                    attempt += 1;
                    let _ = self.retry.backoff_ms();
                    drop(error);
                }
                Err(error) => return Err(error),
            }
        }
    }
}

fn trace_digest(request: &RpcRequest, response: &crate::response::RpcResponse) -> Digest {
    let request_id = request.id().to_le_bytes();
    let response_id = response.id().to_le_bytes();
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&request_id);
    bytes[8..16].copy_from_slice(&response_id);
    Digest::new(bytes)
}
