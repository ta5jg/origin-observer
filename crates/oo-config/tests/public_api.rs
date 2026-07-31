// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-config/tests/public_api.rs
// Purpose : Verify the shipped configuration loads through the public API.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Public API integration tests.
//!
//! These tests load the configuration this repository actually ships. A change
//! to `config/` that breaks an invariant fails here rather than during a run,
//! where it would have produced evidence attributed to the wrong source.

use std::path::PathBuf;

use oo_config::{
    environment::EnvironmentOverrides, loader::load_with_overrides, ChainKind, LoadedConfig,
    WdrpConfidence,
};

fn config_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("config")
}

fn load() -> LoadedConfig {
    load_with_overrides(config_directory(), &EnvironmentOverrides::default())
        .expect("the shipped configuration must load and validate")
}

#[test]
fn the_shipped_configuration_loads_and_validates() {
    let loaded = load();
    assert_eq!(loaded.config.application.name, "Origin Observer");
    assert!(!loaded.config.chains.is_empty());
    assert!(!loaded.config.providers.is_empty());
    assert!(!loaded.config.wallets.is_empty());
}

#[test]
fn the_shipped_configuration_keeps_the_wdrp_thresholds() {
    let loaded = load();
    let research = &loaded.config.research;
    assert_eq!(
        research.minimum_accepted_confidence,
        WdrpConfidence::L5,
        "only independently verified findings may become accepted knowledge"
    );
    assert!(research.require_evidence_for_findings);
    assert!(research.require_reproduction_for_conclusions);
}

#[test]
fn unpinned_latest_block_reads_are_refused_by_default() {
    // An unpinned read cannot be reproduced, so the shipped configuration must
    // not permit one without a deliberate override.
    assert!(!load().config.rpc.allow_unpinned_latest_block);
}

#[test]
fn every_research_question_network_is_declared() {
    let loaded = load();
    for chain in ["ethereum", "bnb", "tron", "bitcoin"] {
        assert!(
            loaded.config.chain(chain).is_some(),
            "the permanent research questions reference {chain}"
        );
    }
}

#[test]
fn a_chain_without_an_endpoint_is_declared_disabled_rather_than_omitted() {
    let loaded = load();
    let bitcoin = loaded.config.chain("bitcoin").expect("bitcoin");
    assert!(bitcoin.rpc_endpoints.is_empty());
    assert!(
        !bitcoin.enabled,
        "a network that cannot be observed must say so rather than disappear"
    );
    assert_eq!(bitcoin.kind, ChainKind::Mainnet);
}

#[test]
fn every_enabled_chain_can_actually_be_observed() {
    let loaded = load();
    for chain in loaded.config.enabled_chains() {
        assert!(
            !chain.rpc_endpoints.is_empty(),
            "{} is enabled but has no endpoint",
            chain.id
        );
    }
}

#[test]
fn every_provider_and_wallet_references_a_declared_chain() {
    let loaded = load();
    for provider in loaded.config.providers.values() {
        for chain in &provider.chains {
            assert!(
                loaded.config.chain(chain).is_some(),
                "provider {} references undeclared chain {chain}",
                provider.id
            );
        }
    }
    for wallet in loaded.config.wallets.values() {
        for chain in &wallet.chains {
            assert!(
                loaded.config.chain(chain).is_some(),
                "wallet {} references undeclared chain {chain}",
                wallet.id
            );
        }
    }
}

#[test]
fn provider_lookup_includes_chain_agnostic_providers() {
    let loaded = load();
    let ethereum = loaded.config.providers_for_chain("ethereum");
    assert!(ethereum.iter().any(|provider| provider.id == "etherscan"));
    assert!(
        ethereum.iter().any(|provider| provider.id == "coingecko"),
        "a provider that declares no chain covers every chain"
    );
    assert!(
        !ethereum.iter().any(|provider| provider.id == "bscscan"),
        "a provider scoped to another chain must not appear"
    );
}

#[test]
fn loading_the_shipped_configuration_is_reproducible() {
    assert_eq!(
        load().provenance.combined_digest(),
        load().provenance.combined_digest()
    );
}
