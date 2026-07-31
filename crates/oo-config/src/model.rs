// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-config/src/model.rs
// Purpose : Typed configuration model for chains, providers and wallets.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Typed configuration model for chains, providers and wallets.
//!
//! Configuration is part of an observation's provenance: which endpoint was
//! called, which provider answered and which research thresholds were in force
//! all change what a finding means. The model therefore keeps every value typed
//! and attributable rather than passing raw strings downstream.

use std::{collections::BTreeMap, fmt, path::PathBuf};

use serde::{Deserialize, Serialize};

/// WDRP confidence level.
///
/// The constitution defines exactly six levels, and only `L5` may become
/// accepted project knowledge. The codes are the serialized form, so a
/// configuration file and a report name the same level the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WdrpConfidence {
    /// Unknown.
    L0,
    /// Hypothesis.
    L1,
    /// Observed.
    L2,
    /// Reproduced.
    L3,
    /// Verified.
    L4,
    /// Independently verified.
    L5,
}

impl WdrpConfidence {
    /// Returns the constitution's meaning for this level.
    #[must_use]
    pub const fn meaning(self) -> &'static str {
        match self {
            Self::L0 => "unknown",
            Self::L1 => "hypothesis",
            Self::L2 => "observed",
            Self::L3 => "reproduced",
            Self::L4 => "verified",
            Self::L5 => "independently verified",
        }
    }

    /// Returns whether a finding at this level may be published as accepted
    /// project knowledge.
    #[must_use]
    pub const fn is_accepted_knowledge(self) -> bool {
        matches!(self, Self::L5)
    }

    /// Returns whether this level requires a reproduction record.
    #[must_use]
    pub const fn requires_reproduction(self) -> bool {
        matches!(self, Self::L3 | Self::L4 | Self::L5)
    }

    /// Parses a level code such as `L3`.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "L0" => Some(Self::L0),
            "L1" => Some(Self::L1),
            "L2" => Some(Self::L2),
            "L3" => Some(Self::L3),
            "L4" => Some(Self::L4),
            "L5" => Some(Self::L5),
            _ => None,
        }
    }
}

impl fmt::Display for WdrpConfidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::L0 => "L0",
            Self::L1 => "L1",
            Self::L2 => "L2",
            Self::L3 => "L3",
            Self::L4 => "L4",
            Self::L5 => "L5",
        };
        formatter.write_str(code)
    }
}

/// Runtime environment the tool is operating in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeEnvironment {
    /// Local development.
    #[default]
    Development,
    /// Controlled research execution.
    Research,
    /// Published, citable execution.
    Publication,
}

/// Log verbosity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Errors only.
    Error,
    /// Errors and warnings.
    Warn,
    /// Normal operational detail.
    #[default]
    Info,
    /// Diagnostic detail.
    Debug,
    /// Full trace detail.
    Trace,
}

/// Log output shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Human-readable output.
    #[default]
    Pretty,
    /// Machine-readable output.
    Json,
}

/// Application identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// Display name.
    pub name: String,
    /// Runtime environment.
    #[serde(default)]
    pub environment: RuntimeEnvironment,
}

/// Logging configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Verbosity.
    #[serde(default)]
    pub level: LogLevel,
    /// Output shape.
    #[serde(default)]
    pub format: LogFormat,
}

/// Filesystem locations for produced research material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataConfig {
    /// Root directory for generated material.
    pub root: PathBuf,
    /// Dataset directory.
    pub datasets: PathBuf,
    /// Evidence directory.
    pub evidence: PathBuf,
    /// Experiment directory.
    pub experiments: PathBuf,
    /// Report output directory.
    pub reports: PathBuf,
    /// Snapshot output directory.
    pub snapshots: PathBuf,
}

/// RPC observation limits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcConfig {
    /// Per-request timeout in seconds.
    pub request_timeout_seconds: u64,
    /// Retry attempts after the first failure.
    pub max_retries: u32,
    /// Whether an observation may read the unpinned latest block.
    ///
    /// An unpinned read cannot be reproduced, so it is refused by default.
    pub allow_unpinned_latest_block: bool,
}

/// Research thresholds that decide what may be reported as a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResearchConfig {
    /// Lowest confidence level accepted as project knowledge.
    pub minimum_accepted_confidence: WdrpConfidence,
    /// Whether a finding must cite evidence.
    pub require_evidence_for_findings: bool,
    /// Whether a conclusion must cite a reproduction record.
    pub require_reproduction_for_conclusions: bool,
}

/// Classification of a configured network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ChainKind {
    /// Production network carrying real value.
    #[default]
    Mainnet,
    /// Public test network.
    Testnet,
    /// Development-oriented public network.
    Devnet,
    /// Private or local network.
    Local,
}

impl ChainKind {
    /// Returns whether the network carries real economic value.
    #[must_use]
    pub const fn is_production(self) -> bool {
        matches!(self, Self::Mainnet)
    }
}

/// Protocol family a configured network belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ChainFamily {
    /// Ethereum Virtual Machine compatible network.
    #[default]
    Evm,
    /// Bitcoin and its derivatives.
    Bitcoin,
    /// TRON network.
    Tron,
    /// Solana network.
    Solana,
}

/// A configured blockchain network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Stable identifier used in evidence records and paths.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Protocol family.
    #[serde(default)]
    pub family: ChainFamily,
    /// Network classification.
    #[serde(default)]
    pub kind: ChainKind,
    /// EVM chain id, where the network has one.
    #[serde(default)]
    pub chain_id: Option<u64>,
    /// Native currency symbol.
    #[serde(default)]
    pub native_symbol: Option<String>,
    /// JSON-RPC endpoints, in the order they should be tried.
    #[serde(default)]
    pub rpc_endpoints: Vec<String>,
    /// Block explorer base URL.
    #[serde(default)]
    pub explorer_url: Option<String>,
    /// Whether the network may be observed.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// Category of external component that may contribute to wallet discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    /// Curated asset registry or token list.
    Registry,
    /// Block explorer.
    Explorer,
    /// Metadata service.
    Metadata,
    /// Logo or image service.
    Image,
    /// Price service.
    Price,
    /// Decentralized exchange or aggregator.
    Dex,
    /// Chain indexer.
    Indexer,
}

