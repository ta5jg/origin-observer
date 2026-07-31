// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/source.rs
// Purpose : Attribute observed bytecode to where and when it was read.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Attribute observed bytecode to where and when it was read.
//!
//! Bytecode is mutable at an address: an upgrade replaces it, and
//! `SELFDESTRUCT` (pre-Cancun) can remove it entirely. An analysis is only
//! reproducible when it names the exact address and block it read, not just
//! "the current code at this address."

/// Where and when bytecode was observed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytecodeSource {
    /// Chain id the address belongs to.
    pub chain_id: u64,
    /// Contract address the bytecode was read from, lowercase `0x`-prefixed.
    pub address: String,
    /// Block reference the read was pinned to.
    pub block: String,
    /// Provider identifier that answered the `eth_getCode` request.
    pub provider_id: String,
}

impl BytecodeSource {
    /// Creates a bytecode source record.
    #[must_use]
    pub fn new(
        chain_id: u64,
        address: impl Into<String>,
        block: impl Into<String>,
        provider_id: impl Into<String>,
    ) -> Self {
        Self {
            chain_id,
            address: address.into().to_ascii_lowercase(),
            block: block.into(),
            provider_id: provider_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_address_is_normalized_to_lowercase() {
        let source = BytecodeSource::new(
            1,
            "0xDAC17F958D2ee523a2206206994597C13D831ec7",
            "0x1",
            "etherscan",
        );
        assert_eq!(source.address, "0xdac17f958d2ee523a2206206994597c13d831ec7");
    }
}
