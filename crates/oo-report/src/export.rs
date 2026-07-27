// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/export.rs
// Purpose : Implement the export module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Report export.

use serde_json::json;

use crate::machine::MachineReport;
use crate::reproduction::ReproductionReport;

/// Exports a machine report to JSON.
#[must_use]
pub fn export_json(report: &MachineReport) -> serde_json::Value {
    json!({
        "finding": {
            "subject": report.finding().subject(),
            "evidence_digest": report.finding().evidence_digest(),
            "decision": format!("{:?}", report.finding().decision()),
            "score": report.finding().score(),
        },
        "conclusion": format!("{:?}", report.conclusion()),
    })
}

/// Exports a reproduction report to JSON.
#[must_use]
pub fn export_reproduction_json(report: &ReproductionReport) -> serde_json::Value {
    let observations = report
        .observations()
        .iter()
        .map(|observation| {
            json!({
                "provider": observation.provider(),
                "subject": observation.subject(),
                "snapshot_digest": observation.snapshot_digest(),
                "evidence_digest": observation.evidence_digest(),
                "decision": observation.decision(),
                "score": observation.score(),
            })
        })
        .collect::<Vec<_>>();

    json!({
        "reproduction": {
            "status": format!("{:?}", report.status()),
            "provider_count": report.observations().len(),
            "consensus_digest": report.consensus_digest(),
        },
        "observations": observations,
    })
}