/// A configured external provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Stable identifier used for attribution in evidence records.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Provider category.
    pub kind: ProviderKind,
    /// Base URL of the provider's public interface.
    pub base_url: String,
    /// Whether the provider requires a credential to answer.
    #[serde(default)]
    pub requires_api_key: bool,
    /// Chain identifiers this provider covers. Empty means every chain.
    #[serde(default)]
    pub chains: Vec<String>,
    /// Whether the provider may be consulted.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// Platform a wallet runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletPlatform {
    /// Browser extension.
    Extension,
    /// Mobile application.
    Mobile,
    /// Desktop application.
    Desktop,
    /// Hardware wallet companion.
    Hardware,
    /// Web application.
    Web,
}

/// A configured wallet under observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Stable identifier used in evidence records.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Platforms the wallet is observed on.
    #[serde(default)]
    pub platforms: Vec<WalletPlatform>,
    /// Chain identifiers the wallet is observed against.
    #[serde(default)]
    pub chains: Vec<String>,
    /// Whether the wallet may be observed.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// The complete assembled configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Application identity.
    pub application: ApplicationConfig,
    /// Logging configuration.
    #[serde(default)]
    pub logging: LoggingConfig,
    /// Output locations.
    pub data: DataConfig,
    /// RPC limits.
    pub rpc: RpcConfig,
    /// Research thresholds.
    pub research: ResearchConfig,
    /// Configured networks, keyed by identifier.
    #[serde(default)]
    pub chains: BTreeMap<String, ChainConfig>,
    /// Configured providers, keyed by identifier.
    #[serde(default)]
    pub providers: BTreeMap<String, ProviderConfig>,
    /// Configured wallets, keyed by identifier.
    #[serde(default)]
    pub wallets: BTreeMap<String, WalletConfig>,
}

impl Config {
    /// Returns the enabled chains in identifier order.
    #[must_use]
    pub fn enabled_chains(&self) -> Vec<&ChainConfig> {
        self.chains.values().filter(|chain| chain.enabled).collect()
    }

    /// Returns the enabled providers in identifier order.
    #[must_use]
    pub fn enabled_providers(&self) -> Vec<&ProviderConfig> {
        self.providers
            .values()
            .filter(|provider| provider.enabled)
            .collect()
    }

    /// Returns the enabled wallets in identifier order.
    #[must_use]
    pub fn enabled_wallets(&self) -> Vec<&WalletConfig> {
        self.wallets
            .values()
            .filter(|wallet| wallet.enabled)
            .collect()
    }

    /// Looks up a chain by identifier.
    #[must_use]
    pub fn chain(&self, id: &str) -> Option<&ChainConfig> {
        self.chains.get(id)
    }

    /// Returns the enabled providers that cover a chain, including the
    /// chain-agnostic ones.
    #[must_use]
    pub fn providers_for_chain(&self, chain_id: &str) -> Vec<&ProviderConfig> {
        self.providers
            .values()
            .filter(|provider| provider.enabled)
            .filter(|provider| {
                provider.chains.is_empty() || provider.chains.iter().any(|chain| chain == chain_id)
            })
            .collect()
    }
}

/// Contents of the chain definition file.
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct ChainsFile {
    #[serde(default)]
    pub(crate) chains: BTreeMap<String, ChainConfig>,
}

/// Contents of the provider definition file.
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct ProvidersFile {
    #[serde(default)]
    pub(crate) providers: BTreeMap<String, ProviderConfig>,
}

/// Contents of the wallet definition file.
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct WalletsFile {
    #[serde(default)]
    pub(crate) wallets: BTreeMap<String, WalletConfig>,
}

/// Contents of the base configuration file.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct DefaultsFile {
    pub(crate) application: ApplicationConfig,
    #[serde(default)]
    pub(crate) logging: LoggingConfig,
    pub(crate) data: DataConfig,
    pub(crate) rpc: RpcConfig,
    pub(crate) research: ResearchConfig,
}

const fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_codes_round_trip() {
        for level in [
            WdrpConfidence::L0,
            WdrpConfidence::L1,
            WdrpConfidence::L2,
            WdrpConfidence::L3,
            WdrpConfidence::L4,
            WdrpConfidence::L5,
        ] {
            assert_eq!(WdrpConfidence::parse(&level.to_string()), Some(level));
        }
        assert_eq!(WdrpConfidence::parse("l5"), Some(WdrpConfidence::L5));
        assert_eq!(WdrpConfidence::parse("L6"), None);
    }

    #[test]
    fn only_l5_is_accepted_project_knowledge() {
        assert!(WdrpConfidence::L5.is_accepted_knowledge());
        assert!(!WdrpConfidence::L4.is_accepted_knowledge());
        assert_eq!(WdrpConfidence::L3.meaning(), "reproduced");
    }

    #[test]
    fn reproduction_is_required_from_l3_upward() {
        assert!(!WdrpConfidence::L2.requires_reproduction());
        assert!(WdrpConfidence::L3.requires_reproduction());
        assert!(WdrpConfidence::L5.requires_reproduction());
    }

    #[test]
    fn confidence_levels_are_ordered_by_strength() {
        assert!(WdrpConfidence::L0 < WdrpConfidence::L3);
        assert!(WdrpConfidence::L5 > WdrpConfidence::L4);
    }
}
