// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-dataset/src/import.rs
// Purpose : Deserialize dataset records from their JSON representation.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Deserialize dataset records from their JSON representation.

use serde::de::DeserializeOwned;

/// Failure importing dataset records.
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    /// The input was not valid JSON, or did not match the target type.
    #[error("dataset content could not be parsed: {0}")]
    Parse(#[from] serde_json::Error),
}

/// Imports a JSON array of records into typed values.
///
/// # Errors
///
/// Returns [`ImportError::Parse`] if `json` is not a valid JSON array of `T`.
pub fn import_records<T: DeserializeOwned>(json: &str) -> Result<Vec<T>, ImportError> {
    Ok(serde_json::from_str(json)?)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Record {
        name: String,
    }

    #[test]
    fn a_well_formed_json_array_imports_into_typed_records() {
        let records: Vec<Record> = import_records(r#"[{"name":"a"},{"name":"b"}]"#).unwrap();
        assert_eq!(
            records,
            vec![
                Record {
                    name: "a".to_owned()
                },
                Record {
                    name: "b".to_owned()
                }
            ]
        );
    }

    #[test]
    fn malformed_json_is_an_explicit_error_not_an_empty_result() {
        let result: Result<Vec<Record>, ImportError> = import_records("not json");
        assert!(matches!(result, Err(ImportError::Parse(_))));
    }
}
