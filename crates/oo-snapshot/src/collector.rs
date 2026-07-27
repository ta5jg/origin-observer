// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/collector.rs
// Purpose : Snapshot collector.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot collector.

use serde_json::Value;

use crate::integrity::digest_bytes;
use crate::normalization::normalize_json;
use crate::request::SnapshotRequest;
use crate::snapshot::SnapshotRecord;

/// Deterministic snapshot collector.
#[derive(Debug, Default, Clone, Copy)]
pub struct SnapshotCollector;

impl SnapshotCollector {
    /// Collects a normalized snapshot from a payload.
    #[must_use]
    pub fn collect(request: &SnapshotRequest, payload: Value) -> SnapshotRecord {
        let normalized = normalize_json(&payload);
        let digest = digest_bytes(&normalized);

        SnapshotRecord::new(
            request.network_id(),
            request.provider_id(),
            request.subject(),
            payload,
            digest,
        )
    }
}
