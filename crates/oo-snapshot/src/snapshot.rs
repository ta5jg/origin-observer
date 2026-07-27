// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/snapshot.rs
// Purpose : Snapshot record model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot record model.

use oo_core::{Digest, NetworkId, ProviderId, SnapshotId};
use serde_json::Value;

/// Integrity-protected normalized snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotRecord {
    id: SnapshotId,
    network_id: NetworkId,
    provider_id: ProviderId,
    subject: String,
    payload: Value,
    digest: Digest,
}

impl SnapshotRecord {
    /// Creates a snapshot record.
    #[must_use]
    pub fn new(
        network_id: NetworkId,
        provider_id: ProviderId,
        subject: impl Into<String>,
        payload: Value,
        digest: Digest,
    ) -> Self {
        Self {
            id: SnapshotId::new(),
            network_id,
            provider_id,
            subject: subject.into(),
            payload,
            digest,
        }
    }

    /// Returns the snapshot identifier.
    #[must_use]
    pub const fn id(&self) -> SnapshotId {
        self.id
    }

    /// Returns the network identifier.
    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    /// Returns the provider identifier.
    #[must_use]
    pub const fn provider_id(&self) -> ProviderId {
        self.provider_id
    }

    /// Returns the snapshot subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the normalized payload.
    #[must_use]
    pub const fn payload(&self) -> &Value {
        &self.payload
    }

    /// Returns the integrity digest.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.digest
    }
}
