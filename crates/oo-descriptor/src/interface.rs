// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/interface.rs
// Purpose : Contract interface descriptor.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Contract interface descriptor.

/// Known contract interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContractInterface {
    /// ERC-20 compatible interface.
    Erc20,
    /// ERC-721 compatible interface.
    Erc721,
    /// ERC-1155 compatible interface.
    Erc1155,
    /// Unknown interface.
    Unknown,
}

/// Interface descriptor extracted from selectors or ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDescriptor {
    interface: ContractInterface,
    selector_count: usize,
}

impl InterfaceDescriptor {
    /// Creates an interface descriptor.
    #[must_use]
    pub const fn new(interface: ContractInterface, selector_count: usize) -> Self {
        Self {
            interface,
            selector_count,
        }
    }

    /// Returns the interface classification.
    #[must_use]
    pub const fn interface(&self) -> ContractInterface {
        self.interface
    }

    /// Returns the number of selectors used for classification.
    #[must_use]
    pub const fn selector_count(&self) -> usize {
        self.selector_count
    }
}
