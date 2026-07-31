// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/dataset.rs
// Purpose : Export a batch of investigations as a reproducible dataset.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Export a batch of investigations as a reproducible dataset.
//!
//! A single investigation's report is enough to answer one question; a
//! dataset of them is what supports research across many. This module
//! flattens each [`InvestigationRecord`] into the row shape
//! [`oo_dataset`] needs and builds the manifest that names the schema,
//! version, record count and content digest together, so the export can
//! later be checked against its own manifest with
//! `oo_dataset::validate_records`.

use oo_dataset::{
    compute_digest, DatasetField, DatasetManifest, DatasetSchema, DatasetVersion, FieldType,
};
use serde::Serialize;

use crate::investigation::InvestigationRecord;

/// One investigation flattened into a dataset row.
#[derive(Debug, Clone, Serialize)]
pub struct InvestigationRow {
    subject: String,
    decision: String,
    score: f64,
    evidence_digest: String,
    live_discovery_evidence: bool,
}

impl InvestigationRow {
    fn from_record(record: &InvestigationRecord) -> Self {
        Self {
            subject: record.plan().subject().to_owned(),
            decision: format!("{:?}", record.outcome().decision()),
            score: record.outcome().score().value(),
            evidence_digest: record.evidence().digest().to_hex(),
            live_discovery_evidence: record.is_attributable_to_live_discovery(),
        }
    }
}

/// Returns the dataset schema every export from this module declares.
#[must_use]
pub fn schema() -> DatasetSchema {
    DatasetSchema::new(vec![
        DatasetField::new("subject", FieldType::Text),
        DatasetField::new("decision", FieldType::Text),
        DatasetField::new("score", FieldType::Float),
        DatasetField::new("evidence_digest", FieldType::Text),
        DatasetField::new("live_discovery_evidence", FieldType::Boolean),
    ])
}

/// Flattens a batch of investigations into dataset rows and their manifest.
///
/// # Errors
///
/// Returns [`oo_dataset::IntegrityError`] if a row somehow fails to
/// serialize; every field here is a primitive, so this is not expected in
/// practice, but is surfaced explicitly rather than assumed impossible.
pub fn export(
    name: impl Into<String>,
    version: DatasetVersion,
    records: &[InvestigationRecord],
) -> Result<(Vec<InvestigationRow>, DatasetManifest), oo_dataset::IntegrityError> {
    let rows: Vec<InvestigationRow> = records.iter().map(InvestigationRow::from_record).collect();
    let digest = compute_digest(&rows)?;
    let manifest = DatasetManifest::new(name, schema(), version, rows.len(), digest);
    Ok((rows, manifest))
}

#[cfg(test)]
mod tests {
    use oo_core::{NetworkId, ProviderId};
    use oo_dataset::validate_records;
    use oo_discovery::DiscoveryEngine;
    use oo_evidence::{EvidenceBuilder, EvidenceSourceKind};
    use oo_snapshot::{normalize_json, SnapshotCollector, SnapshotRequest};
    use serde_json::json;

    use super::*;
    use crate::plan::ObservationPlan;

    fn record(subject: &str) -> InvestigationRecord {
        let plan = ObservationPlan::new(NetworkId::new(), ProviderId::new(), subject);
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

    #[test]
    fn an_export_manifest_validates_against_its_own_rows() {
        let records = vec![record("eth_chainId"), record("eth_getBalance")];
        let (rows, manifest) =
            export("investigations", DatasetVersion::new(1, 0), &records).unwrap();
        assert_eq!(manifest.record_count(), 2);
        assert!(validate_records(&manifest, &rows).is_ok());
    }

    #[test]
    fn an_empty_batch_still_produces_a_valid_manifest() {
        let (rows, manifest) = export("investigations", DatasetVersion::new(1, 0), &[]).unwrap();
        assert_eq!(manifest.record_count(), 0);
        assert!(validate_records(&manifest, &rows).is_ok());
    }

    #[test]
    fn each_row_carries_the_investigations_evidence_digest() {
        let records = vec![record("eth_chainId")];
        let (rows, _) = export("investigations", DatasetVersion::new(1, 0), &records).unwrap();
        assert_eq!(
            rows[0].evidence_digest,
            records[0].evidence().digest().to_hex()
        );
    }
}
