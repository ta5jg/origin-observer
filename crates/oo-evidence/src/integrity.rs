// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/integrity.rs
// Purpose : Evidence integrity helpers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence integrity helpers.

use oo_core::Digest;
use sha2::{Digest as ShaDigest, Sha256};

/// Computes a SHA-256 digest for evidence source bytes.
#[must_use]
pub fn evidence_digest(bytes: &[u8]) -> Digest {
    let hash = Sha256::digest(bytes);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);
    Digest::new(out)
}
