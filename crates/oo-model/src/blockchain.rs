// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/blockchain.rs
// Purpose : Blockchain domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Blockchain domain model.

use oo_core::BlockchainId;

/// Supported blockchain families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockchainKind {
    Bitcoin,
    Ethereum,
    Tron,
    BinanceSmartChain,
    Polygon,
    Avalanche,
    Arbitrum,
    Optimism,
    Base,
    Solana,
    Sui,
    Aptos,
    Cosmos,
    Near,
    Cardano,
    Polkadot,
    Kusama,
    Other,
}

/// Immutable blockchain description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blockchain {
    id: BlockchainId,
    kind: BlockchainKind,
    name: String,
    symbol: String,
    native_asset: String,
    slip44: Option<u32>,
    evm_compatible: bool,
    enabled: bool,
}

impl Blockchain {
    /// Creates a new blockchain.
    #[must_use]
    pub fn new(
        kind: BlockchainKind,
        name: impl Into<String>,
        symbol: impl Into<String>,
        native_asset: impl Into<String>,
    ) -> Self {
        let evm_compatible = matches!(
            kind,
            BlockchainKind::Ethereum
                | BlockchainKind::BinanceSmartChain
                | BlockchainKind::Polygon
                | BlockchainKind::Avalanche
                | BlockchainKind::Arbitrum
                | BlockchainKind::Optimism
                | BlockchainKind::Base
        );

        Self {
            id: BlockchainId::new(),
            kind,
            name: name.into(),
            symbol: symbol.into(),
            native_asset: native_asset.into(),
            slip44: None,
            evm_compatible,
            enabled: true,
        }
    }

    #[must_use]
    pub fn id(&self) -> BlockchainId {
        self.id
    }

    #[must_use]
    pub fn kind(&self) -> BlockchainKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    #[must_use]
    pub fn native_asset(&self) -> &str {
        &self.native_asset
    }

    #[must_use]
    pub fn slip44(&self) -> Option<u32> {
        self.slip44
    }

    pub fn set_slip44(&mut self, value: u32) {
        self.slip44 = Some(value);
    }

    #[must_use]
    pub fn is_evm(&self) -> bool {
        self.evm_compatible
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ethereum_is_evm() {
        let chain = Blockchain::new(BlockchainKind::Ethereum, "Ethereum", "ETH", "Ether");

        assert!(chain.is_evm());
        assert!(chain.is_enabled());
    }

    #[test]
    fn bitcoin_is_not_evm() {
        let chain = Blockchain::new(BlockchainKind::Bitcoin, "Bitcoin", "BTC", "Bitcoin");

        assert!(!chain.is_evm());
    }

    #[test]
    fn enable_disable() {
        let mut chain = Blockchain::new(BlockchainKind::Tron, "TRON", "TRX", "TRX");

        chain.disable();
        assert!(!chain.is_enabled());

        chain.enable();
        assert!(chain.is_enabled());
    }

    #[test]
    fn slip44_assignment() {
        let mut chain = Blockchain::new(BlockchainKind::Ethereum, "Ethereum", "ETH", "Ether");

        chain.set_slip44(60);

        assert_eq!(chain.slip44(), Some(60));
    }

    #[test]
    fn identifiers_are_unique() {
        let a = Blockchain::new(BlockchainKind::Ethereum, "Ethereum", "ETH", "Ether");

        let b = Blockchain::new(BlockchainKind::Ethereum, "Ethereum", "ETH", "Ether");

        assert_ne!(a.id(), b.id());
    }
}
