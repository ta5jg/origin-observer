// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-confidence/src/validation.rs
// Purpose : Validate that a confidence explanation is internally consistent.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate that a confidence explanation is internally consistent.
//!
//! [`explain`](crate::explanation::explain) already enforces these
//! invariants by construction, so a hand-built or deserialized
//! [`ConfidenceExplanation`] (e.g. one read back from storage) is what this
//! module is for: catching a value that claims to be refuted and
//! publishable at once, or publishable without having reached WDRP's `L5`
//! bar.

use crate::explanation::ConfidenceExplanation;

/// One way a confidence explanation can be internally inconsistent.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfidenceValidationError {
    /// The explanation is refuted but still carries a confidence level or
    /// WDRP code, which the reconciliation in [`crate::level`] never
    /// produces for a contradicted status.
    #[error("a refuted explanation must not carry a confidence level or WDRP code")]
    RefutedButLevelled,
    /// The explanation is refuted but marked publishable.
    #[error("a refuted explanation must not be publishable")]
    RefutedButPublishable,
    /// The explanation is publishable but did not reach WDRP's `L5` bar.
    #[error("a publishable explanation must carry the L5 WDRP code")]
    PublishableWithoutL5,
    /// The factor score falls outside the `0.0..=1.0` range that
    /// [`crate::score::ConfidenceScore`] is meant to guarantee.
    #[error("factor score {0} is outside the 0.0..=1.0 range")]
    ScoreOutOfRange(String),
}

/// Validates a confidence explanation's internal consistency.
///
/// # Errors
///
/// Returns the first inconsistency found; see
/// [`ConfidenceValidationError`] for what is checked.
pub fn validate_explanation(
    explanation: &ConfidenceExplanation,
) -> Result<(), ConfidenceValidationError> {
    if explanation.refuted {
        if explanation.level.is_some() || explanation.wdrp_code.is_some() {
            return Err(ConfidenceValidationError::RefutedButLevelled);
        }
        if explanation.publishable {
            return Err(ConfidenceValidationError::RefutedButPublishable);
        }
    }

    if explanation.publishable && explanation.wdrp_code != Some("L5") {
        return Err(ConfidenceValidationError::PublishableWithoutL5);
    }

    let score = explanation.factor_score.value();
    if !(0.0..=1.0).contains(&score) {
        return Err(ConfidenceValidationError::ScoreOutOfRange(
            score.to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use oo_evidence::ReproductionStatus;

    use super::*;
    use crate::explanation::explain;
    use crate::factor::{ConfidenceFactor, ConfidenceFactorKind};

    #[test]
    fn an_explanation_built_by_explain_is_always_valid() {
        let factors = vec![ConfidenceFactor::new(
            ConfidenceFactorKind::EvidenceStrength,
            true,
        )];
        for status in [
            ReproductionStatus::Unknown,
            ReproductionStatus::Observed,
            ReproductionStatus::Reproduced,
            ReproductionStatus::IndependentlyVerified,
            ReproductionStatus::Contradicted,
        ] {
            let explanation = explain(status, factors.clone());
            assert!(validate_explanation(&explanation).is_ok());
        }
    }

    #[test]
    fn a_refuted_explanation_carrying_a_level_is_rejected() {
        let mut explanation = explain(ReproductionStatus::Contradicted, Vec::new());
        explanation.level = Some(oo_model::confidence::ConfidenceLevel::Low);
        assert_eq!(
            validate_explanation(&explanation),
            Err(ConfidenceValidationError::RefutedButLevelled)
        );
    }

    #[test]
    fn a_refuted_explanation_marked_publishable_is_rejected() {
        let mut explanation = explain(ReproductionStatus::Contradicted, Vec::new());
        explanation.publishable = true;
        assert_eq!(
            validate_explanation(&explanation),
            Err(ConfidenceValidationError::RefutedButPublishable)
        );
    }

    #[test]
    fn a_publishable_explanation_without_l5_is_rejected() {
        let mut explanation = explain(ReproductionStatus::Reproduced, Vec::new());
        explanation.publishable = true;
        assert_eq!(
            validate_explanation(&explanation),
            Err(ConfidenceValidationError::PublishableWithoutL5)
        );
    }
}
