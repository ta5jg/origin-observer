// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/model.rs
// Purpose : Application binary interface domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Application binary interface domain model.
//!
//! `AbiType` deliberately does not cover the full Solidity grammar. It covers
//! the subset this crate can encode, decode and reason about correctly:
//! fixed-width value types plus one dynamic type per position for the return
//! shapes WDRP actually observes (`name() -> string`, `balanceOf(address) ->
//! uint256`). A caller asking for a type outside this subset gets
//! [`crate::error::AbiError::UnsupportedType`], not a wrong answer.

use std::fmt;

/// A parameter or return type this crate understands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbiType {
    /// `address` — a 20-byte value, encoded left-padded to 32 bytes.
    Address,
    /// `bool` — encoded as `uint256` with value `0` or `1`.
    Bool,
    /// `uint256`.
    Uint256,
    /// `uint8`, `uint16`, ... `uint256` in 8-bit steps, tracked by bit width.
    Uint(u16),
    /// `bytes32` and other fixed-size byte arrays, tracked by byte length.
    FixedBytes(u8),
    /// `string`, a dynamic UTF-8 value.
    String,
    /// `bytes`, a dynamic byte value.
    Bytes,
}

impl AbiType {
    /// Returns the canonical Solidity spelling of the type.
    #[must_use]
    pub fn canonical_name(self) -> String {
        match self {
            Self::Address => "address".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Uint256 => "uint256".to_owned(),
            Self::Uint(bits) => format!("uint{bits}"),
            Self::FixedBytes(len) => format!("bytes{len}"),
            Self::String => "string".to_owned(),
            Self::Bytes => "bytes".to_owned(),
        }
    }

    /// Returns whether the type is dynamically sized.
    #[must_use]
    pub const fn is_dynamic(self) -> bool {
        matches!(self, Self::String | Self::Bytes)
    }
}

impl fmt::Display for AbiType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical_name())
    }
}

/// One parameter of a function or event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiParameter {
    /// Parameter name. May be empty; the name is not part of the signature.
    pub name: String,
    /// Parameter type.
    pub type_: AbiType,
    /// Whether the parameter is an indexed event topic. Meaningless outside
    /// an event.
    pub indexed: bool,
}

impl AbiParameter {
    /// Creates a non-indexed parameter.
    #[must_use]
    pub fn new(name: impl Into<String>, type_: AbiType) -> Self {
        Self {
            name: name.into(),
            type_,
            indexed: false,
        }
    }

    /// Creates an indexed event parameter.
    #[must_use]
    pub fn indexed(name: impl Into<String>, type_: AbiType) -> Self {
        Self {
            name: name.into(),
            type_,
            indexed: true,
        }
    }
}

/// How a function may affect or read chain state.
///
/// This governs whether a call is safe to issue as `eth_call` (never mutates
/// real state) versus something that would require a signed transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateMutability {
    /// Reads no state.
    Pure,
    /// Reads state but does not write it.
    View,
    /// May receive value but is not otherwise restricted.
    Payable,
    /// May write state.
    NonPayable,
}

impl StateMutability {
    /// Returns whether a call is observation-safe: it can be issued as a
    /// read-only `eth_call` without risk of mutating real state.
    #[must_use]
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::Pure | Self::View)
    }
}

/// A function fragment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiFunction {
    /// Function name.
    pub name: String,
    /// Input parameters, in order.
    pub inputs: Vec<AbiParameter>,
    /// Output parameters, in order.
    pub outputs: Vec<AbiParameter>,
    /// State mutability.
    pub state_mutability: StateMutability,
}

/// An event fragment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiEvent {
    /// Event name.
    pub name: String,
    /// Parameters, in declaration order, each carrying its own `indexed` flag.
    pub inputs: Vec<AbiParameter>,
    /// Whether the event is declared `anonymous`.
    ///
    /// An anonymous event's log does not carry `topic0`; the value is still
    /// computable from the signature and is reported for reference, not
    /// because the EVM emits it.
    pub anonymous: bool,
}
