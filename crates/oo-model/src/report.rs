// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/report.rs
// Purpose : Report summary domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Report summary domain model.

use oo_core::{EvidenceId, ReportId};

/// Report conclusion state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ReportConclusion {
    /// The report does not yet support a conclusion.
    #[default]
    Unknown,
    /// The report supports the hypothesis.
    Supports,
    /// The report contradicts the hypothesis.
    Contradicts,
    /// The report is inconclusive.
    Inconclusive,
}

/// Human and machine report summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportSummary {
    id: ReportId,
    title: String,
    conclusion: ReportConclusion,
    evidence: Vec<EvidenceId>,
}

impl ReportSummary {
    /// Creates a report summary.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: ReportId::new(),
            title: title.into(),
            conclusion: ReportConclusion::Unknown,
            evidence: Vec::new(),
        }
    }

    /// Returns the report identifier.
    #[must_use]
    pub const fn id(&self) -> ReportId {
        self.id
    }

    /// Returns the report title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the report conclusion.
    #[must_use]
    pub const fn conclusion(&self) -> ReportConclusion {
        self.conclusion
    }

    /// Changes the report conclusion.
    pub const fn set_conclusion(&mut self, conclusion: ReportConclusion) {
        self.conclusion = conclusion;
    }

    /// Adds supporting evidence.
    pub fn add_evidence(&mut self, evidence_id: EvidenceId) {
        self.evidence.push(evidence_id);
    }

    /// Returns referenced evidence identifiers.
    #[must_use]
    pub fn evidence(&self) -> &[EvidenceId] {
        &self.evidence
    }
}
