// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/contract.rs
// Purpose : Contract descriptor extraction.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Contract descriptor extraction.

use oo_core::{AddressId, ContractId};
use oo_model::contract::{Contract, ContractKind};

/// Stable descriptor for a smart contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDescriptor {
    id: ContractId,
    address_id: AddressId,
    kind: ContractKind,
    verified: bool,
    proxy: bool,
    implementation: Option<AddressId>,
}

impl ContractDescriptor {
    /// Extracts a contract descriptor.
    #[must_use]
    pub fn from_contract(contract: &Contract) -> Self {
        Self {
            id: contract.id(),
            address_id: contract.address_id(),
            kind: contract.kind(),
            verified: contract.is_verified(),
            proxy: contract.is_proxy(),
            implementation: contract.implementation(),
        }
    }

    /// Returns the contract identifier.
    #[must_use]
    pub const fn id(&self) -> ContractId {
        self.id
    }

    /// Returns the contract address identifier.
    #[must_use]
    pub const fn address_id(&self) -> AddressId {
        self.address_id
    }

    /// Returns whether the contract is verified.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verified
    }

    /// Returns whether the contract is a proxy.
    #[must_use]
    pub const fn is_proxy(&self) -> bool {
        self.proxy
    }

    /// Returns the implementation address if known.
    #[must_use]
    pub const fn implementation(&self) -> Option<AddressId> {
        self.implementation
    }

    /// Returns the contract kind.
    #[must_use]
    pub const fn kind(&self) -> ContractKind {
        self.kind
    }
}
