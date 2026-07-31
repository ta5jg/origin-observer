// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/lib.rs
// Purpose : Model registries, indexers, metadata, image and price providers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Model registries, indexers, metadata, image and price providers.

pub mod attribution;
pub mod capability;
pub mod dex;
pub mod explorer;
pub mod image;
pub mod indexer;
pub mod metadata;
pub mod model;
pub mod price;
pub mod registry;
pub mod resolver;
pub mod validation;

pub use attribution::Attribution;
pub use capability::{ProviderCapability, ProviderCategory};
pub use dex::DexQuote;
pub use explorer::{parse_source_code_response, VerificationResult};
pub use image::{ImageReference, ImageScheme};
pub use indexer::IndexerSnapshot;
pub use metadata::{merge, AssetMetadata, MergedMetadata, MetadataConflict};
pub use model::ProviderIdentity;
pub use price::{disagreements, PriceQuote};
pub use registry::ProviderRegistry;
pub use resolver::ProviderResolver;
pub use validation::validate_provider;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_provider_identity() {
        let provider = ProviderIdentity::new("cloudflare", "https://cloudflare-eth.com");
        assert!(validate_provider(&provider));

        let mut registry = ProviderRegistry::default();
        registry.push(provider);

        assert_eq!(registry.len(), 1);
        assert_eq!(registry.providers()[0].name(), "cloudflare");
    }
}
