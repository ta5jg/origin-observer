// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/asset.rs
// Purpose : Blockchain asset domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Blockchain asset domain model.
//!
//! An [`Asset`] represents a native coin, fungible token, non-fungible token,
//! liquidity position or another economically meaningful object observed on a
//! blockchain network.

use oo_core::error::invalid_argument;
use oo_core::{AssetId, ContractId, NetworkId, Result};

/// Maximum decimal precision accepted by the generic asset model.
pub const MAX_ASSET_DECIMALS: u8 = 38;

/// General asset classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AssetKind {
    /// Native network currency, such as ETH, BTC, BNB or TRX.
    Native,

    /// Fungible token issued by a smart contract or program.
    FungibleToken,

    /// Non-fungible token collection or individual token.
    NonFungibleToken,

    /// Semi-fungible or multi-token asset.
    MultiToken,

    /// Liquidity-provider position or pool share.
    LiquidityPosition,

    /// Wrapped representation of another asset.
    Wrapped,

    /// Stable-value asset.
    Stablecoin,

    /// Governance asset.
    Governance,

    /// Synthetic asset.
    Synthetic,

    /// Asset type has not yet been determined.
    #[default]
    Unknown,
}

/// Common token or asset standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AssetStandard {
    /// Native blockchain currency.
    Native,

    /// Ethereum-compatible fungible token.
    Erc20,

    /// Ethereum-compatible non-fungible token.
    Erc721,

    /// Ethereum-compatible multi-token.
    Erc1155,

    /// TRON fungible token.
    Trc20,

    /// TRON non-fungible token.
    Trc721,

    /// BNB Beacon Chain token standard.
    Bep2,

    /// BNB Smart Chain fungible token.
    Bep20,

    /// Solana Program Library token.
    SplToken,

    /// Solana Token-2022 asset.
    SplToken2022,

    /// Bitcoin or Bitcoin-like native asset.
    UtxoNative,

    /// Cosmos SDK denomination.
    CosmosDenom,

    /// Cardano native asset.
    CardanoNative,

    /// Sui coin type.
    SuiCoin,

    /// Aptos coin or fungible asset.
    AptosCoin,

    /// Project-specific standard.
    Custom,

    /// Standard has not yet been determined.
    #[default]
    Unknown,
}

/// Lifecycle and observation status of an asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AssetStatus {
    /// Asset is active and available for observation.
    #[default]
    Active,

    /// Asset has been paused by its issuer or protocol.
    Paused,

    /// Asset is deprecated but still historically relevant.
    Deprecated,

    /// Asset is suspected to be fraudulent or misleading.
    Suspicious,

    /// Asset has been confirmed as malicious.
    Malicious,

    /// Asset is no longer accessible or operational.
    Inactive,
}

/// Verification state of asset identity or metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AssetVerification {
    /// Asset has not yet been independently verified.
    #[default]
    Unverified,

    /// Asset identity has been verified by one or more trusted sources.
    Verified,

    /// Asset identity is disputed or inconsistent across sources.
    Conflicting,

    /// Asset identity has been rejected.
    Rejected,
}

/// Blockchain asset domain object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    id: AssetId,
    network_id: NetworkId,
    contract_id: Option<ContractId>,
    name: String,
    symbol: String,
    kind: AssetKind,
    standard: AssetStandard,
    status: AssetStatus,
    verification: AssetVerification,
    decimals: Option<u8>,
    total_supply: Option<String>,
    token_id: Option<String>,
    canonical_asset: Option<AssetId>,
    logo_uri: Option<String>,
    metadata_uri: Option<String>,
}

impl Asset {
    /// Creates a native blockchain asset.
    pub fn native(
        network_id: NetworkId,
        name: impl Into<String>,
        symbol: impl Into<String>,
        decimals: u8,
    ) -> Result<Self> {
        let mut asset = Self::new(
            network_id,
            name,
            symbol,
            AssetKind::Native,
            AssetStandard::Native,
        )?;

        asset.set_decimals(decimals)?;

        Ok(asset)
    }

