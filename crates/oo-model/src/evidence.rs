// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/evidence.rs
// Purpose : Evidence domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence domain model.
//!
//! Evidence represents immutable observations collected from blockchain
//! providers. Every observation should be traceable to its source and
//! verifiable through its digest.

use std::collections::BTreeMap;

use oo_core::error::invalid_argument;
use oo_core::{
    AddressId, BlockId, ContractId, Digest, EvidenceId, NetworkId, ProviderId, Result,
    TransactionId,
};

/// Maximum metadata entries stored in a single evidence object.
pub const MAX_METADATA_ENTRIES: usize = 128;

/// Evidence category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EvidenceKind {
    #[default]
    Unknown,

    Block,

    Transaction,

    Address,

    Contract,

    Asset,

    Event,

    Receipt,

    Log,

    Balance,

    Metadata,

    Snapshot,

    Provider,
}

/// Confidence assigned to evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EvidenceConfidence {
    #[default]
    Unknown,

    Low,

    Medium,

    High,

    Verified,
}

/// Evidence lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EvidenceStatus {
    #[default]
    Active,

    Archived,

    Invalidated,
}

/// Immutable observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    id: EvidenceId,

    network_id: NetworkId,

    provider_id: ProviderId,

    digest: Digest,

    timestamp_unix_ms: u128,

    kind: EvidenceKind,

    confidence: EvidenceConfidence,

    status: EvidenceStatus,

    summary: String,

    block_id: Option<BlockId>,

    transaction_id: Option<TransactionId>,

    address_id: Option<AddressId>,

    contract_id: Option<ContractId>,

    metadata: BTreeMap<String, String>,
}

impl Evidence {
    /// Creates new evidence.
    pub fn new(
        network_id: NetworkId,
        provider_id: ProviderId,
        digest: Digest,
        timestamp_unix_ms: u128,
        summary: impl Into<String>,
    ) -> Result<Self> {
        let summary = normalize(summary.into())?;

        Ok(Self {
            id: EvidenceId::new(),
            network_id,
            provider_id,
            digest,
            timestamp_unix_ms,
            kind: EvidenceKind::Unknown,
            confidence: EvidenceConfidence::Unknown,
            status: EvidenceStatus::Active,
            summary,
            block_id: None,
            transaction_id: None,
            address_id: None,
            contract_id: None,
            metadata: BTreeMap::new(),
        })
    }

    #[must_use]
    pub const fn id(&self) -> EvidenceId {
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
    pub const fn digest(&self) -> Digest {
        self.digest
    }

    #[must_use]
    pub const fn timestamp_unix_ms(&self) -> u128 {
        self.timestamp_unix_ms
    }

    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn set_summary(&mut self, summary: impl Into<String>) -> Result<()> {
        self.summary = normalize(summary.into())?;
        Ok(())
    }

    #[must_use]
    pub const fn kind(&self) -> EvidenceKind {
        self.kind
    }

    pub const fn set_kind(&mut self, kind: EvidenceKind) {
        self.kind = kind;
    }

    #[must_use]
    pub const fn confidence(&self) -> EvidenceConfidence {
        self.confidence
    }

    pub const fn set_confidence(&mut self, confidence: EvidenceConfidence) {
        self.confidence = confidence;
    }

    #[must_use]
    pub const fn status(&self) -> EvidenceStatus {
        self.status
    }

    pub const fn archive(&mut self) {
        self.status = EvidenceStatus::Archived;
    }

    pub const fn invalidate(&mut self) {
        self.status = EvidenceStatus::Invalidated;
    }

    pub const fn activate(&mut self) {
        self.status = EvidenceStatus::Active;
    }

    #[must_use]
    pub const fn block_id(&self) -> Option<BlockId> {
        self.block_id
    }

    pub const fn set_block_id(&mut self, id: BlockId) {
        self.block_id = Some(id);
    }

    #[must_use]
    pub const fn transaction_id(&self) -> Option<TransactionId> {
        self.transaction_id
    }

    pub const fn set_transaction_id(&mut self, id: TransactionId) {
        self.transaction_id = Some(id);
    }

    #[must_use]
    pub const fn address_id(&self) -> Option<AddressId> {
        self.address_id
    }

    pub const fn set_address_id(&mut self, id: AddressId) {
        self.address_id = Some(id);
    }

    #[must_use]
    pub const fn contract_id(&self) -> Option<ContractId> {
        self.contract_id
    }

    pub const fn set_contract_id(&mut self, id: ContractId) {
        self.contract_id = Some(id);
    }

    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<()> {
        if self.metadata.len() >= MAX_METADATA_ENTRIES {
            return Err(invalid_argument("metadata capacity exceeded"));
        }

        let key = normalize(key.into())?;
        let value = normalize(value.into())?;

        self.metadata.insert(key, value);

        Ok(())
    }

    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        self.metadata.remove(key)
    }

    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    #[must_use]
    pub fn metadata_value(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
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

    fn digest() -> Digest {
        Digest::new([7; 32])
    }

    fn evidence() -> Evidence {
        Evidence::new(
            NetworkId::new(),
            ProviderId::new(),
            digest(),
            1000,
            "Observed transfer",
        )
        .unwrap()
    }

    #[test]
    fn create_evidence() {
        let evidence = evidence();

        assert_eq!(evidence.summary(), "Observed transfer");

        assert_eq!(evidence.kind(), EvidenceKind::Unknown);
    }

    #[test]
    fn unique_identifier() {
        let a = evidence();
        let b = evidence();

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn summary_changes() {
        let mut evidence = evidence();

        evidence.set_summary("Updated").unwrap();

        assert_eq!(evidence.summary(), "Updated");
    }

    #[test]
    fn lifecycle_changes() {
        let mut evidence = evidence();

        evidence.archive();

        assert_eq!(evidence.status(), EvidenceStatus::Archived);

        evidence.invalidate();

        assert_eq!(evidence.status(), EvidenceStatus::Invalidated);

        evidence.activate();

        assert_eq!(evidence.status(), EvidenceStatus::Active);
    }

    #[test]
    fn metadata_management() {
        let mut evidence = evidence();

        evidence.insert_metadata("rpc", "alchemy").unwrap();

        assert_eq!(evidence.metadata_value("rpc"), Some("alchemy"));

        evidence.remove_metadata("rpc");

        assert_eq!(evidence.metadata_value("rpc"), None);
    }

    #[test]
    fn relationships() {
        let mut evidence = evidence();

        let block = BlockId::new();
        let tx = TransactionId::new();
        let address = AddressId::new();
        let contract = ContractId::new();

        evidence.set_block_id(block);
        evidence.set_transaction_id(tx);
        evidence.set_address_id(address);
        evidence.set_contract_id(contract);

        assert_eq!(evidence.block_id(), Some(block));

        assert_eq!(evidence.transaction_id(), Some(tx));

        assert_eq!(evidence.address_id(), Some(address));

        assert_eq!(evidence.contract_id(), Some(contract));
    }

    #[test]
    fn empty_summary_rejected() {
        assert!(Evidence::new(NetworkId::new(), ProviderId::new(), digest(), 0, "").is_err());
    }
}
