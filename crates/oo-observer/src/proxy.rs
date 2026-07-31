// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/proxy.rs
// Purpose : Classify a contract's proxy architecture from already-fetched data.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Classify a contract's proxy architecture from already-fetched data.
//!
//! `oo_proxy::ProxyResolver` performs its own live RPC calls end to end,
//! which does not fit this crate's orchestrator: it processes data a caller
//! already fetched, and never reaches for the network itself (see
//! [`crate::orchestrator`]). This module mirrors the resolver's decision
//! order using the same pure classification functions, but takes bytecode and
//! interpreted EIP-1967 storage slots as input instead of an RPC client.
//!
//! The one thing this cannot do that the live resolver can is diamond
//! detection (EIP-2535), since that requires an `eth_call` to
//! `supportsInterface` that has no offline equivalent. A contract that is
//! actually a diamond and matches nothing else classifies as
//! [`oo_proxy::ProxyKind::Unknown`] here, with that limitation recorded in
//! the evidence trail rather than silently assumed away.

use oo_proxy::eip1967::Eip1967Slots;
use oo_proxy::{beacon, eip1167, transparent, uups, ProxyKind, ProxyResolution};

/// Classifies a contract's proxy architecture from its bytecode and
/// already-interpreted EIP-1967 storage slots.
///
/// Checks are tried in the same cheapest-first order the live resolver uses:
/// an exact EIP-1167 bytecode match, then the EIP-1967 transparent, UUPS and
/// beacon storage layouts, then the legacy pre-1967 OpenZeppelin slot.
/// Diamond detection is not attempted; see the module documentation.
#[must_use]
pub fn classify_offline(code: &[u8], slots: &Eip1967Slots) -> ProxyResolution {
    if let Some(implementation) = eip1167::detect(code) {
        let mut resolution = ProxyResolution {
            kind: ProxyKind::Eip1167Minimal,
            implementation: Some(implementation),
            admin: None,
            evidence: Vec::new(),
        };
        resolution.record(
            "eip1167 template",
            format!(
                "bytecode is exactly {} bytes and matches the fixed template",
                eip1167::TEMPLATE_LENGTH
            ),
        );
        return resolution;
    }

    if let Some(resolution) = transparent::classify(slots) {
        return resolution;
    }
    if let Some(resolution) = uups::classify(slots) {
        return resolution;
    }
    if let Some(mut resolution) = beacon::classify(slots) {
        resolution.record(
            "resolver",
            "beacon detected offline; resolving the beacon's own implementation() requires a live call this module does not make",
        );
        return resolution;
    }
    if let Some(legacy) = slots.legacy_implementation {
        let mut resolution = ProxyResolution {
            kind: ProxyKind::LegacyOpenZeppelinTransparent,
            implementation: Some(legacy),
            admin: None,
            evidence: Vec::new(),
        };
        resolution.record(
            "legacy_oz.implementation slot",
            "non-zero, decodes to an address; pre-1967 OpenZeppelin layout",
        );
        return resolution;
    }

    let mut resolution = ProxyResolution::unknown();
    resolution.record("eip1167 template", "bytecode did not match");
    resolution.record("eip1967 slots", "none of the known slots were in use");
    resolution.record(
        "diamond loupe",
        "not checked offline; classifying a diamond proxy requires a live supportsInterface call",
    );
    resolution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_minimal_proxy_template_classifies_without_any_storage_data() {
        let implementation = [0x11; 20];
        let code = eip1167::build(implementation);
        let resolution = classify_offline(&code, &Eip1967Slots::default());
        assert_eq!(resolution.kind, ProxyKind::Eip1167Minimal);
        assert_eq!(resolution.implementation, Some(implementation));
    }

    #[test]
    fn transparent_storage_classifies_when_bytecode_does_not_match_eip1167() {
        let slots = Eip1967Slots {
            implementation: Some([0x22; 20]),
            admin: Some([0x33; 20]),
            ..Eip1967Slots::default()
        };
        let resolution = classify_offline(&[0x60, 0x00], &slots);
        assert_eq!(resolution.kind, ProxyKind::Eip1967Transparent);
        assert_eq!(resolution.admin, Some([0x33; 20]));
    }

    #[test]
    fn a_beacon_slot_classifies_as_beacon_and_notes_the_missing_follow_up() {
        let slots = Eip1967Slots {
            beacon: Some([0x44; 20]),
            ..Eip1967Slots::default()
        };
        let resolution = classify_offline(&[0x60, 0x00], &slots);
        assert_eq!(resolution.kind, ProxyKind::Eip1967Beacon);
        assert!(resolution
            .evidence
            .iter()
            .any(|entry| entry.observation.contains("does not make")));
    }

    #[test]
    fn nothing_matching_classifies_as_unknown_and_names_what_was_not_checked() {
        let resolution = classify_offline(&[0x60, 0x00], &Eip1967Slots::default());
        assert_eq!(resolution.kind, ProxyKind::Unknown);
        assert!(resolution
            .evidence
            .iter()
            .any(|entry| entry.check == "diamond loupe"));
    }
}
