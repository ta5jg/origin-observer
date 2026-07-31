// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/investigation.rs
// Purpose : Implement the investigation module for oo-observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Investigation model.

use oo_cache::TimedCacheObservation;
use oo_discovery::{DiscoveryEngine, DiscoveryOutcome};
use oo_evidence::{EvidenceRecord, ReproductionStatus};
use oo_model::cache::CacheState;
use oo_snapshot::SnapshotRecord;

use crate::plan::ObservationPlan;

/// Complete result of one observation investigation.
#[derive(Debug, Clone, PartialEq)]
pub struct InvestigationRecord {
    plan: ObservationPlan,
    snapshot: SnapshotRecord,
    evidence: EvidenceRecord,
    outcome: DiscoveryOutcome,
    cache_observation: Option<TimedCacheObservation>,
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
            cache_observation: None,
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

    /// Attaches the cache observation made while gathering this
    /// investigation's evidence.
    pub fn set_cache_observation(&mut self, observation: TimedCacheObservation) {
        self.cache_observation = Some(observation);
    }

    /// Returns the recorded cache observation, when one was made.
    #[must_use]
    pub const fn cache_observation(&self) -> Option<&TimedCacheObservation> {
        self.cache_observation.as_ref()
    }

    /// Returns whether this investigation's evidence can be cited as live
    /// discovery evidence rather than a possibly cache-explained result.
    ///
    /// An investigation with no recorded cache observation returns `true`:
    /// the absence of cache tracking is not itself evidence of staleness. An
    /// observation whose cache was warm or stale returns `false`; empty or
    /// freshly invalidated caches return `true`.
    #[must_use]
    pub fn is_attributable_to_live_discovery(&self) -> bool {
        self.cache_observation.as_ref().is_none_or(|observation| {
            !matches!(
                observation.observation().state(),
                CacheState::Warm | CacheState::Stale
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use oo_core::{NetworkId, ProviderId};
    use oo_discovery::DiscoveryEngine;
    use oo_evidence::{EvidenceBuilder, EvidenceSourceKind};
    use oo_model::cache::CacheObservation;
    use oo_snapshot::{normalize_json, SnapshotCollector, SnapshotRequest};
    use serde_json::json;

    use super::*;

    fn record() -> InvestigationRecord {
        let plan = ObservationPlan::new(NetworkId::new(), ProviderId::new(), "eth_chainId");
        let request = SnapshotRequest::new(plan.network_id(), plan.provider_id(), plan.subject());
        let snapshot = SnapshotCollector::collect(&request, json!({"result": "0x1"}));
        let evidence_bytes = normalize_json(snapshot.payload());
        let evidence = EvidenceBuilder::new(
            EvidenceSourceKind::Snapshot,
            "snapshot:test",
            snapshot.subject(),
        )
        .bytes(evidence_bytes)
        .build();
        let outcome = DiscoveryEngine.evaluate([&evidence]);
        InvestigationRecord::new(plan, snapshot, evidence, outcome)
    }

    fn timed_observation(state: CacheState) -> TimedCacheObservation {
        TimedCacheObservation::new(
            CacheObservation::new("eth_chainId", state),
            Utc.timestamp_opt(0, 0).unwrap(),
        )
    }

    #[test]
    fn a_record_with_no_cache_observation_is_attributable_to_live_discovery() {
        assert!(record().is_attributable_to_live_discovery());
    }

    #[test]
    fn a_warm_cache_observation_makes_the_record_not_attributable() {
        let mut record = record();
        record.set_cache_observation(timed_observation(CacheState::Warm));
        assert!(!record.is_attributable_to_live_discovery());
    }

    #[test]
    fn an_empty_cache_observation_stays_attributable() {
        let mut record = record();
        record.set_cache_observation(timed_observation(CacheState::Empty));
        assert!(record.is_attributable_to_live_discovery());
    }
}
