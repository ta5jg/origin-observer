// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/human.rs
// Purpose : Implement the human module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Human-readable report rendering.

use crate::machine::MachineReport;

/// Renders a concise human-readable report.
#[must_use]
pub fn render_human(report: &MachineReport) -> String {
    format!(
        "Origin Observer finding\nsubject: {}\ndecision: {:?}\nscore: {:.2}\nconclusion: {:?}\nevidence_digest: {}",
        report.finding().subject(),
        report.finding().decision(),
        report.finding().score(),
        report.conclusion(),
        report.finding().evidence_digest(),
    )
}
