// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/normalization.rs
// Purpose : Snapshot payload normalization.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Snapshot payload normalization.

use serde_json::Value;

/// Serializes a JSON payload into deterministic bytes for integrity hashing.
#[must_use]
pub fn normalize_json(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}
