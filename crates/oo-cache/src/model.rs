// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/model.rs
// Purpose : A cache key's full observation history.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A cache key's full observation history.

use crate::observation::TimedCacheObservation;
use crate::state::CacheTransition;

/// The recorded observation history for one cache key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheProfile {
    key: String,
    observations: Vec<TimedCacheObservation>,
}

impl CacheProfile {
    /// Opens a cache profile for a key.
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            observations: Vec::new(),
        }
    }

    /// Returns the cache key this profile tracks.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Records an observation.
    pub fn record(&mut self, observation: TimedCacheObservation) {
        self.observations.push(observation);
    }

    /// Returns the recorded observations, in insertion order.
    #[must_use]
    pub fn observations(&self) -> &[TimedCacheObservation] {
        &self.observations
    }

    /// Returns the most recent observation, by timestamp rather than
    /// insertion order.
    #[must_use]
    pub fn latest(&self) -> Option<&TimedCacheObservation> {
        self.observations
            .iter()
            .max_by_key(|observation| observation.timestamp())
    }

    /// Returns the state transitions between consecutive observations,
    /// ordered by timestamp.
    #[must_use]
    pub fn transitions(&self) -> Vec<CacheTransition> {
        let mut ordered: Vec<&TimedCacheObservation> = self.observations.iter().collect();
        ordered.sort_by_key(|observation| observation.timestamp());
        ordered
            .windows(2)
            .map(|pair| {
                CacheTransition::new(pair[0].observation().state(), pair[1].observation().state())
            })
            .collect()
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
    fn latest_is_found_by_timestamp_not_insertion_order() {
        let mut profile = CacheProfile::new("balanceOf");
        profile.record(timed(CacheState::Warm, 10));
        profile.record(timed(CacheState::Empty, 5));
        assert_eq!(
            profile.latest().unwrap().observation().state(),
            CacheState::Warm
        );
    }

    #[test]
    fn transitions_are_derived_in_chronological_order_regardless_of_insertion() {
        let mut profile = CacheProfile::new("balanceOf");
        profile.record(timed(CacheState::Empty, 2));
        profile.record(timed(CacheState::Warm, 1));
        let transitions = profile.transitions();
        assert_eq!(transitions.len(), 1);
        assert_eq!(
            transitions[0],
            CacheTransition::new(CacheState::Warm, CacheState::Empty)
        );
    }

    #[test]
    fn a_single_observation_has_no_transitions() {
        let mut profile = CacheProfile::new("balanceOf");
        profile.record(timed(CacheState::Warm, 1));
        assert!(profile.transitions().is_empty());
    }
}
