// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-config/src/validation.rs
// Purpose : Reject configuration that would produce unattributable evidence.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Reject configuration that would produce unattributable evidence.
//!
//! Validation runs after the files are merged and the environment overrides are
//! applied, because an invariant can be satisfied by each layer alone and
//! broken by their combination. Every rejection names the field and the
//! invariant, so the operator can correct the file rather than guess.

use oo_utils::validation::{
    require_http_url, require_identifier, require_non_empty, require_range, require_unique,
};

use crate::{
    error::{ConfigError, ConfigResult},
    model::{ChainConfig, Config, ProviderConfig, WalletConfig},
};

/// Largest accepted RPC timeout, in seconds.
///
/// A longer timeout stops being an observation and becomes an unbounded wait
/// that a reproduction cannot repeat within a stated procedure.
pub const MAX_REQUEST_TIMEOUT_SECONDS: u64 = 600;

/// Largest accepted retry count.
pub const MAX_RETRIES: u64 = 10;

/// Validates a fully assembled configuration.
pub fn validate(config: &Config) -> ConfigResult<()> {
    validate_application(config)?;
    validate_data(config)?;
    validate_rpc(config)?;
    validate_research(config)?;

    for (key, chain) in &config.chains {
        validate_chain(key, chain)?;
    }
    for (key, provider) in &config.providers {
        validate_provider(key, provider, config)?;
    }
    for (key, wallet) in &config.wallets {
        validate_wallet(key, wallet, config)?;
    }

    Ok(())
}

fn validate_application(config: &Config) -> ConfigResult<()> {
    require_non_empty("application.name", &config.application.name)?;
    Ok(())
}

fn validate_data(config: &Config) -> ConfigResult<()> {
    let directories = [
        ("data.root", &config.data.root),
        ("data.datasets", &config.data.datasets),
        ("data.evidence", &config.data.evidence),
        ("data.experiments", &config.data.experiments),
        ("data.reports", &config.data.reports),
        ("data.snapshots", &config.data.snapshots),
    ];
    for (field, path) in directories {
        let value = path.to_string_lossy();
        require_non_empty(field, value.as_ref())?;
    }
    Ok(())
}

fn validate_rpc(config: &Config) -> ConfigResult<()> {
    require_range(
        "rpc.request_timeout_seconds",
        config.rpc.request_timeout_seconds,
        1,
        MAX_REQUEST_TIMEOUT_SECONDS,
    )?;
    require_range(
        "rpc.max_retries",
        u64::from(config.rpc.max_retries),
        0,
        MAX_RETRIES,
    )?;
    Ok(())
}

fn validate_research(config: &Config) -> ConfigResult<()> {
    let research = &config.research;

    // A finding that may become accepted knowledge must cite evidence. Allowing
    // otherwise would let an opinion reach the same status as an observation.
    if !research.require_evidence_for_findings {
        return Err(ConfigError::rejected(
            "research.require_evidence_for_findings must remain true: the WDRP constitution requires evidence before conclusion",
        ));
    }

    // Levels L3 and above mean reproduced, verified or independently verified.
    // Accepting them while reproduction records are optional would let a
    // finding claim reproduction that was never recorded.
    if research.minimum_accepted_confidence.requires_reproduction()
        && !research.require_reproduction_for_conclusions
    {
        return Err(ConfigError::rejected(format!(
            "research.minimum_accepted_confidence is {} ({}), which requires research.require_reproduction_for_conclusions to be true",
            research.minimum_accepted_confidence,
            research.minimum_accepted_confidence.meaning()
        )));
    }

    Ok(())
}

