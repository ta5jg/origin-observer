// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/wallet.rs
// Purpose : Wallet domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Wallet domain model.

use std::collections::BTreeSet;

use oo_core::{AddressId, WalletId};

/// General wallet classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum WalletKind {
    /// Standard user wallet.
    #[default]
    User,

    /// Smart contract wallet.
    SmartContract,

    /// Multi-signature wallet.
    MultiSignature,

    /// Exchange-controlled wallet.
    Exchange,

    /// Treasury wallet.
    Treasury,

    /// Validator wallet.
    Validator,

    /// Custodial wallet.
    Custodial,

    /// Unknown wallet type.
    Unknown,
}

/// Wallet state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum WalletStatus {
    #[default]
    Active,
    Archived,
    Disabled,
}

/// Represents a blockchain wallet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wallet {
    id: WalletId,
    name: String,
    kind: WalletKind,
    status: WalletStatus,
    addresses: BTreeSet<AddressId>,
    tags: BTreeSet<String>,
}

impl Wallet {
    /// Creates a new wallet.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: WalletId::new(),
            name: name.into(),
            kind: WalletKind::default(),
            status: WalletStatus::default(),
            addresses: BTreeSet::new(),
            tags: BTreeSet::new(),
        }
    }

    /// Returns the wallet identifier.
    #[must_use]
    pub const fn id(&self) -> WalletId {
        self.id
    }

    /// Returns the wallet name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Changes the wallet name.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Returns the wallet kind.
    #[must_use]
    pub const fn kind(&self) -> WalletKind {
        self.kind
    }

    /// Sets the wallet kind.
    pub const fn set_kind(&mut self, kind: WalletKind) {
        self.kind = kind;
    }

    /// Returns the wallet status.
    #[must_use]
    pub const fn status(&self) -> WalletStatus {
        self.status
    }

    /// Archives the wallet.
    pub const fn archive(&mut self) {
        self.status = WalletStatus::Archived;
    }

    /// Disables the wallet.
    pub const fn disable(&mut self) {
        self.status = WalletStatus::Disabled;
    }

    /// Activates the wallet.
    pub const fn activate(&mut self) {
        self.status = WalletStatus::Active;
    }

    /// Returns true if the wallet is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.status, WalletStatus::Active)
    }

    /// Adds an address.
    ///
    /// Returns true if the address was newly inserted.
    pub fn add_address(&mut self, address: AddressId) -> bool {
        self.addresses.insert(address)
    }

    /// Removes an address.
    ///
    /// Returns true if the address existed.
    pub fn remove_address(&mut self, address: AddressId) -> bool {
        self.addresses.remove(&address)
    }

    /// Returns true if the wallet contains the supplied address.
    #[must_use]
    pub fn contains_address(&self, address: AddressId) -> bool {
        self.addresses.contains(&address)
    }

    /// Returns all addresses.
    #[must_use]
    pub fn addresses(&self) -> &BTreeSet<AddressId> {
        &self.addresses
    }

    /// Returns the number of addresses.
    #[must_use]
    pub fn address_count(&self) -> usize {
        self.addresses.len()
    }

    /// Adds a tag.
    ///
    /// Returns true if the tag was newly inserted.
    pub fn add_tag(&mut self, tag: impl Into<String>) -> bool {
        self.tags.insert(tag.into())
    }

    /// Removes a tag.
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    /// Returns true if the wallet has the supplied tag.
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    /// Returns all tags.
    #[must_use]
    pub fn tags(&self) -> &BTreeSet<String> {
        &self.tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_creates_unique_identifier() {
        let a = Wallet::new("Primary");
        let b = Wallet::new("Primary");

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn wallet_name_can_change() {
        let mut wallet = Wallet::new("Old");

        wallet.set_name("New");

        assert_eq!(wallet.name(), "New");
    }

    #[test]
    fn wallet_kind_can_change() {
        let mut wallet = Wallet::new("Wallet");

        wallet.set_kind(WalletKind::Treasury);

        assert_eq!(wallet.kind(), WalletKind::Treasury);
    }

    #[test]
    fn wallet_status_changes() {
        let mut wallet = Wallet::new("Wallet");

        assert!(wallet.is_active());

        wallet.disable();
        assert_eq!(wallet.status(), WalletStatus::Disabled);

        wallet.archive();
        assert_eq!(wallet.status(), WalletStatus::Archived);

        wallet.activate();
        assert!(wallet.is_active());
    }

    #[test]
    fn address_management() {
        let mut wallet = Wallet::new("Wallet");

        let address = AddressId::new();

        assert!(wallet.add_address(address));
        assert!(!wallet.add_address(address));

        assert!(wallet.contains_address(address));
        assert_eq!(wallet.address_count(), 1);

        assert!(wallet.remove_address(address));
        assert!(!wallet.contains_address(address));
    }

    #[test]
    fn tag_management() {
        let mut wallet = Wallet::new("Wallet");

        assert!(wallet.add_tag("treasury"));
        assert!(!wallet.add_tag("treasury"));

        assert!(wallet.has_tag("treasury"));

        assert!(wallet.remove_tag("treasury"));

        assert!(!wallet.has_tag("treasury"));
    }
}
