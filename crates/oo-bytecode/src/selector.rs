// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/selector.rs
// Purpose : Compute and recover 4-byte Solidity function selectors.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Compute and recover 4-byte Solidity function selectors.
//!
//! A selector is the first four bytes of the Keccak-256 hash of a function's
//! canonical signature. Ethereum specifically uses Keccak-256, the original
//! 2015 padding, not the later NIST SHA3-256; the two produce different
//! digests for the same input, and using the wrong one silently computes a
//! selector for a function nobody is calling.
//!
//! Solidity's dispatcher compiles to a sequence of `PUSH4 <selector> ...
//! JUMPI` comparisons, so a compiled contract's own selectors are visible in
//! its bytecode as `PUSH4` immediates. Scanning for them recovers candidate
//! selectors without a source ABI.

use sha3::{Digest, Keccak256};

use crate::opcode::{decode, Opcode};

/// A 4-byte function selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Selector([u8; 4]);

impl Selector {
    /// Wraps raw selector bytes.
    #[must_use]
    pub const fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    /// Computes the selector for a canonical function signature such as
    /// `transfer(address,uint256)`.
    ///
    /// The caller is responsible for canonical form: no spaces, parameter
    /// names omitted, types spelled out (`uint256` rather than `uint`). This
    /// function does not validate the signature; [`crate::selector`] callers
    /// that need validated input should go through `oo-abi`'s signature
    /// builder, which knows the full type grammar.
    #[must_use]
    pub fn of_signature(signature: &str) -> Self {
        let hash = Keccak256::digest(signature.as_bytes());
        Self([hash[0], hash[1], hash[2], hash[3]])
    }

    /// Returns the raw selector bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> [u8; 4] {
        self.0
    }

    /// Returns the `0x`-prefixed hexadecimal form.
    #[must_use]
    pub fn to_hex(self) -> String {
        format!(
            "0x{:02x}{:02x}{:02x}{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

/// Scans bytecode for `PUSH4` immediates, which is how a Solidity dispatcher
/// embeds the selectors it recognizes.
///
/// The result is a candidate list, not a proof: a `PUSH4` can appear for a
/// reason unrelated to dispatch (a constant, a packed value). Treat this as an
/// L1 hypothesis to compare against a known interface, never as confirmation
/// that a function is actually callable.
#[must_use]
pub fn candidate_selectors(code: &[u8]) -> Vec<Selector> {
    let Ok(instructions) = decode(code) else {
        // Truncated bytecode still has instructions before the truncation
        // point; a caller that only wants dispatcher hints should not lose all
        // of them to one bad tail. Fall back to a raw window scan instead.
        return scan_raw(code);
    };

    let mut found = Vec::new();
    for instruction in instructions {
        if let Opcode::Push(4) = instruction.opcode {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&instruction.immediate);
            let selector = Selector::new(bytes);
            if !found.contains(&selector) {
                found.push(selector);
            }
        }
    }
    found
}

/// Selector recovery over raw bytes without instruction decoding, used only
/// when the bytecode failed to decode cleanly.
fn scan_raw(code: &[u8]) -> Vec<Selector> {
    let mut found = Vec::new();
    let mut index = 0;
    while index + 5 <= code.len() {
        if code[index] == 0x63 {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&code[index + 1..index + 5]);
            let selector = Selector::new(bytes);
            if !found.contains(&selector) {
                found.push(selector);
            }
        }
        index += 1;
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_selector_matches_the_well_known_value() {
        // 0xa9059cbb is the widely published ERC-20 transfer(address,uint256)
        // selector, independently checkable against any Ethereum tool.
        assert_eq!(
            Selector::of_signature("transfer(address,uint256)").to_hex(),
            "0xa9059cbb"
        );
    }

    #[test]
    fn balance_of_selector_matches_the_well_known_value() {
        assert_eq!(
            Selector::of_signature("balanceOf(address)").to_hex(),
            "0x70a08231"
        );
    }

    #[test]
    fn keccak256_differs_from_sha256_for_the_same_input() {
        // Guards against accidentally linking the wrong hash crate: Ethereum
        // selectors use Keccak-256, not SHA-256, and the two disagree on the
        // same input. If this ever matched, every selector computed here
        // would be silently wrong.
        use sha2::{Digest as Sha2Digest, Sha256};
        let sha256_prefix = &Sha256::digest(b"transfer(address,uint256)")[..4];
        assert_ne!(
            Selector::of_signature("transfer(address,uint256)").as_bytes(),
            sha256_prefix
        );
    }

    #[test]
    fn candidate_selectors_recovers_push4_immediates_from_a_dispatcher() {
        let mut code = Vec::new();
        code.extend([0x63, 0xa9, 0x05, 0x9c, 0xbb]); // PUSH4 transfer selector
        code.extend([0x14]); // EQ
        code.extend([0x63, 0x70, 0xa0, 0x82, 0x31]); // PUSH4 balanceOf selector
        code.push(0x00); // STOP

        let found = candidate_selectors(&code);
        assert_eq!(found.len(), 2);
        assert!(found.contains(&Selector::of_signature("transfer(address,uint256)")));
        assert!(found.contains(&Selector::of_signature("balanceOf(address)")));
    }

    #[test]
    fn a_push4_inside_another_pushs_immediate_is_not_a_candidate() {
        // PUSH5 whose 5-byte immediate happens to contain 0x63 followed by 4
        // bytes must not be misread as a nested PUSH4.
        let code = [0x64, 0x63, 0xa9, 0x05, 0x9c, 0xbb, 0x00];
        assert!(candidate_selectors(&code).is_empty());
    }

    #[test]
    fn duplicate_selectors_are_reported_once() {
        let mut code = Vec::new();
        code.extend([0x63, 0xa9, 0x05, 0x9c, 0xbb]);
        code.extend([0x63, 0xa9, 0x05, 0x9c, 0xbb]);
        assert_eq!(candidate_selectors(&code).len(), 1);
    }

    #[test]
    fn selector_hex_is_lowercase_and_prefixed() {
        assert_eq!(
            Selector::new([0xAB, 0x00, 0xFF, 0x01]).to_hex(),
            "0xab00ff01"
        );
    }
}
