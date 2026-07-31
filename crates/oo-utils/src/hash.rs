// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-utils/src/hash.rs
// Purpose : Deterministic integrity digests for observations and evidence.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Deterministic integrity digests for observations and evidence.
//!
//! Every accepted observation carries an integrity digest so a later reader can
//! confirm that the material behind a finding is the material that was
//! observed. Digests are therefore computed the same way everywhere: SHA-256
//! over length-prefixed parts, so two different groupings of the same bytes
//! cannot produce the same digest.

use std::fmt;

use sha2::{Digest as _, Sha256};

/// Algorithm identifier recorded alongside every digest.
pub const DIGEST_ALGORITHM: &str = "sha256";

/// A content digest in lowercase hexadecimal form.
///
/// The algorithm is part of the displayed value (`sha256:abcd…`) so evidence
/// records stay readable when the project later adds a second algorithm.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest {
    hex: String,
}

impl Digest {
    /// Computes the digest of a byte slice.
    #[must_use]
    pub fn of_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        Self {
            hex: hex::encode(hasher.finalize()),
        }
    }

    /// Computes the digest of a string.
    #[must_use]
    pub fn of_str(value: &str) -> Self {
        Self::of_bytes(value.as_bytes())
    }

    /// Computes the digest of an ordered sequence of parts.
    ///
    /// Each part is length-prefixed, so `["ab", "c"]` and `["a", "bc"]` produce
    /// different digests. Without that prefix an evidence record could be
    /// re-partitioned without changing its digest.
    #[must_use]
    pub fn of_parts<'a, I>(parts: I) -> Self
    where
        I: IntoIterator<Item = &'a [u8]>,
    {
        let mut hasher = Sha256::new();
        for part in parts {
            hasher.update((part.len() as u64).to_be_bytes());
            hasher.update(part);
        }
        Self {
            hex: hex::encode(hasher.finalize()),
        }
    }

    /// Computes the digest of an ordered sequence of string parts.
    #[must_use]
    pub fn of_str_parts<'a, I>(parts: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        Self::of_parts(parts.into_iter().map(str::as_bytes))
    }

    /// Returns the hexadecimal digest without its algorithm prefix.
    #[must_use]
    pub fn hex(&self) -> &str {
        &self.hex
    }

    /// Returns the digest in `algorithm:hex` form for evidence records.
    #[must_use]
    pub fn qualified(&self) -> String {
        format!("{DIGEST_ALGORITHM}:{}", self.hex)
    }

    /// Returns the first `length` hexadecimal characters, for display only.
    ///
    /// A shortened digest identifies a record in a report; it never replaces the
    /// full digest in an evidence field.
    #[must_use]
    pub fn short(&self, length: usize) -> &str {
        let end = length.min(self.hex.len());
        &self.hex[..end]
    }

    /// Returns whether this digest matches the given bytes.
    #[must_use]
    pub fn verifies(&self, bytes: &[u8]) -> bool {
        Self::of_bytes(bytes) == *self
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_is_stable_for_the_same_input() {
        assert_eq!(Digest::of_str("origin"), Digest::of_str("origin"));
    }

    #[test]
    fn digest_matches_the_known_sha256_of_an_empty_input() {
        assert_eq!(
            Digest::of_bytes(&[]).hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn different_inputs_produce_different_digests() {
        assert_ne!(Digest::of_str("usdt"), Digest::of_str("usdc"));
    }

    #[test]
    fn parts_are_length_prefixed_so_regrouping_changes_the_digest() {
        // Without length prefixing these two would collide, and an evidence
        // record could be re-partitioned without invalidating its digest.
        assert_ne!(
            Digest::of_str_parts(["ab", "c"]),
            Digest::of_str_parts(["a", "bc"])
        );
    }

    #[test]
    fn part_order_changes_the_digest() {
        assert_ne!(
            Digest::of_str_parts(["chain", "asset"]),
            Digest::of_str_parts(["asset", "chain"])
        );
    }

    #[test]
    fn qualified_form_names_its_algorithm() {
        let digest = Digest::of_str("origin");
        assert_eq!(digest.qualified(), format!("sha256:{}", digest.hex()));
        assert_eq!(digest.to_string(), digest.qualified());
    }

    #[test]
    fn verification_accepts_the_original_bytes_and_rejects_others() {
        let digest = Digest::of_bytes(b"observed");
        assert!(digest.verifies(b"observed"));
        assert!(!digest.verifies(b"observed "));
    }

    #[test]
    fn short_form_never_panics_on_a_long_request() {
        let digest = Digest::of_str("origin");
        assert_eq!(digest.short(8).len(), 8);
        assert_eq!(digest.short(1_000), digest.hex());
    }
}
