// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/resolver.rs
// Purpose : Select which providers to consult for a question.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Select which providers to consult for a question.
//!
//! Priority is declaration order: `config/providers.toml` lists providers in
//! the order an operator wants them tried, and this module preserves that
//! order rather than inventing a scoring scheme. Selection filters to
//! providers that both match the requested category and cover the requested
//! chain; a provider matching the category but not the chain is not offered
//! at all, rather than being offered and expected to fail.

use crate::capability::{ProviderCapability, ProviderCategory};
use crate::model::ProviderIdentity;

/// Selects providers for one category and chain, in priority order.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProviderResolver;

impl ProviderResolver {
    /// Returns the identities that may be consulted for `category` on
    /// `chain_id`, preserving the input order.
    #[must_use]
    pub fn select<'a>(
        &self,
        providers: &'a [(ProviderIdentity, ProviderCapability)],
        category: ProviderCategory,
        chain_id: u64,
    ) -> Vec<&'a ProviderIdentity> {
        providers
            .iter()
            .filter(|(_, capability)| capability.category == category)
            .filter(|(_, capability)| capability.covers_chain(chain_id))
            .map(|(identity, _)| identity)
            .collect()
    }

    /// Returns the first provider that may be consulted, if any.
    ///
    /// Callers that want a full fallback chain should use [`Self::select`]
    /// and try each in order; this is a convenience for the common case of
    /// consulting only the highest-priority match.
    #[must_use]
    pub fn preferred<'a>(
        &self,
        providers: &'a [(ProviderIdentity, ProviderCapability)],
        category: ProviderCategory,
        chain_id: u64,
    ) -> Option<&'a ProviderIdentity> {
        self.select(providers, category, chain_id)
            .into_iter()
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(
        name: &str,
        category: ProviderCategory,
        chains: Vec<u64>,
    ) -> (ProviderIdentity, ProviderCapability) {
        (
            ProviderIdentity::new(name, "https://example.org"),
            ProviderCapability::scoped(category, chains, false),
        )
    }

    #[test]
    fn selection_preserves_declaration_order() {
        let providers = vec![
            entry("first", ProviderCategory::Price, vec![]),
            entry("second", ProviderCategory::Price, vec![]),
        ];
        let selected = ProviderResolver.select(&providers, ProviderCategory::Price, 1);
        assert_eq!(selected[0].name(), "first");
        assert_eq!(selected[1].name(), "second");
    }

    #[test]
    fn a_category_mismatch_excludes_the_provider() {
        let providers = vec![entry("explorer", ProviderCategory::Explorer, vec![])];
        assert!(ProviderResolver
            .select(&providers, ProviderCategory::Price, 1)
            .is_empty());
    }

    #[test]
    fn a_chain_scoped_provider_is_excluded_for_other_chains() {
        let providers = vec![entry("ethereum-only", ProviderCategory::Explorer, vec![1])];
        assert!(ProviderResolver
            .select(&providers, ProviderCategory::Explorer, 56)
            .is_empty());
        assert_eq!(
            ProviderResolver
                .select(&providers, ProviderCategory::Explorer, 1)
                .len(),
            1
        );
    }

    #[test]
    fn preferred_returns_the_first_match() {
        let providers = vec![
            entry("first", ProviderCategory::Dex, vec![]),
            entry("second", ProviderCategory::Dex, vec![]),
        ];
        let preferred = ProviderResolver.preferred(&providers, ProviderCategory::Dex, 1);
        assert_eq!(preferred.unwrap().name(), "first");
    }
}
