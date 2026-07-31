// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/decoder.rs
// Purpose : Decode ABI-encoded return data for a bounded set of types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Decode ABI-encoded return data for a bounded set of types.
//!
//! Full ABI decoding covers arbitrarily nested tuples and arrays; this module
//! covers exactly what a single-value return needs: one static value
//! (`address`, `bool`, `uintN`, `bytesN`), or one dynamic value (`string`,
//! `bytes`) as the sole return. That subset answers `name()`, `symbol()`,
//! `decimals()`, `totalSupply()`, `balanceOf(address)` and their ERC-721/1155
//! counterparts. A shape outside it returns
//! [`crate::error::AbiError::UnsupportedType`] rather than a guess.

use crate::error::{AbiError, AbiResult};
use crate::model::AbiType;

const WORD: usize = 32;

/// Decodes a single return value of a known type from ABI-encoded data.
pub fn decode(data: &[u8], expected: AbiType) -> AbiResult<DecodedValue> {
    match expected {
        AbiType::Address => Ok(DecodedValue::Address(decode_address(data)?)),
        AbiType::Bool => Ok(DecodedValue::Bool(decode_bool(data)?)),
        AbiType::Uint256 => Ok(DecodedValue::Uint(decode_uint(data)?)),
        AbiType::Uint(bits) => Ok(DecodedValue::Uint(decode_bounded_uint(data, bits)?)),
        AbiType::FixedBytes(len) => Ok(DecodedValue::FixedBytes(decode_fixed_bytes(data, len)?)),
        AbiType::String => Ok(DecodedValue::String(decode_string(data)?)),
        AbiType::Bytes => Ok(DecodedValue::Bytes(decode_bytes(data)?)),
    }
}

/// A decoded value, tagged by the type that produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodedValue {
    /// Decoded `address`.
    Address([u8; 20]),
    /// Decoded `bool`.
    Bool(bool),
    /// Decoded `uintN`, as a big-endian 256-bit value.
    Uint(Uint256),
    /// Decoded `bytesN`.
    FixedBytes(Vec<u8>),
    /// Decoded `string`.
    String(String),
    /// Decoded `bytes`.
    Bytes(Vec<u8>),
}

/// A 256-bit unsigned integer, stored big-endian.
///
/// Ethereum's native integer width is 256 bits; a real ERC-20 `totalSupply`
/// routinely exceeds `u64::MAX` once decimals are applied. This type keeps
/// the full value rather than truncating, and only converts to a native
/// integer when the caller explicitly asks and the value actually fits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uint256([u8; 32]);

impl Uint256 {
    /// Wraps a big-endian 32-byte value.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the big-endian bytes.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Converts to `u64`, if the value fits.
    #[must_use]
    pub fn as_u64(self) -> Option<u64> {
        if self.0[..24].iter().any(|byte| *byte != 0) {
            return None;
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[24..]);
        Some(u64::from_be_bytes(bytes))
    }

    /// Renders the value as a base-10 string, without truncation.
    #[must_use]
    pub fn to_decimal_string(self) -> String {
        // Long division of the big-endian byte value by 10, one digit at a
        // time, so arbitrarily large values render exactly.
        let mut digits = Vec::new();
        let mut value = self.0;
        loop {
            let mut remainder: u32 = 0;
            for byte in &mut value {
                let acc = (remainder << 8) | u32::from(*byte);
                *byte = (acc / 10) as u8;
                remainder = acc % 10;
            }
            digits.push(char::from_digit(remainder, 10).unwrap_or('0'));
            if value.iter().all(|byte| *byte == 0) {
                break;
            }
        }
        digits.iter().rev().collect()
    }
}

fn word_at(data: &[u8], index: usize) -> AbiResult<&[u8; WORD]> {
    let start = index * WORD;
    let end = start + WORD;
    if data.len() < end {
        return Err(AbiError::DataTooShort {
            expected: "one ABI word".to_owned(),
            expected_bytes: end,
            found_bytes: data.len(),
        });
    }
    Ok(data[start..end]
        .try_into()
        .expect("slice is exactly 32 bytes"))
}

fn decode_address(data: &[u8]) -> AbiResult<[u8; 20]> {
    let word = word_at(data, 0)?;
    if word[..12].iter().any(|byte| *byte != 0) {
        return Err(AbiError::MalformedValue("address".to_owned()));
    }
    let mut address = [0u8; 20];
    address.copy_from_slice(&word[12..]);
    Ok(address)
}

