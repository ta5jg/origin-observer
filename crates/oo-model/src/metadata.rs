// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/metadata.rs
// Purpose : Metadata document domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Metadata document domain model.

use oo_core::{AssetId, Digest, MetadataId};

/// Metadata source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MetadataSourceKind {
    /// Source kind is unknown.
    #[default]
    Unknown,
    /// Metadata came from a token URI.
    TokenUri,
    /// Metadata came from a registry.
    Registry,
    /// Metadata came from a wallet cache.
    WalletCache,
    /// Metadata came from manual evidence.
    Manual,
}

/// Asset metadata document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataDocument {
    id: MetadataId,
    asset_id: AssetId,
    source: MetadataSourceKind,
    digest: Option<Digest>,
}

impl MetadataDocument {
    /// Creates a metadata document record.
    #[must_use]
    pub fn new(asset_id: AssetId, source: MetadataSourceKind) -> Self {
        Self {
            id: MetadataId::new(),
            asset_id,
            source,
            digest: None,
        }
    }

    /// Returns the metadata identifier.
    #[must_use]
    pub const fn id(&self) -> MetadataId {
        self.id
    }

    /// Returns the asset identifier.
    #[must_use]
    pub const fn asset_id(&self) -> AssetId {
        self.asset_id
    }

    /// Returns the metadata source class.
    #[must_use]
    pub const fn source(&self) -> MetadataSourceKind {
        self.source
    }

    /// Returns the integrity digest when known.
    #[must_use]
    pub const fn digest(&self) -> Option<Digest> {
        self.digest
    }

    /// Assigns the integrity digest.
    pub const fn set_digest(&mut self, digest: Digest) {
        self.digest = Some(digest);
    }
}
