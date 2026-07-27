// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/transaction.rs
// Purpose : Blockchain transaction domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Blockchain transaction domain model.
//!
//! A [`Transaction`] represents a chain transaction independently of any
//! specific protocol. Chain-specific crates may attach richer call, receipt,
//! event and execution information while this model preserves the shared
//! transaction identity and lifecycle.

use std::collections::BTreeSet;

use oo_core::error::invalid_argument;
use oo_core::{AddressId, AssetId, BlockId, Digest, NetworkId, Result, TransactionId};

/// Maximum accepted transaction input length in bytes.
pub const MAX_TRANSACTION_INPUT_LENGTH: usize = 1_048_576;

/// General transaction classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TransactionKind {
    /// Native currency transfer.
    NativeTransfer,

    /// Fungible-token transfer.
    TokenTransfer,

    /// Non-fungible-token transfer.
    NonFungibleTransfer,

    /// Smart-contract deployment.
    ContractDeployment,

    /// Smart-contract interaction.
    ContractCall,

    /// Asset approval or allowance operation.
    Approval,

    /// Asset mint operation.
    Mint,

    /// Asset burn operation.
    Burn,

    /// Swap or exchange operation.
    Swap,

    /// Liquidity addition or removal.
    Liquidity,

    /// Bridge operation.
    Bridge,

    /// Staking operation.
    Staking,

    /// Governance operation.
    Governance,

    /// Validator or consensus operation.
    Consensus,

    /// Transaction classification has not yet been determined.
    #[default]
    Unknown,
}

/// Lifecycle state of a transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TransactionStatus {
    /// Transaction was observed before inclusion in a block.
    Pending,

    /// Transaction was included but final outcome is not yet known.
    Included,

    /// Transaction execution succeeded.
    Succeeded,

    /// Transaction execution failed.
    Failed,

    /// Transaction was replaced by another transaction.
    Replaced,

    /// Transaction was dropped before inclusion.
    Dropped,

    /// Transaction became invalid because of a chain reorganization.
    Orphaned,

    /// Status has not yet been determined.
    #[default]
    Unknown,
}

/// Confidence assigned to transaction finality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TransactionConfidence {
    /// Confidence has not yet been determined.
    #[default]
    Unknown,

    /// Transaction is known only from a pending pool.
    Pending,

    /// Transaction has at least one confirmation.
    Confirmed,

    /// Transaction is considered final by the network or observer policy.
    Finalized,
}

/// Transaction fee representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionFee {
    asset_id: Option<AssetId>,
    amount: String,
}

impl TransactionFee {
    /// Creates a transaction fee.
    pub fn new(asset_id: Option<AssetId>, amount: impl Into<String>) -> Result<Self> {
        Ok(Self {
            asset_id,
            amount: normalize_unsigned_integer(amount.into(), "transaction fee")?,
        })
    }

    /// Returns the fee asset when known.
    #[must_use]
    pub const fn asset_id(&self) -> Option<AssetId> {
        self.asset_id
    }

    /// Returns the exact base-unit fee amount.
    #[must_use]
    pub fn amount(&self) -> &str {
        &self.amount
    }

    /// Changes the fee asset.
    pub const fn set_asset_id(&mut self, asset_id: AssetId) {
        self.asset_id = Some(asset_id);
    }

    /// Removes the fee asset association.
    pub const fn clear_asset_id(&mut self) {
        self.asset_id = None;
    }

    /// Changes the exact base-unit fee amount.
    pub fn set_amount(&mut self, amount: impl Into<String>) -> Result<()> {
        self.amount = normalize_unsigned_integer(amount.into(), "transaction fee")?;

        Ok(())
    }
}

/// Generic blockchain transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    id: TransactionId,
    network_id: NetworkId,
    hash: Digest,
    block_id: Option<BlockId>,
    block_height: Option<u64>,
    transaction_index: Option<u32>,
    from: Option<AddressId>,
    to: Option<AddressId>,
    created_contract: Option<AddressId>,
    asset_id: Option<AssetId>,
    amount: Option<String>,
    nonce: Option<u128>,
    timestamp_unix_ms: Option<u128>,
    kind: TransactionKind,
    status: TransactionStatus,
    confidence: TransactionConfidence,
    fee: Option<TransactionFee>,
    gas_limit: Option<u128>,
    gas_used: Option<u128>,
    effective_gas_price: Option<String>,
    input: Option<Vec<u8>>,
    method_selector: Option<[u8; 4]>,
    replacement: Option<TransactionId>,
    tags: BTreeSet<String>,
}

