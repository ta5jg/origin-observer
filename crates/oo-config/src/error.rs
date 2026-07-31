// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-config/src/error.rs
// Purpose : Configuration error types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Configuration error types.

use thiserror::Error;

/// Configuration crate result type.
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Errors produced while loading, overriding or validating configuration.
///
/// A configuration failure always names the file and the field, because an
/// observation run that starts from the wrong configuration produces evidence
/// that looks valid and is not.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// A required configuration file was missing.
    #[error("required configuration file was not found: {path}")]
    MissingFile {
        /// Path that was expected to exist.
        path: String,
    },

    /// A configuration file could not be read.
    #[error("configuration file could not be read: {path}: {source}")]
    Unreadable {
        /// Path that failed to read.
        path: String,
        /// Underlying utility error.
        #[source]
        source: oo_utils::UtilsError,
    },

    /// A configuration file was not valid TOML or did not match the schema.
    #[error("configuration file {path} is invalid: {message}")]
    Invalid {
        /// Path that failed to parse.
        path: String,
        /// Parser message.
        message: String,
    },

    /// An environment override could not be interpreted.
    #[error("environment variable {variable} is invalid: expected {expected}, found {found}")]
    InvalidOverride {
        /// Environment variable name.
        variable: String,
        /// Description of the accepted form.
        expected: String,
        /// Value that was rejected. Never a secret: secret variables are not
        /// parsed into typed values and so are never reported here.
        found: String,
    },

    /// The assembled configuration violates a project invariant.
    #[error("configuration is invalid: {message}")]
    Rejected {
        /// Explanation naming the field and the invariant it broke.
        message: String,
    },
}

impl ConfigError {
    /// Creates a [`ConfigError::Invalid`] for a path.
    #[must_use]
    pub fn invalid(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Invalid {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Creates a [`ConfigError::Rejected`] with an explanation.
    #[must_use]
    pub fn rejected(message: impl Into<String>) -> Self {
        Self::Rejected {
            message: message.into(),
        }
    }
}

impl From<oo_utils::UtilsError> for ConfigError {
    fn from(error: oo_utils::UtilsError) -> Self {
        Self::Rejected {
            message: error.to_string(),
        }
    }
}
