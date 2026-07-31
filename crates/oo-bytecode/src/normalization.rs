// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/normalization.rs
// Purpose : Convert hexadecimal bytecode into normalized bytes.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Convert hexadecimal bytecode into normalized bytes.
//!
//! Nodes return `eth_getCode` with an optional `0x` prefix and arbitrary case.
//! Comparing two observations of the same contract must not depend on which
//! spelling a particular provider used, so every downstream analysis works on
//! this normalized byte form rather than on the raw string.

use crate::error::{BytecodeError, BytecodeResult};

/// Parses a hexadecimal bytecode string into bytes.
///
/// A leading `0x` or `0X` is optional and stripped. An empty string (after
/// stripping) is valid and represents an externally owned account or a
/// self-destructed contract, not an error.
pub fn parse_hex(value: &str) -> BytecodeResult<Vec<u8>> {
    let trimmed = value.trim();
    let body = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    if body.is_empty() {
        return Ok(Vec::new());
    }
    if body.len() % 2 != 0 {
        return Err(BytecodeError::InvalidHex(value.to_owned()));
    }

    let mut bytes = Vec::with_capacity(body.len() / 2);
    let raw = body.as_bytes();
    for chunk in raw.chunks(2) {
        let high =
            hex_digit(chunk[0]).ok_or_else(|| BytecodeError::InvalidHex(value.to_owned()))?;
        let low = hex_digit(chunk[1]).ok_or_else(|| BytecodeError::InvalidHex(value.to_owned()))?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

/// Renders bytes as a `0x`-prefixed lowercase hexadecimal string.
#[must_use]
pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(2 + bytes.len() * 2);
    out.push_str("0x");
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0x_prefix_is_optional_and_case_insensitive() {
        assert_eq!(parse_hex("0x6001").unwrap(), vec![0x60, 0x01]);
        assert_eq!(parse_hex("0X6001").unwrap(), vec![0x60, 0x01]);
        assert_eq!(parse_hex("6001").unwrap(), vec![0x60, 0x01]);
        assert_eq!(parse_hex("0xAB").unwrap(), vec![0xAB]);
    }

    #[test]
    fn an_empty_account_is_valid_not_an_error() {
        assert_eq!(parse_hex("0x").unwrap(), Vec::<u8>::new());
        assert_eq!(parse_hex("").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn odd_length_is_rejected() {
        assert!(matches!(
            parse_hex("0x601"),
            Err(BytecodeError::InvalidHex(_))
        ));
    }

    #[test]
    fn non_hex_characters_are_rejected() {
        assert!(matches!(
            parse_hex("0xzz"),
            Err(BytecodeError::InvalidHex(_))
        ));
    }

    #[test]
    fn to_hex_round_trips_through_parse_hex() {
        let bytes = vec![0x60, 0x01, 0xff, 0x00];
        assert_eq!(parse_hex(&to_hex(&bytes)).unwrap(), bytes);
    }

    #[test]
    fn to_hex_always_carries_the_0x_prefix() {
        assert_eq!(to_hex(&[]), "0x");
        assert_eq!(to_hex(&[0xAB]), "0xab");
    }
}
