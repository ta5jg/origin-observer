// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/validation.rs
// Purpose : Validate wallet observations before they are recorded as evidence.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate wallet observations before they are recorded as evidence.

use crate::adapter::find_adapter;
use crate::observation::WalletObservation;

/// Validates a wallet observation.
///
/// The wallet must be a known built-in adapter and the asset locator must be
/// non-empty. An observation against an unregistered wallet identifier is
/// rejected rather than accepted with an unrecognized name, since a typo in a
/// wallet identifier would otherwise silently attribute a finding to nothing.
#[must_use]
pub fn validate_observation(observation: &WalletObservation) -> bool {
    find_adapter(&observation.wallet.config_id).is_some()
        && !observation.asset_locator.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::ManualClock;

    use super::*;
    use crate::cache::CacheState;
    use crate::model::WalletIdentity;

    #[test]
    fn a_known_wallet_with_a_locator_is_valid() {
        let observation = WalletObservation::new(
            WalletIdentity::new("metamask", "MetaMask"),
            None,
            1,
            "0xabc",
            true,
            CacheState::Cold,
            &ManualClock::new(UNIX_EPOCH),
        );
        assert!(validate_observation(&observation));
    }

    #[test]
    fn an_unregistered_wallet_id_is_invalid() {
        let observation = WalletObservation::new(
            WalletIdentity::new("not-a-real-wallet", "?"),
            None,
            1,
            "0xabc",
            true,
            CacheState::Cold,
            &ManualClock::new(UNIX_EPOCH),
        );
        assert!(!validate_observation(&observation));
    }

    #[test]
    fn an_empty_locator_is_invalid() {
        let observation = WalletObservation::new(
            WalletIdentity::new("metamask", "MetaMask"),
            None,
            1,
            "  ",
            true,
            CacheState::Cold,
            &ManualClock::new(UNIX_EPOCH),
        );
        assert!(!validate_observation(&observation));
    }
}
