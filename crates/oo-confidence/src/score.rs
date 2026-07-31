// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-confidence/src/score.rs
// Purpose : A deterministic confidence score derived from named factors.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A deterministic confidence score derived from named factors.

use crate::factor::ConfidenceFactor;

/// A confidence score, always between 0.0 and 1.0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceScore {
    value: f64,
}

impl ConfidenceScore {
    /// Creates a clamped score.
    #[must_use]
    pub fn new(value: f64) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    /// Returns the score value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

/// Scores a set of factors as the fraction satisfied.
///
/// Every factor is weighted equally: this module has no basis for weighing
/// evidence strength above independence or vice versa without a labelled
/// outcome dataset to check a weighting against — the same principle
/// `oo_discovery::prediction` follows for its own weights.
#[must_use]
pub fn score_factors(factors: &[ConfidenceFactor]) -> ConfidenceScore {
    if factors.is_empty() {
        return ConfidenceScore::new(0.0);
    }
    let satisfied = factors.iter().filter(|factor| factor.satisfied).count();
    ConfidenceScore::new(satisfied as f64 / factors.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factor::ConfidenceFactorKind;

    #[test]
    fn all_factors_satisfied_scores_one() {
        let factors = [
            ConfidenceFactor::new(ConfidenceFactorKind::EvidenceStrength, true),
            ConfidenceFactor::new(ConfidenceFactorKind::Verification, true),
        ];
        assert_eq!(score_factors(&factors).value(), 1.0);
    }

    #[test]
    fn half_satisfied_scores_half() {
        let factors = [
            ConfidenceFactor::new(ConfidenceFactorKind::EvidenceStrength, true),
            ConfidenceFactor::new(ConfidenceFactorKind::Verification, false),
        ];
        assert_eq!(score_factors(&factors).value(), 0.5);
    }

    #[test]
    fn no_factors_scores_zero_rather_than_dividing_by_zero() {
        assert_eq!(score_factors(&[]).value(), 0.0);
    }

    #[test]
    fn a_score_is_always_clamped() {
        assert_eq!(ConfidenceScore::new(2.0).value(), 1.0);
        assert_eq!(ConfidenceScore::new(-1.0).value(), 0.0);
    }
}
