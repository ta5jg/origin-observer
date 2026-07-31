// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/error.rs
// Purpose : Bytecode error types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Bytecode error types.

use thiserror::Error;

/// Bytecode crate result type.
pub type BytecodeResult<T> = Result<T, BytecodeError>;

/// Errors produced while normalizing, decoding or analyzing bytecode.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BytecodeError {
    /// The input was not a valid hexadecimal string.
    #[error("bytecode is not valid hexadecimal: {0}")]
    InvalidHex(String),

    /// The bytecode ends mid-instruction: a `PUSH` opcode named more
    /// immediate bytes than the code actually contains.
    ///
    /// Truncated bytecode is not a smaller contract; it is evidence the
    /// capture was incomplete, and treating it as complete would produce a
    /// wrong opcode histogram.
    #[error("bytecode is truncated: PUSH at offset {offset} expects {expected} more byte(s), found {found}")]
    TruncatedPush {
        /// Byte offset of the PUSH opcode.
        offset: usize,
        /// Immediate bytes the opcode expects.
        expected: usize,
        /// Immediate bytes actually available.
        found: usize,
    },
}
