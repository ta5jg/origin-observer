// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/eip1967.rs
// Purpose : Interpret EIP-1967 storage slot reads.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Interpret EIP-1967 storage slot reads.
//!
//! This module does not read storage itself; it interprets the results
//! [`oo_storage::read_layout`] already produced for
//! [`oo_storage::StorageLayout::known_proxy_slots`]. A zero value at a slot
//! means the slot is unused, not that it names the zero address: the two are
//! indistinguishable on-chain, and this module reports "absent" rather than
//! fabricating an address.

use oo_storage::StorageValue;

/// Addresses read from the three EIP-1967 slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Eip1967Slots {
    /// Implementation slot address, if the slot is non-zero.
    pub implementation: Option<[u8; 20]>,
    /// Admin slot address, if the slot is non-zero.
    pub admin: Option<[u8; 20]>,
    /// Beacon slot address, if the slot is non-zero.
    pub beacon: Option<[u8; 20]>,
    /// Whether the EIP-1822 proxiable slot is non-zero.
    pub proxiable_slot_in_use: bool,
    /// Legacy pre-1967 OpenZeppelin implementation slot address, if non-zero.
    pub legacy_implementation: Option<[u8; 20]>,
}

/// Interprets the results of reading
/// [`oo_storage::StorageLayout::known_proxy_slots`].
///
/// A slot whose value fails to decode as an address (non-zero padding bytes)
/// is treated as absent rather than propagating a decode error: a value that
/// does not look like an address is evidence the slot is not being used the
/// way EIP-1967 intends, which is itself useful information a caller can see
/// reflected in `implementation` being `None` despite a non-zero raw value.
#[must_use]
pub fn interpret(values: &[(&str, StorageValue)]) -> Eip1967Slots {
    let get = |name: &str| values.iter().find(|(n, _)| *n == name).map(|(_, v)| *v);

    let address_of = |value: Option<StorageValue>| {
        value.and_then(|v| {
            if v.is_zero() {
                None
            } else {
                v.as_address().ok()
            }
        })
    };

    Eip1967Slots {
        implementation: address_of(get("eip1967.implementation")),
        admin: address_of(get("eip1967.admin")),
        beacon: address_of(get("eip1967.beacon")),
        proxiable_slot_in_use: get("eip1822.proxiable").is_some_and(|v| !v.is_zero()),
        legacy_implementation: address_of(get("legacy_oz.implementation")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oo_storage::parse_storage_value;

    fn address_value(address: [u8; 20]) -> StorageValue {
        let mut hex = "0x".to_owned();
        for byte in address {
            hex.push_str(&format!("{byte:02x}"));
        }
        parse_storage_value(&hex).unwrap()
    }

    #[test]
    fn a_zero_slot_is_absent_not_the_zero_address() {
        let values = [(
            "eip1967.implementation",
            parse_storage_value("0x0").unwrap(),
        )];
        let slots = interpret(&values);
        assert_eq!(slots.implementation, None);
    }

    #[test]
    fn a_populated_implementation_slot_is_decoded() {
        let address = [0x22; 20];
        let values = [("eip1967.implementation", address_value(address))];
        let slots = interpret(&values);
        assert_eq!(slots.implementation, Some(address));
    }

    #[test]
    fn absent_slots_default_to_none() {
        let slots = interpret(&[]);
        assert_eq!(slots.implementation, None);
        assert_eq!(slots.admin, None);
        assert_eq!(slots.beacon, None);
        assert!(!slots.proxiable_slot_in_use);
    }

    #[test]
    fn a_value_with_nonzero_padding_is_treated_as_absent_rather_than_erroring() {
        let malformed = parse_storage_value(&format!("0x01{}", "00".repeat(31))).unwrap();
        let values = [("eip1967.implementation", malformed)];
        let slots = interpret(&values);
        assert_eq!(slots.implementation, None);
    }

    #[test]
    fn the_proxiable_flag_reports_use_without_decoding_an_address() {
        let values = [("eip1822.proxiable", address_value([0x01; 20]))];
        let slots = interpret(&values);
        assert!(slots.proxiable_slot_in_use);
    }
}
