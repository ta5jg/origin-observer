// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/uups.rs
// Purpose : Classify a UUPS (EIP-1822) upgradeable proxy.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Classify a UUPS (EIP-1822) upgradeable proxy.
//!
//! UUPS moves upgrade logic into the implementation contract itself, so the
//! proxy's own storage carries an implementation slot but no admin slot. That
//! absence is the primary signal, and it is a weaker one than transparent
//! detection's: an admin slot can be absent for reasons other than UUPS, so
//! this classification is reported as a hypothesis, never a certainty, and
//! the evidence trail says exactly what was and was not observed.

use crate::eip1967::Eip1967Slots;
use crate::model::{ProxyKind, ProxyResolution};

/// Classifies storage as a UUPS proxy when the implementation slot is in use,
/// the admin slot is not, and evidence corroborates UUPS specifically.
#[must_use]
pub fn classify(slots: &Eip1967Slots) -> Option<ProxyResolution> {
    let implementation = slots.implementation?;
    if slots.admin.is_some() {
        // An admin slot in use means transparent, not UUPS; classification
        // is exclusive between the two on EIP-1967 storage alone.
        return None;
    }

    let mut resolution = ProxyResolution {
        kind: ProxyKind::Eip1967Uups,
        implementation: Some(implementation),
        admin: None,
        evidence: Vec::new(),
    };
    resolution.record(
        "eip1967.implementation slot",
        "non-zero, decodes to an address",
    );
    resolution.record("eip1967.admin slot", "absent (zero or unset)");
    if slots.proxiable_slot_in_use {
        resolution.record(
            "eip1822.proxiable slot",
            "non-zero, consistent with a UUPS-compliant implementation",
        );
    } else {
        resolution.record(
            "eip1822.proxiable slot",
            "absent — classification rests on the missing admin slot alone, a weaker signal",
        );
    }
    Some(resolution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implementation_without_admin_classifies_as_uups() {
        let slots = Eip1967Slots {
            implementation: Some([0x11; 20]),
            admin: None,
            proxiable_slot_in_use: true,
            ..Eip1967Slots::default()
        };
        let resolution = classify(&slots).expect("must classify");
        assert_eq!(resolution.kind, ProxyKind::Eip1967Uups);
        assert_eq!(resolution.implementation, Some([0x11; 20]));
        assert!(resolution.admin.is_none());
    }

    #[test]
    fn an_admin_slot_rules_out_uups() {
        let slots = Eip1967Slots {
            implementation: Some([0x11; 20]),
            admin: Some([0x22; 20]),
            ..Eip1967Slots::default()
        };
        assert!(classify(&slots).is_none());
    }

    #[test]
    fn a_missing_implementation_slot_does_not_classify() {
        assert!(classify(&Eip1967Slots::default()).is_none());
    }

    #[test]
    fn the_evidence_notes_when_the_proxiable_signal_is_missing() {
        let slots = Eip1967Slots {
            implementation: Some([0x11; 20]),
            admin: None,
            proxiable_slot_in_use: false,
            ..Eip1967Slots::default()
        };
        let resolution = classify(&slots).unwrap();
        assert!(resolution
            .evidence
            .iter()
            .any(|entry| entry.observation.contains("weaker signal")));
    }
}
