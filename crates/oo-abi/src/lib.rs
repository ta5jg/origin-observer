// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/lib.rs
// Purpose : Acquire, validate and decode application binary interfaces.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Acquire, validate and decode application binary interfaces.

pub mod acquisition;
pub mod decoder;
pub mod error;
pub mod event;
pub mod function;
pub mod inference;
pub mod model;
pub mod validation;
