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
    },

    /// Print the project roadmap.
    Roadmap,

    /// Print the WDRP research constitution.
    Wdrp,

    /// Print the current workspace status summary.
    Status,
}

impl Cli {
    /// Parses command-line arguments from the current process.
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
