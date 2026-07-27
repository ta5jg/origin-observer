// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/address.rs
// Purpose : Blockchain address domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Blockchain address domain model.
//!
//! An [`Address`] represents an account, contract, program, validator or other
//! addressable entity on a specific blockchain network. The model preserves the
//! original representation while also maintaining a canonical value suitable
//! for comparison and indexing.

use core::fmt;

use oo_core::error::invalid_argument;
use oo_core::{AddressId, NetworkId, Result};

/// Maximum accepted address length.
pub const MAX_ADDRESS_LENGTH: usize = 256;

/// General classification of a blockchain address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AddressKind {
    /// A user-controlled account.
    ExternallyOwned,

    /// A deployed smart contract.
    Contract,

    /// An executable program account.
    Program,

    /// A validator or consensus participant.
    Validator,

    /// A multisignature account.
    Multisig,

    /// A system-controlled account.
    System,

    /// The address type has not yet been determined.
    #[default]
    Unknown,
}

/// Encoding used by an address representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AddressEncoding {
    /// Hexadecimal representation, commonly prefixed with `0x`.
    Hexadecimal,

    /// Base58 representation.
    Base58,

    /// Base58Check representation.
    Base58Check,

    /// Bech32 representation.
    Bech32,

    /// Bech32m representation.
    Bech32m,

    /// Base32 representation.
    Base32,

    /// An implementation-specific encoding.
    Other,

    /// The encoding has not yet been determined.
    #[default]
    Unknown,
}

/// Address validation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AddressValidation {
    /// The address has not been validated by a chain-specific validator.
    #[default]
    Unverified,

    /// The address passed chain-specific validation.
    Valid,

    /// The address failed chain-specific validation.
    Invalid,
}

/// An address belonging to a specific blockchain network.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address {
    id: AddressId,
    network_id: NetworkId,
    value: String,
    canonical_value: String,
    kind: AddressKind,
    encoding: AddressEncoding,
    validation: AddressValidation,
    label: Option<String>,
}

impl Address {
    /// Creates an address on the supplied network.
    ///
    /// The input is trimmed, but otherwise preserved exactly. Hexadecimal
    /// addresses receive a lowercase canonical representation so they can be
    /// compared without regard to letter casing.
    pub fn new(network_id: NetworkId, value: impl Into<String>) -> Result<Self> {
        let value = normalize_input(value.into())?;
        let encoding = detect_encoding(&value);
        let canonical_value = canonicalize(&value, encoding);

        Ok(Self {
            id: AddressId::new(),
            network_id,
            value,
            canonical_value,
            kind: AddressKind::Unknown,
            encoding,
            validation: AddressValidation::Unverified,
            label: None,
        })
    }

    /// Creates an address with an explicitly supplied encoding.
    pub fn with_encoding(
        network_id: NetworkId,
        value: impl Into<String>,
        encoding: AddressEncoding,
    ) -> Result<Self> {
        let value = normalize_input(value.into())?;
        let canonical_value = canonicalize(&value, encoding);

        Ok(Self {
            id: AddressId::new(),
            network_id,
            value,
            canonical_value,
            kind: AddressKind::Unknown,
            encoding,
            validation: AddressValidation::Unverified,
            label: None,
        })
    }

    /// Returns the internal address identifier.
    #[must_use]
    pub const fn id(&self) -> AddressId {
        self.id
    }

    /// Returns the network to which the address belongs.
    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    /// Returns the original normalized address representation.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the canonical address representation.
    #[must_use]
    pub fn canonical_value(&self) -> &str {
        &self.canonical_value
    }

    /// Returns the address classification.
    #[must_use]
    pub const fn kind(&self) -> AddressKind {
        self.kind
    }

    /// Assigns the address classification.
    pub const fn set_kind(&mut self, kind: AddressKind) {
        self.kind = kind;
    }

    /// Returns the detected or explicitly assigned encoding.
    #[must_use]
    pub const fn encoding(&self) -> AddressEncoding {
        self.encoding
    }

    /// Assigns a new encoding and recalculates the canonical representation.
    pub fn set_encoding(&mut self, encoding: AddressEncoding) {
        self.encoding = encoding;
        self.canonical_value = canonicalize(&self.value, encoding);
    }

