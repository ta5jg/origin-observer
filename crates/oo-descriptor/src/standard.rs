// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/standard.rs
// Purpose : Asset standard descriptor helpers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Asset standard descriptor helpers.

use oo_model::asset::{AssetKind, AssetStandard};

/// Descriptor-level classification of an asset standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandardDescriptor {
    kind: AssetKind,
    standard: AssetStandard,
}

impl StandardDescriptor {
    /// Creates a standard descriptor.
    #[must_use]
    pub const fn new(kind: AssetKind, standard: AssetStandard) -> Self {
        Self { kind, standard }
    }

    /// Returns the asset kind.
    #[must_use]
    pub const fn kind(&self) -> AssetKind {
        self.kind
    }

    /// Returns the asset standard.
    #[must_use]
    pub const fn standard(&self) -> AssetStandard {
        self.standard
    }

    /// Returns true for native chain assets.
    #[must_use]
    pub const fn is_native(&self) -> bool {
        matches!(self.kind, AssetKind::Native) && matches!(self.standard, AssetStandard::Native)
    }
}
