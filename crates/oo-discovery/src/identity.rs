// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/identity.rs
// Purpose : Identify the asset one discovery investigation is about.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Identify the asset one discovery investigation is about.

/// Identity of the asset under investigation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiscoveredAssetIdentity {
    /// Chain id the asset belongs to.
    pub chain_id: u64,
    /// Contract address, or a native asset symbol for chain-native assets.
    pub locator: String,
}

impl DiscoveredAssetIdentity {
    /// Creates an asset identity.
    #[must_use]
    pub fn new(chain_id: u64, locator: impl Into<String>) -> Self {
        Self {
            chain_id,
            locator: locator.into(),
        }
    }

    /// Returns a stable string key suitable for grouping observations about
    /// the same asset.
    #[must_use]
    pub fn key(&self) -> String {
        format!("{}:{}", self.chain_id, self.locator.to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_key_is_case_insensitive_in_the_locator() {
        let lower = DiscoveredAssetIdentity::new(1, "0xdac17f958d2ee523a2206206994597c13d831ec7");
        let upper = DiscoveredAssetIdentity::new(1, "0xDAC17F958D2ee523a2206206994597C13D831Ec7");
        assert_eq!(lower.key(), upper.key());
    }

    #[test]
    fn different_chains_never_share_a_key() {
        let ethereum = DiscoveredAssetIdentity::new(1, "0xabc");
        let bnb = DiscoveredAssetIdentity::new(56, "0xabc");
        assert_ne!(ethereum.key(), bnb.key());
    }
}
