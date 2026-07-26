// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/error.rs
// Purpose : Common error infrastructure shared by every Origin Observer crate.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Common error definitions used throughout Origin Observer.

use core::fmt;
use std::error::Error as StdError;

/// Convenient boxed error type.
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

/// Canonical Origin Observer error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    source: Option<BoxError>,
}

impl Error {
    /// Creates a new error.
    #[must_use]
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new error with an underlying source error.
    #[must_use]
    pub fn with_source<E>(kind: ErrorKind, message: impl Into<String>, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            kind,
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Returns the error category.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns true when the error is retryable.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::Network
                | ErrorKind::Timeout
                | ErrorKind::RateLimited
                | ErrorKind::ProviderUnavailable
        )
    }

    /// Returns true when the error is internal.
    #[must_use]
    pub const fn is_internal(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::Internal | ErrorKind::Io | ErrorKind::Serialization
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn StdError + 'static))
    }
}

/// Canonical Origin Observer error categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
    Unknown,

    InvalidArgument,
    InvalidState,
    InvalidData,

    NotFound,
    AlreadyExists,

    PermissionDenied,
    Unauthorized,

    Io,
    Serialization,

    Network,
    Timeout,
    RateLimited,
    ProviderUnavailable,

    Blockchain,
    Contract,
    Wallet,
    Address,

    Configuration,

    Internal,
}

/// Creates an invalid argument error.
#[must_use]
pub fn invalid_argument(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::InvalidArgument, message)
}

/// Creates an invalid state error.
#[must_use]
pub fn invalid_state(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::InvalidState, message)
}

/// Creates an invalid data error.
#[must_use]
pub fn invalid_data(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::InvalidData, message)
}

/// Creates a not found error.
#[must_use]
pub fn not_found(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::NotFound, message)
}

/// Creates an IO error.
#[must_use]
pub fn io_error<E>(message: impl Into<String>, source: E) -> Error
where
    E: StdError + Send + Sync + 'static,
{
    Error::with_source(ErrorKind::Io, message, source)
}

/// Creates a serialization error.
#[must_use]
pub fn serialization_error<E>(message: impl Into<String>, source: E) -> Error
where
    E: StdError + Send + Sync + 'static,
{
    Error::with_source(ErrorKind::Serialization, message, source)
}

/// Creates a network error.
#[must_use]
pub fn network_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Network, message)
}

/// Creates a timeout error.
#[must_use]
pub fn timeout(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Timeout, message)
}

/// Creates a provider unavailable error.
#[must_use]
pub fn provider_unavailable(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::ProviderUnavailable, message)
}

/// Creates an internal error.
#[must_use]
pub fn internal(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Internal, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_error() {
        let error = invalid_argument("invalid wallet");

        assert_eq!(error.kind(), ErrorKind::InvalidArgument);

        assert_eq!(error.message(), "invalid wallet");
    }

    #[test]
    fn retryable_errors() {
        assert!(network_error("network").is_retryable());

        assert!(timeout("timeout").is_retryable());

        assert!(provider_unavailable("provider").is_retryable());

        assert!(!invalid_argument("x").is_retryable());
    }

    #[test]
    fn internal_errors() {
        let err = internal("boom");

        assert!(err.is_internal());
    }

    #[test]
    fn display_contains_message() {
        let err = not_found("wallet");

        let text = err.to_string();

        assert!(text.contains("wallet"));
        assert!(text.contains("NotFound"));
    }
}
