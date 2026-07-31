// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-confidence/src/explanation.rs
// Purpose : A confidence assessment that names what it rests on.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A confidence assessment that names what it rests on.
//!
//! WDRP requires confidence to be explainable and falsifiable: a bare number
//! is not an assessment. Every explanation carries the reproduction status it
//! came from, the factor breakdown, and — when the underlying evidence was
//! contradicted — says so explicitly instead of reporting a plausible-looking
//! low score that looks like "not much evidence yet" rather than "refuted."

use oo_evidence::ReproductionStatus;
use oo_model::confidence::ConfidenceLevel;

use crate::factor::ConfidenceFactor;
use crate::level::{is_publishable, to_confidence_level, to_wdrp_code};
use crate::score::{score_factors, ConfidenceScore};

/// A complete, explainable confidence assessment.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceExplanation {
    /// The reproduction status this assessment is built from.
    pub reproduction: ReproductionStatus,
    /// General confidence level, when the status is not contradicted.
    pub level: Option<ConfidenceLevel>,
    /// WDRP confidence code (`"L0"`–`"L5"`), when the status is not
    /// contradicted.
    pub wdrp_code: Option<&'static str>,
    /// Whether the underlying evidence was contradicted.
    pub refuted: bool,
    /// Factor-by-factor breakdown.
    pub factors: Vec<ConfidenceFactor>,
    /// Fraction of factors satisfied.
    pub factor_score: ConfidenceScore,
    /// Whether this assessment meets WDRP's publication bar.
    pub publishable: bool,
}

/// Builds a confidence explanation from a reproduction status and its
/// supporting factors.
#[must_use]
pub fn explain(
    reproduction: ReproductionStatus,
    factors: Vec<ConfidenceFactor>,
) -> ConfidenceExplanation {
    let refuted = reproduction == ReproductionStatus::Contradicted;
    ConfidenceExplanation {
        reproduction,
        level: to_confidence_level(reproduction),
        wdrp_code: to_wdrp_code(reproduction),
        refuted,
        factor_score: score_factors(&factors),
        factors,
        publishable: is_publishable(reproduction),
    }
}

impl ConfidenceExplanation {
    /// Renders a one-line human-readable summary.
    #[must_use]
    pub fn summary(&self) -> String {
        if self.refuted {
            return "refuted: a later observation contradicted this evidence".to_owned();
        }
        let unmet: Vec<&str> = self
            .factors
            .iter()
            .filter(|factor| !factor.satisfied)
            .map(|factor| factor.kind.name())
            .collect();
        if unmet.is_empty() {
            format!(
                "{} ({}), every factor satisfied",
                self.wdrp_code.unwrap_or("?"),
                self.reproduction_name()
            )
        } else {
            format!(
                "{} ({}), missing: {}",
                self.wdrp_code.unwrap_or("?"),
                self.reproduction_name(),
                unmet.join(", ")
            )
        }
    }

    fn reproduction_name(&self) -> &'static str {
        match self.reproduction {
            ReproductionStatus::Unknown => "unknown",
            ReproductionStatus::Observed => "observed",
            ReproductionStatus::Reproduced => "reproduced",
            ReproductionStatus::IndependentlyVerified => "independently verified",
            ReproductionStatus::Contradicted => "contradicted",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factor::ConfidenceFactorKind;

    #[test]
    fn a_contradicted_status_is_refuted_and_unpublishable() {
        let explanation = explain(ReproductionStatus::Contradicted, Vec::new());
        assert!(explanation.refuted);
        assert!(!explanation.publishable);
        assert!(explanation.level.is_none());
        assert!(explanation.summary().contains("refuted"));
    }

    #[test]
    fn independently_verified_with_every_factor_met_is_publishable() {
        let factors = vec![
            ConfidenceFactor::new(ConfidenceFactorKind::EvidenceStrength, true),
            ConfidenceFactor::new(ConfidenceFactorKind::Verification, true),
            ConfidenceFactor::new(ConfidenceFactorKind::Reproducibility, true),
            ConfidenceFactor::new(ConfidenceFactorKind::Independence, true),
        ];
        let explanation = explain(ReproductionStatus::IndependentlyVerified, factors);
        assert!(explanation.publishable);
        assert_eq!(explanation.wdrp_code, Some("L5"));
        assert!(explanation.summary().contains("every factor satisfied"));
    }

    #[test]
    fn the_summary_names_exactly_which_factors_are_missing() {
        let factors = vec![
            ConfidenceFactor::new(ConfidenceFactorKind::EvidenceStrength, true),
            ConfidenceFactor::new(ConfidenceFactorKind::Independence, false),
        ];
        let explanation = explain(ReproductionStatus::Reproduced, factors);
        assert!(explanation.summary().contains("independence"));
        assert!(!explanation.summary().contains("evidence strength"));
    }
}
