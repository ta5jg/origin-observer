// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/validation.rs
// Purpose : Implement the validation module for oo-observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Observer validation.

use oo_evidence::validate_evidence;
use oo_snapshot::validate_snapshot;

use crate::investigation::InvestigationRecord;
use crate::plan::ObservationPlan;

/// Validates an observation plan.
#[must_use]
pub fn validate_plan(plan: &ObservationPlan) -> bool {
    !plan.subject().trim().is_empty()
}

/// Validates a complete investigation.
#[must_use]
pub fn validate_investigation(record: &InvestigationRecord) -> bool {
    validate_plan(record.plan())
        && validate_snapshot(record.snapshot())
        && validate_evidence(record.evidence())
        && record.snapshot().subject() == record.evidence().subject()
}
