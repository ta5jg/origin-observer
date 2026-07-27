// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/validation.rs
// Purpose : Evidence validation.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence validation.

use crate::model::EvidenceRecord;

/// Validates evidence invariants.
#[must_use]
pub fn validate_evidence(record: &EvidenceRecord) -> bool {
    !record.subject().trim().is_empty() && !record.digest().is_zero()
}
