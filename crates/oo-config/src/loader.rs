// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-config/src/loader.rs
// Purpose : Load, merge and attribute configuration from a directory.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Load, merge and attribute configuration from a directory.
//!
//! Loading produces two things: the typed configuration, and the provenance of
//! the configuration itself. Each file is recorded with its integrity digest,
//! and each environment override is recorded by name, so a report can state
//! exactly which inputs governed a run. A finding whose configuration cannot be
//! identified cannot be reproduced.

use std::path::{Path, PathBuf};

use oo_utils::{fs, Digest};
use serde::Serialize;

use crate::{
    environment::{Credentials, EnvironmentOverrides},
    error::{ConfigError, ConfigResult},
    model::{ChainsFile, Config, DefaultsFile, ProvidersFile, WalletsFile},
    validation,
};

/// File name of the required base configuration.
pub const DEFAULTS_FILE: &str = "default.toml";
/// File name of the chain definitions.
pub const CHAINS_FILE: &str = "chains.toml";
/// File name of the provider definitions.
pub const PROVIDERS_FILE: &str = "providers.toml";
/// File name of the wallet definitions.
pub const WALLETS_FILE: &str = "wallets.toml";

/// One configuration file and the digest of the bytes that were read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigSource {
    /// Path as given to the loader.
    pub path: PathBuf,
    /// Integrity digest of the file contents, in `sha256:hex` form.
    pub digest: String,
    /// Byte length of the file that was read.
    pub bytes: usize,
}

/// Everything that determined the configuration of a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigProvenance {
    /// Directory the configuration was loaded from.
    pub directory: PathBuf,
    /// Files that were read, in load order.
    pub sources: Vec<ConfigSource>,
    /// Names of the environment variables that overrode a file value.
    pub environment_overrides: Vec<String>,
}

impl ConfigProvenance {
    /// Returns a digest covering every configuration input.
    ///
    /// Two runs with the same digest read the same configuration files; a
    /// differing digest is enough to explain a differing result.
    #[must_use]
    pub fn combined_digest(&self) -> Digest {
        let mut parts: Vec<String> = Vec::with_capacity(self.sources.len() * 2);
        for source in &self.sources {
            parts.push(source.path.to_string_lossy().into_owned());
            parts.push(source.digest.clone());
        }
        Digest::of_str_parts(parts.iter().map(String::as_str))
    }

    /// Returns whether any environment variable overrode a file value.
    #[must_use]
    pub fn has_environment_overrides(&self) -> bool {
        !self.environment_overrides.is_empty()
    }
}

/// A loaded configuration together with its provenance and credentials.
#[derive(Debug, Clone)]
pub struct LoadedConfig {
    /// The validated configuration.
    pub config: Config,
    /// Where the configuration came from.
    pub provenance: ConfigProvenance,
    /// Credentials resolved from the environment.
    pub credentials: Credentials,
}

/// Loads configuration from a directory, applying environment overrides.
///
/// `default.toml` is required. The chain, provider and wallet files are
/// optional, but a file that exists and does not parse is an error rather than
/// a silently empty table: a missing definition would otherwise look like a
/// deliberate absence.
pub fn load_from_directory(directory: impl AsRef<Path>) -> ConfigResult<LoadedConfig> {
    let overrides = EnvironmentOverrides::from_env()?;
    load_with_overrides(directory, &overrides)
}

