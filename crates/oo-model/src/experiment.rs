// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/experiment.rs
// Purpose : Experiment domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Experiment domain model.

use oo_core::ExperimentId;

/// Experiment lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ExperimentStatus {
    /// The experiment has not started.
    #[default]
    Planned,
    /// The experiment is running.
    Running,
    /// The experiment completed.
    Completed,
    /// The experiment was rejected or invalidated.
    Rejected,
}

/// Controlled experiment descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Experiment {
    id: ExperimentId,
    question_id: String,
    hypothesis: String,
    status: ExperimentStatus,
}

impl Experiment {
    /// Creates an experiment descriptor.
    #[must_use]
    pub fn new(question_id: impl Into<String>, hypothesis: impl Into<String>) -> Self {
        Self {
            id: ExperimentId::new(),
            question_id: question_id.into(),
            hypothesis: hypothesis.into(),
            status: ExperimentStatus::Planned,
        }
    }

    /// Returns the experiment identifier.
    #[must_use]
    pub const fn id(&self) -> ExperimentId {
        self.id
    }

    /// Returns the permanent research question identifier.
    #[must_use]
    pub fn question_id(&self) -> &str {
        &self.question_id
    }

    /// Returns the hypothesis text.
    #[must_use]
    pub fn hypothesis(&self) -> &str {
        &self.hypothesis
    }

    /// Returns the experiment status.
    #[must_use]
    pub const fn status(&self) -> ExperimentStatus {
        self.status
    }

    /// Changes the experiment status.
    pub const fn set_status(&mut self, status: ExperimentStatus) {
        self.status = status;
    }
}
