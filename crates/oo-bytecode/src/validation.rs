// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/validation.rs
// Purpose : Validate bytecode observations before they enter analysis.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate bytecode observations before they enter analysis.

use crate::error::BytecodeResult;
use crate::opcode::decode;

/// Validates that bytecode decodes cleanly into complete instructions.
///
/// This is a thin wrapper over [`decode`]'s own truncation check, kept as a
/// named entry point so callers that only need a validity check are not
/// required to know that `decode` is where that check lives.
pub fn validate_bytecode(code: &[u8]) -> BytecodeResult<()> {
    decode(code).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::BytecodeError;

    #[test]
    fn well_formed_bytecode_is_valid() {
        assert!(validate_bytecode(&[0x60, 0x01, 0x00]).is_ok());
    }

    #[test]
    fn empty_bytecode_is_valid() {
        assert!(validate_bytecode(&[]).is_ok());
    }

    #[test]
    fn truncated_bytecode_is_invalid() {
        assert!(matches!(
            validate_bytecode(&[0x7f, 0x01]),
            Err(BytecodeError::TruncatedPush { .. })
        ));
    }
}
