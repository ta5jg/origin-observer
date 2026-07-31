// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-storage/src/validation.rs
// Purpose : Validate storage responses before they are decoded.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate storage responses before they are decoded.

use crate::decoder::parse_storage_value;
use crate::error::StorageResult;

/// Validates that a hexadecimal `eth_getStorageAt` response is a well-formed
/// storage word.
///
/// This is a thin named wrapper over [`parse_storage_value`]'s own checks, for
/// callers that only need a validity check and not the decoded value.
pub fn validate_storage_response(hex: &str) -> StorageResult<()> {
    parse_storage_value(hex).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_well_formed_response_is_valid() {
        assert!(validate_storage_response("0x0").is_ok());
        assert!(validate_storage_response(&format!("0x{}", "ab".repeat(32))).is_ok());
    }

    #[test]
    fn a_response_that_is_too_long_is_invalid() {
        assert!(validate_storage_response(&format!("0x{}", "ab".repeat(33))).is_err());
    }

    #[test]
    fn a_response_without_a_0x_prefix_is_invalid() {
        assert!(validate_storage_response("00").is_err());
    }
}
