// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/inference.rs
// Purpose : Score candidate selectors against known standard interfaces.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Score candidate selectors against known standard interfaces.
//!
//! Matching a contract's selectors against ERC-20 or ERC-721 never proves the
//! contract implements that standard: `transferFrom(address,address,uint256)`
//! shares its selector between both standards, and any contract could expose
//! a function that happens to share a selector without honoring the
//! standard's semantics. This module reports a match score — matched against
//! required selectors — as a hypothesis, at whatever WDRP confidence level a
//! selector match alone deserves, and never claims certainty.

use crate::function::FunctionSignature;
use crate::model::{AbiParameter, AbiType};

/// A well-known standard interface, identified by its required selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownStandard {
    /// ERC-20 fungible token, core transfer functions only.
    Erc20Core,
    /// ERC-20 optional metadata extension.
    Erc20Metadata,
    /// ERC-165 interface detection.
    Erc165,
    /// ERC-721 non-fungible token, core functions.
    Erc721Core,
}

impl KnownStandard {
    /// Returns the human-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Erc20Core => "ERC-20 (core)",
            Self::Erc20Metadata => "ERC-20 (metadata)",
            Self::Erc165 => "ERC-165",
            Self::Erc721Core => "ERC-721 (core)",
        }
    }

    /// Returns the canonical signatures this standard requires.
    #[must_use]
    pub fn required_signatures(self) -> Vec<(&'static str, Vec<AbiType>)> {
        match self {
            Self::Erc20Core => vec![
                ("totalSupply", vec![]),
                ("balanceOf", vec![AbiType::Address]),
                ("transfer", vec![AbiType::Address, AbiType::Uint256]),
                (
                    "transferFrom",
                    vec![AbiType::Address, AbiType::Address, AbiType::Uint256],
                ),
                ("approve", vec![AbiType::Address, AbiType::Uint256]),
                ("allowance", vec![AbiType::Address, AbiType::Address]),
            ],
            Self::Erc20Metadata => vec![("name", vec![]), ("symbol", vec![]), ("decimals", vec![])],
            Self::Erc165 => vec![("supportsInterface", vec![AbiType::FixedBytes(4)])],
            Self::Erc721Core => vec![
                ("balanceOf", vec![AbiType::Address]),
                ("ownerOf", vec![AbiType::Uint256]),
                (
                    "safeTransferFrom",
                    vec![AbiType::Address, AbiType::Address, AbiType::Uint256],
                ),
                (
                    "transferFrom",
                    vec![AbiType::Address, AbiType::Address, AbiType::Uint256],
                ),
                ("approve", vec![AbiType::Address, AbiType::Uint256]),
                ("setApprovalForAll", vec![AbiType::Address, AbiType::Bool]),
                ("getApproved", vec![AbiType::Uint256]),
                ("isApprovedForAll", vec![AbiType::Address, AbiType::Address]),
            ],
        }
    }

    /// Returns the 4-byte selectors this standard requires.
    ///
    /// Selectors are derived at call time from the same [`FunctionSignature`]
    /// machinery every other selector in this crate goes through, so the
    /// catalog cannot silently drift from how a real signature would hash.
    #[must_use]
    pub fn required_selectors(self) -> Vec<[u8; 4]> {
        self.required_signatures()
            .into_iter()
            .filter_map(|(name, types)| {
                let inputs: Vec<AbiParameter> = types
                    .into_iter()
                    .map(|type_| AbiParameter::new("", type_))
                    .collect();
                FunctionSignature::of_parts(name, &inputs)
                    .ok()
                    .map(|signature| signature.selector())
            })
            .collect()
    }
}

/// The full catalog this module can match against.
pub const CATALOG: &[KnownStandard] = &[
    KnownStandard::Erc20Core,
    KnownStandard::Erc20Metadata,
    KnownStandard::Erc165,
    KnownStandard::Erc721Core,
];

