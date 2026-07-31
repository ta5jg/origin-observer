// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/verification.rs
// Purpose : Decide whether an experiment's evidence supports its hypothesis.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Decide whether an experiment's evidence supports its hypothesis.

use oo_evidence::ReproductionStatus;

use crate::model::ExperimentDesign;
use crate::repetition::RepetitionSet;
use crate::result::ExpectedOutcome;

/// Verdict for one experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Evidence supports the hypothesis at the given reproduction status.
    Supported(ReproductionStatus),
    /// Evidence contradicts the hypothesis: it is rejected.
    Rejected,
    /// No runs have been recorded yet.
    Pending,
}

/// Verifies an experiment design against its recorded runs.
///
/// The hypothesis itself must be falsifiable before a verdict is meaningful:
/// a design whose hypothesis has no falsifying condition cannot be verified,
/// only asserted, so this returns `None` for one.
#[must_use]
pub fn verify(
    design: &ExperimentDesign,
    expected: &ExpectedOutcome,
    repetitions: &RepetitionSet,
    independently_verified: bool,
) -> Option<Verdict> {
    if !design.hypothesis.is_falsifiable() {
        return None;
    }

    let status = crate::reproduction::derive_status(repetitions, expected, independently_verified);
    Some(match status {
        ReproductionStatus::Unknown => Verdict::Pending,
        ReproductionStatus::Contradicted => Verdict::Rejected,
        other => Verdict::Supported(other),
    })
}

#[cfg(test)]
mod tests {
    use oo_model::experiment::Experiment;

    use super::*;
    use crate::hypothesis::Hypothesis;
    use crate::result::ActualOutcome;

    fn design(falsifiable: bool) -> ExperimentDesign {
        let falsifying = if falsifiable {
            "it is not recognized"
        } else {
            ""
        };
        ExperimentDesign::new(
            Experiment::new("RQ-0005", "USDT is recognized"),
            Hypothesis::new("USDT is recognized", falsifying),
        )
    }

    fn expected() -> ExpectedOutcome {
        ExpectedOutcome {
            statement: "recognized".to_owned(),
        }
    }

    #[test]
    fn an_unfalsifiable_hypothesis_cannot_be_verified() {
        let repetitions = RepetitionSet::default();
        assert_eq!(
            verify(&design(false), &expected(), &repetitions, false),
            None
        );
    }

    #[test]
    fn no_runs_yet_is_pending() {
        let repetitions = RepetitionSet::default();
        assert_eq!(
            verify(&design(true), &expected(), &repetitions, false),
            Some(Verdict::Pending)
        );
    }

    #[test]
    fn matching_runs_are_supported() {
        let mut repetitions = RepetitionSet::default();
        repetitions.record(ActualOutcome {
            statement: "recognized".to_owned(),
            evidence_digest: None,
        });
        assert_eq!(
            verify(&design(true), &expected(), &repetitions, false),
            Some(Verdict::Supported(ReproductionStatus::Observed))
        );
    }

    #[test]
    fn contradicting_runs_are_rejected() {
        let mut repetitions = RepetitionSet::default();
        repetitions.record(ActualOutcome {
            statement: "not recognized".to_owned(),
            evidence_digest: None,
        });
        assert_eq!(
            verify(&design(true), &expected(), &repetitions, false),
            Some(Verdict::Rejected)
        );
    }
}
