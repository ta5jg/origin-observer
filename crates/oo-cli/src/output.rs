// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cli/src/output.rs
// Purpose : Implement the output module for oo-cli.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Implements the output module for oo-cli.

use oo_config::LoadedConfig;
use oo_observer::InvestigationRecord;
use oo_report::{
    export_json, export_reproduction_json, render_human, MachineReport, ReportBuilder,
    ReproductionObservation, ReproductionReport,
};
use oo_snapshot::digest_bytes;
use serde_json::json;

/// Embedded project roadmap.
pub const ROADMAP: &str = include_str!("../../../ROADMAP.md");

/// Embedded WDRP research constitution.
pub const WDRP: &str = include_str!("../../../WDRP.md");

/// Stable status text for a freshly initialized workspace.
pub const STATUS: &str = "Origin Observer workspace initialized.";

/// Writes output to standard output.
pub fn write_stdout(text: &str) {
    println!("{text}");
}

/// Renders an investigation as stable JSON.
#[must_use]
pub fn render_investigation(record: &InvestigationRecord) -> String {
    let payload = json!({
        "subject": record.plan().subject(),
        "semantic": semantic_json(record),
        "snapshot": {
            "id": record.snapshot().id().to_string(),
            "digest": record.snapshot().digest().to_hex(),
        },
        "evidence": {
            "id": record.evidence().id().to_string(),
            "digest": record.evidence().digest().to_hex(),
            "reproduction": format!("{:?}", record.evidence().reproduction()),
        },
        "discovery": {
            "decision": format!("{:?}", record.outcome().decision()),
            "score": record.outcome().score().value(),
            "events": record.outcome().timeline().len(),
        },
    });

    serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_owned())
}

/// Exports an investigation as JSON value.
#[must_use]
pub fn investigation_json(record: &InvestigationRecord) -> serde_json::Value {
    json!({
        "subject": record.plan().subject(),
        "semantic": semantic_json(record),
        "snapshot": {
            "id": record.snapshot().id().to_string(),
            "digest": record.snapshot().digest().to_hex(),
            "payload": record.snapshot().payload(),
        },
        "evidence": {
            "id": record.evidence().id().to_string(),
            "digest": record.evidence().digest().to_hex(),
            "reproduction": format!("{:?}", record.evidence().reproduction()),
        },
        "discovery": {
            "decision": format!("{:?}", record.outcome().decision()),
            "score": record.outcome().score().value(),
            "events": record.outcome().timeline().len(),
        },
    })
}

/// Builds a report from an investigation.
#[must_use]
pub fn build_report(record: &InvestigationRecord) -> MachineReport {
    ReportBuilder.build(record.evidence(), record.outcome())
}

/// Renders a machine report as stable JSON.
#[must_use]
pub fn render_report_json(report: &MachineReport) -> String {
    serde_json::to_string_pretty(&export_json(report)).unwrap_or_else(|_| "{}".to_owned())
}

/// Renders a machine report as human-readable text.
#[must_use]
pub fn render_report_human(report: &MachineReport) -> String {
    render_human(report)
}

/// Renders a multi-provider reproduction report.
#[must_use]
pub fn render_reproduction_report(records: &[(String, InvestigationRecord)]) -> String {
    serde_json::to_string_pretty(&reproduction_json(records)).unwrap_or_else(|_| "{}".to_owned())
}

/// Renders a multi-subject strategy report.
#[must_use]
pub fn render_strategy_report(records: &[(String, InvestigationRecord)]) -> String {
    serde_json::to_string_pretty(&strategy_json(records)).unwrap_or_else(|_| "{}".to_owned())
}

/// Exports a multi-subject strategy report as JSON value.
#[must_use]
pub fn strategy_json(records: &[(String, InvestigationRecord)]) -> serde_json::Value {
    let observations = records
        .iter()
        .map(|(provider, record)| {
            json!({
                "provider": provider,
                "subject": record.plan().subject(),
                "semantic": semantic_json(record),
                "snapshot_digest": record.snapshot().digest().to_hex(),
                "evidence_digest": record.evidence().digest().to_hex(),
                "reproduction": format!("{:?}", record.evidence().reproduction()),
                "decision": format!("{:?}", record.outcome().decision()),
                "score": record.outcome().score().value(),
            })
        })
        .collect::<Vec<_>>();

    json!({
        "strategy": {
            "subject_count": records
                .iter()
                .map(|(_, record)| record.plan().subject())
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            "observation_count": records.len(),
            "decision": strategy_decision(records),
            "findings": strategy_findings(records),
        },
        "observations": observations,
    })
}

