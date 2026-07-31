// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/implementation.rs
// Purpose : Select the best-known implementation address from a resolution.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Select the best-known implementation address from a resolution.

use crate::model::{ProxyKind, ProxyResolution};

/// Returns the implementation address a caller should treat as authoritative,
/// or `None` when the resolution never determined one.
///
/// For [`ProxyKind::Eip1967Beacon`], `resolution.implementation` may still
/// hold the beacon's own address rather than the beacon's reported
/// implementation, depending on whether the caller followed up with
/// [`crate::beacon::resolve_implementation`]. This function does not know
/// which case it is looking at; it returns exactly what the resolution
/// recorded and leaves interpretation of a beacon result to the caller that
/// built it.
#[must_use]
pub fn best_known_address(resolution: &ProxyResolution) -> Option<[u8; 20]> {
    if resolution.kind == ProxyKind::Unknown {
        return None;
    }
    resolution.implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_never_returns_an_address_even_if_one_is_set() {
        let resolution = ProxyResolution {
            kind: ProxyKind::Unknown,
            implementation: Some([0x11; 20]),
            admin: None,
            evidence: Vec::new(),
        };
        assert_eq!(best_known_address(&resolution), None);
    }

    #[test]
    fn a_resolved_kind_returns_its_recorded_address() {
        let resolution = ProxyResolution {
            kind: ProxyKind::Eip1967Transparent,
            implementation: Some([0x11; 20]),
            admin: Some([0x22; 20]),
            evidence: Vec::new(),
        };
        assert_eq!(best_known_address(&resolution), Some([0x11; 20]));
    }
}
