// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/reproduction.rs
// Purpose : Derive a WDRP reproduction status from a set of repeated runs.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Derive a WDRP reproduction status from a set of repeated runs.
//!
//! This is where the confidence contract's levels connect to actual repeated
//! evidence: `L2` requires one matching observation, `L3` requires at least
//! two consistent runs by the same observer, and `L5` (independently
//! verified) requires a second observer's confirmation, which this crate
//! cannot determine on its own — it is an input, not something derivable from
//! repetition alone.

use oo_evidence::ReproductionStatus;

use crate::repetition::RepetitionSet;
use crate::result::ExpectedOutcome;

/// Derives a reproduction status from a repetition set.
///
/// `independently_verified` must come from the caller: whether a second,
/// independent observer confirmed the result is a fact about who ran it, not
/// something this module can infer from the run count alone.
#[must_use]
pub fn derive_status(
    repetitions: &RepetitionSet,
    expected: &ExpectedOutcome,
    independently_verified: bool,
) -> ReproductionStatus {
    if repetitions.is_empty() {
        return ReproductionStatus::Unknown;
    }

    let matches_expected = repetitions
        .runs()
        .iter()
        .all(|run| run.outcome.statement.trim() == expected.statement.trim());

    if !matches_expected {
        return ReproductionStatus::Contradicted;
    }

    // Independent verification and same-observer reproduction both require at
    // least two consistent runs: a lone run has nothing for a second run to
    // have reproduced, regardless of who is claimed to have run it.
    if repetitions.len() >= 2 && repetitions.is_consistent() {
        if independently_verified {
            return ReproductionStatus::IndependentlyVerified;
        }
        return ReproductionStatus::Reproduced;
    }

    ReproductionStatus::Observed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::ActualOutcome;

    fn expected() -> ExpectedOutcome {
        ExpectedOutcome {
            statement: "recognized".to_owned(),
        }
    }

    fn set(statements: &[&str]) -> RepetitionSet {
        let mut set = RepetitionSet::default();
        for statement in statements {
            set.record(ActualOutcome {
                statement: (*statement).to_owned(),
                evidence_digest: None,
            });
        }
        set
    }

    #[test]
    fn no_runs_is_unknown() {
        assert_eq!(
            derive_status(&RepetitionSet::default(), &expected(), false),
            ReproductionStatus::Unknown
        );
    }

    #[test]
    fn one_matching_run_is_observed() {
        assert_eq!(
            derive_status(&set(&["recognized"]), &expected(), false),
            ReproductionStatus::Observed
        );
    }

    #[test]
    fn two_consistent_matching_runs_are_reproduced() {
        assert_eq!(
            derive_status(&set(&["recognized", "recognized"]), &expected(), false),
            ReproductionStatus::Reproduced
        );
    }

    #[test]
    fn a_second_observer_confirming_consistent_runs_is_independently_verified() {
        assert_eq!(
            derive_status(&set(&["recognized", "recognized"]), &expected(), true),
            ReproductionStatus::IndependentlyVerified
        );
    }

    #[test]
    fn an_unexpected_outcome_is_contradicted_regardless_of_run_count() {
        assert_eq!(
            derive_status(&set(&["not recognized"]), &expected(), false),
            ReproductionStatus::Contradicted
        );
    }

    #[test]
    fn a_run_that_diverges_from_a_matching_first_run_is_contradicted() {
        // The first run matched expectations but the second did not: the
        // overall claim is contradicted, not "partially observed."
        let set = set(&["recognized", "not recognized"]);
        assert_eq!(
            derive_status(&set, &expected(), false),
            ReproductionStatus::Contradicted
        );
    }

    #[test]
    fn claiming_independent_verification_without_enough_runs_does_not_grant_it() {
        // A single run cannot be "independently verified": there is nothing
        // for a second observer to have reproduced yet.
        assert_eq!(
            derive_status(&set(&["recognized"]), &expected(), true),
            ReproductionStatus::Observed
        );
    }
}