/// Exports a multi-provider reproduction report as JSON value.
#[must_use]
pub fn reproduction_json(records: &[(String, InvestigationRecord)]) -> serde_json::Value {
    export_reproduction_json(&build_reproduction_report(records))
}

/// Builds a reproduction report from observation records.
#[must_use]
pub fn build_reproduction_report(records: &[(String, InvestigationRecord)]) -> ReproductionReport {
    let observations = records
        .iter()
        .map(|(provider, record)| {
            ReproductionObservation::new(
                provider,
                record.plan().subject(),
                record.snapshot().digest().to_hex(),
                record.evidence().digest().to_hex(),
                format!("{:?}", record.outcome().decision()),
                record.outcome().score().value(),
            )
        })
        .collect::<Vec<_>>();

    ReproductionReport::new(observations)
}

/// Computes a stable hex digest for JSON-RPC params text.
#[must_use]
pub fn params_digest(params_json: &str) -> String {
    digest_bytes(params_json.as_bytes()).to_hex()
}

fn semantic_json(record: &InvestigationRecord) -> serde_json::Value {
    let subject = record.plan().subject();
    if let Some(error) = rpc_error(record.snapshot().payload()) {
        return json!({
            "kind": "rpc_error",
            "subject": subject,
            "code": error.get("code").cloned().unwrap_or(serde_json::Value::Null),
            "message": error.get("message").cloned().unwrap_or(serde_json::Value::Null),
        });
    }

    let result = rpc_result(record.snapshot().payload());

    match subject {
        "eth_chainId" => result
            .and_then(hex_to_u64)
            .map(|chain_id| {
                json!({
                    "kind": "chain",
                    "chain_id": chain_id,
                    "network": known_chain_name(chain_id),
                })
            })
            .unwrap_or_else(|| json!({ "kind": "unknown", "reason": "missing_or_invalid_chain_id" })),
        "eth_getBalance" => result
            .and_then(hex_to_u128)
            .map(|wei| {
                json!({
                    "kind": "native_balance",
                    "wei": wei.to_string(),
                    "is_zero": wei == 0,
                })
            })
            .unwrap_or_else(|| json!({ "kind": "unknown", "reason": "missing_or_invalid_balance" })),
        "eth_getCode" => result
            .map(|code| {
                json!({
                    "kind": "contract_code",
                    "has_code": code != "0x" && code != "0x0",
                    "byte_length": hex_byte_length(code),
                })
            })
            .unwrap_or_else(|| json!({ "kind": "unknown", "reason": "missing_contract_code" })),
        "erc20.name" | "erc20.symbol" => result
            .and_then(decode_abi_string)
            .map(|value| json!({ "kind": "erc20_metadata", "field": subject_field(subject), "value": value }))
            .unwrap_or_else(|| json!({ "kind": "unknown", "field": subject_field(subject), "reason": "missing_or_invalid_abi_string" })),
        "erc20.decimals" => result
            .and_then(hex_to_u64)
            .map(|value| json!({ "kind": "erc20_metadata", "field": "decimals", "value": value }))
            .unwrap_or_else(|| json!({ "kind": "unknown", "field": "decimals", "reason": "missing_or_invalid_uint" })),
        _ => json!({ "kind": "raw_rpc" }),
    }
}

fn strategy_decision(records: &[(String, InvestigationRecord)]) -> &'static str {
    if records.is_empty() {
        return "NoEvidence";
    }

    if records.iter().any(|(_, record)| {
        semantic_json(record)
            .get("kind")
            .and_then(serde_json::Value::as_str)
            == Some("rpc_error")
    }) {
        return "ProviderErrorNeedsReview";
    }

    if records
        .iter()
        .any(|(_, record)| format!("{:?}", record.evidence().reproduction()) == "Contradicted")
    {
        return "Contradicted";
    }

    if records
        .iter()
        .all(|(_, record)| format!("{:?}", record.evidence().reproduction()) == "Reproduced")
    {
        return "ReproducedNeedsReview";
    }

    "ObservedNeedsReview"
}

