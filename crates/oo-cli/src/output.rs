// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cli/src/output.rs
// Purpose : Implement the output module for oo-cli.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Implements the output module for oo-cli.

use oo_config::LoadedConfig;
use oo_dataset::DatasetManifest;
use oo_observer::{InvestigationRecord, InvestigationRow, WalletDisplayView};
use oo_proxy::eip1967::interpret;
use oo_proxy::ProxyResolution;
use oo_report::{
    export_json, export_reproduction_json, render_human, MachineReport, ReportBuilder,
    ReproductionObservation, ReproductionReport,
};
use oo_snapshot::digest_bytes;
use oo_storage::{parse_storage_value, StorageLayout};
use oo_wallet::{built_in_adapters, find_adapter};
use serde_json::json;

use crate::error::CliResult;

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
        "cache": cache_json(record),
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
        "cache": cache_json(record),
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

/// Builds a proxy resolution from the records a `proxy-classification`
/// strategy run produced: one `eth_getCode` observation and one
/// `proxy.<slot-name>` observation per known EIP-1967/1822/legacy-OZ slot.
///
/// A missing or unparsable storage-slot read is not an error — it is treated
/// the same as an absent slot value, consistent with
/// `oo_proxy::eip1967::interpret`. A missing or unparsable `eth_getCode`
/// result is an error: classification cannot proceed without bytecode.
///
/// # Errors
///
/// Returns an error if no `eth_getCode` observation is present, or its
/// result does not parse as bytecode.
pub fn build_proxy_resolution(
    records: &[(String, InvestigationRecord)],
) -> CliResult<ProxyResolution> {
    let code_hex = records
        .iter()
        .find(|(_, record)| record.plan().subject() == "eth_getCode")
        .and_then(|(_, record)| rpc_result(record.snapshot().payload()))
        .ok_or_else(|| {
            anyhow::anyhow!("proxy classification requires an eth_getCode observation")
        })?;
    let code = oo_bytecode::parse_hex(code_hex)
        .map_err(|error| anyhow::anyhow!("invalid contract bytecode: {error}"))?;

    let mut slot_values = Vec::new();
    for (name, _) in StorageLayout::known_proxy_slots().entries() {
        let subject = format!("proxy.{name}");
        let value = records
            .iter()
            .find(|(_, record)| record.plan().subject() == subject)
            .and_then(|(_, record)| rpc_result(record.snapshot().payload()))
            .and_then(|hex| parse_storage_value(hex).ok());
        if let Some(value) = value {
            slot_values.push((*name, value));
        }
    }

    let slots = interpret(&slot_values);
    Ok(oo_observer::classify_proxy_offline(&code, &slots))
}

/// Renders a proxy resolution as stable JSON.
#[must_use]
pub fn render_proxy_resolution(resolution: &ProxyResolution) -> String {
    serde_json::to_string_pretty(&proxy_resolution_json(resolution))
        .unwrap_or_else(|_| "{}".to_owned())
}

/// Exports a proxy resolution as a JSON value.
#[must_use]
pub fn proxy_resolution_json(resolution: &ProxyResolution) -> serde_json::Value {
    json!({
        "proxy": {
            "kind": format!("{:?}", resolution.kind),
            "implementation": resolution.implementation.map(address_hex),
            "admin": resolution.admin.map(address_hex),
            "evidence": resolution.evidence.iter().map(|entry| json!({
                "check": entry.check,
                "observation": entry.observation,
            })).collect::<Vec<_>>(),
        },
    })
}

