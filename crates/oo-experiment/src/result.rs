// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/result.rs
// Purpose : Expected versus actual experiment outcomes.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Expected versus actual experiment outcomes.
//!
//! The expected outcome is written down before the procedure runs — it comes
//! from the hypothesis, not from looking at the result and describing it
//! afterward. Recording both separately, with the comparison as a distinct
//! step, is what keeps a favorable actual outcome from silently editing what
//! was expected.

/// What the hypothesis predicts will happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedOutcome {
    /// The predicted observation, in checkable terms.
    pub statement: String,
}

/// What actually happened when the procedure ran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActualOutcome {
    /// The observed result.
    pub statement: String,
    /// Digest of the evidence backing this observation, when one exists.
    pub evidence_digest: Option<String>,
}

/// One experiment's expected outcome, actual outcome, and their comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentResult {
    /// What was expected.
    pub expected: ExpectedOutcome,
    /// What was observed, once the procedure has run.
    pub actual: Option<ActualOutcome>,
}

impl ExperimentResult {
    /// Creates a result with an expected outcome and no actual outcome yet.
    #[must_use]
    pub fn pending(expected: ExpectedOutcome) -> Self {
        Self {
            expected,
            actual: None,
        }
    }

    /// Records the actual outcome.
    pub fn record(&mut self, actual: ActualOutcome) {
        self.actual = Some(actual);
    }

    /// Returns whether the actual outcome matches the expected one.
    ///
    /// Comparison is exact-text equality: this module does not interpret
    /// "close enough," because deciding what counts as close enough is the
    /// hypothesis author's job at the time the expected outcome is written,
    /// not this module's job after the fact.
    #[must_use]
    pub fn matched(&self) -> Option<bool> {
        self.actual
            .as_ref()
            .map(|actual| actual.statement.trim() == self.expected.statement.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pending_result_has_no_match_verdict() {
        let result = ExperimentResult::pending(ExpectedOutcome {
            statement: "recognized".to_owned(),
        });
        assert_eq!(result.matched(), None);
    }

    #[test]
    fn a_matching_actual_outcome_is_recognized() {
        let mut result = ExperimentResult::pending(ExpectedOutcome {
            statement: "recognized".to_owned(),
        });
        result.record(ActualOutcome {
            statement: "recognized".to_owned(),
            evidence_digest: Some("sha256:abc".to_owned()),
        });
        assert_eq!(result.matched(), Some(true));
    }

    #[test]
    fn a_diverging_actual_outcome_is_recognized_as_a_mismatch() {
        let mut result = ExperimentResult::pending(ExpectedOutcome {
            statement: "recognized".to_owned(),
        });
        result.record(ActualOutcome {
            statement: "not recognized".to_owned(),
            evidence_digest: None,
        });
        assert_eq!(result.matched(), Some(false));
    }
}
