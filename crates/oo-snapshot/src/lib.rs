// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/lib.rs
// Purpose : Collect normalized and integrity-protected state snapshots.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Collect normalized and integrity-protected state snapshots.

pub mod collector;
pub mod integrity;
pub mod manifest;
pub mod normalization;
pub mod request;
pub mod snapshot;
pub mod validation;

pub use collector::SnapshotCollector;
pub use integrity::digest_bytes;
pub use manifest::{SnapshotManifest, SnapshotManifestEntry};
pub use normalization::normalize_json;
pub use request::SnapshotRequest;
pub use snapshot::SnapshotRecord;
pub use validation::validate_snapshot;

#[cfg(test)]
mod tests {
    use oo_core::{NetworkId, ProviderId};
    use serde_json::json;

    use super::*;

    #[test]
    fn collects_integrity_protected_snapshot() {
        let request = SnapshotRequest::new(NetworkId::new(), ProviderId::new(), "eth_chainId");
        let snapshot = SnapshotCollector::collect(&request, json!({"result": "0x1"}));

        assert_eq!(snapshot.subject(), "eth_chainId");
        assert!(validate_snapshot(&snapshot));
    }
}
