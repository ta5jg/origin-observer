// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-storage/src/lib.rs
// Purpose : Inspect smart-contract storage slots and layouts.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Inspect smart-contract storage slots and layouts.

pub mod decoder;
pub mod error;
pub mod layout;
pub mod reader;
pub mod slot;
pub mod standard;
pub mod validation;

pub use decoder::{parse_storage_value, StorageValue};
pub use error::{StorageError, StorageResult};
pub use layout::StorageLayout;
pub use reader::{read_layout, read_storage};
pub use slot::StorageSlot;
pub use standard::{
    eip1822_proxiable_slot, eip1967_admin_slot, eip1967_beacon_slot, eip1967_implementation_slot,
    legacy_openzeppelin_implementation_slot,
};
pub use validation::validate_storage_response;
