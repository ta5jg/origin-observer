// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/model.rs
// Purpose : Implement the model module for oo-provider.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Provider identity model.

use oo_core::ProviderId;

/// Stable identity for an observation provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderIdentity {
    id: ProviderId,
    name: String,
    endpoint: String,
}

impl ProviderIdentity {
    /// Creates a provider identity.
    #[must_use]
    pub fn new(name: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            id: ProviderId::new(),
            name: name.into(),
            endpoint: endpoint.into(),
        }
    }

    /// Returns provider id.
    #[must_use]
    pub const fn id(&self) -> ProviderId {
        self.id
    }

    /// Returns provider name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns endpoint locator.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}
