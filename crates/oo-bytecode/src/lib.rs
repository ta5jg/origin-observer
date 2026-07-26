// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/lib.rs
// Purpose : Analyze and normalize smart-contract runtime bytecode.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Analyze and normalize smart-contract runtime bytecode.

pub mod analysis;
pub mod fingerprint;
pub mod normalization;
pub mod opcode;
pub mod selector;
pub mod source;
pub mod validation;
