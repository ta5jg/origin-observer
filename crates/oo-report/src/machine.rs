// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/machine.rs
// Purpose : Implement the machine module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Machine-readable report model.

use crate::conclusion::ReportConclusion;
use crate::finding::ReportFinding;

/// Machine-readable report.
#[derive(Debug, Clone, PartialEq)]
pub struct MachineReport {
    finding: ReportFinding,
    conclusion: ReportConclusion,
}

impl MachineReport {
    /// Creates a machine-readable report.
    #[must_use]
    pub const fn new(finding: ReportFinding, conclusion: ReportConclusion) -> Self {
        Self {
            finding,
            conclusion,
        }
    }

    /// Returns the finding.
    #[must_use]
    pub const fn finding(&self) -> &ReportFinding {
        &self.finding
    }

    /// Returns the conclusion.
    #[must_use]
    pub const fn conclusion(&self) -> ReportConclusion {
        self.conclusion
    }
}
