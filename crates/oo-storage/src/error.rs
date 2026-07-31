// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-storage/src/error.rs
// Purpose : Storage error types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Storage error types.

use thiserror::Error;

/// Storage crate result type.
pub type StorageResult<T> = Result<T, StorageError>;

/// Errors produced while reading, decoding or validating storage values.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StorageError {
    /// A storage value was not valid hexadecimal.
    #[error("storage value is not valid hexadecimal: {0}")]
    InvalidHex(String),

    /// A storage value was not exactly 32 bytes.
    ///
    /// The EVM's storage word is always 32 bytes; a shorter or longer value
    /// did not come from a genuine `eth_getStorageAt` response and must not
    /// be decoded as one.
    #[error("storage value is {found} byte(s), expected exactly 32")]
    WrongLength {
        /// Bytes actually present.
        found: usize,
    },

    /// A value decoded as an address had non-zero padding bytes.
    #[error("storage value does not encode an address: padding bytes are non-zero")]
    NotAnAddress,

    /// A value decoded as a boolean was neither zero nor one.
    #[error("storage value does not encode a bool: value is neither zero nor one")]
    NotABool,

    /// The RPC read failed.
    #[error("storage read failed: {0}")]
    Rpc(String),
}
