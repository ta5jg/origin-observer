// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/validation.rs
// Purpose : Implement the validation module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Report validation.

use crate::machine::MachineReport;

/// Validates report invariants.
#[must_use]
pub fn validate_report(report: &MachineReport) -> bool {
    !report.finding().subject().trim().is_empty()
        && !report.finding().evidence_digest().trim().is_empty()
}
