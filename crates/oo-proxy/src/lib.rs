// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/lib.rs
// Purpose : Detect and resolve smart-contract proxy architectures.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Detect and resolve smart-contract proxy architectures.

pub mod beacon;
pub mod diamond;
pub mod eip1167;
pub mod eip1967;
pub mod implementation;
pub mod model;
pub mod resolver;
pub mod transparent;
pub mod uups;
pub mod validation;
