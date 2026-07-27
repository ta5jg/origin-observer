// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/validation.rs
// Purpose : Descriptor validation primitives.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Descriptor validation primitives.

/// Validation outcome for a descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorValidation {
    valid: bool,
    reason: String,
}

impl DescriptorValidation {
    /// Creates a successful validation.
    #[must_use]
    pub fn valid(reason: impl Into<String>) -> Self {
        Self {
            valid: true,
            reason: reason.into(),
        }
    }

    /// Creates a failed validation.
    #[must_use]
    pub fn invalid(reason: impl Into<String>) -> Self {
        Self {
            valid: false,
            reason: reason.into(),
        }
    }

    /// Returns whether the descriptor passed validation.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns the validation reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Validates that a descriptor field is present.
#[must_use]
pub fn require_non_empty(value: &str, field: &str) -> DescriptorValidation {
    if value.trim().is_empty() {
        DescriptorValidation::invalid(format!("{field} is empty"))
    } else {
        DescriptorValidation::valid(format!("{field} is present"))
    }
}