fn validate_chain(key: &str, chain: &ChainConfig) -> ConfigResult<()> {
    require_identifier("chains.<id>", key)?;
    if chain.id != key {
        return Err(ConfigError::rejected(format!(
            "chain table key {key} does not match its id field {}",
            chain.id
        )));
    }
    require_identifier(&format!("chains.{key}.id"), &chain.id)?;
    require_non_empty(&format!("chains.{key}.name"), &chain.name)?;

    for (index, endpoint) in chain.rpc_endpoints.iter().enumerate() {
        require_http_url(&format!("chains.{key}.rpc_endpoints[{index}]"), endpoint)?;
    }
    require_unique(
        &format!("chains.{key}.rpc_endpoints"),
        chain.rpc_endpoints.iter().map(String::as_str),
    )?;

    if let Some(explorer) = &chain.explorer_url {
        require_http_url(&format!("chains.{key}.explorer_url"), explorer)?;
    }

    // An enabled network with no endpoint cannot be observed, and a run that
    // silently skipped it would report an absence of evidence as an absence of
    // the asset.
    if chain.enabled && chain.rpc_endpoints.is_empty() {
        return Err(ConfigError::rejected(format!(
            "chains.{key} is enabled but has no rpc_endpoints; disable it or give it an endpoint"
        )));
    }

    Ok(())
}

fn validate_provider(key: &str, provider: &ProviderConfig, config: &Config) -> ConfigResult<()> {
    require_identifier("providers.<id>", key)?;
    if provider.id != key {
        return Err(ConfigError::rejected(format!(
            "provider table key {key} does not match its id field {}",
            provider.id
        )));
    }
    require_non_empty(&format!("providers.{key}.name"), &provider.name)?;
    require_http_url(&format!("providers.{key}.base_url"), &provider.base_url)?;
    require_unique(
        &format!("providers.{key}.chains"),
        provider.chains.iter().map(String::as_str),
    )?;

    for chain in &provider.chains {
        if !config.chains.contains_key(chain) {
            return Err(ConfigError::rejected(format!(
                "providers.{key}.chains references unknown chain {chain}"
            )));
        }
    }

    Ok(())
}