    /// Creates a contract-backed blockchain asset.
    pub fn contract(
        network_id: NetworkId,
        contract_id: ContractId,
        name: impl Into<String>,
        symbol: impl Into<String>,
        kind: AssetKind,
        standard: AssetStandard,
    ) -> Result<Self> {
        if matches!(kind, AssetKind::Native) {
            return Err(invalid_argument(
                "contract-backed asset cannot use the native asset kind",
            ));
        }

        if matches!(standard, AssetStandard::Native) {
            return Err(invalid_argument(
                "contract-backed asset cannot use the native asset standard",
            ));
        }

        let mut asset = Self::new(network_id, name, symbol, kind, standard)?;

        asset.contract_id = Some(contract_id);

        Ok(asset)
    }

    /// Creates an asset using explicit classification.
    pub fn new(
        network_id: NetworkId,
        name: impl Into<String>,
        symbol: impl Into<String>,
        kind: AssetKind,
        standard: AssetStandard,
    ) -> Result<Self> {
        let name = normalize_required_text(name.into(), "asset name")?;

        let symbol = normalize_symbol(symbol.into())?;

        validate_kind_and_standard(kind, standard)?;

        Ok(Self {
            id: AssetId::new(),
            network_id,
            contract_id: None,
            name,
            symbol,
            kind,
            standard,
            status: AssetStatus::Active,
            verification: AssetVerification::Unverified,
            decimals: None,
            total_supply: None,
            token_id: None,
            canonical_asset: None,
            logo_uri: None,
            metadata_uri: None,
        })
    }

    /// Returns the asset identifier.
    #[must_use]
    pub const fn id(&self) -> AssetId {
        self.id
    }

    /// Returns the network identifier.
    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    /// Returns the backing contract identifier when present.
    #[must_use]
    pub const fn contract_id(&self) -> Option<ContractId> {
        self.contract_id
    }

    /// Assigns a backing contract.
    pub fn set_contract(&mut self, contract_id: ContractId) -> Result<()> {
        if self.is_native() {
            return Err(invalid_argument(
                "native asset cannot be associated with a token contract",
            ));
        }

        self.contract_id = Some(contract_id);
        Ok(())
    }

    /// Removes the backing contract.
    pub fn clear_contract(&mut self) {
        self.contract_id = None;
    }

    /// Returns the asset name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Changes the asset name.
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<()> {
        self.name = normalize_required_text(name.into(), "asset name")?;

        Ok(())
    }

    /// Returns the asset symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Changes the asset symbol.
    pub fn set_symbol(&mut self, symbol: impl Into<String>) -> Result<()> {
        self.symbol = normalize_symbol(symbol.into())?;
        Ok(())
    }

    /// Returns the asset classification.
    #[must_use]
    pub const fn kind(&self) -> AssetKind {
        self.kind
    }

    /// Returns the asset standard.
    #[must_use]
    pub const fn standard(&self) -> AssetStandard {
        self.standard
    }

    /// Updates the asset classification and standard.
    pub fn set_classification(&mut self, kind: AssetKind, standard: AssetStandard) -> Result<()> {
        validate_kind_and_standard(kind, standard)?;

        if matches!(kind, AssetKind::Native) && self.contract_id.is_some() {
            return Err(invalid_argument(
                "native asset cannot retain a backing contract",
            ));
        }

        self.kind = kind;
        self.standard = standard;

        Ok(())
    }

    /// Returns the asset lifecycle status.
    #[must_use]
    pub const fn status(&self) -> AssetStatus {
        self.status
    }

    /// Changes the asset lifecycle status.
    pub const fn set_status(&mut self, status: AssetStatus) {
        self.status = status;
    }

