// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/manifest.rs
// Purpose : Snapshot manifest.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot manifest.

use oo_core::{Digest, SnapshotId};

/// Manifest entry for one snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotManifestEntry {
    snapshot_id: SnapshotId,
    digest: Digest,
}

impl SnapshotManifestEntry {
    /// Creates a manifest entry.
    #[must_use]
    pub const fn new(snapshot_id: SnapshotId, digest: Digest) -> Self {
        Self {
            snapshot_id,
            digest,
        }
    }

    /// Returns the snapshot identifier.
    #[must_use]
    pub const fn snapshot_id(&self) -> SnapshotId {
        self.snapshot_id
    }

    /// Returns the snapshot digest.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.digest
    }
}

/// Snapshot manifest.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SnapshotManifest {
    entries: Vec<SnapshotManifestEntry>,
}

impl SnapshotManifest {
    /// Adds a manifest entry.
    pub fn push(&mut self, entry: SnapshotManifestEntry) {
        self.entries.push(entry);
    }

    /// Returns all manifest entries.
    #[must_use]
    pub fn entries(&self) -> &[SnapshotManifestEntry] {
        &self.entries
    }
}
