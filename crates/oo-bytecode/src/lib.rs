// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-bytecode/src/lib.rs
// Purpose : Analyze and normalize smart-contract runtime bytecode.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Analyze and normalize smart-contract runtime bytecode.

pub mod analysis;
pub mod error;
pub mod fingerprint;
pub mod normalization;
pub mod opcode;
pub mod selector;
pub mod source;
pub mod validation;

pub use analysis::{analyze, BytecodeAnalysis};
pub use error::{BytecodeError, BytecodeResult};
pub use fingerprint::{content_digest, structural_fingerprint};
pub use normalization::{parse_hex, to_hex};
pub use opcode::{decode, Instruction, Opcode};
pub use selector::{candidate_selectors, Selector};
pub use source::BytecodeSource;
pub use validation::validate_bytecode;
