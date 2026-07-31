// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/history.rs
// Purpose : Record an investigation into a wallet-recognition case study.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Record an investigation into a wallet-recognition case study.
//!
//! A single investigation is a point observation; a case study is what turns
//! a sequence of them into history. This module is the bridge: given an
//! [`InvestigationRecord`] and the wallet it was made on behalf of, it
//! appends a [`RecognitionEvent`] whose source names the investigation's own
//! evidence digest, so the case study entry can always be traced back to the
//! run that produced it.

use chrono::{DateTime, Utc};
use oo_core::WalletId;
use oo_discovery::DiscoveryDecision;
use oo_history::{AssetCaseStudy, HistoricalSource, RecognitionEvent, TimelineEntry};

use crate::investigation::InvestigationRecord;

/// Appends a recognition event derived from an investigation to a case
/// study's recognition timeline.
///
/// Recognition is read from the investigation's discovery decision:
/// [`DiscoveryDecision::Accept`] recognizes the asset, anything else does
/// not. `observed_at` is caller-supplied rather than taken from a clock
/// internally, matching this workspace's determinism convention.
pub fn record_recognition(
    case_study: &mut AssetCaseStudy,
    wallet_id: WalletId,
    record: &InvestigationRecord,
    observed_at: DateTime<Utc>,
) {
    let recognized = matches!(record.outcome().decision(), DiscoveryDecision::Accept);
    let source = HistoricalSource::new(format!(
        "investigation evidence digest {}",
        record.evidence().digest().to_hex()
    ));
    case_study
        .recognition_timeline_mut()
        .push(TimelineEntry::new(
            observed_at,
            RecognitionEvent::new(wallet_id, recognized, source),
        ));
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use oo_core::{NetworkId, ProviderId};
    use oo_discovery::DiscoveryEngine;
    use oo_evidence::{EvidenceBuilder, EvidenceSourceKind};
    use oo_snapshot::{normalize_json, SnapshotCollector, SnapshotRequest};
    use serde_json::json;

    use super::*;
    use crate::plan::ObservationPlan;

    fn accepted_record() -> InvestigationRecord {
        let plan = ObservationPlan::new(NetworkId::new(), ProviderId::new(), "eth_chainId");
        let request = SnapshotRequest::new(plan.network_id(), plan.provider_id(), plan.subject());
        let snapshot = SnapshotCollector::collect(&request, json!({"result": "0x1"}));
        let evidence_bytes = normalize_json(snapshot.payload());
        let mut evidence = EvidenceBuilder::new(
            EvidenceSourceKind::Snapshot,
            "snapshot:test",
            snapshot.subject(),
        )
        .bytes(evidence_bytes)
        .build();
        evidence.set_reproduction(oo_evidence::ReproductionStatus::IndependentlyVerified);
        let outcome = DiscoveryEngine.evaluate([&evidence]);
        InvestigationRecord::new(plan, snapshot, evidence, outcome)
    }

    #[test]
    fn recording_appends_one_chronological_entry_naming_the_evidence_digest() {
        let mut case_study = AssetCaseStudy::new("RQ-0006");
        let record = accepted_record();
        let digest = record.evidence().digest().to_hex();

        record_recognition(
            &mut case_study,
            WalletId::new(),
            &record,
            Utc.timestamp_opt(1_000, 0).unwrap(),
        );

        assert_eq!(case_study.recognition_timeline().entries().len(), 1);
        let entry = &case_study.recognition_timeline().entries()[0];
        assert!(entry.detail().source().description().contains(&digest));
    }

    #[test]
    fn recognition_reflects_the_investigations_own_decision() {
        let mut case_study = AssetCaseStudy::new("RQ-0006");
        let record = accepted_record();
        let recognized = matches!(record.outcome().decision(), DiscoveryDecision::Accept);

        record_recognition(
            &mut case_study,
            WalletId::new(),
            &record,
            Utc.timestamp_opt(1_000, 0).unwrap(),
        );

        let entry = &case_study.recognition_timeline().entries()[0];
        assert_eq!(entry.detail().recognized(), recognized);
    }
}