fn strategy_findings(records: &[(String, InvestigationRecord)]) -> Vec<serde_json::Value> {
    let mut findings = Vec::new();
    let has_code = records.iter().any(|(_, record)| {
        record.plan().subject() == "eth_getCode"
            && semantic_json(record)
                .get("has_code")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
    });
    let has_zero_balance = records.iter().any(|(_, record)| {
        record.plan().subject() == "eth_getBalance"
            && semantic_json(record)
                .get("is_zero")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
    });

    if records
        .iter()
        .any(|(_, record)| record.plan().subject() == "eth_getCode")
    {
        findings.push(json!({
            "kind": "address_classification",
            "value": if has_code { "contract" } else { "externally_owned_or_empty" },
            "evidence": "eth_getCode",
        }));
    }

    if has_zero_balance {
        findings.push(json!({
            "kind": "native_balance_state",
            "value": "zero",
            "evidence": "eth_getBalance",
        }));
    }

    findings
}

fn rpc_result(payload: &serde_json::Value) -> Option<&str> {
    payload.get("result").and_then(serde_json::Value::as_str)
}

fn rpc_error(payload: &serde_json::Value) -> Option<&serde_json::Value> {
    payload.get("error")
}

fn hex_to_u64(value: &str) -> Option<u64> {
    u64::from_str_radix(value.strip_prefix("0x").unwrap_or(value), 16).ok()
}

fn hex_to_u128(value: &str) -> Option<u128> {
    u128::from_str_radix(value.strip_prefix("0x").unwrap_or(value), 16).ok()
}

fn hex_byte_length(value: &str) -> usize {
    let hex = value.strip_prefix("0x").unwrap_or(value);
    hex.len() / 2
}

fn decode_abi_string(value: &str) -> Option<String> {
    let hex = value.strip_prefix("0x")?;
    if hex.len() < 128 {
        return decode_fixed_bytes_string(hex);
    }

    let len_hex = hex.get(64..128)?;
    let len = usize::from_str_radix(len_hex, 16).ok()?;
    let data_start = 128;
    let data_end = data_start + len.saturating_mul(2);
    let data_hex = hex.get(data_start..data_end)?;
    decode_hex_utf8(data_hex)
}

fn decode_fixed_bytes_string(hex: &str) -> Option<String> {
    decode_hex_utf8(hex.trim_end_matches('0'))
}

fn decode_hex_utf8(hex: &str) -> Option<String> {
    if hex.len() % 2 != 0 {
        return None;
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for index in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(hex.get(index..index + 2)?, 16).ok()?;
        if byte != 0 {
            bytes.push(byte);
        }
    }

    String::from_utf8(bytes)
        .ok()
        .filter(|value| !value.is_empty())
}

fn known_chain_name(chain_id: u64) -> &'static str {
    match chain_id {
        1 => "ethereum-mainnet",
        56 => "bnb-smart-chain",
        137 => "polygon",
        42161 => "arbitrum-one",
        10 => "optimism",
        8453 => "base",
        _ => "unknown",
    }
}

fn subject_field(subject: &str) -> &'static str {
    match subject {
        "erc20.name" => "name",
        "erc20.symbol" => "symbol",
        _ => "unknown",
    }
}

