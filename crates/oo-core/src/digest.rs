// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/digest.rs
// Purpose : Cryptographic digest types and hashing utilities.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Strongly typed digest primitives used throughout Origin Observer.

use core::fmt;
use core::str::FromStr;

/// Length of a SHA-256 digest in bytes.
pub const SHA256_LENGTH: usize = 32;

/// A strongly typed SHA-256 digest.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest([u8; SHA256_LENGTH]);

impl Digest {
    /// Creates a digest from raw bytes.
    #[must_use]
    pub const fn new(bytes: [u8; SHA256_LENGTH]) -> Self {
        Self(bytes)
    }

    /// Returns an all-zero digest.
    #[must_use]
    pub const fn zero() -> Self {
        Self([0u8; SHA256_LENGTH])
    }

    /// Returns the underlying bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; SHA256_LENGTH] {
        &self.0
    }

    /// Returns true if the digest is all zeros.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|b| *b == 0)
    }

    /// Returns the hexadecimal representation.
    #[must_use]
    pub fn to_hex(&self) -> String {
        let mut out = String::with_capacity(64);

        for byte in self.0 {
            use core::fmt::Write;
            let _ = write!(&mut out, "{byte:02x}");
        }

        out
    }

    /// Parses a hexadecimal digest.
    pub fn from_hex(value: &str) -> Result<Self, DigestParseError> {
        if value.len() != 64 {
            return Err(DigestParseError::InvalidLength(value.len()));
        }

        let mut bytes = [0u8; SHA256_LENGTH];

        for i in 0..SHA256_LENGTH {
            bytes[i] = u8::from_str_radix(&value[i * 2..i * 2 + 2], 16)
                .map_err(|_| DigestParseError::InvalidHex)?;
        }

        Ok(Self(bytes))
    }
}

impl Default for Digest {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest({})", self.to_hex())
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl From<[u8; SHA256_LENGTH]> for Digest {
    fn from(value: [u8; SHA256_LENGTH]) -> Self {
        Self(value)
    }
}

impl From<Digest> for [u8; SHA256_LENGTH] {
    fn from(value: Digest) -> Self {
        value.0
    }
}

impl FromStr for Digest {
    type Err = DigestParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_hex(value)
    }
}

/// Errors produced while parsing hexadecimal digests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigestParseError {
    InvalidLength(usize),
    InvalidHex,
}

impl fmt::Display for DigestParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(len) => {
                write!(f, "expected 64 hexadecimal characters, found {}", len)
            }
            Self::InvalidHex => f.write_str("digest contains invalid hexadecimal characters"),
        }
    }
}

impl std::error::Error for DigestParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_digest_is_zero() {
        assert!(Digest::zero().is_zero());
    }

    #[test]
    fn digest_roundtrip_hex() {
        let mut bytes = [0u8; SHA256_LENGTH];

        for (i, b) in bytes.iter_mut().enumerate() {
            *b = i as u8;
        }

        let digest = Digest::new(bytes);

        let text = digest.to_hex();

        let parsed = Digest::from_hex(&text).unwrap();

        assert_eq!(digest, parsed);
    }

    #[test]
    fn invalid_length_is_rejected() {
        assert!(matches!(
            Digest::from_hex("abcd"),
            Err(DigestParseError::InvalidLength(_))
        ));
    }

    #[test]
    fn invalid_hex_is_rejected() {
        let value = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";

        assert!(matches!(
            Digest::from_hex(value),
            Err(DigestParseError::InvalidHex)
        ));
    }
}