fn address_hex(address: [u8; 20]) -> String {
    let mut hex = String::with_capacity(42);
    hex.push_str("0x");
    for byte in address {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex
}

/// Builds one wallet-specific view per adapter for an investigation's
/// discovery decision.
///
/// # Errors
///
/// Returns an error if `wallet_filter` names a wallet configuration id that
/// is not built in.
pub fn build_wallet_views(
    record: &InvestigationRecord,
    wallet_filter: Option<&str>,
) -> CliResult<Vec<WalletDisplayView>> {
    let cache_state = record
        .cache_observation()
        .map_or(oo_model::cache::CacheState::Unknown, |observation| {
            observation.observation().state()
        });
    let decision = record.outcome().decision();

    let adapters = match wallet_filter {
        Some(config_id) => vec![find_adapter(config_id)
            .ok_or_else(|| anyhow::anyhow!("unknown wallet '{config_id}'"))?],
        None => built_in_adapters(),
    };

    Ok(adapters
        .iter()
        .map(|adapter| oo_observer::evaluate_wallet_view(adapter.as_ref(), decision, cache_state))
        .collect())
}

/// Renders wallet views as stable JSON.
#[must_use]
pub fn render_wallet_views(views: &[WalletDisplayView]) -> String {
    serde_json::to_string_pretty(&wallet_views_json(views)).unwrap_or_else(|_| "{}".to_owned())
}

/// Exports wallet views as a JSON value.
#[must_use]
pub fn wallet_views_json(views: &[WalletDisplayView]) -> serde_json::Value {
    json!({
        "wallet_views": views.iter().map(|view| json!({
            "wallet_config_id": view.wallet_config_id,
            "wallet_display_name": view.wallet_display_name,
            "would_display": view.would_display,
            "citable_for_this_wallet": view.citable_for_this_wallet,
            "rationale": view.rationale,
        })).collect::<Vec<_>>(),
    })
}

/// Exports a dataset manifest and its rows as a JSON value.
#[must_use]
pub fn dataset_export_json(
    manifest: &DatasetManifest,
    rows: &[InvestigationRow],
) -> serde_json::Value {
    json!({
        "dataset": {
            "name": manifest.name(),
            "version": {
                "major": manifest.version().major(),
                "minor": manifest.version().minor(),
            },
            "record_count": manifest.record_count(),
            "digest": manifest.digest().to_hex(),
            "schema": manifest.schema().fields().iter().map(|field| json!({
                "name": field.name(),
                "type": format!("{:?}", field.field_type()),
            })).collect::<Vec<_>>(),
        },
        "rows": rows,
    })
}

/// Computes a stable hex digest for JSON-RPC params text.
#[must_use]
pub fn params_digest(params_json: &str) -> String {
    digest_bytes(params_json.as_bytes()).to_hex()
}

/// Renders an investigation's declared cache state, when one was recorded.
///
/// `--cache-state` is always a caller declaration, never a measurement (see
/// [`crate::commands::CacheStateArg`]); surfacing it in every investigation's
/// output keeps that declaration visible instead of leaving it implicit.
fn cache_json(record: &InvestigationRecord) -> serde_json::Value {
    match record.cache_observation() {
        Some(observation) => json!({
            "state": format!("{:?}", observation.observation().state()),
            "attributable_to_live_discovery": record.is_attributable_to_live_discovery(),
        }),
        None => json!({
            "state": null,
            "attributable_to_live_discovery": record.is_attributable_to_live_discovery(),
        }),
    }
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

#[cfg(test)]
mod wallet_and_dataset_tests {
    use oo_core::{NetworkId, ProviderId};
    use oo_dataset::DatasetVersion;
    use oo_observer::{ObservationPlan, ObserverService};

    use super::*;

    fn accepted_record() -> InvestigationRecord {
        let plan = ObservationPlan::new(NetworkId::new(), ProviderId::new(), "eth_getCode");
        let mut record = ObserverService::default()
            .observe(plan, json!({ "result": "0x6001" }))
            .expect("valid investigation");
        record.set_reproduction(oo_evidence::ReproductionStatus::IndependentlyVerified);
        record
    }

    #[test]
    fn build_wallet_views_covers_every_built_in_adapter_by_default() {
        let record = accepted_record();
        let views = build_wallet_views(&record, None).unwrap();
        assert_eq!(views.len(), oo_wallet::built_in_adapters().len());
    }

    #[test]
    fn build_wallet_views_can_be_filtered_to_one_wallet() {
        let record = accepted_record();
        let views = build_wallet_views(&record, Some("metamask")).unwrap();
        assert_eq!(views.len(), 1);
        assert_eq!(views[0].wallet_config_id, "metamask");
    }

    #[test]
    fn an_unknown_wallet_filter_is_an_explicit_error() {
        let record = accepted_record();
        assert!(build_wallet_views(&record, Some("not-a-real-wallet")).is_err());
    }

    #[test]
    fn dataset_export_json_names_the_declared_schema_and_row_count() {
        let records = vec![accepted_record()];
        let (rows, manifest) =
            oo_observer::export_dataset("cli-run", DatasetVersion::new(1, 0), &records).unwrap();
        let json = dataset_export_json(&manifest, &rows);
        assert_eq!(json["dataset"]["record_count"], 1);
        assert_eq!(json["rows"].as_array().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod proxy_tests {
    use oo_core::{NetworkId, ProviderId};
    use oo_observer::{ObservationPlan, ObserverService};
    use oo_proxy::ProxyKind;

    use super::*;

    fn observation(subject: &str, result_hex: &str) -> (String, InvestigationRecord) {
        let plan = ObservationPlan::new(NetworkId::new(), ProviderId::new(), subject);
        let record = ObserverService::default()
            .observe(plan, json!({ "result": result_hex }))
            .expect("a well-formed observation always validates");
        ("test-provider".to_owned(), record)
    }

    fn address_slot_hex(address: [u8; 20]) -> String {
        let mut hex = "0x".to_owned();
        for _ in 0..12 {
            hex.push_str("00");
        }
        for byte in address {
            hex.push_str(&format!("{byte:02x}"));
        }
        hex
    }

    #[test]
    fn a_minimal_proxy_classifies_from_bytecode_alone() {
        let implementation = [0x11u8; 20];
        let code_hex = oo_bytecode::to_hex(&oo_proxy::eip1167::build(implementation));
        let records = vec![observation("eth_getCode", &code_hex)];

        let resolution = build_proxy_resolution(&records).unwrap();
        assert_eq!(resolution.kind, ProxyKind::Eip1167Minimal);
        assert_eq!(resolution.implementation, Some(implementation));
    }

    #[test]
    fn transparent_storage_classifies_from_the_named_proxy_slot_observations() {
        let mut records = vec![observation("eth_getCode", "0x6000")];
        records.push(observation(
            "proxy.eip1967.implementation",
            &address_slot_hex([0x22; 20]),
        ));
        records.push(observation(
            "proxy.eip1967.admin",
            &address_slot_hex([0x33; 20]),
        ));

        let resolution = build_proxy_resolution(&records).unwrap();
        assert_eq!(resolution.kind, ProxyKind::Eip1967Transparent);
        assert_eq!(resolution.implementation, Some([0x22; 20]));
        assert_eq!(resolution.admin, Some([0x33; 20]));
    }

    #[test]
    fn a_missing_slot_observation_is_treated_as_absent_not_an_error() {
        let mut records = vec![observation("eth_getCode", "0x6000")];
        records.push(observation(
            "proxy.eip1967.implementation",
            &address_slot_hex([0x22; 20]),
        ));
        // No proxy.eip1967.admin observation at all.

        let resolution = build_proxy_resolution(&records).unwrap();
        assert_eq!(resolution.kind, ProxyKind::Eip1967Uups);
    }

    #[test]
    fn a_missing_eth_get_code_observation_is_an_explicit_error() {
        let records = vec![observation(
            "proxy.eip1967.implementation",
            &address_slot_hex([0x22; 20]),
        )];
        assert!(build_proxy_resolution(&records).is_err());
    }

    #[test]
    fn nothing_matching_renders_as_json_naming_the_unknown_kind() {
        let records = vec![observation("eth_getCode", "0x6000")];
        let resolution = build_proxy_resolution(&records).unwrap();
        let rendered = render_proxy_resolution(&resolution);
        assert!(rendered.contains("\"kind\": \"Unknown\""));
    }
}