/// Renders the loaded configuration as human-readable text.
///
/// The output states what governs a run: which files were read, their combined
/// digest, which environment variables overrode them and which components are
/// enabled. Credential values never appear; only the provider identifiers that
/// have one.
#[must_use]
pub fn render_config(loaded: &LoadedConfig) -> String {
    let config = &loaded.config;
    let provenance = &loaded.provenance;
    let mut lines = Vec::new();

    lines.push(format!(
        "{} ({:?} environment)",
        config.application.name, config.application.environment
    ));
    lines.push(format!(
        "configuration digest: {}",
        provenance.combined_digest()
    ));
    lines.push(String::new());

    lines.push("Sources".to_owned());
    for source in &provenance.sources {
        lines.push(format!(
            "  {} ({} bytes, {})",
            source.path.display(),
            source.bytes,
            source.digest
        ));
    }
    if provenance.has_environment_overrides() {
        lines.push(format!(
            "  environment overrides: {}",
            provenance.environment_overrides.join(", ")
        ));
    } else {
        lines.push("  environment overrides: none".to_owned());
    }
    lines.push(String::new());

    lines.push("Research thresholds".to_owned());
    lines.push(format!(
        "  minimum accepted confidence: {} ({})",
        config.research.minimum_accepted_confidence,
        config.research.minimum_accepted_confidence.meaning()
    ));
    lines.push(format!(
        "  evidence required for findings: {}",
        config.research.require_evidence_for_findings
    ));
    lines.push(format!(
        "  reproduction required for conclusions: {}",
        config.research.require_reproduction_for_conclusions
    ));
    lines.push(format!(
        "  unpinned latest block allowed: {}",
        config.rpc.allow_unpinned_latest_block
    ));
    lines.push(String::new());

    lines.push(format!(
        "Chains ({} enabled of {})",
        config.enabled_chains().len(),
        config.chains.len()
    ));
    for chain in config.chains.values() {
        let state = if chain.enabled { "enabled" } else { "disabled" };
        lines.push(format!(
            "  {:<18} {:<10} {} endpoint(s)  {}",
            chain.id,
            state,
            chain.rpc_endpoints.len(),
            chain.name
        ));
    }
    lines.push(String::new());

    lines.push(format!(
        "Providers ({} enabled of {})",
        config.enabled_providers().len(),
        config.providers.len()
    ));
    for provider in config.providers.values() {
        let state = if provider.enabled {
            "enabled"
        } else {
            "disabled"
        };
        let credential = if !provider.requires_api_key {
            "no credential needed"
        } else if loaded.credentials.contains(&provider.id) {
            "credential present"
        } else {
            "credential MISSING"
        };
        lines.push(format!(
            "  {:<22} {:<10} {:<9} {}",
            provider.id,
            state,
            format!("{:?}", provider.kind).to_lowercase(),
            credential
        ));
    }
    lines.push(String::new());

    lines.push(format!(
        "Wallets ({} enabled of {})",
        config.enabled_wallets().len(),
        config.wallets.len()
    ));
    for wallet in config.wallets.values() {
        let state = if wallet.enabled {
            "enabled"
        } else {
            "disabled"
        };
        lines.push(format!(
            "  {:<18} {:<10} {} chain(s)  {}",
            wallet.id,
            state,
            wallet.chains.len(),
            wallet.name
        ));
    }

    lines.join("\n")
}

/// Renders the loaded configuration as stable JSON.
#[must_use]
pub fn render_config_json(loaded: &LoadedConfig) -> String {
    let payload = json!({
        "application": {
            "name": loaded.config.application.name,
            "environment": format!("{:?}", loaded.config.application.environment).to_lowercase(),
        },
        "provenance": {
            "directory": loaded.provenance.directory,
            "digest": loaded.provenance.combined_digest().qualified(),
            "sources": loaded.provenance.sources,
            "environment_overrides": loaded.provenance.environment_overrides,
        },
        "research": {
            "minimum_accepted_confidence": loaded.config.research.minimum_accepted_confidence.to_string(),
            "require_evidence_for_findings": loaded.config.research.require_evidence_for_findings,
            "require_reproduction_for_conclusions": loaded.config.research.require_reproduction_for_conclusions,
        },
        "counts": {
            "chains": loaded.config.chains.len(),
            "chains_enabled": loaded.config.enabled_chains().len(),
            "providers": loaded.config.providers.len(),
            "providers_enabled": loaded.config.enabled_providers().len(),
            "wallets": loaded.config.wallets.len(),
            "wallets_enabled": loaded.config.enabled_wallets().len(),
        },
        "credentials_present_for": loaded.credentials.provider_ids(),
    });
    serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_owned())
}

/// Renders the workspace status, including configuration when it is available.
#[must_use]
pub fn render_status(config: Option<&LoadedConfig>) -> String {
    match config {
        Some(loaded) => format!(
            "{STATUS}\nconfiguration: {} ({} chains, {} providers, {} wallets enabled)\ndigest: {}",
            loaded.provenance.directory.display(),
            loaded.config.enabled_chains().len(),
            loaded.config.enabled_providers().len(),
            loaded.config.enabled_wallets().len(),
            loaded.provenance.combined_digest()
        ),
        None => format!("{STATUS}\nconfiguration: not loaded"),
    }
}
