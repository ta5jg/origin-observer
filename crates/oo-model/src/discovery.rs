// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/discovery.rs
// Purpose : Discovery result domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Discovery result domain model.

use oo_core::{AssetId, DiscoveryId, WalletId};

/// Wallet discovery decision for an asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DiscoveryDecision {
    /// No decision has been reached.
    #[default]
    Unknown,
    /// The asset is recognized and presented.
    Recognized,
    /// The asset is known but hidden from the normal asset list.
    Hidden,
    /// The asset is ignored.
    Ignored,
    /// The asset requires manual user action.
    Manual,
}

/// Asset discovery result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryResult {
    id: DiscoveryId,
    asset_id: AssetId,
    wallet_id: Option<WalletId>,
    decision: DiscoveryDecision,
    reason: String,
}

impl DiscoveryResult {
    /// Creates a discovery result.
    #[must_use]
    pub fn new(asset_id: AssetId, decision: DiscoveryDecision, reason: impl Into<String>) -> Self {
        Self {
            id: DiscoveryId::new(),
            asset_id,
            wallet_id: None,
            decision,
            reason: reason.into(),
        }
    }

    /// Returns the discovery result identifier.
    #[must_use]
    pub const fn id(&self) -> DiscoveryId {
        self.id
    }

    /// Returns the observed asset identifier.
    #[must_use]
    pub const fn asset_id(&self) -> AssetId {
        self.asset_id
    }

    /// Returns the wallet identifier when known.
    #[must_use]
    pub const fn wallet_id(&self) -> Option<WalletId> {
        self.wallet_id
    }

    /// Assigns the observed wallet.
    pub const fn set_wallet_id(&mut self, wallet_id: WalletId) {
        self.wallet_id = Some(wallet_id);
    }

    /// Returns the discovery decision.
    #[must_use]
    pub const fn decision(&self) -> DiscoveryDecision {
        self.decision
    }

    /// Returns the reason recorded for the decision.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}
