// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/contract.rs
// Purpose : Smart contract domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Smart contract domain model.

use oo_core::{AddressId, ContractId};

/// High-level smart contract classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ContractKind {
    /// Fungible token contract.
    FungibleToken,

    /// Non-fungible token contract.
    NonFungibleToken,

    /// Multi-token contract.
    MultiToken,

    /// Decentralized exchange.
    DecentralizedExchange,

    /// Liquidity pool.
    LiquidityPool,

    /// Bridge contract.
    Bridge,

    /// Oracle contract.
    Oracle,

    /// Governance / DAO contract.
    Governance,

    /// Wallet / Safe contract.
    Wallet,

    /// Proxy contract.
    Proxy,

    /// Library contract.
    Library,

    /// Unknown contract type.
    #[default]
    Unknown,
}

/// Lifecycle status of a contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ContractStatus {
    #[default]
    Active,
    Paused,
    Deprecated,
    Destroyed,
}

/// Smart contract domain object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    id: ContractId,
    address_id: AddressId,

    name: String,
    symbol: Option<String>,

    kind: ContractKind,
    status: ContractStatus,

    verified: bool,
    proxy: bool,
    implementation: Option<AddressId>,
}

impl Contract {
    /// Creates a new contract.
    #[must_use]
    pub fn new(address_id: AddressId, name: impl Into<String>) -> Self {
        Self {
            id: ContractId::new(),
            address_id,
            name: name.into(),
            symbol: None,
            kind: ContractKind::Unknown,
            status: ContractStatus::Active,
            verified: false,
            proxy: false,
            implementation: None,
        }
    }

    #[must_use]
    pub const fn id(&self) -> ContractId {
        self.id
    }

    #[must_use]
    pub const fn address_id(&self) -> AddressId {
        self.address_id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    #[must_use]
    pub fn symbol(&self) -> Option<&str> {
        self.symbol.as_deref()
    }

    pub fn set_symbol(&mut self, symbol: impl Into<String>) {
        self.symbol = Some(symbol.into());
    }

    pub fn clear_symbol(&mut self) {
        self.symbol = None;
    }

    #[must_use]
    pub const fn kind(&self) -> ContractKind {
        self.kind
    }

    pub const fn set_kind(&mut self, kind: ContractKind) {
        self.kind = kind;
    }

    #[must_use]
    pub const fn status(&self) -> ContractStatus {
        self.status
    }

    pub const fn set_status(&mut self, status: ContractStatus) {
        self.status = status;
    }

    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verified
    }

    pub const fn mark_verified(&mut self) {
        self.verified = true;
    }

    pub const fn clear_verified(&mut self) {
        self.verified = false;
    }

    #[must_use]
    pub const fn is_proxy(&self) -> bool {
        self.proxy
    }

    pub fn set_proxy(&mut self, implementation: AddressId) {
        self.proxy = true;
        self.implementation = Some(implementation);
    }

    pub fn clear_proxy(&mut self) {
        self.proxy = false;
        self.implementation = None;
    }

    #[must_use]
    pub const fn implementation(&self) -> Option<AddressId> {
        self.implementation
    }

    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.status, ContractStatus::Active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_ids() {
        let address = AddressId::new();

        let a = Contract::new(address, "USDT");
        let b = Contract::new(address, "USDT");

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn symbol_management() {
        let mut contract = Contract::new(AddressId::new(), "Token");

        assert_eq!(contract.symbol(), None);

        contract.set_symbol("TKN");

        assert_eq!(contract.symbol(), Some("TKN"));

        contract.clear_symbol();

        assert_eq!(contract.symbol(), None);
    }

    #[test]
    fn verification_flag() {
        let mut contract = Contract::new(AddressId::new(), "Token");

        assert!(!contract.is_verified());

        contract.mark_verified();

        assert!(contract.is_verified());

        contract.clear_verified();

        assert!(!contract.is_verified());
    }

    #[test]
    fn proxy_configuration() {
        let mut contract = Contract::new(AddressId::new(), "Proxy");

        let implementation = AddressId::new();

        contract.set_proxy(implementation);

        assert!(contract.is_proxy());
        assert_eq!(contract.implementation(), Some(implementation));

        contract.clear_proxy();

        assert!(!contract.is_proxy());
        assert_eq!(contract.implementation(), None);
    }

    #[test]
    fn kind_and_status() {
        let mut contract = Contract::new(AddressId::new(), "Router");

        contract.set_kind(ContractKind::DecentralizedExchange);

        contract.set_status(ContractStatus::Paused);

        assert_eq!(contract.kind(), ContractKind::DecentralizedExchange);

        assert_eq!(contract.status(), ContractStatus::Paused);

        assert!(!contract.is_active());
    }

    #[test]
    fn name_can_change() {
        let mut contract = Contract::new(AddressId::new(), "Old");

        contract.set_name("New");

        assert_eq!(contract.name(), "New");
    }
}
