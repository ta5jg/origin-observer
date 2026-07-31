// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/validation.rs
// Purpose : Validate that an invalidation experiment is well-formed.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate that an invalidation experiment is well-formed.

use crate::invalidation::InvalidationExperiment;

/// One way an invalidation experiment can be malformed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CacheValidationError {
    /// The `before` and `after` observations used different cache keys, so
    /// the experiment does not test invalidation of anything in particular.
    #[error(
        "experiment's before/after observations use different cache keys ('{before}' vs '{after}')"
    )]
    KeyMismatch {
        /// The observed key of the `before` observation.
        before: String,
        /// The observed key of the `after` observation.
        after: String,
    },
    /// The `after` observation was not made later than the `before`
    /// observation.
    #[error("experiment's 'after' observation is not later than its 'before' observation")]
    NonChronological,
}

/// Validates that an invalidation experiment's observations share a cache
/// key and are ordered in time.
///
/// # Errors
///
/// Returns [`CacheValidationError`] when the observations disagree on key or
/// are not chronologically ordered.
pub fn validate_invalidation_experiment(
    experiment: &InvalidationExperiment,
) -> Result<(), CacheValidationError> {
    let before_key = experiment.before().observation().key();
    let after_key = experiment.after().observation().key();
    if before_key != after_key {
        return Err(CacheValidationError::KeyMismatch {
            before: before_key.to_owned(),
            after: after_key.to_owned(),
        });
    }

    if experiment.after().timestamp() <= experiment.before().timestamp() {
        return Err(CacheValidationError::NonChronological);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use oo_model::cache::{CacheObservation, CacheState};

    use super::*;
    use crate::observation::TimedCacheObservation;

    fn timed(key: &str, state: CacheState, seconds: i64) -> TimedCacheObservation {
        TimedCacheObservation::new(
            CacheObservation::new(key, state),
            Utc.timestamp_opt(seconds, 0).unwrap(),
        )
    }

    #[test]
    fn a_well_formed_experiment_is_valid() {
        let experiment = InvalidationExperiment::new(
            "balanceOf",
            timed("balanceOf", CacheState::Warm, 1),
            timed("balanceOf", CacheState::Empty, 2),
        );
        assert!(validate_invalidation_experiment(&experiment).is_ok());
    }

    #[test]
    fn mismatched_keys_are_rejected() {
        let experiment = InvalidationExperiment::new(
            "balanceOf",
            timed("balanceOf", CacheState::Warm, 1),
            timed("symbol", CacheState::Empty, 2),
        );
        assert!(matches!(
            validate_invalidation_experiment(&experiment),
            Err(CacheValidationError::KeyMismatch { .. })
        ));
    }

    #[test]
    fn a_non_chronological_pair_is_rejected() {
        let experiment = InvalidationExperiment::new(
            "balanceOf",
            timed("balanceOf", CacheState::Warm, 2),
            timed("balanceOf", CacheState::Empty, 1),
        );
        assert_eq!(
            validate_invalidation_experiment(&experiment),
            Err(CacheValidationError::NonChronological)
        );
    }
}
