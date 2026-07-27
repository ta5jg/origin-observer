// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/cache.rs
// Purpose : Cache observation domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Cache observation domain model.

use oo_core::{CacheId, WalletId};

/// Observed cache state for a wallet or provider surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum CacheState {
    /// Cache state is not known.
    #[default]
    Unknown,
    /// No cached value was observed.
    Empty,
    /// A cached value was observed and appears current.
    Warm,
    /// A cached value was observed but appears stale.
    Stale,
    /// The cache was invalidated during the observation.
    Invalidated,
}

/// A single cache observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheObservation {
    id: CacheId,
    wallet_id: Option<WalletId>,
    key: String,
    state: CacheState,
}

impl CacheObservation {
    /// Creates a cache observation.
    #[must_use]
    pub fn new(key: impl Into<String>, state: CacheState) -> Self {
        Self {
            id: CacheId::new(),
            wallet_id: None,
            key: key.into(),
            state,
        }
    }

    /// Returns the observation identifier.
    #[must_use]
    pub const fn id(&self) -> CacheId {
        self.id
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

    /// Returns the cache key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the observed cache state.
    #[must_use]
    pub const fn state(&self) -> CacheState {
        self.state
    }
}
