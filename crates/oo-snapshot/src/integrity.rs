// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/integrity.rs
// Purpose : Snapshot integrity helpers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot integrity helpers.

use oo_core::Digest;
use sha2::{Digest as ShaDigest, Sha256};

/// Computes a SHA-256 digest for snapshot bytes.
#[must_use]
pub fn digest_bytes(bytes: &[u8]) -> Digest {
    let hash = Sha256::digest(bytes);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);
    Digest::new(out)
}
