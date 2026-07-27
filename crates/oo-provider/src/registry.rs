// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/registry.rs
// Purpose : Implement the registry module for oo-provider.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Provider registry.

use crate::model::ProviderIdentity;

/// Ordered provider registry for one observation run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderRegistry {
    providers: Vec<ProviderIdentity>,
}

impl ProviderRegistry {
    /// Adds a provider.
    pub fn push(&mut self, provider: ProviderIdentity) {
        self.providers.push(provider);
    }

    /// Returns providers.
    #[must_use]
    pub fn providers(&self) -> &[ProviderIdentity] {
        &self.providers
    }

    /// Returns provider count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns true when the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}
