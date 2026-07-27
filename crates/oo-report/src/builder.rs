// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/builder.rs
// Purpose : Implement the builder module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Report builder.

use oo_discovery::DiscoveryOutcome;
use oo_evidence::EvidenceRecord;

use crate::conclusion::ReportConclusion;
use crate::finding::ReportFinding;
use crate::machine::MachineReport;

/// Builds reports from evaluated evidence.
#[derive(Debug, Default, Clone, Copy)]
pub struct ReportBuilder;

impl ReportBuilder {
    /// Builds a machine report from evidence and discovery outcome.
    #[must_use]
    pub fn build(&self, evidence: &EvidenceRecord, outcome: &DiscoveryOutcome) -> MachineReport {
        let finding = ReportFinding::new(
            evidence.subject(),
            evidence.digest().to_hex(),
            outcome.decision(),
            outcome.score().value(),
        );
        let conclusion = ReportConclusion::from_decision(outcome.decision());

        MachineReport::new(finding, conclusion)
    }
}
