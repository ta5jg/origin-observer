// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/experiment.rs
// Purpose : Aggregate multiple invalidation experiments into a success rate.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Aggregate multiple invalidation experiments into a success rate.
//!
//! One experiment says nothing about whether invalidation is reliable; a set
//! of them, repeated under the same or varying conditions, does.

use crate::invalidation::InvalidationExperiment;

/// A collection of invalidation experiments.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InvalidationExperimentSet {
    experiments: Vec<InvalidationExperiment>,
}

impl InvalidationExperimentSet {
    /// Creates an empty experiment set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            experiments: Vec::new(),
        }
    }

    /// Records one experiment.
    pub fn push(&mut self, experiment: InvalidationExperiment) {
        self.experiments.push(experiment);
    }

    /// Returns the recorded experiments.
    #[must_use]
    pub fn experiments(&self) -> &[InvalidationExperiment] {
        &self.experiments
    }

    /// Returns the fraction of experiments that succeeded.
    ///
    /// Returns `0.0` for an empty set rather than dividing by zero: no
    /// experiments run is not evidence that invalidation succeeds.
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        if self.experiments.is_empty() {
            return 0.0;
        }
        let successes = self
            .experiments
            .iter()
            .filter(|experiment| experiment.succeeded())
            .count();
        successes as f64 / self.experiments.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use oo_model::cache::{CacheObservation, CacheState};

    use super::*;
    use crate::observation::TimedCacheObservation;

    fn timed(state: CacheState, seconds: i64) -> TimedCacheObservation {
        TimedCacheObservation::new(
            CacheObservation::new("balanceOf", state),
            Utc.timestamp_opt(seconds, 0).unwrap(),
        )
    }

    #[test]
    fn an_empty_set_has_zero_success_rate_not_a_division_error() {
        assert_eq!(InvalidationExperimentSet::new().success_rate(), 0.0);
    }

    #[test]
    fn success_rate_is_the_fraction_of_successful_experiments() {
        let mut set = InvalidationExperimentSet::new();
        set.push(InvalidationExperiment::new(
            "a",
            timed(CacheState::Warm, 1),
            timed(CacheState::Empty, 2),
        ));
        set.push(InvalidationExperiment::new(
            "b",
            timed(CacheState::Warm, 1),
            timed(CacheState::Warm, 2),
        ));
        assert_eq!(set.success_rate(), 0.5);
    }
}
