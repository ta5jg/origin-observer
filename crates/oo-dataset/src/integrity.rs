// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-dataset/src/integrity.rs
// Purpose : Compute a deterministic content digest over a dataset's records.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Compute a deterministic content digest over a dataset's records.
//!
//! Each record is serialized to JSON and length-prefixed before hashing, the
//! same discipline `oo_utils::hash` uses for evidence digests: without a
//! length prefix, two different record boundaries could serialize to the
//! same concatenated bytes and collide.

use oo_core::Digest;
use serde::Serialize;
use sha2::{Digest as _, Sha256};

/// Failure computing a dataset digest.
#[derive(Debug, thiserror::Error)]
pub enum IntegrityError {
    /// A record could not be serialized to JSON.
    #[error("record could not be serialized for digest computation: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// Computes a deterministic digest over an ordered sequence of records.
///
/// # Errors
///
/// Returns [`IntegrityError::Serialize`] if any record fails to serialize.
pub fn compute_digest<T: Serialize>(records: &[T]) -> Result<Digest, IntegrityError> {
    let mut hasher = Sha256::new();
    for record in records {
        let bytes = serde_json::to_vec(record)?;
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(&bytes);
    }
    let digest: [u8; 32] = hasher.finalize().into();
    Ok(Digest::from(digest))
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[derive(Serialize)]
    struct Record {
        name: &'static str,
    }

    #[test]
    fn the_same_records_produce_the_same_digest() {
        let records = [Record { name: "a" }, Record { name: "b" }];
        assert_eq!(
            compute_digest(&records).unwrap(),
            compute_digest(&records).unwrap()
        );
    }

    #[test]
    fn different_records_produce_different_digests() {
        let first = [Record { name: "a" }];
        let second = [Record { name: "b" }];
        assert_ne!(
            compute_digest(&first).unwrap(),
            compute_digest(&second).unwrap()
        );
    }

    #[test]
    fn regrouping_records_changes_the_digest() {
        let split = [Record { name: "ab" }, Record { name: "c" }];
        let merged = [Record { name: "a" }, Record { name: "bc" }];
        assert_ne!(
            compute_digest(&split).unwrap(),
            compute_digest(&merged).unwrap()
        );
    }

    #[test]
    fn an_empty_record_set_still_produces_a_digest() {
        let records: [Record; 0] = [];
        assert!(!compute_digest(&records).unwrap().is_zero());
    }
}
