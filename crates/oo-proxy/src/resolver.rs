// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/resolver.rs
// Purpose : Resolve a contract's proxy architecture end to end.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Resolve a contract's proxy architecture end to end.
//!
//! Resolution tries the cheapest, most specific check first and only reaches
//! for a live call when storage alone cannot decide: EIP-1167 is an exact
//! bytecode match requiring one `eth_getCode`; EIP-1967 and the legacy
//! OpenZeppelin layout come from one batch of storage reads; diamond
//! detection is tried last because it costs an additional `eth_call` and
//! reports only self-declared interface support. A contract matching none of
//! these returns [`crate::model::ProxyKind::Unknown`] with the checks that
//! were performed recorded as evidence — never a guess.

use oo_rpc::{RpcClient, RpcRequest, RpcTransport};
use oo_storage::{read_layout, StorageLayout};
use serde_json::json;

use crate::diamond;
use crate::eip1167;
use crate::eip1967;
use crate::error::{ProxyError, ProxyResult};
use crate::model::{ProxyKind, ProxyResolution};
use crate::transparent;
use crate::uups;
use crate::validation::validate_address;

/// Resolves proxy architecture for one contract address at one pinned block.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProxyResolver;

impl ProxyResolver {
    /// Runs the full resolution sequence.
    pub async fn resolve<T>(
        &self,
        client: &RpcClient<T>,
        address: &str,
        block: &str,
    ) -> ProxyResult<ProxyResolution>
    where
        T: RpcTransport,
    {
        validate_address(address)?;

        if let Some(resolution) = self.try_eip1167(client, address, block).await? {
            return Ok(resolution);
        }

        let slots = self.read_eip1967_slots(client, address, block).await?;

        if let Some(resolution) = transparent::classify(&slots) {
            return Ok(resolution);
        }
        if let Some(resolution) = uups::classify(&slots) {
            return Ok(resolution);
        }
        if let Some(mut resolution) = crate::beacon::classify(&slots) {
            resolution.record(
                "resolver",
                "beacon detected; call resolve_implementation separately to reach the final address",
            );
            return Ok(resolution);
        }
        if let Some(legacy) = slots.legacy_implementation {
            let mut resolution = ProxyResolution {
                kind: ProxyKind::LegacyOpenZeppelinTransparent,
                implementation: Some(legacy),
                admin: None,
                evidence: Vec::new(),
            };
            resolution.record(
                "legacy_oz.implementation slot",
                "non-zero, decodes to an address; pre-1967 OpenZeppelin layout",
            );
            return Ok(resolution);
        }

        if let Some(resolution) = diamond::classify(client, address, block).await? {
            return Ok(resolution);
        }

        let mut resolution = ProxyResolution::unknown();
        resolution.record("eip1167 template", "bytecode did not match");
        resolution.record("eip1967 slots", "none of the known slots were in use");
        resolution.record(
            "diamond loupe",
            "supportsInterface(IDiamondLoupe) did not return true",
        );
        Ok(resolution)
    }

    async fn try_eip1167<T>(
        &self,
        client: &RpcClient<T>,
        address: &str,
        block: &str,
    ) -> ProxyResult<Option<ProxyResolution>>
    where
        T: RpcTransport,
    {
        let request = RpcRequest::new(1, "eth_getCode", json!([address, block]))
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
            return Err(ProxyError::Bytecode(
                "eth_getCode result was not a string".to_owned(),
            ));
        };
        let code = oo_bytecode::parse_hex(&hex).map_err(ProxyError::BytecodeDecode)?;

        let Some(implementation) = eip1167::detect(&code) else {
            return Ok(None);
        };
        let mut resolution = ProxyResolution {
            kind: ProxyKind::Eip1167Minimal,
            implementation: Some(implementation),
            admin: None,
            evidence: Vec::new(),
        };
        resolution.record(
            "eip1167 template",
            format!(
                "bytecode is exactly {} bytes and matches the fixed template",
                eip1167::TEMPLATE_LENGTH
            ),
        );
        Ok(Some(resolution))
    }

    async fn read_eip1967_slots<T>(
        &self,
        client: &RpcClient<T>,
        address: &str,
        block: &str,
    ) -> ProxyResult<eip1967::Eip1967Slots>
    where
        T: RpcTransport,
    {
        let layout = StorageLayout::known_proxy_slots();
        let values = read_layout(client, address, &layout, block)
            .await
            .map_err(ProxyError::Storage)?;
        Ok(eip1967::interpret(&values))
    }
}

