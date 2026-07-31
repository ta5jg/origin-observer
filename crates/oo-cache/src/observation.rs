// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/observation.rs
// Purpose : A cache observation anchored to when it was made.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A cache observation anchored to when it was made.
//!
//! [`oo_model::cache::CacheObservation`] records what was observed but not
//! when. An invalidation experiment needs the "when," since a before/after
//! pair is only meaningful if the after-observation actually happened later.

use chrono::{DateTime, Utc};
use oo_model::cache::CacheObservation;

/// A cache observation with the time it was made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimedCacheObservation {
    observation: CacheObservation,
    timestamp: DateTime<Utc>,
}

impl TimedCacheObservation {
    /// Anchors a cache observation to a timestamp.
    #[must_use]
    pub const fn new(observation: CacheObservation, timestamp: DateTime<Utc>) -> Self {
        Self {
            observation,
            timestamp,
        }
    }

    /// Returns the underlying observation.
    #[must_use]
    pub const fn observation(&self) -> &CacheObservation {
        &self.observation
    }

    /// Returns when the observation was made.
    #[must_use]
    pub const fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use oo_model::cache::CacheState;

    use super::*;

    #[test]
    fn a_timed_observation_carries_both_the_observation_and_its_time() {
        let timestamp = Utc.timestamp_opt(100, 0).unwrap();
        let timed = TimedCacheObservation::new(
            CacheObservation::new("balanceOf", CacheState::Warm),
            timestamp,
        );
        assert_eq!(timed.observation().state(), CacheState::Warm);
        assert_eq!(timed.timestamp(), timestamp);
    }
}
