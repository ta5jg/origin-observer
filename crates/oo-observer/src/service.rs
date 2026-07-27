// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/service.rs
// Purpose : Implement the service module for oo-observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Observer service facade.

use serde_json::Value;

use crate::investigation::InvestigationRecord;
use crate::orchestrator::ObserverOrchestrator;
use crate::plan::ObservationPlan;
use crate::validation::validate_investigation;

/// High-level service for local observation runs.
#[derive(Debug, Default, Clone, Copy)]
pub struct ObserverService {
    orchestrator: ObserverOrchestrator,
}

impl ObserverService {
    /// Runs an observation and returns the validated investigation.
    #[must_use]
    pub fn observe(&self, plan: ObservationPlan, payload: Value) -> Option<InvestigationRecord> {
        let record = self.orchestrator.observe(plan, payload);
        validate_investigation(&record).then_some(record)
    }
}