    /// Returns whether the asset is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.status, AssetStatus::Active)
    }

    /// Returns whether the asset is considered suspicious or malicious.
    #[must_use]
    pub const fn is_risky(&self) -> bool {
        matches!(
            self.status,
            AssetStatus::Suspicious | AssetStatus::Malicious
        )
    }

    /// Returns the asset verification state.
    #[must_use]
    pub const fn verification(&self) -> AssetVerification {
        self.verification
    }

    /// Changes the asset verification state.
    pub const fn set_verification(&mut self, verification: AssetVerification) {
        self.verification = verification;
    }

    /// Marks the asset as verified.
    pub const fn mark_verified(&mut self) {
        self.verification = AssetVerification::Verified;
    }

    /// Returns whether the asset identity is verified.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        matches!(self.verification, AssetVerification::Verified)
    }

    /// Returns the decimal precision when known.
    #[must_use]
    pub const fn decimals(&self) -> Option<u8> {
        self.decimals
    }

    /// Assigns decimal precision.
    pub fn set_decimals(&mut self, decimals: u8) -> Result<()> {
        if decimals > MAX_ASSET_DECIMALS {
            return Err(invalid_argument(format!(
                "asset decimals must not exceed {MAX_ASSET_DECIMALS}"
            )));
        }

        self.decimals = Some(decimals);
        Ok(())
    }

    /// Removes decimal precision information.
    pub const fn clear_decimals(&mut self) {
        self.decimals = None;
    }

    /// Returns total supply as its exact source representation.
    #[must_use]
    pub fn total_supply(&self) -> Option<&str> {
        self.total_supply.as_deref()
    }

    /// Assigns total supply without converting it to a fixed integer width.
    pub fn set_total_supply(&mut self, total_supply: impl Into<String>) -> Result<()> {
        let total_supply = normalize_unsigned_integer(total_supply.into(), "total supply")?;

        self.total_supply = Some(total_supply);
        Ok(())
    }

    /// Removes total supply information.
    pub fn clear_total_supply(&mut self) {
        self.total_supply = None;
    }

    /// Returns the NFT or multi-token identifier when present.
    #[must_use]
    pub fn token_id(&self) -> Option<&str> {
        self.token_id.as_deref()
    }

    /// Assigns an NFT or multi-token identifier.
    pub fn set_token_id(&mut self, token_id: impl Into<String>) -> Result<()> {
        if !matches!(
            self.kind,
            AssetKind::NonFungibleToken | AssetKind::MultiToken
        ) {
            return Err(invalid_argument(
                "token identifier is only valid for NFT or multi-token assets",
            ));
        }

        self.token_id = Some(normalize_required_text(
            token_id.into(),
            "token identifier",
        )?);

        Ok(())
    }

    /// Removes the token identifier.
    pub fn clear_token_id(&mut self) {
        self.token_id = None;
    }

    /// Returns the canonical underlying asset for wrapped or synthetic assets.
    #[must_use]
    pub const fn canonical_asset(&self) -> Option<AssetId> {
        self.canonical_asset
    }

    /// Associates this asset with its canonical underlying asset.
    pub fn set_canonical_asset(&mut self, asset_id: AssetId) -> Result<()> {
        if asset_id == self.id {
            return Err(invalid_argument(
                "asset cannot reference itself as its canonical asset",
            ));
        }

        self.canonical_asset = Some(asset_id);
        Ok(())
    }

    /// Removes the canonical asset relationship.
    pub const fn clear_canonical_asset(&mut self) {
        self.canonical_asset = None;
    }

    /// Returns the logo URI.
    #[must_use]
    pub fn logo_uri(&self) -> Option<&str> {
        self.logo_uri.as_deref()
    }

    /// Assigns the logo URI.
    pub fn set_logo_uri(&mut self, uri: impl Into<String>) -> Result<()> {
        self.logo_uri = Some(normalize_uri(uri.into(), "logo URI")?);

        Ok(())
    }

    /// Removes the logo URI.
    pub fn clear_logo_uri(&mut self) {
        self.logo_uri = None;
    }

    /// Returns the metadata URI.
    #[must_use]
    pub fn metadata_uri(&self) -> Option<&str> {
        self.metadata_uri.as_deref()
    }

    /// Assigns the metadata URI.
    pub fn set_metadata_uri(&mut self, uri: impl Into<String>) -> Result<()> {
        self.metadata_uri = Some(normalize_uri(uri.into(), "metadata URI")?);

        Ok(())
    }

    /// Removes the metadata URI.
    pub fn clear_metadata_uri(&mut self) {
        self.metadata_uri = None;
    }

    /// Returns whether this is a native network currency.
    #[must_use]
    pub const fn is_native(&self) -> bool {
        matches!(self.kind, AssetKind::Native) || matches!(self.standard, AssetStandard::Native)
    }

    /// Returns whether this asset is contract-backed.
    #[must_use]
    pub const fn is_contract_backed(&self) -> bool {
        self.contract_id.is_some()
    }

    /// Returns whether this represents an NFT-like asset.
    #[must_use]
    pub const fn is_non_fungible(&self) -> bool {
        matches!(
            self.kind,
            AssetKind::NonFungibleToken | AssetKind::MultiToken
        )
    }
}

