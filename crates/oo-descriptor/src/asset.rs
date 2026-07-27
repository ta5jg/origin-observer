// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/asset.rs
// Purpose : Asset descriptor extraction.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Asset descriptor extraction.

use oo_core::{AssetId, NetworkId};
use oo_model::asset::{Asset, AssetKind, AssetStandard, AssetVerification};

use crate::standard::StandardDescriptor;
use crate::validation::DescriptorValidation;

/// Stable descriptor for a blockchain asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetDescriptor {
    id: AssetId,
    network_id: NetworkId,
    name: String,
    symbol: String,
    standard: StandardDescriptor,
    verification: AssetVerification,
}

impl AssetDescriptor {
    /// Extracts a descriptor from an asset model.
    #[must_use]
    pub fn from_asset(asset: &Asset) -> Self {
        Self {
            id: asset.id(),
            network_id: asset.network_id(),
            name: asset.name().to_owned(),
            symbol: asset.symbol().to_owned(),
            standard: StandardDescriptor::new(asset.kind(), asset.standard()),
            verification: asset.verification(),
        }
    }

    /// Returns the asset identifier.
    #[must_use]
    pub const fn id(&self) -> AssetId {
        self.id
    }

    /// Returns the asset network.
    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    /// Returns the asset standard descriptor.
    #[must_use]
    pub const fn standard(&self) -> StandardDescriptor {
        self.standard
    }

    /// Returns whether this describes a native asset.
    #[must_use]
    pub const fn is_native(&self) -> bool {
        matches!(self.standard.kind(), AssetKind::Native)
            && matches!(self.standard.standard(), AssetStandard::Native)
    }

    /// Validates descriptor invariants.
    #[must_use]
    pub fn validate(&self) -> DescriptorValidation {
        if self.name.is_empty() || self.symbol.is_empty() {
            DescriptorValidation::invalid("asset name or symbol is empty")
        } else {
            let _ = self.verification;
            DescriptorValidation::valid("asset descriptor is complete")
        }
    }
}
