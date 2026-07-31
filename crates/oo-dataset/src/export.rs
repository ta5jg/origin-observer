// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-dataset/src/export.rs
// Purpose : Serialize dataset records to a deterministic JSON representation.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Serialize dataset records to a deterministic JSON representation.
//!
//! Field order in the output is fixed by the record type's declaration order,
//! not by a hash map, so exporting the same records twice always produces
//! byte-identical output — a requirement for the digest in
//! [`crate::integrity`] to mean anything.

use serde::Serialize;

/// Failure exporting dataset records.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    /// A record could not be serialized to JSON.
    #[error("dataset records could not be serialized: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// Exports records as a JSON array.
///
/// # Errors
///
/// Returns [`ExportError::Serialize`] if any record fails to serialize.
pub fn export_records<T: Serialize>(records: &[T]) -> Result<String, ExportError> {
    Ok(serde_json::to_string(records)?)
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
    fn exporting_the_same_records_twice_produces_identical_output() {
        let records = [Record { name: "a" }, Record { name: "b" }];
        assert_eq!(
            export_records(&records).unwrap(),
            export_records(&records).unwrap()
        );
    }

    #[test]
    fn export_output_is_a_json_array() {
        let records = [Record { name: "a" }];
        let json = export_records(&records).unwrap();
        assert!(json.starts_with('[') && json.ends_with(']'));
    }
}
