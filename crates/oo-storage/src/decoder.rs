// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-storage/src/decoder.rs
// Purpose : Decode a raw 32-byte storage word into typed values.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Decode a raw 32-byte storage word into typed values.
//!
//! A storage slot is an opaque 32-byte word; what it means depends entirely on
//! what the contract declared there. This module decodes the shapes proxy
//! detection actually needs — an address in the low 20 bytes, a boolean flag,
//! a raw integer — and refuses a value that does not satisfy the requested
//! shape rather than silently returning a wrong interpretation.

use crate::error::{StorageError, StorageResult};

/// A raw 32-byte storage value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageValue([u8; 32]);

impl StorageValue {
    /// Wraps a raw 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the raw bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns whether every byte is zero.
    ///
    /// An unset slot reads as all zeros; this is how proxy detection tells
    /// "this proxy kind's slot is not in use" from "it names the zero
    /// address," which for a proxy implementation slot means the same thing.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|byte| *byte == 0)
    }

    /// Decodes the low 20 bytes as an address, requiring the top 12 bytes to
    /// be zero.
    pub fn as_address(&self) -> StorageResult<[u8; 20]> {
        if self.0[..12].iter().any(|byte| *byte != 0) {
            return Err(StorageError::NotAnAddress);
        }
        let mut address = [0u8; 20];
        address.copy_from_slice(&self.0[12..]);
        Ok(address)
    }

    /// Decodes the value as a boolean, requiring it to be exactly zero or
    /// one.
    pub fn as_bool(&self) -> StorageResult<bool> {
        if self.0[..31].iter().any(|byte| *byte != 0) {
            return Err(StorageError::NotABool);
        }
        match self.0[31] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(StorageError::NotABool),
        }
    }

    /// Returns the value as a big-endian unsigned integer, converted to
    /// `u64` only when the high 24 bytes are zero.
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        if self.0[..24].iter().any(|byte| *byte != 0) {
            return None;
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[24..]);
        Some(u64::from_be_bytes(bytes))
    }
}

/// Parses a hexadecimal `eth_getStorageAt` response into a [`StorageValue`].
///
/// Nodes reply with a variable-length hex string, left-padding omitted: `0x0`
/// for an unset slot, not 64 zero digits. The value is left-padded to 32
/// bytes here so it always represents the true storage word.
pub fn parse_storage_value(hex: &str) -> StorageResult<StorageValue> {
    let trimmed = hex.trim();
    let body = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .ok_or_else(|| StorageError::InvalidHex(hex.to_owned()))?;

    if body.len() > 64 {
        return Err(StorageError::WrongLength {
            found: body.len().div_ceil(2),
        });
    }

    let padded = format!("{:0>64}", body);
    let mut bytes = [0u8; 32];
    for (index, chunk) in padded.as_bytes().chunks(2).enumerate() {
        let text =
            std::str::from_utf8(chunk).map_err(|_| StorageError::InvalidHex(hex.to_owned()))?;
        bytes[index] =
            u8::from_str_radix(text, 16).map_err(|_| StorageError::InvalidHex(hex.to_owned()))?;
    }
    Ok(StorageValue::new(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unset_slot_parses_as_zero() {
        let value = parse_storage_value("0x0").unwrap();
        assert!(value.is_zero());
    }

    #[test]
    fn a_short_hex_response_is_left_padded_to_a_full_word() {
        let value = parse_storage_value("0x1").unwrap();
        assert_eq!(value.as_bytes()[31], 1);
        assert!(value.as_bytes()[..31].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn an_address_is_decoded_from_the_low_20_bytes() {
        let mut hex = "0x".to_owned();
        hex.push_str(&"00".repeat(12));
        hex.push_str(&"ab".repeat(20));
        let value = parse_storage_value(&hex).unwrap();
        assert_eq!(value.as_address().unwrap(), [0xAB; 20]);
    }

    #[test]
    fn nonzero_padding_is_rejected_as_an_address() {
        let mut hex = "0x01".to_owned();
        hex.push_str(&"00".repeat(11));
        hex.push_str(&"ab".repeat(20));
        let value = parse_storage_value(&hex).unwrap();
        assert!(value.as_address().is_err());
    }

    #[test]
    fn bool_accepts_only_zero_and_one() {
        assert!(!parse_storage_value("0x0").unwrap().as_bool().unwrap());
        assert!(parse_storage_value("0x1").unwrap().as_bool().unwrap());
        assert!(parse_storage_value("0x2").unwrap().as_bool().is_err());
    }

    #[test]
    fn u64_conversion_fails_when_the_value_does_not_fit() {
        let mut hex = "0x01".to_owned();
        hex.push_str(&"00".repeat(31));
        let value = parse_storage_value(&hex).unwrap();
        assert_eq!(value.as_u64(), None);
    }

    #[test]
    fn an_oversized_response_is_rejected_rather_than_truncated() {
        let hex = format!("0x{}", "ff".repeat(40));
        assert!(matches!(
            parse_storage_value(&hex),
            Err(StorageError::WrongLength { .. })
        ));
    }

    #[test]
    fn a_response_missing_the_0x_prefix_is_rejected() {
        assert!(matches!(
            parse_storage_value("1234"),
            Err(StorageError::InvalidHex(_))
        ));
    }
}
