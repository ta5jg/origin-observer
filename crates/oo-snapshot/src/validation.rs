// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/validation.rs
// Purpose : Snapshot validation.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot validation.

use crate::snapshot::SnapshotRecord;

/// Validates snapshot invariants.
#[must_use]
pub fn validate_snapshot(snapshot: &SnapshotRecord) -> bool {
    !snapshot.subject().trim().is_empty() && !snapshot.digest().is_zero()
}