impl Transaction {
    /// Creates a transaction with its network and chain hash.
    #[must_use]
    pub fn new(network_id: NetworkId, hash: Digest) -> Self {
        Self {
            id: TransactionId::new(),
            network_id,
            hash,
            block_id: None,
            block_height: None,
            transaction_index: None,
            from: None,
            to: None,
            created_contract: None,
            asset_id: None,
            amount: None,
            nonce: None,
            timestamp_unix_ms: None,
            kind: TransactionKind::Unknown,
            status: TransactionStatus::Unknown,
            confidence: TransactionConfidence::Unknown,
            fee: None,
            gas_limit: None,
            gas_used: None,
            effective_gas_price: None,
            input: None,
            method_selector: None,
            replacement: None,
            tags: BTreeSet::new(),
        }
    }

    /// Returns the internal transaction identifier.
    #[must_use]
    pub const fn id(&self) -> TransactionId {
        self.id
    }

    /// Returns the network identifier.
    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    /// Returns the chain transaction hash.
    #[must_use]
    pub const fn hash(&self) -> Digest {
        self.hash
    }

    /// Changes the chain transaction hash.
    pub const fn set_hash(&mut self, hash: Digest) {
        self.hash = hash;
    }

    /// Returns the containing block identifier when known.
    #[must_use]
    pub const fn block_id(&self) -> Option<BlockId> {
        self.block_id
    }

    /// Associates the transaction with a block.
    pub const fn set_block_id(&mut self, block_id: BlockId) {
        self.block_id = Some(block_id);
    }

    /// Removes the block association.
    pub const fn clear_block_id(&mut self) {
        self.block_id = None;
    }

    /// Returns the containing block height when known.
    #[must_use]
    pub const fn block_height(&self) -> Option<u64> {
        self.block_height
    }

    /// Sets the containing block height.
    pub const fn set_block_height(&mut self, block_height: u64) {
        self.block_height = Some(block_height);
    }

    /// Removes the block-height association.
    pub const fn clear_block_height(&mut self) {
        self.block_height = None;
    }

    /// Returns the transaction index inside its block.
    #[must_use]
    pub const fn transaction_index(&self) -> Option<u32> {
        self.transaction_index
    }

    /// Sets the transaction index inside its block.
    pub const fn set_transaction_index(&mut self, transaction_index: u32) {
        self.transaction_index = Some(transaction_index);
    }

    /// Removes the transaction index.
    pub const fn clear_transaction_index(&mut self) {
        self.transaction_index = None;
    }

    /// Returns the sender address.
    #[must_use]
    pub const fn from(&self) -> Option<AddressId> {
        self.from
    }

    /// Assigns the sender address.
    pub const fn set_from(&mut self, address_id: AddressId) {
        self.from = Some(address_id);
    }

    /// Removes the sender address.
    pub const fn clear_from(&mut self) {
        self.from = None;
    }

    /// Returns the recipient address.
    #[must_use]
    pub const fn to(&self) -> Option<AddressId> {
        self.to
    }

    /// Assigns the recipient address.
    pub const fn set_to(&mut self, address_id: AddressId) {
        self.to = Some(address_id);
    }

    /// Removes the recipient address.
    pub const fn clear_to(&mut self) {
        self.to = None;
    }

    /// Returns the address created by a deployment transaction.
    #[must_use]
    pub const fn created_contract(&self) -> Option<AddressId> {
        self.created_contract
    }

    /// Assigns the created contract address.
    pub const fn set_created_contract(&mut self, address_id: AddressId) {
        self.created_contract = Some(address_id);
    }

    /// Removes the created-contract association.
    pub const fn clear_created_contract(&mut self) {
        self.created_contract = None;
    }

    /// Returns the transferred or affected asset.
    #[must_use]
    pub const fn asset_id(&self) -> Option<AssetId> {
        self.asset_id
    }

    /// Assigns the affected asset.
    pub const fn set_asset_id(&mut self, asset_id: AssetId) {
        self.asset_id = Some(asset_id);
    }

    /// Removes the asset association.
    pub const fn clear_asset_id(&mut self) {
        self.asset_id = None;
    }

    /// Returns the exact base-unit transaction amount.
    #[must_use]
    pub fn amount(&self) -> Option<&str> {
        self.amount.as_deref()
    }

    /// Assigns the exact base-unit transaction amount.
    pub fn set_amount(&mut self, amount: impl Into<String>) -> Result<()> {
        self.amount = Some(normalize_unsigned_integer(
            amount.into(),
            "transaction amount",
        )?);

        Ok(())
    }

    /// Removes the transaction amount.
    pub fn clear_amount(&mut self) {
        self.amount = None;
    }

