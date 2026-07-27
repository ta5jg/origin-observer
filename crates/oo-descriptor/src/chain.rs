// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/chain.rs
// Purpose : Chain descriptor extraction.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Chain descriptor extraction.

use oo_core::BlockchainId;
use oo_model::blockchain::{Blockchain, BlockchainKind};

/// Stable descriptor for a blockchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainDescriptor {
    id: BlockchainId,
    kind: BlockchainKind,
    symbol: String,
    native_asset: String,
    evm_compatible: bool,
}

impl ChainDescriptor {
    /// Extracts a chain descriptor.
    #[must_use]
    pub fn from_blockchain(blockchain: &Blockchain) -> Self {
        Self {
            id: blockchain.id(),
            kind: blockchain.kind(),
            symbol: blockchain.symbol().to_owned(),
            native_asset: blockchain.native_asset().to_owned(),
            evm_compatible: blockchain.is_evm(),
        }
    }

    /// Returns the blockchain identifier.
    #[must_use]
    pub const fn id(&self) -> BlockchainId {
        self.id
    }

    /// Returns whether the chain is EVM-compatible.
    #[must_use]
    pub const fn is_evm_compatible(&self) -> bool {
        self.evm_compatible
    }

    /// Returns the native asset symbol/name.
    #[must_use]
    pub fn native_asset(&self) -> &str {
        &self.native_asset
    }

    /// Returns the chain kind.
    #[must_use]
    pub const fn kind(&self) -> BlockchainKind {
        self.kind
    }

    /// Returns the chain symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}
