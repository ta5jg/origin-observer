// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cli/src/commands.rs
// Purpose : Implement the commands module for oo-cli.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Implements the commands module for oo-cli.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// Output format for observation runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ObserveFormat {
    /// Emit the full investigation JSON.
    InvestigationJson,
    /// Emit the machine-readable report JSON.
    ReportJson,
    /// Emit a concise human-readable report.
    Human,
}

/// Built-in observation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ObserveStrategy {
    /// Observe the chain id with eth_chainId.
    ChainId,
    /// Observe account balance with eth_getBalance.
    Balance,
    /// Observe contract bytecode with eth_getCode.
    ContractCode,
    /// Observe ERC-20 name, symbol and decimals with eth_call.
    Erc20Metadata,
    /// Observe wallet balance and code state.
    WalletOverview,
    /// Classify a contract's proxy architecture from its bytecode and known
    /// EIP-1967/1822/legacy-OZ storage slots.
    ProxyClassification,
    /// Read the underlying eth_getCode observation's discovery decision
    /// through every built-in wallet adapter's documented capability.
    WalletView,
}

/// Cache state to attach to an observation.
///
/// The CLI has no way to observe a real wallet's cache directly — an RPC
/// endpoint has no concept of a wallet's client-side cache — so this is
/// always a value the caller declares about their own test setup (`empty`
/// after clearing state, `warm` immediately after a prior run), never a
/// measurement. It defaults to `empty` because a fresh CLI invocation is the
/// closest available approximation, and the declared state is always echoed
/// back in the output rather than silently assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum CacheStateArg {
    /// Cache state is not known.
    Unknown,
    /// No cached value was present.
    #[default]
    Empty,
    /// A cached value was present and appeared current.
    Warm,
    /// A cached value was present but appeared stale.
    Stale,
    /// The cache was invalidated as part of this run.
    Invalidated,
}

impl From<CacheStateArg> for oo_model::cache::CacheState {
    fn from(value: CacheStateArg) -> Self {
        match value {
            CacheStateArg::Unknown => Self::Unknown,
            CacheStateArg::Empty => Self::Empty,
            CacheStateArg::Warm => Self::Warm,
            CacheStateArg::Stale => Self::Stale,
            CacheStateArg::Invalidated => Self::Invalidated,
        }
    }
}

/// Origin Observer command-line arguments.
#[derive(Debug, Parser)]
#[command(
    name = "oo",
    version,
    about = "Evidence-based wallet discovery research tooling."
)]
pub struct Cli {
    /// Command to execute.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Supported Origin Observer commands.
#[derive(Debug, Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Command {
    /// Run a local fixture observation through the investigation pipeline.
    Observe {
        /// Observation subject.
        #[arg(long, default_value = "eth_chainId")]
        subject: String,

        /// Built-in strategy that fills subject, method and params.
        #[arg(long, value_enum)]
        strategy: Option<ObserveStrategy>,

        /// Address used by the balance strategy.
        #[arg(long)]
        address: Option<String>,

        /// JSON-RPC endpoint URL. Repeat to run a multi-provider reproduction check.
        #[arg(long)]
        rpc_url: Vec<String>,

        /// Named provider in name=url form. Repeat to run named reproduction checks.
        #[arg(long)]
        provider: Vec<String>,

        /// JSON-RPC method for live RPC mode.
        #[arg(long, default_value = "eth_chainId")]
        method: String,

        /// JSON-RPC params JSON for live RPC mode.
        #[arg(long, default_value = "[]")]
        params_json: String,

        /// JSON-RPC style result value for the fixture payload.
        #[arg(long, default_value = "0x1")]
        result: String,

        /// Full JSON payload to observe. Overrides --result when provided.
        #[arg(long)]
        payload_json: Option<String>,

        /// Path to a JSON payload file. Overrides --payload-json and --result.
        #[arg(long)]
        payload_file: Option<PathBuf>,

        /// Output format.
        #[arg(long, value_enum, default_value_t = ObserveFormat::InvestigationJson)]
        format: ObserveFormat,

        /// Directory to write observation artifacts.
        #[arg(long)]
        out: Option<PathBuf>,

        /// Cache state to declare for this run's observations. The CLI
        /// cannot observe a wallet's cache directly; this is always a
        /// caller-declared assumption, echoed back in the output.
        #[arg(long, value_enum, default_value_t = CacheStateArg::default())]
        cache_state: CacheStateArg,

        /// Restrict the wallet-view strategy to one built-in wallet's
        /// configuration id (e.g. `metamask`). Applies to every adapter when
        /// omitted.
        #[arg(long)]
        wallet: Option<String>,

        /// Append this run's recognition to a case-study JSON file, creating
        /// it if it does not exist. Requires `--question-id`.
        #[arg(long)]
        record_history: Option<PathBuf>,

        /// Permanent research question the case study addresses, e.g.
        /// `RQ-0006`. Required with `--record-history`.
        #[arg(long)]
        question_id: Option<String>,
    },

    /// Print the project roadmap.
    Roadmap,

    /// Print the WDRP research constitution.
    Wdrp,

    /// Print the current workspace status summary.
    Status,

    /// Load the project configuration and report what governs a run.
    Config {
        /// Directory holding default.toml, chains.toml, providers.toml and wallets.toml.
        #[arg(long, default_value = "config")]
        dir: PathBuf,

        /// Emit machine-readable JSON instead of human-readable text.
        #[arg(long)]
        json: bool,
    },
}

impl Cli {
    /// Parses command-line arguments from the current process.
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
