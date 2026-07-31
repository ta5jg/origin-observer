// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-storage/src/reader.rs
// Purpose : Read a storage slot through the RPC transport.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Read a storage slot through the RPC transport.
//!
//! This module issues exactly one JSON-RPC shape — `eth_getStorageAt` — and
//! leaves pinning, retry and rate-limit policy to the `oo-rpc` client it is
//! given. It does not open its own connections or bypass the client's
//! configured pin policy: an unpinned storage read is exactly as
//! unreproducible as an unpinned `eth_call`, and `oo-rpc` already refuses one
//! for this method.

use oo_rpc::{RpcClient, RpcRequest, RpcTransport};
use serde_json::{json, Value};

use crate::decoder::{parse_storage_value, StorageValue};
use crate::error::{StorageError, StorageResult};
use crate::slot::StorageSlot;

/// Reads one storage slot for a contract address at a pinned block.
///
/// `block` follows `eth_getStorageAt` conventions: a `0x`-prefixed block
/// number, or a tag such as `"latest"` when the client's pin policy allows it.
pub async fn read_storage<T>(
    client: &RpcClient<T>,
    address: &str,
    slot: StorageSlot,
    block: &str,
) -> StorageResult<StorageValue>
where
    T: RpcTransport,
{
    let request = RpcRequest::new(
        1,
        "eth_getStorageAt",
        json!([address, slot.to_hex(), block]),
    )
    .map_err(|error| StorageError::Rpc(error.to_string()))?;

    let trace = client
        .observe(request)
        .await
        .map_err(|error| StorageError::Rpc(error.to_string()))?;

    let value = trace
        .response()
        .clone()
        .into_result()
        .map_err(|error| StorageError::Rpc(error.to_string()))?;

    let Value::String(hex) = value else {
        return Err(StorageError::InvalidHex(value.to_string()));
    };
    parse_storage_value(&hex)
}

/// Reads every slot in a layout for one address at a pinned block, in
/// declaration order.
///
/// A caller that only needs one or two slots should call
/// [`read_storage`] directly; this exists for the common case of probing
/// every known proxy slot in a single deterministic sequence.
pub async fn read_layout<T>(
    client: &RpcClient<T>,
    address: &str,
    layout: &crate::layout::StorageLayout,
    block: &str,
) -> StorageResult<Vec<(&'static str, StorageValue)>>
where
    T: RpcTransport,
{
    let mut results = Vec::with_capacity(layout.entries().len());
    for (name, slot) in layout.entries() {
        let value = read_storage(client, address, *slot, block).await?;
        results.push((*name, value));
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use oo_core::ProviderId;
    use oo_rpc::{FixtureTransport, RpcEndpoint, RpcResponse};

    use super::*;
    use crate::layout::StorageLayout;

    fn endpoint() -> RpcEndpoint {
        RpcEndpoint::parse("https://rpc.example.invalid").expect("endpoint")
    }

    #[tokio::test]
    async fn a_pinned_storage_read_decodes_the_response() {
        let slot = StorageSlot::from_index(0);
        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_getStorageAt",
            json!([
                "0xdac17f958d2ee523a2206206994597c13d831ec7",
                slot.to_hex(),
                "0x1220000"
            ]),
            RpcResponse::success(0, json!("0x1")),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        let value = read_storage(
            &client,
            "0xdac17f958d2ee523a2206206994597c13d831ec7",
            slot,
            "0x1220000",
        )
        .await
        .unwrap();
        assert!(value.as_bool().unwrap());
    }

    #[tokio::test]
    async fn an_unpinned_read_is_refused_by_the_underlying_client() {
        let slot = StorageSlot::from_index(0);
        let client = RpcClient::new(ProviderId::new(), endpoint(), FixtureTransport::new());
        let error = read_storage(&client, "0xabc", slot, "latest").await;
        assert!(error.is_err());
    }

    #[tokio::test]
    async fn reading_a_layout_visits_every_named_slot() {
        let layout = StorageLayout::known_proxy_slots();
        let mut fixture = FixtureTransport::new();
        for (_, slot) in layout.entries() {
            fixture.insert_for(
                "eth_getStorageAt",
                json!(["0xabc", slot.to_hex(), "0x1"]),
                RpcResponse::success(0, json!("0x0")),
            );
        }

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        let results = read_layout(&client, "0xabc", &layout, "0x1").await.unwrap();
        assert_eq!(results.len(), layout.entries().len());
        assert!(results.iter().all(|(_, value)| value.is_zero()));
    }
}
