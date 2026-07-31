// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/opcode.rs
// Purpose : EVM opcode classification and instruction decoding.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! EVM opcode classification and instruction decoding.
//!
//! Bytecode is not a flat byte array to scan for signatures: a `PUSH1..PUSH32`
//! opcode is followed by its own immediate data, and a byte equal to
//! `DELEGATECALL`'s opcode inside a PUSH's immediate is data, not an
//! instruction. `decode` walks the code the way the EVM does, so later
//! analysis sees instructions rather than a coincidence.

use crate::error::{BytecodeError, BytecodeResult};

/// Opcodes this crate distinguishes by name.
///
/// The EVM defines roughly 140 opcodes; only the ones relevant to proxy
/// detection, security signalling and control flow are named here. Everything
/// else decodes as [`Opcode::Other`], which still advances correctly because
/// `push_size` is derived from the raw byte, not from the name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Stop,
    Jump,
    JumpI,
    JumpDest,
    SLoad,
    SStore,
    Call,
    StaticCall,
    CallCode,
    DelegateCall,
    Create,
    Create2,
    Return,
    Revert,
    SelfDestruct,
    CodeCopy,
    ExtCodeSize,
    ExtCodeCopy,
    ExtCodeHash,
    Push(u8),
    Dup(u8),
    Swap(u8),
    Log(u8),
    /// Any opcode not named above.
    Other(u8),
}

impl Opcode {
    /// Decodes the opcode at a raw byte.
    #[must_use]
    pub const fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => Self::Stop,
            0x39 => Self::CodeCopy,
            0x3b => Self::ExtCodeSize,
            0x3c => Self::ExtCodeCopy,
            0x3f => Self::ExtCodeHash,
            0x54 => Self::SLoad,
            0x55 => Self::SStore,
            0x56 => Self::Jump,
            0x57 => Self::JumpI,
            0x5b => Self::JumpDest,
            0x60..=0x7f => Self::Push(byte - 0x5f),
            0x80..=0x8f => Self::Dup(byte - 0x7f),
            0x90..=0x9f => Self::Swap(byte - 0x8f),
            0xa0..=0xa4 => Self::Log(byte - 0xa0),
            0xf0 => Self::Create,
            0xf1 => Self::Call,
            0xf2 => Self::CallCode,
            0xf3 => Self::Return,
            0xf4 => Self::DelegateCall,
            0xf5 => Self::Create2,
            0xfa => Self::StaticCall,
            0xfd => Self::Revert,
            0xff => Self::SelfDestruct,
            other => Self::Other(other),
        }
    }

    /// Returns the number of immediate bytes this opcode consumes.
    ///
    /// Only `PUSH1..PUSH32` carries immediate data; every other opcode is
    /// exactly one byte.
    #[must_use]
    pub const fn immediate_len(self) -> usize {
        match self {
            Self::Push(size) => size as usize,
            _ => 0,
        }
    }

    /// Returns whether this opcode ends execution of the current call frame.
    #[must_use]
    pub const fn is_terminator(self) -> bool {
        matches!(
            self,
            Self::Stop | Self::Return | Self::Revert | Self::SelfDestruct
        )
    }
}

/// One decoded instruction: its opcode, byte offset and immediate data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// Offset of the opcode byte within the code.
    pub offset: usize,
    /// Decoded opcode.
    pub opcode: Opcode,
    /// Immediate bytes following a `PUSH` opcode, empty otherwise.
    pub immediate: Vec<u8>,
}

/// Decodes bytecode into an ordered instruction sequence.
///
/// Decoding stops with an error rather than a partial result when a `PUSH`
/// opcode names more immediate bytes than the code contains: the input was
/// truncated, and a partial instruction list would misrepresent it as valid.
pub fn decode(code: &[u8]) -> BytecodeResult<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut offset = 0;

    while offset < code.len() {
        let opcode = Opcode::from_byte(code[offset]);
        let immediate_len = opcode.immediate_len();
        let available = code.len() - offset - 1;

        if immediate_len > available {
            return Err(BytecodeError::TruncatedPush {
                offset,
                expected: immediate_len,
                found: available,
            });
        }

        let immediate = code[offset + 1..offset + 1 + immediate_len].to_vec();
        instructions.push(Instruction {
            offset,
            opcode,
            immediate,
        });
        offset += 1 + immediate_len;
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_opcodes_report_their_immediate_length() {
        assert_eq!(Opcode::from_byte(0x60).immediate_len(), 1); // PUSH1
        assert_eq!(Opcode::from_byte(0x7f).immediate_len(), 32); // PUSH32
        assert_eq!(Opcode::Stop.immediate_len(), 0);
    }

    #[test]
    fn a_byte_inside_a_push_immediate_is_not_decoded_as_an_instruction() {
        // PUSH1 0xf4 must not be read as a DELEGATECALL instruction: 0xf4 here
        // is data, one position past the PUSH opcode itself.
        let code = [0x60, 0xf4, 0x00];
        let instructions = decode(&code).unwrap();
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0].opcode, Opcode::Push(1));
        assert_eq!(instructions[0].immediate, vec![0xf4]);
        assert_eq!(instructions[1].opcode, Opcode::Stop);
        assert_eq!(instructions[1].offset, 2);
    }

    #[test]
    fn a_truncated_push_is_an_explicit_error_not_a_partial_result() {
        let code = [0x7f, 0x01, 0x02]; // PUSH32 with only 2 of 32 bytes present
        let error = decode(&code).unwrap_err();
        assert_eq!(
            error,
            BytecodeError::TruncatedPush {
                offset: 0,
                expected: 32,
                found: 2
            }
        );
    }

    #[test]
    fn terminators_are_recognized() {
        for byte in [0x00, 0xf3, 0xfd, 0xff] {
            assert!(Opcode::from_byte(byte).is_terminator(), "{byte:#04x}");
        }
        assert!(!Opcode::from_byte(0xf4).is_terminator());
    }

    #[test]
    fn dup_swap_and_log_carry_their_operand_count() {
        assert_eq!(Opcode::from_byte(0x80), Opcode::Dup(1));
        assert_eq!(Opcode::from_byte(0x8f), Opcode::Dup(16));
        assert_eq!(Opcode::from_byte(0x90), Opcode::Swap(1));
        assert_eq!(Opcode::from_byte(0xa4), Opcode::Log(4));
    }

    #[test]
    fn an_empty_code_decodes_to_no_instructions() {
        assert_eq!(decode(&[]).unwrap(), Vec::new());
    }
}
