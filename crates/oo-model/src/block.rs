// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/block.rs
// Purpose : Blockchain block domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Blockchain block domain model.

use oo_core::{BlockId, Digest, NetworkId};

/// Confidence level assigned to a discovered block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum BlockConfidence {
    #[default]
    Unknown,
    Pending,
    Confirmed,
    Finalized,
}

/// Lifecycle state of a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum BlockStatus {
    Pending,

    #[default]
    Accepted,

    Orphaned,

    Rejected,
}

/// Generic blockchain block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    id: BlockId,
    network_id: NetworkId,

    height: u64,

    hash: Digest,

    parent_hash: Option<Digest>,

    timestamp_unix_ms: u128,

    transaction_count: u32,

    gas_used: Option<u128>,

    gas_limit: Option<u128>,

    size_bytes: Option<u64>,

    confidence: BlockConfidence,

    status: BlockStatus,
}

impl Block {
    /// Creates a new block.
    #[must_use]
    pub fn new(network_id: NetworkId, height: u64, hash: Digest, timestamp_unix_ms: u128) -> Self {
        Self {
            id: BlockId::new(),
            network_id,
            height,
            hash,
            parent_hash: None,
            timestamp_unix_ms,
            transaction_count: 0,
            gas_used: None,
            gas_limit: None,
            size_bytes: None,
            confidence: BlockConfidence::Unknown,
            status: BlockStatus::Accepted,
        }
    }

    #[must_use]
    pub const fn id(&self) -> BlockId {
        self.id
    }

    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    #[must_use]
    pub const fn height(&self) -> u64 {
        self.height
    }

    pub const fn set_height(&mut self, height: u64) {
        self.height = height;
    }

    #[must_use]
    pub const fn hash(&self) -> Digest {
        self.hash
    }

    pub const fn set_hash(&mut self, hash: Digest) {
        self.hash = hash;
    }

    #[must_use]
    pub const fn parent_hash(&self) -> Option<Digest> {
        self.parent_hash
    }

    pub const fn set_parent_hash(&mut self, hash: Digest) {
        self.parent_hash = Some(hash);
    }

    pub const fn clear_parent_hash(&mut self) {
        self.parent_hash = None;
    }

    #[must_use]
    pub const fn timestamp_unix_ms(&self) -> u128 {
        self.timestamp_unix_ms
    }

    pub const fn set_timestamp_unix_ms(&mut self, value: u128) {
        self.timestamp_unix_ms = value;
    }

    #[must_use]
    pub const fn transaction_count(&self) -> u32 {
        self.transaction_count
    }

    pub const fn set_transaction_count(&mut self, count: u32) {
        self.transaction_count = count;
    }

    #[must_use]
    pub const fn gas_used(&self) -> Option<u128> {
        self.gas_used
    }

    pub const fn set_gas_used(&mut self, gas: u128) {
        self.gas_used = Some(gas);
    }

    pub const fn clear_gas_used(&mut self) {
        self.gas_used = None;
    }

    #[must_use]
    pub const fn gas_limit(&self) -> Option<u128> {
        self.gas_limit
    }

    pub const fn set_gas_limit(&mut self, gas: u128) {
        self.gas_limit = Some(gas);
    }

    pub const fn clear_gas_limit(&mut self) {
        self.gas_limit = None;
    }

    #[must_use]
    pub const fn size_bytes(&self) -> Option<u64> {
        self.size_bytes
    }

    pub const fn set_size_bytes(&mut self, size: u64) {
        self.size_bytes = Some(size);
    }

    pub const fn clear_size_bytes(&mut self) {
        self.size_bytes = None;
    }

    #[must_use]
    pub const fn confidence(&self) -> BlockConfidence {
        self.confidence
    }

    pub const fn set_confidence(&mut self, confidence: BlockConfidence) {
        self.confidence = confidence;
    }

    #[must_use]
    pub const fn status(&self) -> BlockStatus {
        self.status
    }

    pub const fn set_status(&mut self, status: BlockStatus) {
        self.status = status;
    }

    #[must_use]
    pub const fn is_finalized(&self) -> bool {
        matches!(self.confidence, BlockConfidence::Finalized)
    }

    #[must_use]
    pub const fn is_orphaned(&self) -> bool {
        matches!(self.status, BlockStatus::Orphaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(byte: u8) -> Digest {
        Digest::new([byte; 32])
    }

    #[test]
    fn unique_identifiers() {
        let network = NetworkId::new();

        let a = Block::new(network, 1, digest(1), 1000);
        let b = Block::new(network, 1, digest(1), 1000);

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn parent_hash_management() {
        let mut block = Block::new(NetworkId::new(), 100, digest(1), 1000);

        assert_eq!(block.parent_hash(), None);

        block.set_parent_hash(digest(2));

        assert_eq!(block.parent_hash(), Some(digest(2)));

        block.clear_parent_hash();

        assert_eq!(block.parent_hash(), None);
    }

    #[test]
    fn gas_information() {
        let mut block = Block::new(NetworkId::new(), 100, digest(1), 1000);

        block.set_gas_used(21_000);
        block.set_gas_limit(30_000_000);

        assert_eq!(block.gas_used(), Some(21_000));
        assert_eq!(block.gas_limit(), Some(30_000_000));
    }

    #[test]
    fn finalized_block() {
        let mut block = Block::new(NetworkId::new(), 1, digest(1), 1000);

        assert!(!block.is_finalized());

        block.set_confidence(BlockConfidence::Finalized);

        assert!(block.is_finalized());
    }

    #[test]
    fn orphan_detection() {
        let mut block = Block::new(NetworkId::new(), 1, digest(1), 1000);

        assert!(!block.is_orphaned());

        block.set_status(BlockStatus::Orphaned);

        assert!(block.is_orphaned());
    }

    #[test]
    fn transaction_count_changes() {
        let mut block = Block::new(NetworkId::new(), 15, digest(9), 5000);

        block.set_transaction_count(245);

        assert_eq!(block.transaction_count(), 245);
    }
}
