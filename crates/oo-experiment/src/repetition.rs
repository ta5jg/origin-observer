// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/repetition.rs
// Purpose : Track repeated runs of the same experiment.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Track repeated runs of the same experiment.
//!
//! A single run cannot establish reproducibility; that requires the same
//! procedure run more than once and compared. This module holds the set of
//! runs and reports whether they agree, without deciding what agreement means
//! for confidence — that is [`crate::reproduction`]'s job.

use crate::result::ActualOutcome;

/// One repeated run's outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepetitionRun {
    /// Run index, starting at 1.
    pub index: u32,
    /// Outcome observed on this run.
    pub outcome: ActualOutcome,
}

/// A set of repeated runs of one experiment.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepetitionSet {
    runs: Vec<RepetitionRun>,
}

impl RepetitionSet {
    /// Records a run's outcome.
    pub fn record(&mut self, outcome: ActualOutcome) {
        let index = self.runs.len() as u32 + 1;
        self.runs.push(RepetitionRun { index, outcome });
    }

    /// Returns every recorded run.
    #[must_use]
    pub fn runs(&self) -> &[RepetitionRun] {
        &self.runs
    }

    /// Returns the number of runs recorded.
    #[must_use]
    pub fn len(&self) -> usize {
        self.runs.len()
    }

    /// Returns whether no runs have been recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    /// Returns whether every recorded run reported the same outcome
    /// statement.
    ///
    /// A single run is trivially "consistent" with itself; this only means
    /// something once at least two runs exist, which callers deciding
    /// reproduction status should check separately.
    #[must_use]
    pub fn is_consistent(&self) -> bool {
        let mut statements = self.runs.iter().map(|run| run.outcome.statement.trim());
        let Some(first) = statements.next() else {
            return true;
        };
        statements.all(|statement| statement == first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outcome(statement: &str) -> ActualOutcome {
        ActualOutcome {
            statement: statement.to_owned(),
            evidence_digest: None,
        }
    }

    #[test]
    fn runs_are_indexed_from_one_in_order() {
        let mut set = RepetitionSet::default();
        set.record(outcome("recognized"));
        set.record(outcome("recognized"));
        assert_eq!(set.runs()[0].index, 1);
        assert_eq!(set.runs()[1].index, 2);
    }

    #[test]
    fn matching_runs_are_consistent() {
        let mut set = RepetitionSet::default();
        set.record(outcome("recognized"));
        set.record(outcome("recognized"));
        assert!(set.is_consistent());
    }

    #[test]
    fn diverging_runs_are_inconsistent() {
        let mut set = RepetitionSet::default();
        set.record(outcome("recognized"));
        set.record(outcome("not recognized"));
        assert!(!set.is_consistent());
    }

    #[test]
    fn an_empty_set_is_trivially_consistent() {
        assert!(RepetitionSet::default().is_consistent());
    }
}