fn decode_bool(data: &[u8]) -> AbiResult<bool> {
    let word = word_at(data, 0)?;
    if word[..31].iter().any(|byte| *byte != 0) {
        return Err(AbiError::MalformedValue("bool".to_owned()));
    }
    match word[31] {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(AbiError::MalformedValue("bool".to_owned())),
    }
}

fn decode_uint(data: &[u8]) -> AbiResult<Uint256> {
    Ok(Uint256::from_be_bytes(*word_at(data, 0)?))
}

fn decode_bounded_uint(data: &[u8], bits: u16) -> AbiResult<Uint256> {
    let word = word_at(data, 0)?;
    let unused_bytes = 32usize.saturating_sub(usize::from(bits) / 8);
    if word[..unused_bytes].iter().any(|byte| *byte != 0) {
        return Err(AbiError::MalformedValue(format!("uint{bits}")));
    }
    Ok(Uint256::from_be_bytes(*word))
}

fn decode_fixed_bytes(data: &[u8], len: u8) -> AbiResult<Vec<u8>> {
    let len = usize::from(len);
    if len == 0 || len > WORD {
        return Err(AbiError::UnsupportedType(format!("bytes{len}")));
    }
    let word = word_at(data, 0)?;
    // A fixedN value is left-aligned in its word: unlike uint/address/bool, the
    // padding bytes are on the right, not the left.
    Ok(word[..len].to_vec())
}

fn dynamic_bytes(data: &[u8]) -> AbiResult<Vec<u8>> {
    let head = word_at(data, 0)?;
    if head[..24].iter().any(|byte| *byte != 0) {
        return Err(AbiError::MalformedValue("dynamic offset".to_owned()));
    }
    let mut offset_bytes = [0u8; 8];
    offset_bytes.copy_from_slice(&head[24..]);
    let offset = u64::from_be_bytes(offset_bytes) as usize;

    if offset + WORD > data.len() {
        return Err(AbiError::OffsetOutOfRange {
            offset,
            length: data.len(),
        });
    }
    let length_word = word_at(&data[offset..], 0)?;
    if length_word[..24].iter().any(|byte| *byte != 0) {
        return Err(AbiError::MalformedValue("dynamic length".to_owned()));
    }
    let mut length_bytes = [0u8; 8];
    length_bytes.copy_from_slice(&length_word[24..]);
    let length = u64::from_be_bytes(length_bytes) as usize;

    let body_start = offset + WORD;
    let body_end = body_start + length;
    if body_end > data.len() {
        return Err(AbiError::DataTooShort {
            expected: "dynamic value body".to_owned(),
            expected_bytes: body_end,
            found_bytes: data.len(),
        });
    }
    Ok(data[body_start..body_end].to_vec())
}

fn decode_string(data: &[u8]) -> AbiResult<String> {
    let bytes = dynamic_bytes(data)?;
    String::from_utf8(bytes).map_err(|_| AbiError::MalformedValue("string".to_owned()))
}

