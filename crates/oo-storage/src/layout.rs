// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-storage/src/layout.rs
// Purpose : Name storage slots for the reads a proxy detection needs.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Name storage slots for the reads a proxy detection needs.

use crate::slot::StorageSlot;
use crate::standard::{
    eip1822_proxiable_slot, eip1967_admin_slot, eip1967_beacon_slot, eip1967_implementation_slot,
    legacy_openzeppelin_implementation_slot,
};

/// A named collection of storage slots to read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageLayout {
    entries: Vec<(&'static str, StorageSlot)>,
}

impl StorageLayout {
    /// Returns the layout of every proxy-standard slot this crate knows,
    /// suitable for one batch of reads that covers every proxy kind at once.
    #[must_use]
    pub fn known_proxy_slots() -> Self {
        Self {
            entries: vec![
                ("eip1967.implementation", eip1967_implementation_slot()),
                ("eip1967.admin", eip1967_admin_slot()),
                ("eip1967.beacon", eip1967_beacon_slot()),
                ("eip1822.proxiable", eip1822_proxiable_slot()),
                (
                    "legacy_oz.implementation",
                    legacy_openzeppelin_implementation_slot(),
                ),
            ],
        }
    }

    /// Builds a layout of sequentially declared state variables, in
    /// declaration order.
    #[must_use]
    pub fn sequential(names: &[&'static str]) -> Self {
        Self {
            entries: names
                .iter()
                .enumerate()
                .map(|(index, name)| (*name, StorageSlot::from_index(index as u64)))
                .collect(),
        }
    }

    /// Returns the slot for a named entry.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<StorageSlot> {
        self.entries
            .iter()
            .find(|(entry_name, _)| *entry_name == name)
            .map(|(_, slot)| *slot)
    }

    /// Returns every named entry.
    #[must_use]
    pub fn entries(&self) -> &[(&'static str, StorageSlot)] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_proxy_slots_are_all_present_and_distinct() {
        let layout = StorageLayout::known_proxy_slots();
        assert_eq!(layout.entries().len(), 5);
        assert!(layout.get("eip1967.implementation").is_some());
        assert!(layout.get("eip1822.proxiable").is_some());
        assert!(layout.get("unknown").is_none());

        let mut slots: Vec<_> = layout.entries().iter().map(|(_, slot)| *slot).collect();
        let before = slots.len();
        slots.sort();
        slots.dedup();
        assert_eq!(slots.len(), before, "known proxy slots must not collide");
    }

    #[test]
    fn sequential_layout_assigns_indices_in_declaration_order() {
        let layout = StorageLayout::sequential(&["owner", "paused", "balance"]);
        assert_eq!(layout.get("owner"), Some(StorageSlot::from_index(0)));
        assert_eq!(layout.get("paused"), Some(StorageSlot::from_index(1)));
        assert_eq!(layout.get("balance"), Some(StorageSlot::from_index(2)));
    }
}