    /// Returns the validation state.
    #[must_use]
    pub const fn validation(&self) -> AddressValidation {
        self.validation
    }

    /// Marks the address as valid.
    pub const fn mark_valid(&mut self) {
        self.validation = AddressValidation::Valid;
    }

    /// Marks the address as invalid.
    pub const fn mark_invalid(&mut self) {
        self.validation = AddressValidation::Invalid;
    }

    /// Resets the address validation state.
    pub const fn clear_validation(&mut self) {
        self.validation = AddressValidation::Unverified;
    }

    /// Returns whether the address has passed chain-specific validation.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        matches!(self.validation, AddressValidation::Valid)
    }

    /// Returns whether the address has failed chain-specific validation.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        matches!(self.validation, AddressValidation::Invalid)
    }

    /// Returns whether the address has not yet been validated.
    #[must_use]
    pub const fn is_unverified(&self) -> bool {
        matches!(self.validation, AddressValidation::Unverified)
    }

    /// Returns the optional human-readable label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Assigns a human-readable label.
    pub fn set_label(&mut self, label: impl Into<String>) -> Result<()> {
        let label = label.into().trim().to_owned();

        if label.is_empty() {
            return Err(invalid_argument("address label must not be empty"));
        }

        self.label = Some(label);
        Ok(())
    }

    /// Removes the human-readable label.
    pub fn clear_label(&mut self) {
        self.label = None;
    }

    /// Returns whether this address has the same canonical value and network as
    /// another address.
    #[must_use]
    pub fn same_location(&self, other: &Self) -> bool {
        self.network_id == other.network_id && self.canonical_value == other.canonical_value
    }

    /// Returns whether this appears to be a zero hexadecimal address.
    #[must_use]
    pub fn is_zero_address(&self) -> bool {
        if self.encoding != AddressEncoding::Hexadecimal {
            return false;
        }

        self.canonical_value
            .strip_prefix("0x")
            .is_some_and(|body| !body.is_empty() && body.bytes().all(|character| character == b'0'))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

/// Removes surrounding whitespace and performs format-independent checks.
fn normalize_input(value: String) -> Result<String> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(invalid_argument("address value must not be empty"));
    }

    if value.len() > MAX_ADDRESS_LENGTH {
        return Err(invalid_argument(format!(
            "address length must not exceed {MAX_ADDRESS_LENGTH} characters"
        )));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(invalid_argument(
            "address value must not contain whitespace",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_argument(
            "address value must not contain control characters",
        ));
    }

    Ok(value)
}

/// Produces a comparison-friendly representation without changing the original
/// address value.
fn canonicalize(value: &str, encoding: AddressEncoding) -> String {
    match encoding {
        AddressEncoding::Hexadecimal => value.to_ascii_lowercase(),
        AddressEncoding::Bech32 | AddressEncoding::Bech32m => value.to_ascii_lowercase(),
        _ => value.to_owned(),
    }
}

/// Performs conservative encoding detection.
///
/// Detection does not imply chain-specific validity. Complete validation belongs
/// to provider or protocol-specific crates.
fn detect_encoding(value: &str) -> AddressEncoding {
    if is_hexadecimal(value) {
        return AddressEncoding::Hexadecimal;
    }

    if looks_like_bech32(value) {
        return AddressEncoding::Bech32;
    }

    if looks_like_base58(value) {
        return AddressEncoding::Base58;
    }

    AddressEncoding::Unknown
}

fn is_hexadecimal(value: &str) -> bool {
    let body = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"));

    body.is_some_and(|body| {
        !body.is_empty() && body.bytes().all(|character| character.is_ascii_hexdigit())
    })
}

fn looks_like_bech32(value: &str) -> bool {
    let Some(separator) = value.rfind('1') else {
        return false;
    };

    let human_readable_part = &value[..separator];
    let data_part = &value[separator + 1..];

    if human_readable_part.is_empty() || data_part.len() < 6 {
        return false;
    }

    let entirely_lowercase = value == value.to_ascii_lowercase();
    let entirely_uppercase = value == value.to_ascii_uppercase();

    if !entirely_lowercase && !entirely_uppercase {
        return false;
    }

    const BECH32_CHARSET: &str = "023456789acdefghjklmnpqrstuvwxyz";

    human_readable_part
        .bytes()
        .all(|character| character.is_ascii_alphanumeric() || character == b'-')
        && data_part
            .to_ascii_lowercase()
            .chars()
            .all(|character| BECH32_CHARSET.contains(character))
}

