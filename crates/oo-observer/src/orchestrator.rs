// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/orchestrator.rs
// Purpose : Implement the orchestrator module for oo-observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Observation orchestrator.

use oo_discovery::DiscoveryEngine;
use oo_evidence::{EvidenceBuilder, EvidenceSourceKind};
use oo_snapshot::{normalize_json, SnapshotCollector, SnapshotRequest};
use serde_json::Value;

use crate::investigation::InvestigationRecord;
use crate::plan::ObservationPlan;

/// Orchestrates the deterministic local observation pipeline.
#[derive(Debug, Default, Clone, Copy)]
pub struct ObserverOrchestrator {
    discovery: DiscoveryEngine,
}

impl ObserverOrchestrator {
    /// Runs one observation from an already-fetched payload.
    #[must_use]
    pub fn observe(&self, plan: ObservationPlan, payload: Value) -> InvestigationRecord {
        let request = SnapshotRequest::new(plan.network_id(), plan.provider_id(), plan.subject());
        let snapshot = SnapshotCollector::collect(&request, payload);
        let evidence_bytes = normalize_json(snapshot.payload());
        let locator = format!("snapshot:{}", snapshot.id());
        let evidence =
            EvidenceBuilder::new(EvidenceSourceKind::Snapshot, locator, snapshot.subject())
                .bytes(evidence_bytes)
                .build();
        let outcome = self.discovery.evaluate([&evidence]);

        InvestigationRecord::new(plan, snapshot, evidence, outcome)
    }
}