#[cfg(test)]
mod tests {
    use oo_core::ProviderId;
    use oo_rpc::{FixtureTransport, RpcEndpoint, RpcResponse};

    use super::*;

    const ADDRESS: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    const BLOCK: &str = "0x1220000";

    fn endpoint() -> RpcEndpoint {
        RpcEndpoint::parse("https://rpc.example.invalid").expect("endpoint")
    }

    fn empty_slots_fixture() -> FixtureTransport {
        let mut fixture = FixtureTransport::new();
        for (_, slot) in StorageLayout::known_proxy_slots().entries() {
            fixture.insert_for(
                "eth_getStorageAt",
                json!([ADDRESS, slot.to_hex(), BLOCK]),
                RpcResponse::success(0, json!("0x0")),
            );
        }
        fixture
    }

    #[tokio::test]
    async fn a_minimal_proxy_resolves_without_reading_storage() {
        let implementation = [0x11; 20];
        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_getCode",
            json!([ADDRESS, BLOCK]),
            RpcResponse::success(
                0,
                json!(oo_bytecode::to_hex(&eip1167::build(implementation))),
            ),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        let resolution = ProxyResolver
            .resolve(&client, ADDRESS, BLOCK)
            .await
            .unwrap();
        assert_eq!(resolution.kind, ProxyKind::Eip1167Minimal);
        assert_eq!(resolution.implementation, Some(implementation));
    }

    #[tokio::test]
    async fn a_transparent_proxy_resolves_from_storage() {
        let mut fixture = empty_slots_fixture();
        fixture.insert_for(
            "eth_getCode",
            json!([ADDRESS, BLOCK]),
            RpcResponse::success(0, json!("0x60006000")),
        );

        let layout = StorageLayout::known_proxy_slots();
        let mut implementation_word = vec![0u8; 12];
        implementation_word.extend_from_slice(&[0x22; 20]);
        fixture.insert_for(
            "eth_getStorageAt",
            json!([
                ADDRESS,
                layout.get("eip1967.implementation").unwrap().to_hex(),
                BLOCK
            ]),
            RpcResponse::success(0, json!(oo_bytecode::to_hex(&implementation_word))),
        );
        let mut admin_word = vec![0u8; 12];
        admin_word.extend_from_slice(&[0x33; 20]);
        fixture.insert_for(
            "eth_getStorageAt",
            json!([
                ADDRESS,
                layout.get("eip1967.admin").unwrap().to_hex(),
                BLOCK
            ]),
            RpcResponse::success(0, json!(oo_bytecode::to_hex(&admin_word))),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        let resolution = ProxyResolver
            .resolve(&client, ADDRESS, BLOCK)
            .await
            .unwrap();
        assert_eq!(resolution.kind, ProxyKind::Eip1967Transparent);
        assert_eq!(resolution.implementation, Some([0x22; 20]));
        assert_eq!(resolution.admin, Some([0x33; 20]));
    }

    #[tokio::test]
    async fn a_contract_matching_nothing_resolves_to_unknown_with_evidence() {
        let mut fixture = empty_slots_fixture();
        fixture.insert_for(
            "eth_getCode",
            json!([ADDRESS, BLOCK]),
            RpcResponse::success(0, json!("0x60006000")),
        );

        let signature = oo_abi::FunctionSignature::of_parts(
            "supportsInterface",
            &[oo_abi::AbiParameter::new(
                "",
                oo_abi::AbiType::FixedBytes(4),
            )],
        )
        .unwrap();
        let mut call_data = signature.selector_hex();
        call_data.push_str("1f931c1c");
        call_data.push_str(&"0".repeat(56));
        fixture.insert_for(
            "eth_call",
            json!([{"to": ADDRESS, "data": call_data}, BLOCK]),
            RpcResponse::success(0, json!(format!("0x{}", "0".repeat(64)))),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        let resolution = ProxyResolver
            .resolve(&client, ADDRESS, BLOCK)
            .await
            .unwrap();
        assert_eq!(resolution.kind, ProxyKind::Unknown);
        assert!(!resolution.evidence.is_empty());
    }

    #[tokio::test]
    async fn an_invalid_address_is_rejected_before_any_call_is_made() {
        let client = RpcClient::new(ProviderId::new(), endpoint(), FixtureTransport::new());
        let error = ProxyResolver
            .resolve(&client, "not-an-address", BLOCK)
            .await;
        assert!(matches!(error, Err(ProxyError::InvalidAddress(_))));
    }
}
