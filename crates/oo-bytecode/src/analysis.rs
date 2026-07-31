// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/analysis.rs
// Purpose : Summarize decoded bytecode into security-relevant signals.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Summarize decoded bytecode into security-relevant signals.
//!
//! `BytecodeAnalysis` reports what is present, not what it means. A contract
//! containing `DELEGATECALL` might be a proxy, a library caller or something
//! else entirely; deciding which is `oo-proxy`'s job, working from evidence
//! this module makes available rather than guessing here.

use std::collections::BTreeMap;

use crate::error::BytecodeResult;
use crate::fingerprint::{content_digest, structural_fingerprint};
use crate::opcode::{decode, Opcode};
use oo_core::Digest;

/// Signals extracted from one bytecode observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytecodeAnalysis {
    /// Length of the code in bytes.
    pub length: usize,
    /// Digest of the exact bytes.
    pub content_digest: Digest,
    /// Digest that ignores embedded PUSH constants.
    pub structural_fingerprint: Digest,
    /// Whether the code contains `DELEGATECALL`.
    pub has_delegate_call: bool,
    /// Whether the code contains `CALLCODE`, the deprecated predecessor of
    /// `DELEGATECALL` that shares its risk profile.
    pub has_call_code: bool,
    /// Whether the code contains `SELFDESTRUCT`.
    pub has_self_destruct: bool,
    /// Whether the code contains `CREATE2`, relevant to deterministic
    /// deployment and address-prediction analysis.
    pub has_create2: bool,
    /// Whether every reachable terminator (`STOP`/`RETURN`/`REVERT`) is
    /// present in the code at all. `false` means the code never explicitly
    /// terminates, which is unusual and worth flagging rather than an
    /// analysis silently treating it as normal.
    pub has_terminator: bool,
    /// Count of each opcode by name, for opcodes distinguished by this crate.
    pub opcode_histogram: BTreeMap<&'static str, usize>,
}

/// Analyzes bytecode into its security-relevant signals.
pub fn analyze(code: &[u8]) -> BytecodeResult<BytecodeAnalysis> {
    let instructions = decode(code)?;

    let mut histogram: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut has_delegate_call = false;
    let mut has_call_code = false;
    let mut has_self_destruct = false;
    let mut has_create2 = false;
    let mut has_terminator = false;

    for instruction in &instructions {
        *histogram
            .entry(opcode_name(instruction.opcode))
            .or_default() += 1;
        match instruction.opcode {
            Opcode::DelegateCall => has_delegate_call = true,
            Opcode::CallCode => has_call_code = true,
            Opcode::SelfDestruct => has_self_destruct = true,
            Opcode::Create2 => has_create2 = true,
            other if other.is_terminator() => has_terminator = true,
            _ => {}
        }
    }

    Ok(BytecodeAnalysis {
        length: code.len(),
        content_digest: content_digest(code),
        structural_fingerprint: structural_fingerprint(code)?,
        has_delegate_call,
        has_call_code,
        has_self_destruct,
        has_create2,
        has_terminator,
        opcode_histogram: histogram,
    })
}

const fn opcode_name(opcode: Opcode) -> &'static str {
    match opcode {
        Opcode::Stop => "STOP",
        Opcode::Jump => "JUMP",
        Opcode::JumpI => "JUMPI",
        Opcode::JumpDest => "JUMPDEST",
        Opcode::SLoad => "SLOAD",
        Opcode::SStore => "SSTORE",
        Opcode::Call => "CALL",
        Opcode::StaticCall => "STATICCALL",
        Opcode::CallCode => "CALLCODE",
        Opcode::DelegateCall => "DELEGATECALL",
        Opcode::Create => "CREATE",
        Opcode::Create2 => "CREATE2",
        Opcode::Return => "RETURN",
        Opcode::Revert => "REVERT",
        Opcode::SelfDestruct => "SELFDESTRUCT",
        Opcode::CodeCopy => "CODECOPY",
        Opcode::ExtCodeSize => "EXTCODESIZE",
        Opcode::ExtCodeCopy => "EXTCODECOPY",
        Opcode::ExtCodeHash => "EXTCODEHASH",
        Opcode::Push(_) => "PUSH",
        Opcode::Dup(_) => "DUP",
        Opcode::Swap(_) => "SWAP",
        Opcode::Log(_) => "LOG",
        Opcode::Other(_) => "OTHER",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_account_analyzes_to_no_signals() {
        let analysis = analyze(&[]).unwrap();
        assert_eq!(analysis.length, 0);
        assert!(!analysis.has_delegate_call);
        assert!(!analysis.has_terminator);
        assert!(analysis.opcode_histogram.is_empty());
    }

    #[test]
    fn delegatecall_is_detected() {
        let code = [0xf4, 0x00]; // DELEGATECALL, STOP
        let analysis = analyze(&code).unwrap();
        assert!(analysis.has_delegate_call);
        assert!(analysis.has_terminator);
        assert_eq!(analysis.opcode_histogram.get("DELEGATECALL"), Some(&1));
    }

    #[test]
    fn a_byte_inside_a_push_immediate_does_not_trigger_a_false_signal() {
        // The DELEGATECALL opcode value appears here only as PUSH1 data.
        let code = [0x60, 0xf4, 0x00];
        let analysis = analyze(&code).unwrap();
        assert!(!analysis.has_delegate_call);
    }

    #[test]
    fn structural_fingerprint_is_included_and_ignores_constants() {
        let first = analyze(&[0x60, 0xaa, 0x00]).unwrap();
        let second = analyze(&[0x60, 0xbb, 0x00]).unwrap();
        assert_eq!(first.structural_fingerprint, second.structural_fingerprint);
        assert_ne!(first.content_digest, second.content_digest);
    }

    #[test]
    fn code_with_no_terminator_is_flagged() {
        let code = [0x60, 0x01]; // PUSH1 0x01, nothing else
        let analysis = analyze(&code).unwrap();
        assert!(!analysis.has_terminator);
    }

    #[test]
    fn truncated_bytecode_fails_rather_than_producing_a_partial_analysis() {
        assert!(analyze(&[0x7f, 0x01]).is_err());
    }
}
