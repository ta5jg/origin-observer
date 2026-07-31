// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/error.rs
// Purpose : ABI error types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! ABI error types.

use thiserror::Error;

/// ABI crate result type.
pub type AbiResult<T> = Result<T, AbiError>;

/// Errors produced while building signatures, decoding data or validating an
/// ABI fragment.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AbiError {
    /// A parameter type is not one this crate can encode, decode or include
    /// in a canonical signature.
    ///
    /// The Solidity type grammar is open-ended (arbitrary tuples, nested
    /// arrays); this crate covers the fixed-width and single-dynamic-value
    /// subset that answers WDRP's actual questions. Refusing an unsupported
    /// type is the honest outcome; guessing at its encoding is not.
    #[error("unsupported ABI type: {0}")]
    UnsupportedType(String),

    /// A function or event name was empty or contained characters outside a
    /// Solidity identifier.
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),

    /// Call data or return data was not valid hexadecimal.
    #[error("ABI data is not valid hexadecimal: {0}")]
    InvalidHex(String),

    /// Encoded data was shorter than the type it was decoded against
    /// requires.
    #[error(
        "ABI data is too short to decode a {expected}: expected at least {expected_bytes} byte(s), found {found_bytes}"
    )]
    DataTooShort {
        /// Type that was being decoded.
        expected: String,
        /// Bytes required.
        expected_bytes: usize,
        /// Bytes available.
        found_bytes: usize,
    },

    /// A decoded dynamic-type offset pointed outside the data.
    #[error("ABI dynamic offset {offset} is outside the {length}-byte data")]
    OffsetOutOfRange {
        /// Offset that was read.
        offset: usize,
        /// Total data length.
        length: usize,
    },

    /// A decoded value did not satisfy the type's own encoding constraints,
    /// such as an address whose top 12 bytes were not zero.
    #[error("decoded value does not satisfy the {0} encoding")]
    MalformedValue(String),
}
