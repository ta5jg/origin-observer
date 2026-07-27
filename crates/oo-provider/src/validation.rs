// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/validation.rs
// Purpose : Implement the validation module for oo-provider.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Provider validation.

use crate::model::ProviderIdentity;

/// Validates provider identity.
#[must_use]
pub fn validate_provider(provider: &ProviderIdentity) -> bool {
    !provider.name().trim().is_empty() && !provider.endpoint().trim().is_empty()
}
