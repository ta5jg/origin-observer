// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/observation.rs
// Purpose : One recorded wallet discovery observation.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! One recorded wallet discovery observation.

use oo_core::Clock;

use crate::cache::CacheState;
use crate::model::WalletIdentity;
use crate::version::WalletVersion;

/// One recorded observation of whether a wallet recognized an asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletObservation {
    /// Wallet observed.
    pub wallet: WalletIdentity,
    /// Wallet client version, when known.
    pub version: Option<WalletVersion>,
    /// Chain id the asset belongs to.
    pub chain_id: u64,
    /// Asset locator: a contract address, or a native asset symbol.
    pub asset_locator: String,
    /// Whether the wallet presented the asset to the user.
    pub recognized: bool,
    /// Cache state at the time of observation.
    pub cache_state: CacheState,
    /// UNIX timestamp, in seconds, the observation was recorded.
    pub observed_unix_seconds: u64,
}

impl WalletObservation {
    /// Records an observation using the given clock.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        wallet: WalletIdentity,
        version: Option<WalletVersion>,
        chain_id: u64,
        asset_locator: impl Into<String>,
        recognized: bool,
        cache_state: CacheState,
        clock: &dyn Clock,
    ) -> Self {
        Self {
            wallet,
            version,
            chain_id,
            asset_locator: asset_locator.into(),
            recognized,
            cache_state,
            observed_unix_seconds: clock.unix_seconds(),
        }
    }

    /// Returns whether this observation is citable as a live-discovery
    /// finding rather than a possibly cache-explained result.
    #[must_use]
    pub const fn is_live_discovery_evidence(&self) -> bool {
        self.cache_state.is_attributable_to_live_discovery()
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::ManualClock;

    use super::*;

    #[test]
    fn an_observation_records_the_clocks_time() {
        let clock = ManualClock::new(UNIX_EPOCH + std::time::Duration::from_secs(42));
        let observation = WalletObservation::new(
            WalletIdentity::new("metamask", "MetaMask"),
            None,
            1,
            "0xdac17f958d2ee523a2206206994597c13d831ec7",
            true,
            CacheState::Cold,
            &clock,
        );
        assert_eq!(observation.observed_unix_seconds, 42);
        assert!(observation.is_live_discovery_evidence());
    }

    #[test]
    fn a_warm_cache_observation_is_not_live_discovery_evidence() {
        let clock = ManualClock::new(UNIX_EPOCH);
        let observation = WalletObservation::new(
            WalletIdentity::new("metamask", "MetaMask"),
            None,
            1,
            "0xabc",
            true,
            CacheState::Warm,
            &clock,
        );
        assert!(!observation.is_live_discovery_evidence());
    }
}
