// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/result.rs
// Purpose : Canonical Result type and helper extensions for Origin Observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Canonical `Result` implementation shared by every Origin Observer crate.

use crate::error::{
    internal, invalid_argument, invalid_data, invalid_state, network_error, not_found,
    provider_unavailable, timeout, Error,
};

/// Canonical Origin Observer result type.
pub type Result<T> = core::result::Result<T, Error>;

/// Extension methods implemented for every Origin Observer result.
pub trait ResultExt<T> {
    /// Returns true when the result is successful.
    fn succeeded(&self) -> bool;

    /// Returns true when the result contains an error.
    fn failed(&self) -> bool;

    /// Returns the contained value or panics with a descriptive message.
    fn expect_success(self, message: &str) -> T;

    /// Maps the contained value.
    fn map_success<U, F>(self, op: F) -> Result<U>
    where
        F: FnOnce(T) -> U;

    /// Executes a closure only when the result is successful.
    fn inspect_success<F>(self, op: F) -> Self
    where
        F: FnOnce(&T),
        Self: Sized;

    /// Executes a closure only when the result is an error.
    fn inspect_error<F>(self, op: F) -> Self
    where
        F: FnOnce(&Error),
        Self: Sized;
}

impl<T> ResultExt<T> for Result<T> {
    fn succeeded(&self) -> bool {
        self.is_ok()
    }

    fn failed(&self) -> bool {
        self.is_err()
    }

    fn expect_success(self, message: &str) -> T {
        match self {
            Ok(value) => value,
            Err(error) => panic!("{message}: {error}"),
        }
    }

    fn map_success<U, F>(self, op: F) -> Result<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(op)
    }

    fn inspect_success<F>(self, op: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Ok(ref value) = self {
            op(value);
        }

        self
    }

    fn inspect_error<F>(self, op: F) -> Self
    where
        F: FnOnce(&Error),
    {
        if let Err(ref error) = self {
            op(error);
        }

        self
    }
}

/// Returns a successful result.
#[must_use]
pub fn success<T>(value: T) -> Result<T> {
    Ok(value)
}

/// Returns an error result.
#[must_use]
pub fn failure<T>(error: Error) -> Result<T> {
    Err(error)
}

/// Creates a successful unit result.
#[must_use]
pub fn done() -> Result<()> {
    Ok(())
}

/// Returns an invalid argument error result.
#[must_use]
pub fn invalid_argument_result<T>(message: impl Into<String>) -> Result<T> {
    Err(invalid_argument(message))
}

/// Returns an invalid state error result.
#[must_use]
pub fn invalid_state_result<T>(message: impl Into<String>) -> Result<T> {
    Err(invalid_state(message))
}

/// Returns an invalid data error result.
#[must_use]
pub fn invalid_data_result<T>(message: impl Into<String>) -> Result<T> {
    Err(invalid_data(message))
}

/// Returns a not found error result.
#[must_use]
pub fn not_found_result<T>(message: impl Into<String>) -> Result<T> {
    Err(not_found(message))
}

/// Returns a network error result.
#[must_use]
pub fn network_error_result<T>(message: impl Into<String>) -> Result<T> {
    Err(network_error(message))
}

/// Returns a timeout error result.
#[must_use]
pub fn timeout_result<T>(message: impl Into<String>) -> Result<T> {
    Err(timeout(message))
}

/// Returns a provider unavailable error result.
#[must_use]
pub fn provider_unavailable_result<T>(message: impl Into<String>) -> Result<T> {
    Err(provider_unavailable(message))
}

/// Returns an internal error result.
#[must_use]
pub fn internal_result<T>(message: impl Into<String>) -> Result<T> {
    Err(internal(message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_result() {
        let result = success(42);

        assert!(result.succeeded());
        assert!(!result.failed());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn failed_result() {
        let result: Result<u32> = invalid_argument_result("invalid");

        assert!(result.failed());
        assert!(!result.succeeded());
    }

    #[test]
    fn map_success() {
        let result = success(10).map_success(|value| value * 2);

        assert_eq!(result.unwrap(), 20);
    }

    #[test]
    fn inspect_success() {
        let mut called = false;

        let result = success(7).inspect_success(|_| {
            called = true;
        });

        assert!(called);
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn inspect_error() {
        let mut called = false;

        let _: Result<()> = invalid_argument_result("x").inspect_error(|_| {
            called = true;
        });

        assert!(called);
    }

    #[test]
    fn done_returns_ok() {
        assert!(done().is_ok());
    }
}
