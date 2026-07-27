// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/network.rs
// Purpose : Network descriptor extraction.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Network descriptor extraction.

use oo_core::{BlockchainId, NetworkId};
use oo_model::network::{Network, NetworkKind};

/// Stable descriptor for a blockchain network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkDescriptor {
    id: NetworkId,
    blockchain_id: BlockchainId,
    kind: NetworkKind,
    chain_id: Option<u64>,
    rpc_endpoint_count: usize,
}

impl NetworkDescriptor {
    /// Extracts a network descriptor.
    #[must_use]
    pub fn from_network(network: &Network) -> Self {
        Self {
            id: network.id(),
            blockchain_id: network.blockchain_id(),
            kind: network.kind(),
            chain_id: network.chain_id(),
            rpc_endpoint_count: network.rpc_endpoints().len(),
        }
    }

    /// Returns the network identifier.
    #[must_use]
    pub const fn id(&self) -> NetworkId {
        self.id
    }

    /// Returns the parent blockchain identifier.
    #[must_use]
    pub const fn blockchain_id(&self) -> BlockchainId {
        self.blockchain_id
    }

    /// Returns the configured chain id.
    #[must_use]
    pub const fn chain_id(&self) -> Option<u64> {
        self.chain_id
    }

    /// Returns whether at least one RPC endpoint is configured.
    #[must_use]
    pub const fn has_rpc_endpoint(&self) -> bool {
        self.rpc_endpoint_count > 0
    }

    /// Returns the network kind.
    #[must_use]
    pub const fn kind(&self) -> NetworkKind {
        self.kind
    }
}
