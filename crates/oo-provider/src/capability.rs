// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/capability.rs
// Purpose : Describe what an external provider can answer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Describe what an external provider can answer.
//!
//! A provider's category decides which discovery question it can help answer,
//! and its chain scope decides which networks it may be consulted for. This
//! mirrors `oo-config::ProviderKind`, deliberately not shared with it: the
//! configuration crate declares what exists in `config/providers.toml`, this
//! crate operationalizes it at runtime, and coupling the two would make a
//! config-file schema change ripple into every provider call site.

/// Category of external component a provider represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProviderCategory {
    /// Curated asset registry or token list.
    Registry,
    /// Block explorer.
    Explorer,
    /// Metadata service (name, symbol, decimals).
    Metadata,
    /// Logo or image service.
    Image,
    /// Price service.
    Price,
    /// Decentralized exchange or aggregator.
    Dex,
    /// Chain indexer.
    Indexer,
}

/// What a provider can answer and where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCapability {
    /// Category of question this provider answers.
    pub category: ProviderCategory,
    /// Chain ids this provider covers. Empty means every chain.
    pub chains: Vec<u64>,
    /// Whether a credential is required to consult this provider.
    pub requires_api_key: bool,
}

impl ProviderCapability {
    /// Creates a capability covering every chain.
    #[must_use]
    pub const fn any_chain(category: ProviderCategory, requires_api_key: bool) -> Self {
        Self {
            category,
            chains: Vec::new(),
            requires_api_key,
        }
    }

    /// Creates a capability scoped to specific chains.
    #[must_use]
    pub const fn scoped(
        category: ProviderCategory,
        chains: Vec<u64>,
        requires_api_key: bool,
    ) -> Self {
        Self {
            category,
            chains,
            requires_api_key,
        }
    }

    /// Returns whether this provider may be consulted for a chain.
    #[must_use]
    pub fn covers_chain(&self, chain_id: u64) -> bool {
        self.chains.is_empty() || self.chains.contains(&chain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unscoped_capability_covers_every_chain() {
        let capability = ProviderCapability::any_chain(ProviderCategory::Price, false);
        assert!(capability.covers_chain(1));
        assert!(capability.covers_chain(56));
    }

    #[test]
    fn a_scoped_capability_only_covers_its_chains() {
        let capability = ProviderCapability::scoped(ProviderCategory::Explorer, vec![1], true);
        assert!(capability.covers_chain(1));
        assert!(!capability.covers_chain(56));
    }
}
