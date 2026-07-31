// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-utils/src/validation.rs
// Purpose : Shared field validators that name what failed.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Shared field validators that name what failed.
//!
//! A validator returns the offending field name in its error, so a failure in
//! configuration, a descriptor or an evidence record is actionable from the
//! message alone.

use crate::error::{UtilsError, UtilsResult};

/// Requires a non-empty value after trimming.
pub fn require_non_empty(field: &str, value: &str) -> UtilsResult<()> {
    if value.trim().is_empty() {
        return Err(UtilsError::empty(field));
    }
    Ok(())
}

/// Requires a value no longer than `maximum` characters.
pub fn require_max_length(field: &str, value: &str, maximum: usize) -> UtilsResult<()> {
    let actual = value.chars().count();
    if actual > maximum {
        return Err(UtilsError::too_long(field, actual, maximum));
    }
    Ok(())
}

/// Requires a lowercase identifier of ASCII letters, digits, `-` and `_`.
///
/// Identifiers name chains, providers and wallets in configuration and in file
/// paths, so they are restricted to a form that is safe in both.
pub fn require_identifier(field: &str, value: &str) -> UtilsResult<()> {
    require_non_empty(field, value)?;
    let valid = value.chars().all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '-' | '_')
    });
    if !valid || value.starts_with(['-', '_']) || value.ends_with(['-', '_']) {
        return Err(UtilsError::malformed(
            field,
            "lowercase ASCII letters, digits, '-' or '_', not starting or ending with a separator",
        ));
    }
    Ok(())
}

/// Requires an `http` or `https` URL.
///
/// Any other scheme is rejected rather than normalized: a research tool must
/// not silently reinterpret the endpoint it was told to observe.
pub fn require_http_url(field: &str, value: &str) -> UtilsResult<()> {
    require_non_empty(field, value)?;
    if !(value.starts_with("http://") || value.starts_with("https://")) {
        return Err(UtilsError::malformed(field, "an http:// or https:// URL"));
    }
    if value.len() <= "https://".len() {
        return Err(UtilsError::malformed(field, "a URL with a host"));
    }
    Ok(())
}

/// Requires a value within an inclusive numeric range.
pub fn require_range(field: &str, value: u64, minimum: u64, maximum: u64) -> UtilsResult<()> {
    if value < minimum || value > maximum {
        return Err(UtilsError::malformed(
            field,
            format!("a value between {minimum} and {maximum}"),
        ));
    }
    Ok(())
}

/// Requires that a collection contains no duplicate entries.
///
/// Duplicate identifiers in configuration silently shadow one another, which
/// would make an observation attribute itself to the wrong source.
pub fn require_unique<'a, I>(field: &str, values: I) -> UtilsResult<()>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut seen: Vec<&str> = Vec::new();
    for value in values {
        if seen.contains(&value) {
            return Err(UtilsError::malformed(
                field,
                format!("unique entries, but {value} appears more than once"),
            ));
        }
        seen.push(value);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_values_are_rejected_with_their_field_name() {
        let error = require_non_empty("chain.name", "   ").unwrap_err();
        assert!(error.to_string().contains("chain.name"));
        assert!(require_non_empty("chain.name", "ethereum").is_ok());
    }

    #[test]
    fn length_is_measured_in_characters_not_bytes() {
        assert!(require_max_length("note", "ıııı", 4).is_ok());
        let error = require_max_length("note", "ııııı", 4).unwrap_err();
        assert!(error.to_string().contains("5 characters"));
    }

    #[test]
    fn identifiers_reject_uppercase_spaces_and_edge_separators() {
        assert!(require_identifier("chain.id", "ethereum-mainnet").is_ok());
        assert!(require_identifier("chain.id", "bnb_smart_chain").is_ok());
        assert!(require_identifier("chain.id", "Ethereum").is_err());
        assert!(require_identifier("chain.id", "ethereum mainnet").is_err());
        assert!(require_identifier("chain.id", "-ethereum").is_err());
        assert!(require_identifier("chain.id", "ethereum-").is_err());
    }

    #[test]
    fn only_http_and_https_endpoints_are_accepted() {
        assert!(require_http_url("rpc", "https://rpc.example.org").is_ok());
        assert!(require_http_url("rpc", "http://localhost:8545").is_ok());
        assert!(require_http_url("rpc", "ws://rpc.example.org").is_err());
        assert!(require_http_url("rpc", "https://").is_err());
    }

    #[test]
    fn ranges_are_inclusive_at_both_ends() {
        assert!(require_range("timeout", 1, 1, 10).is_ok());
        assert!(require_range("timeout", 10, 1, 10).is_ok());
        assert!(require_range("timeout", 0, 1, 10).is_err());
        assert!(require_range("timeout", 11, 1, 10).is_err());
    }

    #[test]
    fn duplicate_entries_are_reported_by_value() {
        assert!(require_unique("chains", ["ethereum", "bnb"]).is_ok());
        let error = require_unique("chains", ["ethereum", "ethereum"]).unwrap_err();
        assert!(error
            .to_string()
            .contains("ethereum appears more than once"));
    }
}
