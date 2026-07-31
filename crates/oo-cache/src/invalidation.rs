// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/invalidation.rs
// Purpose : A before/after experiment testing whether a cache was invalidated.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A before/after experiment testing whether a cache was invalidated.

use crate::observation::TimedCacheObservation;
use crate::state::CacheTransition;

/// A before/after pair of observations of the same cache key, testing
/// whether an invalidation action actually invalidated it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidationExperiment {
    key: String,
    before: TimedCacheObservation,
    after: TimedCacheObservation,
}

impl InvalidationExperiment {
    /// Records an invalidation experiment.
    #[must_use]
    pub fn new(
        key: impl Into<String>,
        before: TimedCacheObservation,
        after: TimedCacheObservation,
    ) -> Self {
        Self {
            key: key.into(),
            before,
            after,
        }
    }

    /// Returns the cache key this experiment observed.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the observation made before the invalidation action.
    #[must_use]
    pub const fn before(&self) -> &TimedCacheObservation {
        &self.before
    }

    /// Returns the observation made after the invalidation action.
    #[must_use]
    pub const fn after(&self) -> &TimedCacheObservation {
        &self.after
    }

    /// Returns the state transition between the two observations.
    #[must_use]
    pub fn transition(&self) -> CacheTransition {
        CacheTransition::new(
            self.before.observation().state(),
            self.after.observation().state(),
        )
    }

    /// Returns whether the invalidation action succeeded.
    #[must_use]
    pub fn succeeded(&self) -> bool {
        self.transition().is_invalidation()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use oo_model::cache::{CacheObservation, CacheState};

    use super::*;

    fn timed(state: CacheState, seconds: i64) -> TimedCacheObservation {
        TimedCacheObservation::new(
            CacheObservation::new("balanceOf", state),
            Utc.timestamp_opt(seconds, 0).unwrap(),
        )
    }

    #[test]
    fn a_warm_to_empty_experiment_succeeds() {
        let experiment = InvalidationExperiment::new(
            "balanceOf",
            timed(CacheState::Warm, 1),
            timed(CacheState::Empty, 2),
        );
        assert!(experiment.succeeded());
    }

    #[test]
    fn a_warm_to_warm_experiment_fails() {
        let experiment = InvalidationExperiment::new(
            "balanceOf",
            timed(CacheState::Warm, 1),
            timed(CacheState::Warm, 2),
        );
        assert!(!experiment.succeeded());
    }
}