fn validate_wallet(key: &str, wallet: &WalletConfig, config: &Config) -> ConfigResult<()> {
    require_identifier("wallets.<id>", key)?;
    if wallet.id != key {
        return Err(ConfigError::rejected(format!(
            "wallet table key {key} does not match its id field {}",
            wallet.id
        )));
    }
    require_non_empty(&format!("wallets.{key}.name"), &wallet.name)?;
    require_unique(
        &format!("wallets.{key}.chains"),
        wallet.chains.iter().map(String::as_str),
    )?;

    for chain in &wallet.chains {
        if !config.chains.contains_key(chain) {
            return Err(ConfigError::rejected(format!(
                "wallets.{key}.chains references unknown chain {chain}"
            )));
        }
    }

    if wallet.enabled && wallet.platforms.is_empty() {
        return Err(ConfigError::rejected(format!(
            "wallets.{key} is enabled but names no platform; an observation must record where the wallet ran"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::model::{
        ApplicationConfig, ChainFamily, ChainKind, DataConfig, LoggingConfig, ProviderKind,
        ResearchConfig, RpcConfig, RuntimeEnvironment, WalletPlatform, WdrpConfidence,
    };

    fn chain(id: &str) -> ChainConfig {
        ChainConfig {
            id: id.to_owned(),
            name: "Ethereum Mainnet".to_owned(),
            family: ChainFamily::Evm,
            kind: ChainKind::Mainnet,
            chain_id: Some(1),
            native_symbol: Some("ETH".to_owned()),
            rpc_endpoints: vec!["https://rpc.example.org".to_owned()],
            explorer_url: Some("https://explorer.example.org".to_owned()),
            enabled: true,
        }
    }

    fn base() -> Config {
        let mut chains = BTreeMap::new();
        chains.insert("ethereum".to_owned(), chain("ethereum"));
        Config {
            application: ApplicationConfig {
                name: "Origin Observer".to_owned(),
                environment: RuntimeEnvironment::Development,
            },
            logging: LoggingConfig::default(),
            data: DataConfig {
                root: "./data".into(),
                datasets: "./datasets".into(),
                evidence: "./evidence".into(),
                experiments: "./experiments".into(),
                reports: "./reports".into(),
                snapshots: "./snapshots".into(),
            },
            rpc: RpcConfig {
                request_timeout_seconds: 30,
                max_retries: 2,
                allow_unpinned_latest_block: false,
            },
            research: ResearchConfig {
                minimum_accepted_confidence: WdrpConfidence::L5,
                require_evidence_for_findings: true,
                require_reproduction_for_conclusions: true,
            },
            chains,
            providers: BTreeMap::new(),
            wallets: BTreeMap::new(),
        }
    }

    #[test]
    fn a_representative_configuration_is_accepted() {
        assert!(validate(&base()).is_ok());
    }

    #[test]
    fn evidence_may_not_be_made_optional() {
        let mut config = base();
        config.research.require_evidence_for_findings = false;
        let error = validate(&config).unwrap_err().to_string();
        assert!(error.contains("evidence before conclusion"), "{error}");
    }

    #[test]
    fn accepting_reproduced_findings_requires_reproduction_records() {
        let mut config = base();
        config.research.require_reproduction_for_conclusions = false;
        config.research.minimum_accepted_confidence = WdrpConfidence::L3;
        let error = validate(&config).unwrap_err().to_string();
        assert!(
            error.contains("require_reproduction_for_conclusions"),
            "{error}"
        );

        // Below L3 the requirement does not apply.
        config.research.minimum_accepted_confidence = WdrpConfidence::L2;
        assert!(validate(&config).is_ok());
    }

    #[test]
    fn an_enabled_chain_without_an_endpoint_is_rejected() {
        let mut config = base();
        config
            .chains
            .get_mut("ethereum")
            .expect("chain")
            .rpc_endpoints
            .clear();
        let error = validate(&config).unwrap_err().to_string();
        assert!(error.contains("no rpc_endpoints"), "{error}");
    }

    #[test]
    fn a_disabled_chain_without_an_endpoint_is_accepted() {
        let mut config = base();
        let chain = config.chains.get_mut("ethereum").expect("chain");
        chain.rpc_endpoints.clear();
        chain.enabled = false;
        assert!(validate(&config).is_ok());
    }

    #[test]
    fn a_table_key_must_match_its_identifier() {
        let mut config = base();
        config.chains.insert("bnb".to_owned(), chain("ethereum"));
        let error = validate(&config).unwrap_err().to_string();
        assert!(error.contains("does not match its id"), "{error}");
    }

    #[test]
    fn non_http_endpoints_are_rejected() {
        let mut config = base();
        config
            .chains
            .get_mut("ethereum")
            .expect("chain")
            .rpc_endpoints = vec!["ws://rpc.example.org".to_owned()];
        assert!(validate(&config).is_err());
    }

    #[test]
    fn duplicate_endpoints_are_rejected() {
        let mut config = base();
        config
            .chains
            .get_mut("ethereum")
            .expect("chain")
            .rpc_endpoints = vec![
            "https://rpc.example.org".to_owned(),
            "https://rpc.example.org".to_owned(),
        ];
        let error = validate(&config).unwrap_err().to_string();
        assert!(error.contains("more than once"), "{error}");
    }

    #[test]
    fn a_provider_may_not_reference_an_unknown_chain() {
        let mut config = base();
        config.providers.insert(
            "coin-gecko".to_owned(),
            ProviderConfig {
                id: "coin-gecko".to_owned(),
                name: "CoinGecko".to_owned(),
                kind: ProviderKind::Price,
                base_url: "https://api.example.org".to_owned(),
                requires_api_key: false,
                chains: vec!["polkadot".to_owned()],
                enabled: true,
            },
        );
        let error = validate(&config).unwrap_err().to_string();
        assert!(error.contains("unknown chain polkadot"), "{error}");
    }

    #[test]
    fn an_enabled_wallet_must_name_a_platform() {
        let mut config = base();
        config.wallets.insert(
            "metamask".to_owned(),
            WalletConfig {
                id: "metamask".to_owned(),
                name: "MetaMask".to_owned(),
                platforms: Vec::new(),
                chains: vec!["ethereum".to_owned()],
                enabled: true,
            },
        );
        let error = validate(&config).unwrap_err().to_string();
        assert!(error.contains("names no platform"), "{error}");

        config
            .wallets
            .get_mut("metamask")
            .expect("wallet")
            .platforms = vec![WalletPlatform::Extension];
        assert!(validate(&config).is_ok());
    }

    #[test]
    fn an_out_of_range_timeout_is_rejected() {
        let mut config = base();
        config.rpc.request_timeout_seconds = 0;
        assert!(validate(&config).is_err());
        config.rpc.request_timeout_seconds = MAX_REQUEST_TIMEOUT_SECONDS + 1;
        assert!(validate(&config).is_err());
    }
}
