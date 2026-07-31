// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/resolution.rs
// Purpose : Resolve what a set of wallets decided about one asset.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Resolve what a set of wallets decided about one asset.
//!
//! Recognition is not unanimous across wallets, and that disagreement is
//! itself a finding: an asset five wallets recognize and one ignores says
//! something different than one no wallet recognizes at all. This module
//! resolves a set of per-wallet observations into that comparison, without
//! discarding which specific wallets fell on which side.

use oo_wallet::WalletObservation;

/// What one set of wallets decided about one asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecognitionResolution {
    /// Config ids of wallets that recognized the asset.
    pub recognizing: Vec<String>,
    /// Config ids of wallets that did not.
    pub ignoring: Vec<String>,
}

impl RecognitionResolution {
    /// Resolves recognition from a set of wallet observations.
    ///
    /// Only live-discovery-attributable observations are counted — a
    /// warm-cache observation cannot be attributed to this run's discovery
    /// conditions, so including it would blur cache influence into what looks
    /// like a decision.
    #[must_use]
    pub fn resolve(observations: &[WalletObservation]) -> Self {
        let mut recognizing = Vec::new();
        let mut ignoring = Vec::new();
        for observation in observations {
            if !observation.is_live_discovery_evidence() {
                continue;
            }
            let target = if observation.recognized {
                &mut recognizing
            } else {
                &mut ignoring
            };
            target.push(observation.wallet.config_id.clone());
        }
        Self {
            recognizing,
            ignoring,
        }
    }

    /// Returns the total number of wallets counted.
    #[must_use]
    pub fn total(&self) -> usize {
        self.recognizing.len() + self.ignoring.len()
    }

    /// Returns the recognition rate, or `0.0` when no wallet was counted.
    #[must_use]
    pub fn rate(&self) -> f64 {
        if self.total() == 0 {
            return 0.0;
        }
        self.recognizing.len() as f64 / self.total() as f64
    }

    /// Returns whether every counted wallet agreed, in either direction.
    #[must_use]
    pub fn is_unanimous(&self) -> bool {
        self.total() > 0 && (self.recognizing.is_empty() || self.ignoring.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::ManualClock;
    use oo_wallet::{CacheState, WalletIdentity};

    use super::*;

    fn observation(wallet: &str, recognized: bool, cache: CacheState) -> WalletObservation {
        WalletObservation::new(
            WalletIdentity::new(wallet, wallet),
            None,
            1,
            "0xabc",
            recognized,
            cache,
            &ManualClock::new(UNIX_EPOCH),
        )
    }

    #[test]
    fn disagreement_is_preserved_by_wallet_name() {
        let observations = vec![
            observation("metamask", true, CacheState::Cold),
            observation("trust-wallet", false, CacheState::Cold),
        ];
        let resolution = RecognitionResolution::resolve(&observations);
        assert_eq!(resolution.recognizing, vec!["metamask".to_owned()]);
        assert_eq!(resolution.ignoring, vec!["trust-wallet".to_owned()]);
        assert!(!resolution.is_unanimous());
        assert_eq!(resolution.rate(), 0.5);
    }

    #[test]
    fn warm_cache_observations_are_excluded_from_the_decision() {
        let observations = vec![
            observation("metamask", true, CacheState::Cold),
            observation("trust-wallet", true, CacheState::Warm),
        ];
        let resolution = RecognitionResolution::resolve(&observations);
        assert_eq!(
            resolution.total(),
            1,
            "the warm-cache observation is not attributable"
        );
    }

    #[test]
    fn unanimous_recognition_is_detected() {
        let observations = vec![
            observation("metamask", true, CacheState::Cold),
            observation("trust-wallet", true, CacheState::Cold),
        ];
        assert!(RecognitionResolution::resolve(&observations).is_unanimous());
    }

    #[test]
    fn no_observations_has_a_zero_rate_without_dividing_by_zero() {
        assert_eq!(RecognitionResolution::resolve(&[]).rate(), 0.0);
    }
}
