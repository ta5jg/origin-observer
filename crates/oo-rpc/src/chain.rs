// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/chain.rs
// Purpose : Confirm an endpoint serves the chain an observation expects.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Confirm an endpoint serves the chain an observation expects.
//!
//! An endpoint URL is a claim, not a fact. A misconfigured entry, a redirected
//! host or a provider that quietly serves a testnet would produce observations
//! attributed to the wrong network, and every finding built on them would be
//! wrong in a way no later stage can detect. The chain id is therefore read
//! from the endpoint itself and compared against the expectation before the
//! observation is trusted.

use serde_json::{json, Value};

use crate::{
    endpoint::RpcEndpoint,
    error::{RpcError, RpcResult},
    request::RpcRequest,
    response::RpcResponse,
    transport::RpcTransport,
};

/// JSON-RPC method that reports the chain id.
pub const CHAIN_ID_METHOD: &str = "eth_chainId";

/// Builds the chain identification request.
pub fn chain_id_request(id: u64) -> RpcResult<RpcRequest> {
    RpcRequest::new(id, CHAIN_ID_METHOD, json!([]))
}

/// Reads the chain id out of a response.
///
/// Nodes answer with a hexadecimal string; some return a plain number. Both are
/// accepted, and anything else is an explicit failure rather than a default.
pub fn chain_id_from_response(endpoint: &RpcEndpoint, response: &RpcResponse) -> RpcResult<u64> {
    let value = response.clone().into_result()?;
    let observed = match &value {
        Value::String(text) => text
            .strip_prefix("0x")
            .and_then(|hex| u64::from_str_radix(hex, 16).ok())
            .or_else(|| text.parse::<u64>().ok()),
        Value::Number(number) => number.as_u64(),
        _ => None,
    };

    observed.ok_or_else(|| RpcError::UnreadableChainId {
        endpoint: endpoint.url().to_string(),
        value: value.to_string(),
    })
}

/// Compares an observed chain id against the expected one.
pub fn verify_chain_id(endpoint: &RpcEndpoint, observed: u64, expected: u64) -> RpcResult<()> {
    if observed == expected {
        return Ok(());
    }
    Err(RpcError::ChainMismatch {
        endpoint: endpoint.url().to_string(),
        observed,
        expected,
    })
}

/// Asks an endpoint which chain it serves and checks it against `expected`.
pub async fn confirm_chain<T>(
    transport: &T,
    endpoint: &RpcEndpoint,
    expected: u64,
    request_id: u64,
) -> RpcResult<u64>
where
    T: RpcTransport,
{
    let request = chain_id_request(request_id)?;
    let response = transport.send(endpoint, &request).await?;
    let observed = chain_id_from_response(endpoint, &response)?;
    verify_chain_id(endpoint, observed, expected)?;
    Ok(observed)
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

    struct StaticTransport(RpcResponse);

    #[async_trait]
    impl RpcTransport for StaticTransport {
        async fn send(
            &self,
            _endpoint: &RpcEndpoint,
            _request: &RpcRequest,
        ) -> RpcResult<RpcResponse> {
            Ok(self.0.clone())
        }
    }

    fn endpoint() -> RpcEndpoint {
        RpcEndpoint::parse("https://rpc.example.invalid").expect("endpoint")
    }

    #[test]
    fn a_hexadecimal_chain_id_is_read() {
        let response = RpcResponse::success(1, json!("0x38"));
        assert_eq!(
            chain_id_from_response(&endpoint(), &response).expect("chain id"),
            56
        );
    }

    #[test]
    fn a_numeric_chain_id_is_read() {
        let response = RpcResponse::success(1, json!(137));
        assert_eq!(
            chain_id_from_response(&endpoint(), &response).expect("chain id"),
            137
        );
    }

    #[test]
    fn an_unusable_chain_id_is_an_explicit_failure() {
        let response = RpcResponse::success(1, json!({"chain": "ethereum"}));
        let error = chain_id_from_response(&endpoint(), &response).expect_err("must fail");
        assert!(error.to_string().contains("unusable chain id"), "{error}");
    }

    #[test]
    fn a_node_error_is_reported_rather_than_treated_as_a_missing_chain_id() {
        let response = RpcResponse::failure(1, -32601, "method not found");
        let error = chain_id_from_response(&endpoint(), &response).expect_err("must fail");
        assert!(error.to_string().contains("method not found"), "{error}");
    }

    #[test]
    fn a_matching_chain_is_accepted() {
        assert!(verify_chain_id(&endpoint(), 1, 1).is_ok());
    }

    #[test]
    fn a_mismatched_chain_names_both_ids() {
        let error = verify_chain_id(&endpoint(), 11_155_111, 1).expect_err("must fail");
        let message = error.to_string();
        assert!(message.contains("11155111"), "{message}");
        assert!(message.contains("expected chain 1"), "{message}");
    }

    #[tokio::test]
    async fn confirming_a_chain_reads_it_from_the_endpoint() {
        let transport = StaticTransport(RpcResponse::success(1, json!("0x1")));
        let observed = confirm_chain(&transport, &endpoint(), 1, 1)
            .await
            .expect("confirmed");
        assert_eq!(observed, 1);
    }

    #[tokio::test]
    async fn confirming_a_chain_rejects_the_wrong_network() {
        // A testnet answering where mainnet was configured must stop the run,
        // not produce observations labelled mainnet.
        let transport = StaticTransport(RpcResponse::success(1, json!("0xaa36a7")));
        let error = confirm_chain(&transport, &endpoint(), 1, 1)
            .await
            .expect_err("must fail");
        assert!(matches!(error, RpcError::ChainMismatch { .. }), "{error}");
    }
}