    /// Returns the transaction nonce.
    #[must_use]
    pub const fn nonce(&self) -> Option<u128> {
        self.nonce
    }

    /// Assigns the transaction nonce.
    pub const fn set_nonce(&mut self, nonce: u128) {
        self.nonce = Some(nonce);
    }

    /// Removes the transaction nonce.
    pub const fn clear_nonce(&mut self) {
        self.nonce = None;
    }

    /// Returns the transaction timestamp in Unix milliseconds.
    #[must_use]
    pub const fn timestamp_unix_ms(&self) -> Option<u128> {
        self.timestamp_unix_ms
    }

    /// Assigns the transaction timestamp in Unix milliseconds.
    pub const fn set_timestamp_unix_ms(&mut self, timestamp_unix_ms: u128) {
        self.timestamp_unix_ms = Some(timestamp_unix_ms);
    }

    /// Removes the transaction timestamp.
    pub const fn clear_timestamp_unix_ms(&mut self) {
        self.timestamp_unix_ms = None;
    }

    /// Returns the transaction classification.
    #[must_use]
    pub const fn kind(&self) -> TransactionKind {
        self.kind
    }

    /// Changes the transaction classification.
    pub const fn set_kind(&mut self, kind: TransactionKind) {
        self.kind = kind;
    }

    /// Returns the transaction lifecycle status.
    #[must_use]
    pub const fn status(&self) -> TransactionStatus {
        self.status
    }

    /// Changes the transaction lifecycle status.
    pub const fn set_status(&mut self, status: TransactionStatus) {
        self.status = status;
    }

    /// Returns the transaction confidence.
    #[must_use]
    pub const fn confidence(&self) -> TransactionConfidence {
        self.confidence
    }

    /// Changes the transaction confidence.
    pub const fn set_confidence(&mut self, confidence: TransactionConfidence) {
        self.confidence = confidence;
    }

    /// Returns the transaction fee.
    #[must_use]
    pub const fn fee(&self) -> Option<&TransactionFee> {
        self.fee.as_ref()
    }

    /// Assigns the transaction fee.
    pub fn set_fee(&mut self, fee: TransactionFee) {
        self.fee = Some(fee);
    }

    /// Removes the transaction fee.
    pub fn clear_fee(&mut self) {
        self.fee = None;
    }

    /// Returns the gas or execution-unit limit.
    #[must_use]
    pub const fn gas_limit(&self) -> Option<u128> {
        self.gas_limit
    }

    /// Assigns the gas or execution-unit limit.
    pub const fn set_gas_limit(&mut self, gas_limit: u128) {
        self.gas_limit = Some(gas_limit);
    }

    /// Removes the gas limit.
    pub const fn clear_gas_limit(&mut self) {
        self.gas_limit = None;
    }

    /// Returns consumed gas or execution units.
    #[must_use]
    pub const fn gas_used(&self) -> Option<u128> {
        self.gas_used
    }

    /// Assigns consumed gas or execution units.
    pub const fn set_gas_used(&mut self, gas_used: u128) {
        self.gas_used = Some(gas_used);
    }

    /// Removes consumed gas information.
    pub const fn clear_gas_used(&mut self) {
        self.gas_used = None;
    }

    /// Returns the exact effective gas price.
    #[must_use]
    pub fn effective_gas_price(&self) -> Option<&str> {
        self.effective_gas_price.as_deref()
    }

    /// Assigns the exact effective gas price.
    pub fn set_effective_gas_price(&mut self, value: impl Into<String>) -> Result<()> {
        self.effective_gas_price = Some(normalize_unsigned_integer(
            value.into(),
            "effective gas price",
        )?);

        Ok(())
    }

    /// Removes the effective gas price.
    pub fn clear_effective_gas_price(&mut self) {
        self.effective_gas_price = None;
    }

    /// Returns the raw transaction input.
    #[must_use]
    pub fn input(&self) -> Option<&[u8]> {
        self.input.as_deref()
    }

    /// Assigns the raw transaction input.
    pub fn set_input(&mut self, input: impl Into<Vec<u8>>) -> Result<()> {
        let input = input.into();

        if input.len() > MAX_TRANSACTION_INPUT_LENGTH {
            return Err(invalid_argument(format!(
                "transaction input must not exceed \
                 {MAX_TRANSACTION_INPUT_LENGTH} bytes"
            )));
        }

        self.method_selector = input.get(..4).and_then(|value| value.try_into().ok());

        self.input = Some(input);

        Ok(())
    }

    /// Removes the raw transaction input and method selector.
    pub fn clear_input(&mut self) {
        self.input = None;
        self.method_selector = None;
    }

