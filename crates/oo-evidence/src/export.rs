// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/export.rs
// Purpose : Evidence export.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence export.

use serde_json::json;

use crate::model::EvidenceRecord;

/// Exports one evidence record as JSON.
#[must_use]
pub fn export_json(record: &EvidenceRecord) -> serde_json::Value {
    json!({
        "id": record.id().to_string(),
        "subject": record.subject(),
        "digest": record.digest().to_hex(),
        "source": {
            "kind": format!("{:?}", record.source().kind()),
            "locator": record.source().locator(),
        },
        "reproduction": format!("{:?}", record.reproduction()),
    })
}
