// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/comparison.rs
// Purpose : Compare two cache observations along a named dimension.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Compare two cache observations along a named dimension.
//!
//! Whether the comparison is between two wallets, two providers, or the same
//! surface at two points in time, the arithmetic is identical: do the states
//! match, and how far apart in time were they observed. One function
//! parametrized by a caller-supplied dimension label covers all of those
//! cases.

use chrono::Duration;

use crate::observation::TimedCacheObservation;

/// The result of comparing two cache observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheComparison {
    /// What is being compared: `"wallet"`, `"provider"`, `"temporal"`, or
    /// another caller-supplied label.
    pub dimension: &'static str,
    /// Whether the two observations reported the same cache state.
    pub states_match: bool,
    /// The signed time gap between the subject and the reference
    /// observation: positive when the subject was observed later.
    pub time_gap: Duration,
}

/// Compares two timed cache observations along a named dimension.
#[must_use]
pub fn compare(
    dimension: &'static str,
    subject: &TimedCacheObservation,
    reference: &TimedCacheObservation,
) -> CacheComparison {
    CacheComparison {
        dimension,
        states_match: subject.observation().state() == reference.observation().state(),
        time_gap: subject
            .timestamp()
            .signed_duration_since(reference.timestamp()),
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono::Utc;
    use oo_model::cache::{CacheObservation, CacheState};

    use super::*;

    fn timed(state: CacheState, seconds: i64) -> TimedCacheObservation {
        TimedCacheObservation::new(
            CacheObservation::new("balanceOf", state),
            Utc.timestamp_opt(seconds, 0).unwrap(),
        )
    }

    #[test]
    fn matching_states_are_reported_as_matching() {
        let comparison = compare(
            "wallet",
            &timed(CacheState::Warm, 10),
            &timed(CacheState::Warm, 5),
        );
        assert_eq!(comparison.dimension, "wallet");
        assert!(comparison.states_match);
        assert_eq!(comparison.time_gap, Duration::seconds(5));
    }

    #[test]
    fn differing_states_are_reported_as_not_matching() {
        let comparison = compare(
            "provider",
            &timed(CacheState::Warm, 1),
            &timed(CacheState::Stale, 1),
        );
        assert!(!comparison.states_match);
        assert_eq!(comparison.time_gap, Duration::zero());
    }
}
