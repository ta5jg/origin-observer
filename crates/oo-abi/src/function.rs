// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/function.rs
// Purpose : Canonical function signatures and their 4-byte selectors.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Canonical function signatures and their 4-byte selectors.
//!
//! A selector is only meaningful relative to the exact signature that
//! produced it: `transfer(address,uint256)` and `transfer(address, uint256)`
//! (with a space) hash to different values even though Solidity accepts both
//! spellings in source. This module is the single place that renders a
//! canonical form, so `oo-bytecode`'s selector arithmetic and this crate's
//! signature parsing always agree on what "the" selector for a function is.

use sha3::{Digest, Keccak256};

use crate::error::AbiResult;
use crate::model::{AbiFunction, AbiParameter};
use crate::validation::validate_identifier;

/// A validated function signature and its derived selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignature {
    /// Canonical form, e.g. `transfer(address,uint256)`.
    canonical: String,
    /// 4-byte selector.
    selector: [u8; 4],
}

impl FunctionSignature {
    /// Builds the canonical signature and selector for a function.
    ///
    /// Only the function name and input types feed the signature; output
    /// types and mutability play no part in Solidity's selector derivation.
    pub fn of(function: &AbiFunction) -> AbiResult<Self> {
        Self::of_parts(&function.name, &function.inputs)
    }

    /// Builds a signature directly from a name and input types, without
    /// requiring a full [`AbiFunction`].
    pub fn of_parts(name: &str, inputs: &[AbiParameter]) -> AbiResult<Self> {
        validate_identifier(name)?;
        let canonical = canonical_signature(name, inputs);
        let selector = keccak_selector(&canonical);
        Ok(Self {
            canonical,
            selector,
        })
    }

    /// Returns the canonical signature text.
    #[must_use]
    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    /// Returns the 4-byte selector.
    #[must_use]
    pub const fn selector(&self) -> [u8; 4] {
        self.selector
    }

    /// Returns the `0x`-prefixed hexadecimal selector.
    #[must_use]
    pub fn selector_hex(&self) -> String {
        format!(
            "0x{:02x}{:02x}{:02x}{:02x}",
            self.selector[0], self.selector[1], self.selector[2], self.selector[3]
        )
    }
}

/// Renders `name(type1,type2,...)` with no spaces and no parameter names.
#[must_use]
pub fn canonical_signature(name: &str, inputs: &[AbiParameter]) -> String {
    let types: Vec<String> = inputs
        .iter()
        .map(|parameter| parameter.type_.canonical_name())
        .collect();
    format!("{name}({})", types.join(","))
}

fn keccak_selector(signature: &str) -> [u8; 4] {
    let hash = Keccak256::digest(signature.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AbiType;

    #[test]
    fn transfer_signature_matches_the_well_known_selector() {
        let signature = FunctionSignature::of_parts(
            "transfer",
            &[
                AbiParameter::new("to", AbiType::Address),
                AbiParameter::new("amount", AbiType::Uint256),
            ],
        )
        .unwrap();
        assert_eq!(signature.canonical(), "transfer(address,uint256)");
        assert_eq!(signature.selector_hex(), "0xa9059cbb");
    }

    #[test]
    fn a_function_with_no_arguments_has_empty_parentheses() {
        let signature = FunctionSignature::of_parts("name", &[]).unwrap();
        assert_eq!(signature.canonical(), "name()");
    }

    #[test]
    fn parameter_names_do_not_affect_the_signature() {
        let with_name = FunctionSignature::of_parts(
            "balanceOf",
            &[AbiParameter::new("owner", AbiType::Address)],
        )
        .unwrap();
        let without_name =
            FunctionSignature::of_parts("balanceOf", &[AbiParameter::new("", AbiType::Address)])
                .unwrap();
        assert_eq!(with_name.canonical(), without_name.canonical());
        assert_eq!(with_name.selector(), without_name.selector());
    }

    #[test]
    fn an_empty_function_name_is_rejected() {
        assert!(FunctionSignature::of_parts("", &[]).is_err());
        assert!(FunctionSignature::of_parts("  ", &[]).is_err());
    }

    #[test]
    fn a_name_with_invalid_characters_is_rejected() {
        assert!(FunctionSignature::of_parts("trans fer", &[]).is_err());
        assert!(FunctionSignature::of_parts("transfer(", &[]).is_err());
    }
}
