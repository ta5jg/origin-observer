// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-dataset/src/manifest.rs
// Purpose : Describe a dataset: its schema, version, size and digest.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Describe a dataset: its schema, version, size and digest.

use oo_core::Digest;

use crate::schema::DatasetSchema;
use crate::versioning::DatasetVersion;

/// A complete description of one dataset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetManifest {
    name: String,
    schema: DatasetSchema,
    version: DatasetVersion,
    record_count: usize,
    digest: Digest,
}

impl DatasetManifest {
    /// Describes a dataset.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        schema: DatasetSchema,
        version: DatasetVersion,
        record_count: usize,
        digest: Digest,
    ) -> Self {
        Self {
            name: name.into(),
            schema,
            version,
            record_count,
            digest,
        }
    }

    /// Returns the dataset's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the dataset's schema.
    #[must_use]
    pub const fn schema(&self) -> &DatasetSchema {
        &self.schema
    }

    /// Returns the dataset's version.
    #[must_use]
    pub const fn version(&self) -> DatasetVersion {
        self.version
    }

    /// Returns the declared record count.
    #[must_use]
    pub const fn record_count(&self) -> usize {
        self.record_count
    }

    /// Returns the declared content digest.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.digest
    }

    /// Returns whether this manifest describes a record set with the given
    /// count and digest.
    #[must_use]
    pub fn describes(&self, record_count: usize, digest: Digest) -> bool {
        self.record_count == record_count && self.digest.as_bytes() == digest.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::{DatasetField, FieldType};

    use super::*;

    fn manifest() -> DatasetManifest {
        DatasetManifest::new(
            "recognition-cases",
            DatasetSchema::new(vec![DatasetField::new("wallet_id", FieldType::Text)]),
            DatasetVersion::new(1, 0),
            3,
            Digest::from([7u8; 32]),
        )
    }

    #[test]
    fn a_manifest_describes_the_record_count_and_digest_it_was_built_with() {
        let manifest = manifest();
        assert!(manifest.describes(3, Digest::from([7u8; 32])));
    }

    #[test]
    fn a_manifest_does_not_describe_a_different_record_count() {
        let manifest = manifest();
        assert!(!manifest.describes(4, Digest::from([7u8; 32])));
    }

    #[test]
    fn a_manifest_does_not_describe_a_different_digest() {
        let manifest = manifest();
        assert!(!manifest.describes(3, Digest::from([9u8; 32])));
    }
}