fn validate_kind_and_standard(kind: AssetKind, standard: AssetStandard) -> Result<()> {
    let kind_is_native = matches!(kind, AssetKind::Native);
    let standard_is_native = matches!(standard, AssetStandard::Native);

    if kind_is_native != standard_is_native {
        return Err(invalid_argument(
            "native asset kind and native asset standard must be used together",
        ));
    }

    Ok(())
}

fn normalize_required_text(value: String, field: &str) -> Result<String> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(invalid_argument(format!("{field} must not be empty")));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_argument(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(value)
}

fn normalize_symbol(value: String) -> Result<String> {
    let value = normalize_required_text(value, "asset symbol")?;

    if value.chars().any(char::is_whitespace) {
        return Err(invalid_argument("asset symbol must not contain whitespace"));
    }

    if value.len() > 32 {
        return Err(invalid_argument(
            "asset symbol must not exceed 32 characters",
        ));
    }

    Ok(value)
}

fn normalize_unsigned_integer(value: String, field: &str) -> Result<String> {
    let value = value.trim();

    if value.is_empty() {
        return Err(invalid_argument(format!("{field} must not be empty")));
    }

    if !value.bytes().all(|character| character.is_ascii_digit()) {
        return Err(invalid_argument(format!(
            "{field} must contain only decimal digits"
        )));
    }

    let normalized = value.trim_start_matches('0');

    if normalized.is_empty() {
        Ok("0".to_owned())
    } else {
        Ok(normalized.to_owned())
    }
}

