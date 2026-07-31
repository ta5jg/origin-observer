// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/fingerprint.rs
// Purpose : Identify bytecode by content and by structure.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Identify bytecode by content and by structure.
//!
//! Two fingerprints answer two different questions. The content digest asks
//! "is this the exact same bytecode": one differing byte changes it. The
//! structural fingerprint asks "is this the same compiled logic with different
//! embedded constants": a minimal proxy for token A and the same template for
//! token B differ only in a 20-byte embedded address, and a content digest
//! would call them unrelated when they are the same pattern.

use oo_core::Digest;
use sha2::{Digest as ShaDigest, Sha256};

use crate::error::BytecodeResult;
use crate::opcode::{decode, Opcode};

/// Digests bytecode bytes exactly as observed.
#[must_use]
pub fn content_digest(code: &[u8]) -> Digest {
    let hash = Sha256::digest(code);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);
    Digest::new(out)
}

/// Digests bytecode with every `PUSH` immediate zeroed out.
///
/// This collapses embedded addresses, constants and offsets while preserving
/// the opcode sequence, so structurally identical bytecode compiled with
/// different constructor arguments produces the same fingerprint.
pub fn structural_fingerprint(code: &[u8]) -> BytecodeResult<Digest> {
    let instructions = decode(code)?;
    let mut hasher = Sha256::new();
    for instruction in &instructions {
        hasher.update([opcode_tag(instruction.opcode)]);
        if let Opcode::Push(size) = instruction.opcode {
            hasher.update([size]);
        }
    }
    let hash = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);
    Ok(Digest::new(out))
}

/// Maps an opcode to a stable single-byte tag for the structural fingerprint.
///
/// A raw opcode byte cannot be reused directly, because `Push(1)` and
/// `Push(2)` would otherwise contribute different tag bytes even though the
/// structural fingerprint is meant to treat "some push" uniformly aside from
/// its declared size, which is hashed separately.
const fn opcode_tag(opcode: Opcode) -> u8 {
    match opcode {
        Opcode::Stop => 0x00,
        Opcode::CodeCopy => 0x01,
        Opcode::ExtCodeSize => 0x02,
        Opcode::ExtCodeCopy => 0x03,
        Opcode::ExtCodeHash => 0x04,
        Opcode::SLoad => 0x05,
        Opcode::SStore => 0x06,
        Opcode::Jump => 0x07,
        Opcode::JumpI => 0x08,
        Opcode::JumpDest => 0x09,
        Opcode::Create => 0x0a,
        Opcode::Call => 0x0b,
        Opcode::CallCode => 0x0c,
        Opcode::Return => 0x0d,
        Opcode::DelegateCall => 0x0e,
        Opcode::Create2 => 0x0f,
        Opcode::StaticCall => 0x10,
        Opcode::Revert => 0x11,
        Opcode::SelfDestruct => 0x12,
        Opcode::Push(_) => 0x13,
        Opcode::Dup(size) => 0x20 + size,
        Opcode::Swap(size) => 0x40 + size,
        Opcode::Log(size) => 0x60 + size,
        Opcode::Other(byte) => byte,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_digest_is_stable_for_identical_bytes() {
        let code = [0x60, 0x01, 0x00];
        assert_eq!(content_digest(&code), content_digest(&code));
    }

    #[test]
    fn content_digest_changes_with_a_single_byte() {
        assert_ne!(content_digest(&[0x60, 0x01]), content_digest(&[0x60, 0x02]));
    }

    #[test]
    fn structural_fingerprint_ignores_embedded_constants() {
        // Same opcode sequence, different embedded 1-byte constant (as if two
        // minimal proxies pointed at different implementation addresses).
        let first = [0x60, 0xaa, 0x60, 0xbb, 0x00];
        let second = [0x60, 0xcc, 0x60, 0xdd, 0x00];
        assert_eq!(
            structural_fingerprint(&first).unwrap(),
            structural_fingerprint(&second).unwrap()
        );
        assert_ne!(content_digest(&first), content_digest(&second));
    }

    #[test]
    fn structural_fingerprint_still_distinguishes_different_opcode_sequences() {
        let first = [0x60, 0x01, 0x00]; // PUSH1, STOP
        let second = [0x60, 0x01, 0xfd]; // PUSH1, REVERT
        assert_ne!(
            structural_fingerprint(&first).unwrap(),
            structural_fingerprint(&second).unwrap()
        );
    }

    #[test]
    fn structural_fingerprint_distinguishes_push_sizes() {
        let push1 = [0x60, 0x01];
        let push2 = [0x61, 0x00, 0x01];
        assert_ne!(
            structural_fingerprint(&push1).unwrap(),
            structural_fingerprint(&push2).unwrap()
        );
    }

    #[test]
    fn structural_fingerprint_rejects_truncated_bytecode() {
        assert!(structural_fingerprint(&[0x7f, 0x01]).is_err());
    }
}