/// Match result for one standard against an observed selector set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardMatch {
    /// Standard being scored.
    pub standard: KnownStandard,
    /// Required selectors that were present in the observation.
    pub matched: usize,
    /// Required selectors in total.
    pub required: usize,
}

impl StandardMatch {
    /// Returns whether every required selector was present.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.matched == self.required
    }

    /// Returns the match ratio as a percentage, rounded down.
    #[must_use]
    pub fn percentage(&self) -> u8 {
        if self.required == 0 {
            return 0;
        }
        ((self.matched * 100) / self.required) as u8
    }
}

/// Scores every catalog standard against an observed selector set.
///
/// Results are sorted by match percentage, highest first, then by name for a
/// stable order between equal scores. A caller deciding how much confidence a
/// match deserves must weigh `is_complete` and note that a selector overlap
/// between two standards (as ERC-20 and ERC-721 share `transferFrom`) can let
/// both score non-trivially from the same bytecode: the overlap is real, not
/// an error in this module.
#[must_use]
pub fn match_standards(observed_selectors: &[[u8; 4]]) -> Vec<StandardMatch> {
    let mut results: Vec<StandardMatch> = CATALOG
        .iter()
        .map(|&standard| {
            let required = standard.required_selectors();
            let matched = required
                .iter()
                .filter(|selector| observed_selectors.contains(selector))
                .count();
            StandardMatch {
                standard,
                matched,
                required: required.len(),
            }
        })
        .collect();

    results.sort_by(|left, right| {
        right
            .percentage()
            .cmp(&left.percentage())
            .then_with(|| left.standard.name().cmp(right.standard.name()))
    });
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selector(signature: &str) -> [u8; 4] {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(signature.as_bytes());
        [hash[0], hash[1], hash[2], hash[3]]
    }

    #[test]
    fn a_full_erc20_selector_set_matches_completely() {
        let observed = [
            selector("totalSupply()"),
            selector("balanceOf(address)"),
            selector("transfer(address,uint256)"),
            selector("transferFrom(address,address,uint256)"),
            selector("approve(address,uint256)"),
            selector("allowance(address,address)"),
        ];
        let results = match_standards(&observed);
        let erc20 = results
            .iter()
            .find(|result| result.standard == KnownStandard::Erc20Core)
            .unwrap();
        assert!(erc20.is_complete());
        assert_eq!(erc20.percentage(), 100);
    }

    #[test]
    fn an_empty_bytecode_matches_nothing() {
        let results = match_standards(&[]);
        assert!(results.iter().all(|result| result.matched == 0));
    }

    #[test]
    fn overlapping_selectors_score_both_standards_honestly() {
        // transferFrom and approve are shared between ERC-20 and ERC-721; a
        // contract exposing only these should show partial matches on both,
        // not a false-positive complete match on either.
        let observed = [
            selector("transferFrom(address,address,uint256)"),
            selector("approve(address,uint256)"),
        ];
        let results = match_standards(&observed);
        let erc20 = results
            .iter()
            .find(|result| result.standard == KnownStandard::Erc20Core)
            .unwrap();
        let erc721 = results
            .iter()
            .find(|result| result.standard == KnownStandard::Erc721Core)
            .unwrap();
        assert_eq!(erc20.matched, 2);
        assert!(!erc20.is_complete());
        assert_eq!(erc721.matched, 2);
        assert!(!erc721.is_complete());
    }

    #[test]
    fn results_are_sorted_by_match_percentage_descending() {
        let observed = [selector("supportsInterface(bytes4)")];
        let results = match_standards(&observed);
        for pair in results.windows(2) {
            assert!(pair[0].percentage() >= pair[1].percentage());
        }
    }

    #[test]
    fn required_selectors_are_derived_not_hardcoded() {
        // Cross-check: the catalog's own selector derivation must agree with
        // an independently computed selector for the same signature.
        let erc165 = KnownStandard::Erc165.required_selectors();
        assert_eq!(erc165, vec![selector("supportsInterface(bytes4)")]);
    }
}
