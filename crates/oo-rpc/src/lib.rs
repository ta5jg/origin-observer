// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/lib.rs
// Purpose : Perform deterministic and attributable JSON-RPC communication.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Perform deterministic and attributable JSON-RPC communication.

pub mod batch;
pub mod chain;
pub mod client;
pub mod endpoint;
pub mod error;
pub mod fixture;
pub mod http;
pub mod pin;
pub mod ratelimit;
pub mod request;
pub mod response;
pub mod retry;
pub mod trace;
pub mod transport;

pub use batch::RpcBatch;
pub use chain::{confirm_chain, verify_chain_id};
pub use client::{exchange_digest, RpcClient};
pub use endpoint::{RpcEndpoint, RpcEndpointKind};
pub use error::{RpcError, RpcResult};
pub use fixture::{fixture_key, FixtureRecord, FixtureTransport, RecordingTransport};
pub use http::HttpTransport;
pub use pin::{BlockPin, PinPolicy};
pub use ratelimit::{RateDecision, RateLimit, RateLimiter};
pub use request::RpcRequest;
pub use response::{RpcResponse, RpcResponseError};
pub use retry::RetryPolicy;
pub use trace::RpcTrace;
pub use transport::RpcTransport;

#[cfg(test)]
mod tests {
    use oo_core::ProviderId;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn fixture_client_records_trace() {
        let endpoint = RpcEndpoint::parse("https://rpc.example.invalid").unwrap();
        let request = RpcRequest::new(1, "eth_chainId", json!([])).unwrap();
        let response = RpcResponse::success(1, json!("0x1"));

        let mut fixture = FixtureTransport::new();
        fixture.insert_for("eth_chainId", json!([]), response);

        let client = RpcClient::new(ProviderId::new(), endpoint, fixture);
        let trace = client.observe(request).await.unwrap();

        assert_eq!(trace.request().method(), "eth_chainId");
        assert_eq!(trace.response().id(), 1);
        assert_eq!(trace.attempt(), 1);
    }

    #[test]
    fn endpoint_rejects_unsupported_scheme() {
        assert!(RpcEndpoint::parse("ftp://rpc.example.invalid").is_err());
    }
}
