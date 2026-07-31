// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/validation.rs
// Purpose : Validate an experiment design before it is registered.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate an experiment design before it is registered.

use crate::model::ExperimentDesign;

/// Validates that an experiment design is ready to run.
///
/// A design is valid only when its hypothesis is falsifiable and its
/// procedure has at least one step: a design with no procedure cannot be
/// repeated by anyone, which fails WDRP's reproducibility requirement before
/// a single observation is even made.
#[must_use]
pub fn validate_design(design: &ExperimentDesign) -> bool {
    design.hypothesis.is_falsifiable()
        && !design.procedure.is_empty()
        && !design.experiment.question_id().trim().is_empty()
}

#[cfg(test)]
mod tests {
    use oo_model::experiment::Experiment;

    use super::*;
    use crate::hypothesis::Hypothesis;

    #[test]
    fn a_complete_design_is_valid() {
        let mut design = ExperimentDesign::new(
            Experiment::new("RQ-0005", "USDT is recognized"),
            Hypothesis::new("USDT is recognized", "it is not recognized"),
        );
        design.procedure.push("pin the block");
        assert!(validate_design(&design));
    }

    #[test]
    fn a_design_with_no_procedure_is_invalid() {
        let design = ExperimentDesign::new(
            Experiment::new("RQ-0005", "USDT is recognized"),
            Hypothesis::new("USDT is recognized", "it is not recognized"),
        );
        assert!(!validate_design(&design));
    }

    #[test]
    fn a_design_with_an_unfalsifiable_hypothesis_is_invalid() {
        let mut design = ExperimentDesign::new(
            Experiment::new("RQ-0005", "USDT is recognized"),
            Hypothesis::new("USDT is recognized", ""),
        );
        design.procedure.push("pin the block");
        assert!(!validate_design(&design));
    }
}