fn normalize_uri(value: String, field: &str) -> Result<String> {
    let value = normalize_required_text(value, field)?;

    if value.chars().any(char::is_whitespace) {
        return Err(invalid_argument(format!(
            "{field} must not contain whitespace"
        )));
    }

    let supported = ["https://", "http://", "ipfs://", "ar://", "data:"];

    if !supported.iter().any(|prefix| value.starts_with(prefix)) {
        return Err(invalid_argument(format!(
            "{field} uses an unsupported URI scheme"
        )));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn network_id() -> NetworkId {
        NetworkId::new()
    }

    #[test]
    fn creates_native_asset() {
        let asset = Asset::native(network_id(), "Ether", "ETH", 18).unwrap();

        assert!(asset.is_native());
        assert!(!asset.is_contract_backed());
        assert_eq!(asset.kind(), AssetKind::Native);
        assert_eq!(asset.standard(), AssetStandard::Native);
        assert_eq!(asset.decimals(), Some(18));
    }

    #[test]
    fn creates_contract_asset() {
        let contract_id = ContractId::new();

        let asset = Asset::contract(
            network_id(),
            contract_id,
            "Tether USD",
            "USDT",
            AssetKind::Stablecoin,
            AssetStandard::Erc20,
        )
        .unwrap();

        assert_eq!(asset.contract_id(), Some(contract_id));
        assert!(asset.is_contract_backed());
        assert!(!asset.is_native());
    }

    #[test]
    fn rejects_inconsistent_native_classification() {
        let result = Asset::new(
            network_id(),
            "Invalid",
            "INV",
            AssetKind::Native,
            AssetStandard::Erc20,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_contract_backed_native_asset() {
        let result = Asset::contract(
            network_id(),
            ContractId::new(),
            "Ether",
            "ETH",
            AssetKind::Native,
            AssetStandard::Native,
        );

        assert!(result.is_err());
    }

    #[test]
    fn validates_asset_name_and_symbol() {
        assert!(Asset::native(network_id(), "", "ETH", 18,).is_err());

        assert!(Asset::native(network_id(), "Ether", "E TH", 18,).is_err());
    }

    #[test]
    fn decimal_precision_is_bounded() {
        let result = Asset::native(network_id(), "Extreme", "EXT", MAX_ASSET_DECIMALS + 1);

        assert!(result.is_err());
    }

    #[test]
    fn total_supply_is_normalized() {
        let mut asset = Asset::contract(
            network_id(),
            ContractId::new(),
            "Token",
            "TKN",
            AssetKind::FungibleToken,
            AssetStandard::Erc20,
        )
        .unwrap();

        asset.set_total_supply("0001000").unwrap();

        assert_eq!(asset.total_supply(), Some("1000"));

        asset.set_total_supply("0000").unwrap();

        assert_eq!(asset.total_supply(), Some("0"));
    }

    #[test]
    fn invalid_total_supply_is_rejected() {
        let mut asset = Asset::contract(
            network_id(),
            ContractId::new(),
            "Token",
            "TKN",
            AssetKind::FungibleToken,
            AssetStandard::Erc20,
        )
        .unwrap();

        assert!(asset.set_total_supply("-1").is_err());
        assert!(asset.set_total_supply("1.5").is_err());
    }

    #[test]
    fn token_id_is_restricted_to_nft_assets() {
        let mut fungible = Asset::contract(
            network_id(),
            ContractId::new(),
            "Token",
            "TKN",
            AssetKind::FungibleToken,
            AssetStandard::Erc20,
        )
        .unwrap();

        assert!(fungible.set_token_id("1").is_err());

        let mut nft = Asset::contract(
            network_id(),
            ContractId::new(),
            "Collection",
            "NFT",
            AssetKind::NonFungibleToken,
            AssetStandard::Erc721,
        )
        .unwrap();

        nft.set_token_id("42").unwrap();

        assert_eq!(nft.token_id(), Some("42"));
        assert!(nft.is_non_fungible());
    }

    #[test]
    fn canonical_asset_cannot_reference_self() {
        let mut asset = Asset::contract(
            network_id(),
            ContractId::new(),
            "Wrapped Ether",
            "WETH",
            AssetKind::Wrapped,
            AssetStandard::Erc20,
        )
        .unwrap();

        assert!(asset.set_canonical_asset(asset.id()).is_err());

        let underlying = AssetId::new();

        asset.set_canonical_asset(underlying).unwrap();

        assert_eq!(asset.canonical_asset(), Some(underlying));
    }

    #[test]
    fn uri_fields_accept_supported_schemes() {
        let mut asset = Asset::contract(
            network_id(),
            ContractId::new(),
            "Token",
            "TKN",
            AssetKind::FungibleToken,
            AssetStandard::Erc20,
        )
        .unwrap();

        asset.set_logo_uri("ipfs://example-logo").unwrap();

        asset
            .set_metadata_uri("https://example.invalid/metadata.json")
            .unwrap();

        assert_eq!(asset.logo_uri(), Some("ipfs://example-logo"));

        assert_eq!(
            asset.metadata_uri(),
            Some("https://example.invalid/metadata.json")
        );
    }

    #[test]
    fn unsupported_uri_scheme_is_rejected() {
        let mut asset = Asset::contract(
            network_id(),
            ContractId::new(),
            "Token",
            "TKN",
            AssetKind::FungibleToken,
            AssetStandard::Erc20,
        )
        .unwrap();

        assert!(asset
            .set_logo_uri("ftp://example.invalid/logo.png")
            .is_err());
    }

    #[test]
    fn status_and_verification_can_be_managed() {
        let mut asset = Asset::native(network_id(), "Ether", "ETH", 18).unwrap();

        assert!(asset.is_active());
        assert!(!asset.is_verified());

        asset.set_status(AssetStatus::Suspicious);
        asset.mark_verified();

        assert!(asset.is_risky());
        assert!(asset.is_verified());
    }

    #[test]
    fn identifiers_are_unique() {
        let first = Asset::native(network_id(), "Ether", "ETH", 18).unwrap();

        let second = Asset::native(first.network_id(), "Ether", "ETH", 18).unwrap();

        assert_ne!(first.id(), second.id());
    }
}
