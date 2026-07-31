// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/diamond.rs
// Purpose : Detect an EIP-2535 diamond via ERC-165 interface detection.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Detect an EIP-2535 diamond via ERC-165 interface detection.
//!
//! A diamond's storage layout is not standardized the way EIP-1967's is, so
//! detection here does not read storage at all. It calls
//! `supportsInterface(bytes4)` — an ERC-165 method, whose own selector this
//! crate derives rather than hardcodes — with the widely published
//! `IDiamondLoupe` interface id. That id is a fixed 4-byte constant rather
//! than something this crate can re-derive from first principles (it is the
//! XOR of several facet function selectors, defined by the loupe interface
//! declared in EIP-2535); it is recorded as a named constant precisely so it
//! is one place to check against the current EIP-2535 text if this module is
//! ever suspected of misclassifying.
//!
//! A `true` result here means the contract *claims* loupe support, which is
//! self-reported by the contract's own `supportsInterface` implementation. It
//! is evidence, not proof: nothing prevents a non-diamond contract from
//! returning `true` for this interface id.

use oo_abi::{decode, AbiType, DecodedValue, FunctionSignature};
use oo_rpc::{RpcClient, RpcRequest, RpcTransport};
use serde_json::json;

use crate::error::{ProxyError, ProxyResult};
use crate::model::{ProxyKind, ProxyResolution};

/// `IDiamondLoupe` interface id per EIP-2535.
pub const DIAMOND_LOUPE_INTERFACE_ID: [u8; 4] = [0x1f, 0x93, 0x1c, 0x1c];

/// Calls `supportsInterface(interfaceId)` and decodes the boolean result.
pub async fn supports_interface<T>(
    client: &RpcClient<T>,
    address: &str,
    interface_id: [u8; 4],
    block: &str,
) -> ProxyResult<bool>
where
    T: RpcTransport,
{
    let call_signature = supports_interface_signature()?;
    let mut call_data = call_signature.selector_hex();
    call_data.push_str(&hex_word_from_bytes4(interface_id));

    let request = RpcRequest::new(
        1,
        "eth_call",
        json!([{"to": address, "data": call_data}, block]),
    )
    .map_err(|error| ProxyError::Rpc(error.to_string()))?;

    let trace = client
        .observe(request)
        .await
        .map_err(|error| ProxyError::Rpc(error.to_string()))?;

    let value = trace
        .response()
        .clone()
        .into_result()
        .map_err(|error| ProxyError::Rpc(error.to_string()))?;
    let serde_json::Value::String(hex) = value else {
        return Err(ProxyError::Rpc(
            "eth_call result was not a string".to_owned(),
        ));
    };
    let bytes = oo_bytecode::parse_hex(&hex).map_err(ProxyError::BytecodeDecode)?;
    match decode(&bytes, AbiType::Bool)? {
        DecodedValue::Bool(supported) => Ok(supported),
        _ => unreachable!("decode(_, AbiType::Bool) always returns DecodedValue::Bool"),
    }
}

/// Classifies a contract as a diamond when it reports loupe support.
///
/// Unlike the EIP-1967 classifications, this cannot run from a single storage
/// read: it requires a live call, so it is a separate entry point rather than
/// a pure function over already-read state.
pub async fn classify<T>(
    client: &RpcClient<T>,
    address: &str,
    block: &str,
) -> ProxyResult<Option<ProxyResolution>>
where
    T: RpcTransport,
{
    let supported = supports_interface(client, address, DIAMOND_LOUPE_INTERFACE_ID, block).await?;
    if !supported {
        return Ok(None);
    }

    let mut resolution = ProxyResolution {
        kind: ProxyKind::Diamond,
        implementation: None,
        admin: None,
        evidence: Vec::new(),
    };
    resolution.record(
        "supportsInterface(IDiamondLoupe)",
        "returned true; this is self-reported by the contract, not independently verified",
    );
    Ok(Some(resolution))
}

fn supports_interface_signature() -> ProxyResult<FunctionSignature> {
    FunctionSignature::of_parts(
        "supportsInterface",
        &[oo_abi::AbiParameter::new("", AbiType::FixedBytes(4))],
    )
    .map_err(ProxyError::Abi)
}

fn hex_word_from_bytes4(interface_id: [u8; 4]) -> String {
    // ABI-encodes a bytes4 argument: left-aligned in its word, right-padded
    // with zero, matching Solidity's fixed-bytes calldata encoding.
    let mut word = String::with_capacity(64);
    for byte in interface_id {
        word.push_str(&format!("{byte:02x}"));
    }
    word.push_str(&"0".repeat(56));
    word
}

#[cfg(test)]
mod tests {
    use oo_core::ProviderId;
    use oo_rpc::{FixtureTransport, RpcEndpoint, RpcResponse};

    use super::*;

    fn endpoint() -> RpcEndpoint {
        RpcEndpoint::parse("https://rpc.example.invalid").expect("endpoint")
    }

    #[test]
    fn hex_word_pads_a_bytes4_value_on_the_right() {
        let word = hex_word_from_bytes4([0x1f, 0x93, 0x1c, 0x1c]);
        assert_eq!(word.len(), 64);
        assert_eq!(&word[..8], "1f931c1c");
        assert!(word[8..].chars().all(|c| c == '0'));
    }

    #[tokio::test]
    async fn a_contract_reporting_loupe_support_is_classified_as_a_diamond() {
        let signature = supports_interface_signature().unwrap();
        let mut call_data = signature.selector_hex();
        call_data.push_str(&hex_word_from_bytes4(DIAMOND_LOUPE_INTERFACE_ID));

        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_call",
            json!([{"to": "0xabc", "data": call_data}, "0x1"]),
            RpcResponse::success(0, json!(format!("0x{}", "0".repeat(63) + "1"))),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        let resolution = classify(&client, "0xabc", "0x1").await.unwrap();
        assert_eq!(resolution.unwrap().kind, ProxyKind::Diamond);
    }

    #[tokio::test]
    async fn a_contract_reporting_no_support_is_not_classified() {
        let signature = supports_interface_signature().unwrap();
        let mut call_data = signature.selector_hex();
        call_data.push_str(&hex_word_from_bytes4(DIAMOND_LOUPE_INTERFACE_ID));

        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_call",
            json!([{"to": "0xabc", "data": call_data}, "0x1"]),
            RpcResponse::success(0, json!(format!("0x{}", "0".repeat(64)))),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        assert!(classify(&client, "0xabc", "0x1").await.unwrap().is_none());
    }
}