fn looks_like_base58(value: &str) -> bool {
    const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    value.len() >= 20
        && value
            .chars()
            .all(|character| BASE58_ALPHABET.contains(character))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn network_id() -> NetworkId {
        NetworkId::new()
    }

    #[test]
    fn creates_address() {
        let address = Address::new(network_id(), "0x1234567890abcdef").unwrap();

        assert_eq!(address.value(), "0x1234567890abcdef");
        assert_eq!(address.encoding(), AddressEncoding::Hexadecimal);
        assert_eq!(address.kind(), AddressKind::Unknown);
        assert!(address.is_unverified());
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let address = Address::new(network_id(), "  TQn9Y2khEsLJW1ChVWFMSMeRDow5KcbLSE  ").unwrap();

        assert_eq!(address.value(), "TQn9Y2khEsLJW1ChVWFMSMeRDow5KcbLSE");
    }

    #[test]
    fn rejects_empty_address() {
        let result = Address::new(network_id(), "   ");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_internal_whitespace() {
        let result = Address::new(network_id(), "0x1234 5678");

        assert!(result.is_err());
    }

    #[test]
    fn hexadecimal_canonicalization_is_case_insensitive() {
        let network_id = network_id();

        let first = Address::new(network_id, "0xAABBCCDDEEFF").unwrap();

        let second = Address::new(network_id, "0xaabbccddeeff").unwrap();

        assert_eq!(first.canonical_value(), "0xaabbccddeeff");
        assert!(first.same_location(&second));
    }

    #[test]
    fn same_value_on_different_networks_is_not_same_location() {
        let first = Address::new(network_id(), "0x1234").unwrap();

        let second = Address::new(network_id(), "0x1234").unwrap();

        assert!(!first.same_location(&second));
    }

    #[test]
    fn address_kind_can_be_assigned() {
        let mut address = Address::new(network_id(), "0x1234").unwrap();

        address.set_kind(AddressKind::Contract);

        assert_eq!(address.kind(), AddressKind::Contract);
    }

    #[test]
    fn validation_state_can_be_managed() {
        let mut address = Address::new(network_id(), "0x1234").unwrap();

        address.mark_valid();

        assert!(address.is_valid());
        assert!(!address.is_invalid());

        address.mark_invalid();

        assert!(address.is_invalid());

        address.clear_validation();

        assert!(address.is_unverified());
    }

    #[test]
    fn label_can_be_managed() {
        let mut address = Address::new(network_id(), "0x1234").unwrap();

        address.set_label("Treasury").unwrap();

        assert_eq!(address.label(), Some("Treasury"));

        address.clear_label();

        assert_eq!(address.label(), None);
    }

    #[test]
    fn empty_label_is_rejected() {
        let mut address = Address::new(network_id(), "0x1234").unwrap();

        assert!(address.set_label("   ").is_err());
    }

    #[test]
    fn detects_base58_address() {
        let address = Address::new(network_id(), "TQn9Y2khEsLJW1ChVWFMSMeRDow5KcbLSE").unwrap();

        assert_eq!(address.encoding(), AddressEncoding::Base58);
    }

    #[test]
    fn recognizes_zero_hexadecimal_address() {
        let zero =
            Address::new(network_id(), "0x0000000000000000000000000000000000000000").unwrap();

        let non_zero =
            Address::new(network_id(), "0x0000000000000000000000000000000000000001").unwrap();

        assert!(zero.is_zero_address());
        assert!(!non_zero.is_zero_address());
    }

    #[test]
    fn display_preserves_original_value() {
        let address = Address::new(network_id(), "0xAaBbCcDd").unwrap();

        assert_eq!(address.to_string(), "0xAaBbCcDd");
    }

    #[test]
    fn identifiers_are_unique() {
        let first = Address::new(network_id(), "0x1234").unwrap();

        let second = Address::new(first.network_id(), "0x1234").unwrap();

        assert_ne!(first.id(), second.id());
        assert!(first.same_location(&second));
    }
}