fn decode_bytes(data: &[u8]) -> AbiResult<Vec<u8>> {
    dynamic_bytes(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(low_bytes: &[u8]) -> Vec<u8> {
        let mut word = vec![0u8; WORD - low_bytes.len()];
        word.extend_from_slice(low_bytes);
        word
    }

    #[test]
    fn an_address_is_read_from_the_low_20_bytes_of_the_word() {
        let address_bytes: [u8; 20] = [0xAB; 20];
        let data = word(&address_bytes);
        assert_eq!(decode_address(&data).unwrap(), address_bytes);
    }

    #[test]
    fn an_address_with_nonzero_padding_is_malformed() {
        let mut data = word(&[0xAB; 20]);
        data[0] = 0x01; // padding byte must be zero
        assert!(matches!(
            decode_address(&data),
            Err(AbiError::MalformedValue(_))
        ));
    }

    #[test]
    fn bool_decodes_zero_and_one_only() {
        assert!(!decode_bool(&word(&[0])).unwrap());
        assert!(decode_bool(&word(&[1])).unwrap());
        assert!(decode_bool(&word(&[2])).is_err());
    }

    #[test]
    fn uint256_round_trips_a_large_value() {
        let mut bytes = [0u8; 32];
        bytes[31] = 0xff;
        bytes[30] = 0xff;
        bytes[0] = 0x01; // exceeds u64
        let value = decode_uint(&bytes).unwrap();
        assert_eq!(value.to_be_bytes(), bytes);
        assert_eq!(value.as_u64(), None);
    }

    #[test]
    fn uint256_fits_u64_when_the_high_bytes_are_zero() {
        let mut bytes = [0u8; 32];
        bytes[31] = 42;
        assert_eq!(decode_uint(&bytes).unwrap().as_u64(), Some(42));
    }

    #[test]
    fn decimal_string_matches_a_known_large_value() {
        // 10^18, a typical ERC-20 "one full token" raw amount.
        let value = Uint256::from_be_bytes({
            let mut bytes = [0u8; 32];
            let raw = 1_000_000_000_000_000_000u128.to_be_bytes();
            bytes[16..].copy_from_slice(&raw);
            bytes
        });
        assert_eq!(value.to_decimal_string(), "1000000000000000000");
    }

    #[test]
    fn decimal_string_of_zero_is_zero() {
        assert_eq!(Uint256::from_be_bytes([0u8; 32]).to_decimal_string(), "0");
    }

    #[test]
    fn bounded_uint_rejects_a_value_wider_than_its_declared_bits() {
        let mut bytes = [0u8; 32];
        bytes[30] = 0x01; // sets a bit outside uint8's range
        bytes[31] = 0x00;
        assert!(decode_bounded_uint(&bytes, 8).is_err());
    }

    #[test]
    fn fixed_bytes_is_left_aligned_unlike_uint() {
        // bytes4 value 0xdeadbeef occupies the FIRST 4 bytes, zero-padded on
        // the right — the opposite alignment from uint/address.
        let mut data = [0u8; 32];
        data[0..4].copy_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(
            decode_fixed_bytes(&data, 4).unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
    }

    #[test]
    fn a_dynamic_string_decodes_name_style_returns() {
        // offset=0x20, length=6, "Tether" padded to 32 bytes.
        let mut data = Vec::new();
        data.extend(word(&[0x20]));
        data.extend(word(&[6]));
        let mut body = b"Tether".to_vec();
        body.resize(32, 0);
        data.extend(body);

        assert_eq!(decode_string(&data).unwrap(), "Tether");
    }

    #[test]
    fn an_offset_outside_the_data_is_rejected() {
        let data = word(&[0xff]); // offset = 255, far outside a 32-byte buffer
        assert!(matches!(
            decode_string(&data),
            Err(AbiError::OffsetOutOfRange { .. })
        ));
    }

    #[test]
    fn a_length_exceeding_the_available_body_is_rejected() {
        let mut data = Vec::new();
        data.extend(word(&[0x20]));
        data.extend(word(&[100])); // claims 100 bytes but supplies none
        assert!(matches!(
            decode_string(&data),
            Err(AbiError::DataTooShort { .. })
        ));
    }

    #[test]
    fn invalid_utf8_in_a_dynamic_string_is_malformed_not_lossy() {
        let mut data = Vec::new();
        data.extend(word(&[0x20]));
        data.extend(word(&[1]));
        let mut body = vec![0xff]; // not valid UTF-8
        body.resize(32, 0);
        data.extend(body);
        assert!(matches!(
            decode_string(&data),
            Err(AbiError::MalformedValue(_))
        ));
    }

    #[test]
    fn dynamic_bytes_does_not_require_utf8() {
        let mut data = Vec::new();
        data.extend(word(&[0x20]));
        data.extend(word(&[1]));
        let mut body = vec![0xff];
        body.resize(32, 0);
        data.extend(body);
        assert_eq!(decode_bytes(&data).unwrap(), vec![0xff]);
    }

    #[test]
    fn decode_dispatches_by_type() {
        let data = word(&[1]);
        assert_eq!(
            decode(&data, AbiType::Bool).unwrap(),
            DecodedValue::Bool(true)
        );
    }

    #[test]
    fn short_data_is_reported_with_the_missing_amount() {
        let error = word_at(&[0u8; 10], 0).unwrap_err();
        assert!(matches!(
            error,
            AbiError::DataTooShort {
                expected_bytes: 32,
                found_bytes: 10,
                ..
            }
        ));
    }
}
