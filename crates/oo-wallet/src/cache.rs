// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/cache.rs
// Purpose : Wallet cache state relevant to discovery.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Wallet cache state relevant to discovery.
//!
//! A wallet that already recognized an asset in a prior session may keep
//! recognizing it even if the condition that first triggered recognition no
//! longer holds. Cache state must therefore be recorded as part of every
//! observation, or a warm-cache result gets mistaken for a cold-cache
//! discovery decision.

/// Whether a wallet's cache held prior state for the asset under
/// observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheState {
    /// No prior session touched this asset; recognition (or its absence)
    /// reflects the wallet's live discovery logic.
    Cold,
    /// A prior session already recognized or interacted with this asset, so
    /// recognition may be explained by the cache rather than fresh discovery.
    Warm,
}

impl CacheState {
    /// Returns whether an observation under this state can be attributed to
    /// live discovery logic rather than a possibly stale cache entry.
    #[must_use]
    pub const fn is_attributable_to_live_discovery(self) -> bool {
        matches!(self, Self::Cold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_cold_cache_is_attributable_to_live_discovery() {
        assert!(CacheState::Cold.is_attributable_to_live_discovery());
        assert!(!CacheState::Warm.is_attributable_to_live_discovery());
    }
}
