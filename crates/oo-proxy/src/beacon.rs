// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/beacon.rs
// Purpose : Classify a beacon proxy and resolve its implementation.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Classify a beacon proxy and resolve its implementation.
//!
//! A beacon proxy's EIP-1967 beacon slot names another contract — the beacon
//! — which itself exposes `implementation()`. Resolving the final
//! implementation therefore takes two steps: read the beacon slot, then call
//! the beacon. This module handles both: classification from slot data alone,
//! and the follow-up call for a caller that has an RPC client available.

use oo_abi::FunctionSignature;
use oo_rpc::{RpcClient, RpcRequest, RpcResponse, RpcTransport};
use serde_json::json;

use crate::eip1967::Eip1967Slots;
use crate::error::{ProxyError, ProxyResult};
use crate::model::{ProxyKind, ProxyResolution};

/// Classifies storage as a beacon proxy when the beacon slot is in use.
///
/// The resolved `implementation` here is the beacon's own address; call
/// [`resolve_implementation`] to follow up with the beacon itself.
#[must_use]
pub fn classify(slots: &Eip1967Slots) -> Option<ProxyResolution> {
    let beacon = slots.beacon?;

    let mut resolution = ProxyResolution {
        kind: ProxyKind::Eip1967Beacon,
        implementation: Some(beacon),
        admin: None,
        evidence: Vec::new(),
    };
    resolution.record("eip1967.beacon slot", "non-zero, decodes to an address");
    resolution.record(
        "implementation resolution",
        "not yet followed up; the recorded address is the beacon, not the final implementation",
    );
    Some(resolution)
}

/// Calls a beacon's `implementation()` and decodes the result as an address.
pub async fn resolve_implementation<T>(
    client: &RpcClient<T>,
    beacon_address: &str,
    block: &str,
) -> ProxyResult<[u8; 20]>
where
    T: RpcTransport,
{
    let signature = FunctionSignature::of_parts("implementation", &[])?;
    let request = RpcRequest::new(
        1,
        "eth_call",
        json!([
            {"to": beacon_address, "data": signature.selector_hex()},
            block
        ]),
    )
    .map_err(|error| ProxyError::Rpc(error.to_string()))?;

    let trace = client
        .observe(request)
        .await
        .map_err(|error| ProxyError::Rpc(error.to_string()))?;

    decode_address_result(trace.response())
}

fn decode_address_result(response: &RpcResponse) -> ProxyResult<[u8; 20]> {
    use oo_abi::{decode, AbiType, DecodedValue};

    let value = response
        .clone()
        .into_result()
        .map_err(|error| ProxyError::Rpc(error.to_string()))?;
    let serde_json::Value::String(hex) = value else {
        return Err(ProxyError::Rpc(
            "eth_call result was not a string".to_owned(),
        ));
    };
    let bytes = oo_bytecode::parse_hex(&hex).map_err(ProxyError::BytecodeDecode)?;
    match decode(&bytes, AbiType::Address)? {
        DecodedValue::Address(address) => Ok(address),
        _ => unreachable!("decode(_, AbiType::Address) always returns DecodedValue::Address"),
    }
}

#[cfg(test)]
mod tests {
    use oo_core::ProviderId;
    use oo_rpc::{FixtureTransport, RpcEndpoint};

    use super::*;

    #[test]
    fn a_populated_beacon_slot_classifies_as_beacon() {
        let slots = Eip1967Slots {
            beacon: Some([0x33; 20]),
            ..Eip1967Slots::default()
        };
        let resolution = classify(&slots).expect("must classify");
        assert_eq!(resolution.kind, ProxyKind::Eip1967Beacon);
        assert_eq!(resolution.implementation, Some([0x33; 20]));
    }

    #[test]
    fn an_absent_beacon_slot_does_not_classify() {
        assert!(classify(&Eip1967Slots::default()).is_none());
    }

    #[tokio::test]
    async fn resolving_the_implementation_calls_the_beacon_and_decodes_the_address() {
        let signature = FunctionSignature::of_parts("implementation", &[]).unwrap();
        let mut fixture = FixtureTransport::new();
        let mut result_word = vec![0u8; 12];
        result_word.extend_from_slice(&[0x44; 20]);
        fixture.insert_for(
            "eth_call",
            json!([
                {"to": "0xbeac0000000000000000000000000000000000", "data": signature.selector_hex()},
                "0x1"
            ]),
            RpcResponse::success(0, json!(oo_bytecode::to_hex(&result_word))),
        );

        let client = RpcClient::new(
            ProviderId::new(),
            RpcEndpoint::parse("https://rpc.example.invalid").unwrap(),
            fixture,
        );
        let implementation =
            resolve_implementation(&client, "0xbeac0000000000000000000000000000000000", "0x1")
                .await
                .unwrap();
        assert_eq!(implementation, [0x44; 20]);
    }
}
