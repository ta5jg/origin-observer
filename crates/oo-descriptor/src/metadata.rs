// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/metadata.rs
// Purpose : Metadata descriptor extraction.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Metadata descriptor extraction.

use oo_core::{Digest, MetadataId};
use oo_model::metadata::{MetadataDocument, MetadataSourceKind};

/// Stable descriptor for asset metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataDescriptor {
    id: MetadataId,
    source: MetadataSourceKind,
    digest: Option<Digest>,
}

impl MetadataDescriptor {
    /// Extracts a metadata descriptor.
    #[must_use]
    pub fn from_document(document: &MetadataDocument) -> Self {
        Self {
            id: document.id(),
            source: document.source(),
            digest: document.digest(),
        }
    }

    /// Returns the metadata identifier.
    #[must_use]
    pub const fn id(&self) -> MetadataId {
        self.id
    }

    /// Returns the source kind.
    #[must_use]
    pub const fn source(&self) -> MetadataSourceKind {
        self.source
    }

    /// Returns the integrity digest when known.
    #[must_use]
    pub const fn digest(&self) -> Option<Digest> {
        self.digest
    }
}
