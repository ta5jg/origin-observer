// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/investigation.rs
// Purpose : Implement the investigation module for oo-observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Investigation model.

use oo_discovery::{DiscoveryEngine, DiscoveryOutcome};
use oo_evidence::{EvidenceRecord, ReproductionStatus};
use oo_snapshot::SnapshotRecord;

use crate::plan::ObservationPlan;

/// Complete result of one observation investigation.
#[derive(Debug, Clone, PartialEq)]
pub struct InvestigationRecord {
    plan: ObservationPlan,
    snapshot: SnapshotRecord,
    evidence: EvidenceRecord,
    outcome: DiscoveryOutcome,
}

impl InvestigationRecord {
    /// Creates an investigation record.
    #[must_use]
    pub const fn new(
        plan: ObservationPlan,
        snapshot: SnapshotRecord,
        evidence: EvidenceRecord,
        outcome: DiscoveryOutcome,
    ) -> Self {
        Self {
            plan,
            snapshot,
            evidence,
            outcome,
        }
    }

    /// Returns the observation plan.
    #[must_use]
    pub const fn plan(&self) -> &ObservationPlan {
        &self.plan
    }

    /// Returns the captured snapshot.
    #[must_use]
    pub const fn snapshot(&self) -> &SnapshotRecord {
        &self.snapshot
    }

    /// Returns the evidence record.
    #[must_use]
    pub const fn evidence(&self) -> &EvidenceRecord {
        &self.evidence
    }

    /// Returns the discovery outcome.
    #[must_use]
    pub const fn outcome(&self) -> &DiscoveryOutcome {
        &self.outcome
    }

    /// Updates evidence reproduction and refreshes the discovery outcome.
    pub fn set_reproduction(&mut self, reproduction: ReproductionStatus) {
        self.evidence.set_reproduction(reproduction);
        self.outcome = DiscoveryEngine.evaluate([&self.evidence]);
    }
}
