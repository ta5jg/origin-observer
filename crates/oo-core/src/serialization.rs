// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/serialization.rs
// Purpose : Serialization traits and helpers shared across Origin Observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Serialization infrastructure shared by every Origin Observer crate.
//!
//! This module intentionally defines small serialization traits without
//! depending on a specific format. JSON, CBOR, MessagePack, Bincode or any
//! future encoding can build on top of these abstractions.

use crate::error::serialization_error;
use crate::result::Result;

/// Trait implemented by types that can serialize themselves into bytes.
pub trait BinarySerializable {
    /// Serializes the value into a byte vector.
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

/// Trait implemented by types that can be reconstructed from bytes.
pub trait BinaryDeserializable: Sized {
    /// Reconstructs the value from bytes.
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}

/// Trait implemented by types supporting textual serialization.
pub trait TextSerializable {
    /// Serializes the value into UTF-8 text.
    fn to_text(&self) -> Result<String>;
}

/// Trait implemented by types supporting textual deserialization.
pub trait TextDeserializable: Sized {
    /// Reconstructs a value from UTF-8 text.
    fn from_text(text: &str) -> Result<Self>;
}

/// Copies bytes into an owned buffer.
#[must_use]
pub fn clone_bytes(bytes: &[u8]) -> Vec<u8> {
    bytes.to_vec()
}

/// Converts UTF-8 text into bytes.
#[must_use]
pub fn text_to_bytes(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}

/// Converts UTF-8 bytes into text.
pub fn bytes_to_text(bytes: &[u8]) -> Result<String> {
    String::from_utf8(bytes.to_vec()).map_err(|e| serialization_error("invalid UTF-8 sequence", e))
}

/// Returns true when two serialized byte sequences are identical.
#[must_use]
pub fn bytes_equal(left: &[u8], right: &[u8]) -> bool {
    left == right
}

/// Serializes any value implementing [`BinarySerializable`].
pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
where
    T: BinarySerializable,
{
    value.to_bytes()
}

/// Deserializes a value implementing [`BinaryDeserializable`].
pub fn deserialize<T>(bytes: &[u8]) -> Result<T>
where
    T: BinaryDeserializable,
{
    T::from_bytes(bytes)
}

/// A trivial binary wrapper useful for tests and raw payloads.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BinaryBlob {
    bytes: Vec<u8>,
}

impl BinaryBlob {
    /// Creates a new binary blob.
    #[must_use]
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// Returns the contained bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl BinarySerializable for BinaryBlob {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.bytes.clone())
    }
}

impl BinaryDeserializable for BinaryBlob {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self::new(bytes))
    }
}

impl TextSerializable for BinaryBlob {
    fn to_text(&self) -> Result<String> {
        bytes_to_text(&self.bytes)
    }
}

impl TextDeserializable for BinaryBlob {
    fn from_text(text: &str) -> Result<Self> {
        Ok(Self::new(text.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_bytes_copies_data() {
        let bytes = [1u8, 2, 3];

        let cloned = clone_bytes(&bytes);

        assert_eq!(cloned, vec![1, 2, 3]);
    }

    #[test]
    fn utf8_roundtrip() {
        let original = "Origin Observer";

        let bytes = text_to_bytes(original);

        let restored = bytes_to_text(&bytes).unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn binary_blob_roundtrip() {
        let blob = BinaryBlob::new([10u8, 20, 30]);

        let encoded = serialize(&blob).unwrap();

        let decoded = deserialize::<BinaryBlob>(&encoded).unwrap();

        assert_eq!(blob, decoded);
    }

    #[test]
    fn bytes_comparison() {
        assert!(bytes_equal(&[1, 2, 3], &[1, 2, 3]));

        assert!(!bytes_equal(&[1, 2, 3], &[3, 2, 1]));
    }

    #[test]
    fn invalid_utf8_is_rejected() {
        let invalid = [0xff, 0xfe];

        assert!(bytes_to_text(&invalid).is_err());
    }
}
