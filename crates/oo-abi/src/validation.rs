// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/validation.rs
// Purpose : Validate ABI identifiers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate ABI identifiers.

use crate::error::{AbiError, AbiResult};

/// Validates a Solidity function or event identifier.
///
/// Solidity identifiers start with a letter, `_` or `$`, and continue with
/// letters, digits, `_` or `$`. Rejecting anything else here means a
/// malformed name fails at signature construction rather than producing a
/// selector for a string nothing could actually call.
pub fn validate_identifier(name: &str) -> AbiResult<()> {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Err(AbiError::InvalidIdentifier(name.to_owned()));
    };
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return Err(AbiError::InvalidIdentifier(name.to_owned()));
    }
    if !chars
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '$')
    {
        return Err(AbiError::InvalidIdentifier(name.to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_identifiers_are_valid() {
        assert!(validate_identifier("transfer").is_ok());
        assert!(validate_identifier("balanceOf").is_ok());
        assert!(validate_identifier("_internal").is_ok());
        assert!(validate_identifier("$special").is_ok());
        assert!(validate_identifier("transfer2").is_ok());
    }

    #[test]
    fn an_empty_identifier_is_rejected() {
        assert!(validate_identifier("").is_err());
    }

    #[test]
    fn an_identifier_starting_with_a_digit_is_rejected() {
        assert!(validate_identifier("2transfer").is_err());
    }

    #[test]
    fn whitespace_and_punctuation_are_rejected() {
        assert!(validate_identifier("trans fer").is_err());
        assert!(validate_identifier("transfer(").is_err());
        assert!(validate_identifier("transfer.x").is_err());
    }
}
