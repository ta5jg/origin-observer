// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/model.rs
// Purpose : The complete design of one controlled experiment.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! The complete design of one controlled experiment.

use oo_model::experiment::Experiment;

use crate::control::ExperimentControl;
use crate::hypothesis::Hypothesis;
use crate::procedure::Procedure;
use crate::variable::ExperimentVariable;

/// A fully specified experiment: the base descriptor plus everything the
/// scientific method requires around it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentDesign {
    /// Base experiment descriptor (id, question, status).
    pub experiment: Experiment,
    /// The falsifiable hypothesis under test.
    pub hypothesis: Hypothesis,
    /// Variables involved.
    pub variables: Vec<ExperimentVariable>,
    /// Conditions held constant.
    pub controls: Vec<ExperimentControl>,
    /// Conditions that must hold before the procedure starts.
    pub preconditions: Vec<String>,
    /// The ordered, repeatable procedure.
    pub procedure: Procedure,
}

impl ExperimentDesign {
    /// Creates an experiment design.
    #[must_use]
    pub fn new(experiment: Experiment, hypothesis: Hypothesis) -> Self {
        Self {
            experiment,
            hypothesis,
            variables: Vec::new(),
            controls: Vec::new(),
            preconditions: Vec::new(),
            procedure: Procedure::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_design_starts_with_empty_collections() {
        let design = ExperimentDesign::new(
            Experiment::new(
                "RQ-0005",
                "USDT is recognized because it is in the default list",
            ),
            Hypothesis::new("claim", "falsifying condition"),
        );
        assert!(design.variables.is_empty());
        assert!(design.controls.is_empty());
        assert!(design.procedure.is_empty());
    }
}
