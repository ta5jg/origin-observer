// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-utils/src/error.rs
// Purpose : Utility error types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Utility error types.

use thiserror::Error;

/// Utility crate result type.
pub type UtilsResult<T> = Result<T, UtilsError>;

/// Errors produced by the shared utilities.
///
/// Every variant names the value that failed, because a research tool that
/// reports "invalid input" without identifying the input cannot be debugged
/// from its own output.
#[derive(Debug, Error)]
pub enum UtilsError {
    /// A required value was empty.
    #[error("{field} must not be empty")]
    Empty {
        /// Name of the field that was empty.
        field: String,
    },

    /// A value exceeded its permitted length.
    #[error("{field} is {actual} characters, exceeding the maximum of {maximum}")]
    TooLong {
        /// Name of the field that was too long.
        field: String,
        /// Observed length.
        actual: usize,
        /// Permitted length.
        maximum: usize,
    },

    /// A value did not match its required shape.
    #[error("{field} does not match the required form: {expected}")]
    Malformed {
        /// Name of the field that was malformed.
        field: String,
        /// Human-readable description of the expected form.
        expected: String,
    },

    /// A filesystem operation failed.
    #[error("filesystem operation failed for {path}: {source}")]
    Filesystem {
        /// Path the operation targeted.
        path: String,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

impl UtilsError {
    /// Creates an [`UtilsError::Empty`] for a named field.
    #[must_use]
    pub fn empty(field: impl Into<String>) -> Self {
        Self::Empty {
            field: field.into(),
        }
    }

    /// Creates an [`UtilsError::TooLong`] for a named field.
    #[must_use]
    pub fn too_long(field: impl Into<String>, actual: usize, maximum: usize) -> Self {
        Self::TooLong {
            field: field.into(),
            actual,
            maximum,
        }
    }

    /// Creates an [`UtilsError::Malformed`] for a named field.
    #[must_use]
    pub fn malformed(field: impl Into<String>, expected: impl Into<String>) -> Self {
        Self::Malformed {
            field: field.into(),
            expected: expected.into(),
        }
    }

    /// Creates an [`UtilsError::Filesystem`] for a path.
    #[must_use]
    pub fn filesystem(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::Filesystem {
            path: path.into(),
            source,
        }
    }
}
