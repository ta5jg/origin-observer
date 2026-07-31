// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-confidence/src/engine.rs
// Purpose : Combine and compare confidence explanations.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Combine and compare confidence explanations.
//!
//! Two operations live here. [`aggregate`] combines several observations of
//! *the same claim* into one verdict: a contradiction anywhere refutes the
//! whole claim (contradiction propagates), and a set with no positive
//! evidence stays unknown rather than being inflated by absence (unknown
//! propagates). [`compare`] is the general operation behind the roadmap's
//! wallet/asset/provider/network/temporal comparison list: comparing two
//! *different* things' confidence is the same arithmetic regardless of what
//! is being compared, so one function parametrized by a caller-supplied
//! dimension label replaces five near-identical modules.

use oo_evidence::ReproductionStatus;

use crate::explanation::{explain, ConfidenceExplanation};
use crate::factor::ConfidenceFactor;

/// Combines several observations of one claim into a single verdict.
///
/// `sources` pairs each observation's reproduction status with the factors
/// that supported it. The combined factor list is the union across every
/// non-contradicting, non-unknown source, so the aggregate explanation still
/// names what supported it.
#[must_use]
pub fn aggregate(sources: &[(ReproductionStatus, Vec<ConfidenceFactor>)]) -> ConfidenceExplanation {
    if sources
        .iter()
        .any(|(status, _)| *status == ReproductionStatus::Contradicted)
    {
        return explain(ReproductionStatus::Contradicted, Vec::new());
    }

    let positive: Vec<&(ReproductionStatus, Vec<ConfidenceFactor>)> = sources
        .iter()
        .filter(|(status, _)| *status != ReproductionStatus::Unknown)
        .collect();

    if positive.is_empty() {
        return explain(ReproductionStatus::Unknown, Vec::new());
    }

    let strongest = positive
        .iter()
        .map(|(status, _)| *status)
        .max_by_key(strength_rank)
        .expect("positive is non-empty");

    let factors: Vec<ConfidenceFactor> = positive
        .iter()
        .flat_map(|(_, factors)| factors.iter().copied())
        .collect();

    explain(strongest, factors)
}

/// Orders reproduction statuses by evidentiary strength, explicitly, rather
/// than relying on declaration-order derived `Ord` (which would place
/// `Contradicted` above `IndependentlyVerified`).
const fn strength_rank(status: &ReproductionStatus) -> u8 {
    match status {
        ReproductionStatus::Unknown => 0,
        ReproductionStatus::Observed => 1,
        ReproductionStatus::Reproduced => 2,
        ReproductionStatus::IndependentlyVerified => 3,
        ReproductionStatus::Contradicted => 0,
    }
}

/// A comparison between two confidence explanations along a named dimension.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceComparison {
    /// What is being compared: `"wallet"`, `"asset"`, `"provider"`,
    /// `"network"`, `"temporal"`, or another caller-supplied label.
    pub dimension: &'static str,
    /// The subject's factor score.
    pub subject_score: f64,
    /// The reference's factor score.
    pub reference_score: f64,
    /// Whether the subject and reference reached the same reproduction
    /// status.
    pub reproduction_matches: bool,
}

impl ConfidenceComparison {
    /// Returns the signed difference: positive means the subject scored
    /// higher than the reference.
    #[must_use]
    pub fn difference(&self) -> f64 {
        self.subject_score - self.reference_score
    }
}

/// Compares two confidence explanations along a named dimension.
#[must_use]
pub fn compare(
    dimension: &'static str,
    subject: &ConfidenceExplanation,
    reference: &ConfidenceExplanation,
) -> ConfidenceComparison {
    ConfidenceComparison {
        dimension,
        subject_score: subject.factor_score.value(),
        reference_score: reference.factor_score.value(),
        reproduction_matches: subject.reproduction == reference.reproduction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factor::ConfidenceFactorKind;

    fn factor(satisfied: bool) -> Vec<ConfidenceFactor> {
        vec![ConfidenceFactor::new(
            ConfidenceFactorKind::EvidenceStrength,
            satisfied,
        )]
    }

    #[test]
    fn a_single_contradiction_refutes_the_whole_aggregate() {
        let sources = vec![
            (ReproductionStatus::IndependentlyVerified, factor(true)),
            (ReproductionStatus::Contradicted, Vec::new()),
        ];
        let aggregated = aggregate(&sources);
        assert!(aggregated.refuted);
    }

    #[test]
    fn all_unknown_sources_stay_unknown() {
        let sources = vec![
            (ReproductionStatus::Unknown, Vec::new()),
            (ReproductionStatus::Unknown, Vec::new()),
        ];
        let aggregated = aggregate(&sources);
        assert_eq!(aggregated.reproduction, ReproductionStatus::Unknown);
    }

    #[test]
    fn the_aggregate_reports_the_strongest_non_contradicting_status() {
        let sources = vec![
            (ReproductionStatus::Observed, factor(true)),
            (ReproductionStatus::IndependentlyVerified, factor(true)),
        ];
        let aggregated = aggregate(&sources);
        assert_eq!(
            aggregated.reproduction,
            ReproductionStatus::IndependentlyVerified
        );
    }

    #[test]
    fn unknown_sources_do_not_drag_down_a_positive_aggregate() {
        let sources = vec![
            (ReproductionStatus::Unknown, Vec::new()),
            (ReproductionStatus::Reproduced, factor(true)),
        ];
        let aggregated = aggregate(&sources);
        assert_eq!(aggregated.reproduction, ReproductionStatus::Reproduced);
    }

    #[test]
    fn an_empty_source_set_is_unknown() {
        assert_eq!(aggregate(&[]).reproduction, ReproductionStatus::Unknown);
    }

    #[test]
    fn comparison_reports_the_score_difference_by_dimension() {
        let subject = explain(ReproductionStatus::Reproduced, factor(true));
        let reference = explain(ReproductionStatus::Reproduced, factor(false));
        let comparison = compare("wallet", &subject, &reference);
        assert_eq!(comparison.dimension, "wallet");
        assert!(comparison.difference() > 0.0);
        assert!(comparison.reproduction_matches);
    }
}