    /// Returns the first four transaction-input bytes when available.
    #[must_use]
    pub const fn method_selector(&self) -> Option<[u8; 4]> {
        self.method_selector
    }

    /// Returns the replacement transaction.
    #[must_use]
    pub const fn replacement(&self) -> Option<TransactionId> {
        self.replacement
    }

    /// Marks this transaction as replaced by another transaction.
    pub fn mark_replaced_by(&mut self, transaction_id: TransactionId) -> Result<()> {
        if transaction_id == self.id {
            return Err(invalid_argument("transaction cannot replace itself"));
        }

        self.replacement = Some(transaction_id);
        self.status = TransactionStatus::Replaced;

        Ok(())
    }

    /// Removes replacement information.
    pub const fn clear_replacement(&mut self) {
        self.replacement = None;
    }

    /// Adds a normalized transaction tag.
    pub fn add_tag(&mut self, tag: impl Into<String>) -> Result<bool> {
        let tag = normalize_required_text(tag.into(), "transaction tag")?;

        Ok(self.tags.insert(tag))
    }

    /// Removes a transaction tag.
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    /// Returns whether the transaction has a tag.
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    /// Returns all transaction tags.
    #[must_use]
    pub fn tags(&self) -> &BTreeSet<String> {
        &self.tags
    }

    /// Returns whether transaction execution succeeded.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        matches!(self.status, TransactionStatus::Succeeded)
    }

    /// Returns whether transaction execution failed.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        matches!(self.status, TransactionStatus::Failed)
    }

    /// Returns whether the transaction is pending.
    #[must_use]
    pub const fn is_pending(&self) -> bool {
        matches!(self.status, TransactionStatus::Pending)
            || matches!(self.confidence, TransactionConfidence::Pending)
    }

    /// Returns whether the transaction is finalized.
    #[must_use]
    pub const fn is_finalized(&self) -> bool {
        matches!(self.confidence, TransactionConfidence::Finalized)
    }

    /// Returns whether the transaction belongs to a block.
    #[must_use]
    pub const fn is_included(&self) -> bool {
        self.block_id.is_some() || self.block_height.is_some()
    }

    /// Returns whether the transaction deploys a contract.
    #[must_use]
    pub const fn is_contract_creation(&self) -> bool {
        matches!(self.kind, TransactionKind::ContractDeployment) || self.created_contract.is_some()
    }
}

fn normalize_required_text(value: String, field: &str) -> Result<String> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(invalid_argument(format!("{field} must not be empty")));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_argument(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(value)
}