/// Loads configuration from a directory using explicit overrides.
pub fn load_with_overrides(
    directory: impl AsRef<Path>,
    overrides: &EnvironmentOverrides,
) -> ConfigResult<LoadedConfig> {
    let directory = directory.as_ref();
    let mut sources = Vec::new();

    let defaults_path = directory.join(DEFAULTS_FILE);
    let defaults_raw = read_required(&defaults_path)?;
    sources.push(source_of(&defaults_path, &defaults_raw));
    let defaults: DefaultsFile = parse(&defaults_path, &defaults_raw)?;

    let mut config = Config {
        application: defaults.application,
        logging: defaults.logging,
        data: defaults.data,
        rpc: defaults.rpc,
        research: defaults.research,
        chains: Default::default(),
        providers: Default::default(),
        wallets: Default::default(),
    };

    let chains_path = directory.join(CHAINS_FILE);
    if let Some(raw) = read_optional(&chains_path)? {
        sources.push(source_of(&chains_path, &raw));
        let file: ChainsFile = parse(&chains_path, &raw)?;
        config.chains = file.chains;
    }

    let providers_path = directory.join(PROVIDERS_FILE);
    if let Some(raw) = read_optional(&providers_path)? {
        sources.push(source_of(&providers_path, &raw));
        let file: ProvidersFile = parse(&providers_path, &raw)?;
        config.providers = file.providers;
    }

    let wallets_path = directory.join(WALLETS_FILE);
    if let Some(raw) = read_optional(&wallets_path)? {
        sources.push(source_of(&wallets_path, &raw));
        let file: WalletsFile = parse(&wallets_path, &raw)?;
        config.wallets = file.wallets;
    }

    // Overrides are applied before validation, so an override cannot introduce
    // a state the validator would have rejected in a file.
    overrides.apply(&mut config);
    validation::validate(&config)?;

    Ok(LoadedConfig {
        config,
        provenance: ConfigProvenance {
            directory: directory.to_path_buf(),
            sources,
            environment_overrides: overrides.applied_variables().to_vec(),
        },
        credentials: overrides.credentials().clone(),
    })
}

fn read_required(path: &Path) -> ConfigResult<String> {
    if !fs::is_file(path) {
        return Err(ConfigError::MissingFile {
            path: path.display().to_string(),
        });
    }
    fs::read_to_string(path).map_err(|source| ConfigError::Unreadable {
        path: path.display().to_string(),
        source,
    })
}

fn read_optional(path: &Path) -> ConfigResult<Option<String>> {
    if !fs::is_file(path) {
        return Ok(None);
    }
    fs::read_to_string(path)
        .map(Some)
        .map_err(|source| ConfigError::Unreadable {
            path: path.display().to_string(),
            source,
        })
}

fn parse<T>(path: &Path, raw: &str) -> ConfigResult<T>
where
    T: serde::de::DeserializeOwned,
{
    toml::from_str(raw)
        .map_err(|error| ConfigError::invalid(path.display().to_string(), error.to_string()))
}

fn source_of(path: &Path, raw: &str) -> ConfigSource {
    ConfigSource {
        path: path.to_path_buf(),
        digest: Digest::of_str(raw).qualified(),
        bytes: raw.len(),
    }
}

#[cfg(test)]
mod tests {
    use std::{fs as std_fs, path::PathBuf};

    use super::*;
    use crate::model::{ChainKind, WdrpConfidence};

    const DEFAULTS: &str = r#"
[application]
name = "Origin Observer"
environment = "development"

[logging]
level = "info"
format = "pretty"

[data]
root = "./data"
datasets = "./datasets"
evidence = "./evidence"
experiments = "./experiments"
reports = "./reports/generated"
snapshots = "./snapshots/generated"

[rpc]
request_timeout_seconds = 30
max_retries = 2
allow_unpinned_latest_block = false

[research]
minimum_accepted_confidence = "L5"
require_evidence_for_findings = true
require_reproduction_for_conclusions = true
"#;

    const CHAINS: &str = r#"
[chains.ethereum]
id = "ethereum"
name = "Ethereum Mainnet"
family = "evm"
kind = "mainnet"
chain_id = 1
native_symbol = "ETH"
rpc_endpoints = ["https://rpc.example.org"]
explorer_url = "https://explorer.example.org"
"#;

