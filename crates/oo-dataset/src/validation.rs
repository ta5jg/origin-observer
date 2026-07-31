// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-dataset/src/validation.rs
// Purpose : Validate that a dataset's records match their manifest.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate that a dataset's records match their manifest.

use serde::Serialize;

use crate::integrity::compute_digest;
use crate::manifest::DatasetManifest;

/// One way a dataset's records can fail to match their manifest.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DatasetValidationError {
    /// The manifest's declared record count does not match the number of
    /// records provided.
    #[error("manifest declares {declared} records but {actual} were provided")]
    RecordCountMismatch {
        /// The count the manifest declared.
        declared: usize,
        /// The count actually provided.
        actual: usize,
    },
    /// The records could not be serialized to compute their digest.
    #[error("records could not be serialized to compute their digest: {0}")]
    Unserializable(String),
    /// The computed digest does not match the manifest's declared digest.
    #[error("manifest digest does not match the digest of the provided records")]
    DigestMismatch,
}

/// Validates that a set of records matches its manifest's declared count and
/// digest.
///
/// # Errors
///
/// Returns [`DatasetValidationError`] for a count or digest mismatch, or an
/// unserializable record.
pub fn validate_records<T: Serialize>(
    manifest: &DatasetManifest,
    records: &[T],
) -> Result<(), DatasetValidationError> {
    if manifest.record_count() != records.len() {
        return Err(DatasetValidationError::RecordCountMismatch {
            declared: manifest.record_count(),
            actual: records.len(),
        });
    }

    let digest = compute_digest(records)
        .map_err(|error| DatasetValidationError::Unserializable(error.to_string()))?;

    if digest.as_bytes() != manifest.digest().as_bytes() {
        return Err(DatasetValidationError::DigestMismatch);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;
    use crate::schema::DatasetSchema;
    use crate::versioning::DatasetVersion;

    #[derive(Serialize)]
    struct Record {
        name: &'static str,
    }

    #[test]
    fn records_matching_their_manifest_are_valid() {
        let records = [Record { name: "a" }];
        let digest = compute_digest(&records).unwrap();
        let manifest = DatasetManifest::new(
            "d",
            DatasetSchema::default(),
            DatasetVersion::new(1, 0),
            1,
            digest,
        );
        assert!(validate_records(&manifest, &records).is_ok());
    }

    #[test]
    fn a_record_count_mismatch_is_rejected() {
        let records = [Record { name: "a" }, Record { name: "b" }];
        let digest = compute_digest(&records).unwrap();
        let manifest = DatasetManifest::new(
            "d",
            DatasetSchema::default(),
            DatasetVersion::new(1, 0),
            1,
            digest,
        );
        assert!(matches!(
            validate_records(&manifest, &records),
            Err(DatasetValidationError::RecordCountMismatch {
                declared: 1,
                actual: 2
            })
        ));
    }

    #[test]
    fn a_digest_mismatch_is_rejected() {
        let records = [Record { name: "a" }];
        let manifest = DatasetManifest::new(
            "d",
            DatasetSchema::default(),
            DatasetVersion::new(1, 0),
            1,
            oo_core::Digest::zero(),
        );
        assert_eq!(
            validate_records(&manifest, &records),
            Err(DatasetValidationError::DigestMismatch)
        );
    }
}
