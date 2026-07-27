// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/address.rs
// Purpose : Address descriptor extraction.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Address descriptor extraction.

use oo_model::address::{Address, AddressEncoding, AddressKind, AddressValidation};

use crate::validation::DescriptorValidation;

/// Stable descriptor for a blockchain address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressDescriptor {
    value: String,
    canonical_value: String,
    encoding: AddressEncoding,
    kind: AddressKind,
    validation: AddressValidation,
}

impl AddressDescriptor {
    /// Extracts a descriptor from an address model.
    #[must_use]
    pub fn from_address(address: &Address) -> Self {
        Self {
            value: address.value().to_owned(),
            canonical_value: address.canonical_value().to_owned(),
            encoding: address.encoding(),
            kind: address.kind(),
            validation: address.validation(),
        }
    }

    /// Returns the canonical value.
    #[must_use]
    pub fn canonical_value(&self) -> &str {
        &self.canonical_value
    }

    /// Returns the detected encoding.
    #[must_use]
    pub const fn encoding(&self) -> AddressEncoding {
        self.encoding
    }

    /// Validates descriptor invariants.
    #[must_use]
    pub fn validate(&self) -> DescriptorValidation {
        if self.value.is_empty() || self.canonical_value.is_empty() {
            DescriptorValidation::invalid("address descriptor is empty")
        } else {
            DescriptorValidation::valid("address descriptor is complete")
        }
    }
}