    fn scratch(name: &str) -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("oo-config-loader-{name}-{}", std::process::id()));
        let _ = std_fs::remove_dir_all(&path);
        std_fs::create_dir_all(&path).expect("scratch directory");
        path
    }

    fn write(directory: &Path, name: &str, contents: &str) {
        std_fs::write(directory.join(name), contents).expect("write configuration");
    }

    #[test]
    fn a_directory_with_only_defaults_loads() {
        let directory = scratch("defaults-only");
        write(&directory, DEFAULTS_FILE, DEFAULTS);

        let loaded = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect("configuration");
        assert_eq!(loaded.config.application.name, "Origin Observer");
        assert_eq!(
            loaded.config.research.minimum_accepted_confidence,
            WdrpConfidence::L5
        );
        assert!(loaded.config.chains.is_empty());
        assert_eq!(loaded.provenance.sources.len(), 1);
        let _ = std_fs::remove_dir_all(&directory);
    }

    #[test]
    fn chain_definitions_are_merged_and_attributed() {
        let directory = scratch("chains");
        write(&directory, DEFAULTS_FILE, DEFAULTS);
        write(&directory, CHAINS_FILE, CHAINS);

        let loaded = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect("configuration");
        let chain = loaded.config.chain("ethereum").expect("ethereum");
        assert_eq!(chain.kind, ChainKind::Mainnet);
        assert_eq!(chain.chain_id, Some(1));
        assert!(chain.enabled, "chains default to enabled");

        assert_eq!(loaded.provenance.sources.len(), 2);
        assert!(loaded
            .provenance
            .sources
            .iter()
            .all(|source| source.digest.starts_with("sha256:")));
        let _ = std_fs::remove_dir_all(&directory);
    }

    #[test]
    fn a_missing_base_file_names_the_path() {
        let directory = scratch("missing");
        let error = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect_err("must fail");
        assert!(error.to_string().contains(DEFAULTS_FILE), "{error}");
        let _ = std_fs::remove_dir_all(&directory);
    }

    #[test]
    fn an_unparsable_optional_file_is_an_error_not_an_empty_table() {
        let directory = scratch("unparsable");
        write(&directory, DEFAULTS_FILE, DEFAULTS);
        write(&directory, CHAINS_FILE, "chains = [not toml");

        let error = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect_err("must fail");
        assert!(error.to_string().contains(CHAINS_FILE), "{error}");
        let _ = std_fs::remove_dir_all(&directory);
    }

    #[test]
    fn environment_overrides_are_applied_and_recorded() {
        let directory = scratch("overrides");
        write(&directory, DEFAULTS_FILE, DEFAULTS);

        let overrides = EnvironmentOverrides::from_pairs([
            ("OO_LOG_LEVEL", "debug"),
            ("OO_RPC_TIMEOUT_SECONDS", "45"),
        ])
        .expect("overrides");
        let loaded = load_with_overrides(&directory, &overrides).expect("configuration");

        assert_eq!(loaded.config.rpc.request_timeout_seconds, 45);
        assert!(loaded.provenance.has_environment_overrides());
        assert_eq!(
            loaded.provenance.environment_overrides,
            vec![
                "OO_LOG_LEVEL".to_owned(),
                "OO_RPC_TIMEOUT_SECONDS".to_owned()
            ]
        );
        let _ = std_fs::remove_dir_all(&directory);
    }

    #[test]
    fn an_override_cannot_introduce_a_state_validation_rejects() {
        let directory = scratch("override-invalid");
        write(&directory, DEFAULTS_FILE, DEFAULTS);

        let overrides = EnvironmentOverrides::from_pairs([("OO_RPC_TIMEOUT_SECONDS", "100000")])
            .expect("overrides");
        assert!(load_with_overrides(&directory, &overrides).is_err());
        let _ = std_fs::remove_dir_all(&directory);
    }

    #[test]
    fn provenance_digest_changes_when_a_file_changes() {
        let directory = scratch("digest");
        write(&directory, DEFAULTS_FILE, DEFAULTS);
        let first = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect("configuration")
            .provenance
            .combined_digest();

        write(
            &directory,
            DEFAULTS_FILE,
            &DEFAULTS.replace("max_retries = 2", "max_retries = 3"),
        );
        let second = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect("configuration")
            .provenance
            .combined_digest();

        assert_ne!(first, second);
        let _ = std_fs::remove_dir_all(&directory);
    }

    #[test]
    fn loading_the_same_directory_twice_produces_the_same_digest() {
        let directory = scratch("stable-digest");
        write(&directory, DEFAULTS_FILE, DEFAULTS);
        write(&directory, CHAINS_FILE, CHAINS);

        let first = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect("configuration")
            .provenance
            .combined_digest();
        let second = load_with_overrides(&directory, &EnvironmentOverrides::default())
            .expect("configuration")
            .provenance
            .combined_digest();

        assert_eq!(first, second);
        let _ = std_fs::remove_dir_all(&directory);
    }
}
