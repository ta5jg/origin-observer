// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/request.rs
// Purpose : Snapshot request model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot request model.

use oo_core::{NetworkId, ProviderId};

/// Request to capture a reproducible observation snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotRequest {
    network_id: NetworkId,
    provider_id: ProviderId,
    subject: String,
}

impl SnapshotRequest {
    /// Creates a snapshot request.
    #[must_use]
    pub fn new(network_id: NetworkId, provider_id: ProviderId, subject: impl Into<String>) -> Self {
        Self {
            network_id,
            provider_id,
            subject: subject.into(),
        }
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
}
