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

pub use acquisition::{AbiAcquisition, AcquisitionKind};
pub use decoder::{decode, DecodedValue, Uint256};
pub use error::{AbiError, AbiResult};
pub use event::EventSignature;
pub use function::FunctionSignature;
pub use inference::{match_standards, KnownStandard, StandardMatch, CATALOG};
pub use model::{AbiEvent, AbiFunction, AbiParameter, AbiType, StateMutability};
pub use validation::validate_identifier;
