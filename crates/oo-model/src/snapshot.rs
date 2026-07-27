// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/snapshot.rs
// Purpose : Immutable observation snapshot domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot domain model.
//!
//! A snapshot represents a coherent observation of blockchain state collected
//! at a single logical point in time. It groups blocks, transactions,
//! evidence and related objects without owning the underlying domain objects.

use std::collections::BTreeSet;

use oo_core::error::invalid_argument;
use oo_core::{
    BlockId, Digest, EvidenceId, NetworkId, ProviderId, Result, SnapshotId, TransactionId,
};

/// Maximum snapshot label length.
pub const MAX_SNAPSHOT_LABEL_LENGTH: usize = 128;

/// Snapshot purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum SnapshotKind {
    #[default]
    Unknown,

    Manual,

    Scheduled,

    Incremental,

    Full,

    Archive,

    Verification,
}

/// Snapshot lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum SnapshotStatus {
    Building,

    #[default]
    Complete,

    Archived,

    Invalidated,
}

/// Immutable snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    id: SnapshotId,
    network_id: NetworkId,
    provider_id: ProviderId,

    created_unix_ms: u128,

    digest: Option<Digest>,

    label: Option<String>,

    kind: SnapshotKind,

    status: SnapshotStatus,

    blocks: BTreeSet<BlockId>,

    transactions: BTreeSet<TransactionId>,

    evidence: BTreeSet<EvidenceId>,
}

impl Snapshot {
    /// Creates an empty snapshot.
    #[must_use]
    pub fn new(network_id: NetworkId, provider_id: ProviderId, created_unix_ms: u128) -> Self {
        Self {
            id: SnapshotId::new(),
            network_id,
            provider_id,
            created_unix_ms,
            digest: None,
            label: None,
            kind: SnapshotKind::Unknown,
            status: SnapshotStatus::Complete,
            blocks: BTreeSet::new(),
            transactions: BTreeSet::new(),
            evidence: BTreeSet::new(),
        }
    }

    #[must_use]
    pub const fn id(&self) -> SnapshotId {
        self.id
    }

    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    #[must_use]
    pub const fn provider_id(&self) -> ProviderId {
        self.provider_id
    }

    #[must_use]
    pub const fn created_unix_ms(&self) -> u128 {
        self.created_unix_ms
    }

    #[must_use]
    pub const fn digest(&self) -> Option<Digest> {
        self.digest
    }

    pub const fn set_digest(&mut self, digest: Digest) {
        self.digest = Some(digest);
    }

    pub const fn clear_digest(&mut self) {
        self.digest = None;
    }

    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_label(&mut self, label: impl Into<String>) -> Result<()> {
        let label = normalize(label.into())?;

        if label.len() > MAX_SNAPSHOT_LABEL_LENGTH {
            return Err(invalid_argument("snapshot label is too long"));
        }

        self.label = Some(label);

        Ok(())
    }

    pub fn clear_label(&mut self) {
        self.label = None;
    }

    #[must_use]
    pub const fn kind(&self) -> SnapshotKind {
        self.kind
    }

    pub const fn set_kind(&mut self, kind: SnapshotKind) {
        self.kind = kind;
    }

    #[must_use]
    pub const fn status(&self) -> SnapshotStatus {
        self.status
    }

    pub const fn set_status(&mut self, status: SnapshotStatus) {
        self.status = status;
    }

    pub fn add_block(&mut self, id: BlockId) -> bool {
        self.blocks.insert(id)
    }

    pub fn add_transaction(&mut self, id: TransactionId) -> bool {
        self.transactions.insert(id)
    }

    pub fn add_evidence(&mut self, id: EvidenceId) -> bool {
        self.evidence.insert(id)
    }

    pub fn remove_block(&mut self, id: BlockId) -> bool {
        self.blocks.remove(&id)
    }

    pub fn remove_transaction(&mut self, id: TransactionId) -> bool {
        self.transactions.remove(&id)
    }

    pub fn remove_evidence(&mut self, id: EvidenceId) -> bool {
        self.evidence.remove(&id)
    }

    #[must_use]
    pub fn blocks(&self) -> &BTreeSet<BlockId> {
        &self.blocks
    }

    #[must_use]
    pub fn transactions(&self) -> &BTreeSet<TransactionId> {
        &self.transactions
    }

    #[must_use]
    pub fn evidence(&self) -> &BTreeSet<EvidenceId> {
        &self.evidence
    }

    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    #[must_use]
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    #[must_use]
    pub fn evidence_count(&self) -> usize {
        self.evidence.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty() && self.transactions.is_empty() && self.evidence.is_empty()
    }
}

fn normalize(value: String) -> Result<String> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(invalid_argument("value must not be empty"));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_argument("value contains control characters"));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> Snapshot {
        Snapshot::new(NetworkId::new(), ProviderId::new(), 1000)
    }

    #[test]
    fn unique_ids() {
        let a = snapshot();
        let b = snapshot();

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn label_management() {
        let mut s = snapshot();

        s.set_label("Genesis").unwrap();

        assert_eq!(s.label(), Some("Genesis"),);

        s.clear_label();

        assert_eq!(s.label(), None);
    }

    #[test]
    fn collections_work() {
        let mut s = snapshot();

        let block = BlockId::new();
        let tx = TransactionId::new();
        let ev = EvidenceId::new();

        assert!(s.add_block(block));
        assert!(s.add_transaction(tx));
        assert!(s.add_evidence(ev));

        assert_eq!(s.block_count(), 1);
        assert_eq!(s.transaction_count(), 1);
        assert_eq!(s.evidence_count(), 1);

        assert!(s.remove_block(block));
        assert!(s.remove_transaction(tx));
        assert!(s.remove_evidence(ev));

        assert!(s.is_empty());
    }

    #[test]
    fn digest_management() {
        let mut s = snapshot();

        let digest = Digest::new([9; 32]);

        s.set_digest(digest);

        assert_eq!(s.digest(), Some(digest),);

        s.clear_digest();

        assert_eq!(s.digest(), None);
    }

    #[test]
    fn lifecycle_changes() {
        let mut s = snapshot();

        s.set_kind(SnapshotKind::Full);
        s.set_status(SnapshotStatus::Archived);

        assert_eq!(s.kind(), SnapshotKind::Full,);

        assert_eq!(s.status(), SnapshotStatus::Archived,);
    }

    #[test]
    fn empty_label_rejected() {
        let mut s = snapshot();

        assert!(s.set_label("").is_err());
    }
}
