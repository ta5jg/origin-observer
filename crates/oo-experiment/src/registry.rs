// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/registry.rs
// Purpose : Keep every experiment design addressable by its research question.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Keep every experiment design addressable by its research question.

use std::collections::BTreeMap;

use crate::model::ExperimentDesign;

/// Registry of experiment designs, keyed by permanent research question id.
#[derive(Debug, Clone, Default)]
pub struct ExperimentRegistry {
    designs: BTreeMap<String, Vec<ExperimentDesign>>,
}

impl ExperimentRegistry {
    /// Registers a design under its research question.
    pub fn register(&mut self, design: ExperimentDesign) {
        self.designs
            .entry(design.experiment.question_id().to_owned())
            .or_default()
            .push(design);
    }

    /// Returns every design registered for a research question, in
    /// registration order.
    #[must_use]
    pub fn for_question(&self, question_id: &str) -> &[ExperimentDesign] {
        self.designs
            .get(question_id)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    /// Returns every research question with at least one registered design.
    #[must_use]
    pub fn questions(&self) -> Vec<&str> {
        self.designs.keys().map(String::as_str).collect()
    }

    /// Returns the total number of registered designs across all questions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.designs.values().map(Vec::len).sum()
    }

    /// Returns whether no designs are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.designs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use oo_model::experiment::Experiment;

    use super::*;
    use crate::hypothesis::Hypothesis;

    fn design(question: &str, claim: &str) -> ExperimentDesign {
        ExperimentDesign::new(
            Experiment::new(question, claim),
            Hypothesis::new(claim, "the opposite is observed"),
        )
    }

    #[test]
    fn designs_are_grouped_by_research_question() {
        let mut registry = ExperimentRegistry::default();
        registry.register(design("RQ-0005", "USDT is recognized"));
        registry.register(design("RQ-0005", "USDT is recognized on a second wallet"));
        registry.register(design("RQ-0006", "our asset is not recognized"));

        assert_eq!(registry.for_question("RQ-0005").len(), 2);
        assert_eq!(registry.for_question("RQ-0006").len(), 1);
        assert_eq!(registry.len(), 3);
    }

    #[test]
    fn an_unregistered_question_returns_an_empty_slice_not_an_error() {
        let registry = ExperimentRegistry::default();
        assert!(registry.for_question("RQ-9999").is_empty());
    }
}
