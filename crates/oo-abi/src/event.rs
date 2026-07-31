// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/event.rs
// Purpose : Canonical event signatures and their topic0 hashes.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Canonical event signatures and their topic0 hashes.
//!
//! A function selector truncates its Keccak-256 hash to 4 bytes; an event
//! topic does not. Using the truncated form here would silently produce the
//! wrong value for every log lookup, since the EVM emits and indexes the full
//! 32-byte hash as `topic0`.

use sha3::{Digest, Keccak256};

use crate::error::AbiResult;
use crate::model::{AbiEvent, AbiParameter};
use crate::validation::validate_identifier;

/// A validated event signature and its derived topic0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventSignature {
    canonical: String,
    topic0: [u8; 32],
    anonymous: bool,
}

impl EventSignature {
    /// Builds the canonical signature and topic0 for an event.
    ///
    /// `indexed` flags do not affect the signature: `Transfer(address
    /// indexed,address indexed,uint256)` and `Transfer(address,address,
    /// uint256)` hash identically, matching how Solidity itself derives the
    /// event selector.
    pub fn of(event: &AbiEvent) -> AbiResult<Self> {
        validate_identifier(&event.name)?;
        let canonical = canonical_signature(&event.name, &event.inputs);
        Ok(Self {
            topic0: keccak_topic(&canonical),
            canonical,
            anonymous: event.anonymous,
        })
    }

    /// Returns the canonical signature text.
    #[must_use]
    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    /// Returns the 32-byte topic0.
    #[must_use]
    pub const fn topic0(&self) -> [u8; 32] {
        self.topic0
    }

    /// Returns the `0x`-prefixed hexadecimal topic0.
    #[must_use]
    pub fn topic0_hex(&self) -> String {
        let mut out = String::with_capacity(66);
        out.push_str("0x");
        for byte in self.topic0 {
            out.push_str(&format!("{byte:02x}"));
        }
        out
    }

    /// Returns whether the event is declared `anonymous`.
    ///
    /// An anonymous event's emitted log does not carry this topic0 as its
    /// first topic; the value above is still the correct signature hash for
    /// matching purposes, just not present in the log the way a named
    /// event's would be.
    #[must_use]
    pub const fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

/// Renders `Name(type1,type2,...)`, ignoring `indexed` flags and names.
#[must_use]
pub fn canonical_signature(name: &str, inputs: &[AbiParameter]) -> String {
    let types: Vec<String> = inputs
        .iter()
        .map(|parameter| parameter.type_.canonical_name())
        .collect();
    format!("{name}({})", types.join(","))
}

fn keccak_topic(signature: &str) -> [u8; 32] {
    let hash = Keccak256::digest(signature.as_bytes());
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AbiType;

    #[test]
    fn transfer_event_matches_the_well_known_topic0() {
        let event = AbiEvent {
            name: "Transfer".to_owned(),
            inputs: vec![
                AbiParameter::indexed("from", AbiType::Address),
                AbiParameter::indexed("to", AbiType::Address),
                AbiParameter::new("value", AbiType::Uint256),
            ],
            anonymous: false,
        };
        let signature = EventSignature::of(&event).unwrap();
        assert_eq!(signature.canonical(), "Transfer(address,address,uint256)");
        // Widely published ERC-20 Transfer topic0, independently checkable.
        assert_eq!(
            signature.topic0_hex(),
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        );
    }

    #[test]
    fn indexed_flags_do_not_change_the_signature() {
        let indexed = AbiEvent {
            name: "Approval".to_owned(),
            inputs: vec![
                AbiParameter::indexed("owner", AbiType::Address),
                AbiParameter::indexed("spender", AbiType::Address),
                AbiParameter::new("value", AbiType::Uint256),
            ],
            anonymous: false,
        };
        let not_indexed = AbiEvent {
            name: "Approval".to_owned(),
            inputs: vec![
                AbiParameter::new("owner", AbiType::Address),
                AbiParameter::new("spender", AbiType::Address),
                AbiParameter::new("value", AbiType::Uint256),
            ],
            anonymous: false,
        };
        assert_eq!(
            EventSignature::of(&indexed).unwrap().topic0(),
            EventSignature::of(&not_indexed).unwrap().topic0()
        );
    }

    #[test]
    fn anonymous_events_still_compute_a_topic_but_are_flagged() {
        let event = AbiEvent {
            name: "Custom".to_owned(),
            inputs: vec![],
            anonymous: true,
        };
        let signature = EventSignature::of(&event).unwrap();
        assert!(signature.is_anonymous());
        assert_ne!(signature.topic0(), [0u8; 32]);
    }

    #[test]
    fn an_invalid_event_name_is_rejected() {
        let event = AbiEvent {
            name: String::new(),
            inputs: vec![],
            anonymous: false,
        };
        assert!(EventSignature::of(&event).is_err());
    }
}