fn normalize_unsigned_integer(value: String, field: &str) -> Result<String> {
    let value = value.trim();

    if value.is_empty() {
        return Err(invalid_argument(format!("{field} must not be empty")));
    }

    if !value.bytes().all(|character| character.is_ascii_digit()) {
        return Err(invalid_argument(format!(
            "{field} must contain only decimal digits"
        )));
    }

    let normalized = value.trim_start_matches('0');

    if normalized.is_empty() {
        Ok("0".to_owned())
    } else {
        Ok(normalized.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(byte: u8) -> Digest {
        Digest::new([byte; 32])
    }

    fn transaction() -> Transaction {
        Transaction::new(NetworkId::new(), digest(1))
    }

    #[test]
    fn creates_transaction() {
        let transaction = transaction();

        assert_eq!(transaction.hash(), digest(1));
        assert_eq!(transaction.kind(), TransactionKind::Unknown);
        assert_eq!(transaction.status(), TransactionStatus::Unknown);
        assert!(!transaction.is_included());
    }

    #[test]
    fn transaction_identifiers_are_unique() {
        let network_id = NetworkId::new();
        let hash = digest(1);

        let first = Transaction::new(network_id, hash);
        let second = Transaction::new(network_id, hash);

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn block_information_can_be_managed() {
        let mut transaction = transaction();
        let block_id = BlockId::new();

        transaction.set_block_id(block_id);
        transaction.set_block_height(123);
        transaction.set_transaction_index(7);

        assert_eq!(transaction.block_id(), Some(block_id));
        assert_eq!(transaction.block_height(), Some(123));
        assert_eq!(transaction.transaction_index(), Some(7));
        assert!(transaction.is_included());

        transaction.clear_block_id();
        transaction.clear_block_height();
        transaction.clear_transaction_index();

        assert!(!transaction.is_included());
    }

    #[test]
    fn address_information_can_be_managed() {
        let mut transaction = transaction();
        let sender = AddressId::new();
        let recipient = AddressId::new();

        transaction.set_from(sender);
        transaction.set_to(recipient);

        assert_eq!(transaction.from(), Some(sender));
        assert_eq!(transaction.to(), Some(recipient));

        transaction.clear_from();
        transaction.clear_to();

        assert_eq!(transaction.from(), None);
        assert_eq!(transaction.to(), None);
    }

    #[test]
    fn amount_is_normalized() {
        let mut transaction = transaction();

        transaction.set_amount("000001000").unwrap();

        assert_eq!(transaction.amount(), Some("1000"));

        transaction.set_amount("0000").unwrap();

        assert_eq!(transaction.amount(), Some("0"));
    }

    #[test]
    fn invalid_amount_is_rejected() {
        let mut transaction = transaction();

        assert!(transaction.set_amount("-1").is_err());
        assert!(transaction.set_amount("1.5").is_err());
        assert!(transaction.set_amount("one").is_err());
    }

    #[test]
    fn fee_can_be_assigned() {
        let asset_id = AssetId::new();

        let fee = TransactionFee::new(Some(asset_id), "00021000").unwrap();

        assert_eq!(fee.asset_id(), Some(asset_id));
        assert_eq!(fee.amount(), "21000");

        let mut transaction = transaction();

        transaction.set_fee(fee);

        assert_eq!(transaction.fee().map(TransactionFee::amount), Some("21000"));

        transaction.clear_fee();

        assert_eq!(transaction.fee(), None);
    }

    #[test]
    fn gas_information_can_be_managed() {
        let mut transaction = transaction();

        transaction.set_gas_limit(100_000);
        transaction.set_gas_used(75_000);
        transaction
            .set_effective_gas_price("0005000000000")
            .unwrap();

        assert_eq!(transaction.gas_limit(), Some(100_000));
        assert_eq!(transaction.gas_used(), Some(75_000));
        assert_eq!(transaction.effective_gas_price(), Some("5000000000"));
    }

    #[test]
    fn input_derives_method_selector() {
        let mut transaction = transaction();

        transaction
            .set_input(vec![0xa9, 0x05, 0x9c, 0xbb, 0x01, 0x02])
            .unwrap();

        assert_eq!(
            transaction.method_selector(),
            Some([0xa9, 0x05, 0x9c, 0xbb])
        );

        assert_eq!(
            transaction.input(),
            Some([0xa9, 0x05, 0x9c, 0xbb, 0x01, 0x02,].as_slice())
        );

        transaction.clear_input();

        assert_eq!(transaction.input(), None);
        assert_eq!(transaction.method_selector(), None);
    }

    #[test]
    fn short_input_has_no_selector() {
        let mut transaction = transaction();

        transaction.set_input(vec![0x01, 0x02, 0x03]).unwrap();

        assert_eq!(transaction.method_selector(), None);
    }

    #[test]
    fn replacement_changes_status() {
        let mut transaction = transaction();
        let replacement = TransactionId::new();

        transaction.mark_replaced_by(replacement).unwrap();

        assert_eq!(transaction.replacement(), Some(replacement));

        assert_eq!(transaction.status(), TransactionStatus::Replaced);
    }

    #[test]
    fn transaction_cannot_replace_itself() {
        let mut transaction = transaction();

        assert!(transaction.mark_replaced_by(transaction.id()).is_err());
    }

    #[test]
    fn lifecycle_helpers_work() {
        let mut transaction = transaction();

        transaction.set_status(TransactionStatus::Pending);

        assert!(transaction.is_pending());

        transaction.set_status(TransactionStatus::Succeeded);

        assert!(transaction.is_successful());
        assert!(!transaction.is_failed());

        transaction.set_confidence(TransactionConfidence::Finalized);

        assert!(transaction.is_finalized());
    }

    #[test]
    fn contract_creation_is_detected() {
        let mut transaction = transaction();

        transaction.set_kind(TransactionKind::ContractDeployment);

        assert!(transaction.is_contract_creation());

        transaction.set_kind(TransactionKind::Unknown);
        transaction.set_created_contract(AddressId::new());

        assert!(transaction.is_contract_creation());
    }

    #[test]
    fn tags_can_be_managed() {
        let mut transaction = transaction();

        assert!(transaction.add_tag("high-value").unwrap());

        assert!(!transaction.add_tag("high-value").unwrap());

        assert!(transaction.has_tag("high-value"));

        assert!(transaction.remove_tag("high-value"));

        assert!(!transaction.has_tag("high-value"));
    }

    #[test]
    fn empty_tag_is_rejected() {
        let mut transaction = transaction();

        assert!(transaction.add_tag("   ").is_err());
    }
}
