// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/transparent.rs
// Purpose : Classify an EIP-1967 transparent upgradeable proxy.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Classify an EIP-1967 transparent upgradeable proxy.
//!
//! A transparent proxy is the ordinary case: both the implementation and
//! admin slots are populated, and the admin address is what governs upgrades.
//! When the admin slot is absent, the storage layout still matches EIP-1967
//! but the upgrade authority lives elsewhere, which is the UUPS pattern
//! ([`crate::uups`]) instead.

use crate::eip1967::Eip1967Slots;
use crate::model::{ProxyKind, ProxyResolution};

/// Classifies storage as a transparent proxy when both the implementation and
/// admin slots are in use.
#[must_use]
pub fn classify(slots: &Eip1967Slots) -> Option<ProxyResolution> {
    let (implementation, admin) = (slots.implementation?, slots.admin?);

    let mut resolution = ProxyResolution {
        kind: ProxyKind::Eip1967Transparent,
        implementation: Some(implementation),
        admin: Some(admin),
        evidence: Vec::new(),
    };
    resolution.record(
        "eip1967.implementation slot",
        "non-zero, decodes to an address",
    );
    resolution.record("eip1967.admin slot", "non-zero, decodes to an address");
    Some(resolution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_slots_present_classifies_as_transparent() {
        let slots = Eip1967Slots {
            implementation: Some([0x11; 20]),
            admin: Some([0x22; 20]),
            ..Eip1967Slots::default()
        };
        let resolution = classify(&slots).expect("must classify");
        assert_eq!(resolution.kind, ProxyKind::Eip1967Transparent);
        assert_eq!(resolution.implementation, Some([0x11; 20]));
        assert_eq!(resolution.admin, Some([0x22; 20]));
        assert_eq!(resolution.evidence.len(), 2);
    }

    #[test]
    fn a_missing_admin_slot_does_not_classify_as_transparent() {
        let slots = Eip1967Slots {
            implementation: Some([0x11; 20]),
            admin: None,
            ..Eip1967Slots::default()
        };
        assert!(classify(&slots).is_none());
    }

    #[test]
    fn a_missing_implementation_slot_does_not_classify_as_transparent() {
        let slots = Eip1967Slots {
            implementation: None,
            admin: Some([0x22; 20]),
            ..Eip1967Slots::default()
        };
        assert!(classify(&slots).is_none());
    }
}
