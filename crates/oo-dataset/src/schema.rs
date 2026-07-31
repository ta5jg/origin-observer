// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-dataset/src/schema.rs
// Purpose : Declare the fields a dataset's records are expected to carry.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Declare the fields a dataset's records are expected to carry.
//!
//! A schema does not enforce anything at (de)serialization time — the actual
//! record type does that through `serde`. It exists so a manifest can name
//! what a dataset contains before a caller deserializes a single record, and
//! so [`crate::validation`] can catch a schema that names the same field
//! twice.

use std::collections::BTreeSet;

/// A field's data type, as declared in a dataset schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    /// Free-form text.
    Text,
    /// A whole number.
    Integer,
    /// A floating-point number.
    Float,
    /// A boolean flag.
    Boolean,
    /// A point in time.
    Timestamp,
}

/// One field in a dataset schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetField {
    name: String,
    field_type: FieldType,
}

impl DatasetField {
    /// Declares a field.
    #[must_use]
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            field_type,
        }
    }

    /// Returns the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the field's data type.
    #[must_use]
    pub const fn field_type(&self) -> FieldType {
        self.field_type
    }
}

/// The set of fields a dataset's records are expected to carry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DatasetSchema {
    fields: Vec<DatasetField>,
}

impl DatasetSchema {
    /// Declares a schema from its fields.
    #[must_use]
    pub const fn new(fields: Vec<DatasetField>) -> Self {
        Self { fields }
    }

    /// Returns the declared fields.
    #[must_use]
    pub fn fields(&self) -> &[DatasetField] {
        &self.fields
    }

    /// Returns whether a field with the given name is declared.
    #[must_use]
    pub fn contains_field(&self, name: &str) -> bool {
        self.fields.iter().any(|field| field.name == name)
    }

    /// Returns whether any field name is declared more than once.
    #[must_use]
    pub fn has_duplicate_field_names(&self) -> bool {
        let mut seen = BTreeSet::new();
        !self
            .fields
            .iter()
            .all(|field| seen.insert(field.name.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_declared_field_is_found_by_name() {
        let schema = DatasetSchema::new(vec![DatasetField::new(
            "recognized_at",
            FieldType::Timestamp,
        )]);
        assert!(schema.contains_field("recognized_at"));
        assert!(!schema.contains_field("missing"));
    }

    #[test]
    fn duplicate_field_names_are_detected() {
        let schema = DatasetSchema::new(vec![
            DatasetField::new("id", FieldType::Text),
            DatasetField::new("id", FieldType::Integer),
        ]);
        assert!(schema.has_duplicate_field_names());
    }

    #[test]
    fn distinct_field_names_are_not_flagged_as_duplicates() {
        let schema = DatasetSchema::new(vec![
            DatasetField::new("id", FieldType::Text),
            DatasetField::new("count", FieldType::Integer),
        ]);
        assert!(!schema.has_duplicate_field_names());
    }
}
