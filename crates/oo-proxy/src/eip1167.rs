// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/eip1167.rs
// Purpose : Detect the EIP-1167 minimal proxy bytecode template.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Detect the EIP-1167 minimal proxy bytecode template.
//!
//! EIP-1167 defines a fixed 45-byte runtime: a 10-byte prefix, a 20-byte
//! embedded implementation address, and a 15-byte suffix that delegatecalls
//! to it and forwards the result. Because the template is fixed, detection is
//! an exact byte comparison, not a heuristic — either the bytecode is this
//! template or it is not a minimal proxy.

/// Bytes preceding the embedded implementation address.
const PREFIX: [u8; 10] = [0x36, 0x3d, 0x3d, 0x37, 0x3d, 0x3d, 0x3d, 0x36, 0x3d, 0x73];

/// Bytes following the embedded implementation address.
const SUFFIX: [u8; 15] = [
    0x5a, 0xf4, 0x3d, 0x82, 0x80, 0x3e, 0x90, 0x3d, 0x91, 0x60, 0x2b, 0x57, 0xfd, 0x5b, 0xf3,
];

/// Total length of a well-formed minimal proxy: prefix + address + suffix.
pub const TEMPLATE_LENGTH: usize = PREFIX.len() + 20 + SUFFIX.len();

/// Detects an EIP-1167 minimal proxy and extracts its embedded
/// implementation address.
///
/// Returns `None` for any bytecode that does not match the template exactly,
/// including bytecode that is merely similar (a proxy generator using a
/// slightly different template is not this one).
#[must_use]
pub fn detect(code: &[u8]) -> Option<[u8; 20]> {
    if code.len() != TEMPLATE_LENGTH {
        return None;
    }
    if code[..PREFIX.len()] != PREFIX {
        return None;
    }
    if code[PREFIX.len() + 20..] != SUFFIX {
        return None;
    }
    let mut address = [0u8; 20];
    address.copy_from_slice(&code[PREFIX.len()..PREFIX.len() + 20]);
    Some(address)
}

/// Builds the EIP-1167 minimal proxy bytecode for a given implementation
/// address.
///
/// Used to produce known-good fixtures for tests, and available to any caller
/// that needs to recognize a minimal proxy it expects to deploy.
#[must_use]
pub fn build(implementation: [u8; 20]) -> Vec<u8> {
    let mut code = Vec::with_capacity(TEMPLATE_LENGTH);
    code.extend_from_slice(&PREFIX);
    code.extend_from_slice(&implementation);
    code.extend_from_slice(&SUFFIX);
    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use oo_bytecode::{decode, Opcode};

    #[test]
    fn build_and_detect_round_trip() {
        let implementation = [0xAB; 20];
        let code = build(implementation);
        assert_eq!(code.len(), TEMPLATE_LENGTH);
        assert_eq!(detect(&code), Some(implementation));
    }

    #[test]
    fn a_wrong_length_is_not_detected() {
        let mut code = build([0xAB; 20]);
        code.push(0x00);
        assert_eq!(detect(&code), None);
    }

    #[test]
    fn a_correct_length_with_a_different_prefix_is_not_detected() {
        let mut code = build([0xAB; 20]);
        code[0] = 0x00;
        assert_eq!(detect(&code), None);
    }

    #[test]
    fn a_correct_length_with_a_different_suffix_is_not_detected() {
        let mut code = build([0xAB; 20]);
        let last = code.len() - 1;
        code[last] = 0x00;
        assert_eq!(detect(&code), None);
    }

    #[test]
    fn the_template_decodes_to_a_sane_delegatecall_forwarding_sequence() {
        // Self-consistency check on the constant template itself: it must
        // decode cleanly and contain exactly the instructions a minimal proxy
        // needs, with the JUMPI target actually landing on the JUMPDEST.
        let code = build([0x11; 20]);
        let instructions = decode(&code).expect("template must decode cleanly");

        assert!(instructions
            .iter()
            .any(|i| i.opcode == Opcode::DelegateCall));
        assert!(instructions.iter().any(|i| i.opcode == Opcode::JumpI));
        assert!(instructions.iter().any(|i| i.opcode == Opcode::JumpDest));
        assert!(instructions.iter().any(|i| i.opcode == Opcode::Revert));
        assert!(instructions.iter().any(|i| i.opcode == Opcode::Return));

        let jumpdest_offset = instructions
            .iter()
            .find(|i| i.opcode == Opcode::JumpDest)
            .map(|i| i.offset)
            .expect("JUMPDEST present");
        let push_before_jumpi = instructions
            .iter()
            .find(|i| i.opcode == Opcode::Push(1))
            .expect("PUSH1 jump target present");
        let target = usize::from(push_before_jumpi.immediate[0]);
        assert_eq!(
            target, jumpdest_offset,
            "JUMPI target must land exactly on JUMPDEST"
        );
    }

    #[test]
    fn arbitrary_bytecode_is_not_mistaken_for_a_minimal_proxy() {
        assert_eq!(detect(&[0x60, 0x01, 0x00]), None);
        assert_eq!(detect(&[]), None);
    }
}
