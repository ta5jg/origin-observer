// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cli/src/main.rs
// Purpose : Start the Origin Observer command-line application.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Origin Observer command-line entry point.

use commands::{Cli, Command, ObserveFormat, ObserveStrategy};
use error::CliResult;
use oo_core::{NetworkId, ProviderId};
use oo_evidence::ReproductionStatus;
use oo_observer::{ObservationPlan, ObserverService};
use oo_provider::{ProviderIdentity, ProviderRegistry};
use oo_report::ReportReproductionStatus;
use oo_rpc::{
    HttpTransport, PinPolicy, RateLimit, RateLimiter, RetryPolicy, RpcClient, RpcEndpoint,
    RpcRequest, RpcTransport,
};
use serde_json::json;
use std::path::Path;

mod commands;
mod error;
mod output;

/// How an observation run treats block pinning, retries and endpoint load.
///
/// The policy is read from the project configuration so a run behaves the way
/// the committed configuration says it does, and a run without configuration
/// still refuses unreproducible reads rather than defaulting to permissive.
#[derive(Debug, Clone)]
struct ObservationPolicy {
    pin: PinPolicy,
    retry: RetryPolicy,
    limiter: RateLimiter,
}

impl ObservationPolicy {
    /// Base delay between retries. Doubling from here stays inside the bound
    /// the retry policy enforces.
    const BACKOFF_MS: u64 = 250;

    fn from_config_directory(directory: &str) -> Self {
        oo_config::load_from_directory(directory).map_or_else(
            |_| Self::strict(),
            |loaded| Self {
                pin: PinPolicy::from_allow_unpinned(loaded.config.rpc.allow_unpinned_latest_block),
                retry: RetryPolicy::new(loaded.config.rpc.max_retries + 1, Self::BACKOFF_MS),
                limiter: RateLimiter::new(RateLimit::default()),
            },
        )
    }

    fn strict() -> Self {
        Self {
            pin: PinPolicy::Required,
            retry: RetryPolicy::default(),
            limiter: RateLimiter::new(RateLimit::default()),
        }
    }

    fn apply<T: RpcTransport>(&self, client: RpcClient<T>) -> RpcClient<T> {
        client
            .with_pin_policy(self.pin)
            .with_retry(self.retry)
            .with_rate_limiter(self.limiter.clone())
    }
}

#[derive(Debug, Clone)]
struct ObservationSpec {
    subject: String,
    method: String,
    params_json: String,
}

fn main() -> CliResult<()> {
    let cli = Cli::parse_args();

    match cli.command {
        Some(Command::Observe {
            subject,
            strategy,
            address,
            rpc_url,
            provider,
            method,
            params_json,
            result,
            payload_json,
            payload_file,
            format,
            out,
        }) => {
            let specs = observation_specs(strategy, address, subject, method, params_json)?;
            let provider_id = ProviderId::new();
            let provider_registry = build_provider_registry(provider, rpc_url)?;
            // Observation policy comes from the project configuration when one
            // is present. Without it the strict defaults apply: pinned reads
            // only, no retry, and a polite request rate.
            let observation_policy = ObservationPolicy::from_config_directory("config");
            if !provider_registry.is_empty() {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                let mut records = Vec::new();

                for spec in &specs {
                    let plan =
                        ObservationPlan::new(NetworkId::new(), provider_id, spec.subject.clone());
                    for provider in provider_registry.providers() {
                        let endpoint =
                            RpcEndpoint::parse(provider.endpoint()).map_err(|error| {
                                anyhow::anyhow!(
                                    "invalid provider endpoint {}: {error}",
                                    provider.endpoint()
                                )
                            })?;
                        let params = serde_json::from_str(&spec.params_json).map_err(|error| {
                            anyhow::anyhow!("invalid params for {}: {error}", spec.subject)
                        })?;
                        let request = RpcRequest::new(1, spec.method.clone(), params)
                            .map_err(|error| anyhow::anyhow!("invalid RPC request: {error}"))?;
                        let client = observation_policy.apply(RpcClient::new(
                            provider_id,
                            endpoint,
                            HttpTransport::new(),
                        ));
                        let trace = runtime.block_on(client.observe(request)).map_err(|error| {
                            anyhow::anyhow!(
                                "RPC observation failed for {}: {error}",
                                provider.name()
                            )
                        })?;
                        let payload = trace.response().to_json_value();
                        let record = ObserverService::default()
                            .observe(plan.clone(), payload)
                            .ok_or_else(|| anyhow::anyhow!("observation failed validation"))?;

                        records.push((provider.name().to_owned(), record));
                    }
                }

                apply_reproduction_status_by_subject(&mut records);

                if strategy == Some(ObserveStrategy::ProxyClassification) {
                    let resolution = output::build_proxy_resolution(&records)?;
                    if let Some(out) = out.as_deref() {
                        write_proxy_artifacts(out, &records, &resolution)?;
                    }
                    output::write_stdout(&output::render_proxy_resolution(&resolution));
                    return Ok(());
                }

                if records.len() == 1 {
                    let (_, record) = records
                        .into_iter()
                        .next()
                        .expect("one record exists after length check");
                    if let Some(out) = out.as_deref() {
                        write_single_artifacts(
                            out,
                            &record,
                            record.plan().subject(),
                            &specs[0].params_json,
                        )?;
                    }
                    write_observation(format, &record);
                } else if specs.len() == 1 {
                    if let Some(out) = out.as_deref() {
                        write_reproduction_artifacts(
                            out,
                            &records,
                            &specs[0].method,
                            &specs[0].params_json,
                        )?;
                    }
                    output::write_stdout(&output::render_reproduction_report(&records));
                } else {
                    if let Some(out) = out.as_deref() {
                        write_strategy_artifacts(out, &records, &specs)?;
                    }
                    output::write_stdout(&output::render_strategy_report(&records));
                }

                return Ok(());
            }

            let payload = match (payload_file, payload_json) {
                (Some(payload_file), _) => {
                    let payload_text = std::fs::read_to_string(&payload_file).map_err(|error| {
                        anyhow::anyhow!(
                            "failed to read --payload-file {}: {error}",
                            payload_file.display()
                        )
                    })?;
                    serde_json::from_str(&payload_text).map_err(|error| {
                        anyhow::anyhow!(
                            "invalid JSON in --payload-file {}: {error}",
                            payload_file.display()
                        )
                    })?
                }
                (None, Some(payload_json)) => serde_json::from_str(&payload_json)
                    .map_err(|error| anyhow::anyhow!("invalid --payload-json: {error}"))?,
                (None, None) => json!({ "result": result }),
            };
            if specs.len() != 1 {
                return Err(anyhow::anyhow!(
                    "multi-call strategies require --provider or --rpc-url"
                ));
            }
            let plan =
                ObservationPlan::new(NetworkId::new(), provider_id, specs[0].subject.clone());
            let record = ObserverService::default()
                .observe(plan, payload)
                .ok_or_else(|| anyhow::anyhow!("observation failed validation"))?;

            if let Some(out) = out.as_deref() {
                write_single_artifacts(
                    out,
                    &record,
                    record.plan().subject(),
                    &specs[0].params_json,
                )?;
            }
            write_observation(format, &record);
        }
        Some(Command::Roadmap) => output::write_stdout(output::ROADMAP),
        Some(Command::Wdrp) => output::write_stdout(output::WDRP),
        Some(Command::Config { dir, json }) => {
            let loaded = oo_config::load_from_directory(&dir)?;
            let rendered = if json {
                output::render_config_json(&loaded)
            } else {
                output::render_config(&loaded)
            };
            output::write_stdout(&rendered);
        }
        Some(Command::Status) | None => {
            // Status reports the configuration when one is present and says so
            // plainly when it is not. A missing configuration is a fact about
            // the workspace, not a failure of the command.
            let loaded = oo_config::load_from_directory("config").ok();
            output::write_stdout(&output::render_status(loaded.as_ref()));
        }
    }

    Ok(())
}

fn observation_specs(
    strategy: Option<ObserveStrategy>,
    address: Option<String>,
    subject: String,
    method: String,
    params_json: String,
) -> CliResult<Vec<ObservationSpec>> {
    match strategy {
        Some(ObserveStrategy::ChainId) => Ok(vec![ObservationSpec {
            subject: "eth_chainId".to_owned(),
            method: "eth_chainId".to_owned(),
            params_json: "[]".to_owned(),
        }]),
        Some(ObserveStrategy::Balance) => {
            let address =
                address.ok_or_else(|| anyhow::anyhow!("--strategy balance requires --address"))?;
            Ok(vec![balance_spec(&address)?])
        }
        Some(ObserveStrategy::ContractCode) => {
            let address = address
                .ok_or_else(|| anyhow::anyhow!("--strategy contract-code requires --address"))?;
            Ok(vec![contract_code_spec(&address)?])
        }
        Some(ObserveStrategy::Erc20Metadata) => {
            let address = address
                .ok_or_else(|| anyhow::anyhow!("--strategy erc20-metadata requires --address"))?;
            Ok(erc20_metadata_specs(&address)?)
        }
        Some(ObserveStrategy::WalletOverview) => {
            let address = address
                .ok_or_else(|| anyhow::anyhow!("--strategy wallet-overview requires --address"))?;
            Ok(vec![balance_spec(&address)?, contract_code_spec(&address)?])
        }
        Some(ObserveStrategy::ProxyClassification) => {
            let address = address.ok_or_else(|| {
                anyhow::anyhow!("--strategy proxy-classification requires --address")
            })?;
            proxy_classification_specs(&address)
        }
        None => Ok(vec![ObservationSpec {
            subject,
            method,
            params_json,
        }]),
    }
}

fn balance_spec(address: &str) -> CliResult<ObservationSpec> {
    Ok(ObservationSpec {
        subject: "eth_getBalance".to_owned(),
        method: "eth_getBalance".to_owned(),
        params_json: serde_json::to_string(&json!([address, "latest"]))?,
    })
}

fn contract_code_spec(address: &str) -> CliResult<ObservationSpec> {
    Ok(ObservationSpec {
        subject: "eth_getCode".to_owned(),
        method: "eth_getCode".to_owned(),
        params_json: serde_json::to_string(&json!([address, "latest"]))?,
    })
}

fn erc20_metadata_specs(address: &str) -> CliResult<Vec<ObservationSpec>> {
    Ok(vec![
        eth_call_spec("erc20.name", address, "0x06fdde03")?,
        eth_call_spec("erc20.symbol", address, "0x95d89b41")?,
        eth_call_spec("erc20.decimals", address, "0x313ce567")?,
    ])
}

fn eth_call_spec(subject: &str, to: &str, data: &str) -> CliResult<ObservationSpec> {
    Ok(ObservationSpec {
        subject: subject.to_owned(),
        method: "eth_call".to_owned(),
        params_json: serde_json::to_string(&json!([{ "to": to, "data": data }, "latest"]))?,
    })
}

/// One `eth_getCode` call plus one `eth_getStorageAt` call per known
/// EIP-1967/1822/legacy-OZ proxy slot, named `proxy.<slot-name>` so
/// [`output::build_proxy_resolution`] can match each response back to the
/// slot it answers.
fn proxy_classification_specs(address: &str) -> CliResult<Vec<ObservationSpec>> {
    let mut specs = vec![contract_code_spec(address)?];
    for (name, slot) in oo_storage::StorageLayout::known_proxy_slots().entries() {
        specs.push(ObservationSpec {
            subject: format!("proxy.{name}"),
            method: "eth_getStorageAt".to_owned(),
            params_json: serde_json::to_string(&json!([address, slot.to_hex(), "latest"]))?,
        });
    }
    Ok(specs)
}

fn write_single_artifacts(
    out: &Path,
    record: &oo_observer::InvestigationRecord,
    method: &str,
    params_json: &str,
) -> CliResult<()> {
    std::fs::create_dir_all(out)?;
    let report = output::build_report(record);
    write_json_file(
        out.join("investigation.json"),
        &output::investigation_json(record),
    )?;
    write_json_file(out.join("report.json"), &oo_report::export_json(&report))?;
    write_json_file(
        out.join("manifest.json"),
        &json!({
            "manifest_version": 1,
            "schema": "origin-observer.observation.v1",
            "artifact_kind": "single_observation",
            "subject": record.plan().subject(),
            "method": method,
            "params_digest": output::params_digest(params_json),
            "snapshot_digest": record.snapshot().digest().to_hex(),
            "evidence_digest": record.evidence().digest().to_hex(),
            "reproduction": format!("{:?}", record.evidence().reproduction()),
            "decision": format!("{:?}", record.outcome().decision()),
            "files": {
                "investigation": "investigation.json",
                "report": "report.json",
            },
        }),
    )
}

fn write_reproduction_artifacts(
    out: &Path,
    records: &[(String, oo_observer::InvestigationRecord)],
    method: &str,
    params_json: &str,
) -> CliResult<()> {
    std::fs::create_dir_all(out)?;

    for (index, (_, record)) in records.iter().enumerate() {
        let filename = format!("observation-{}.json", index + 1);
        write_json_file(out.join(filename), &output::investigation_json(record))?;
    }

    let observation_files = (1..=records.len())
        .map(|index| format!("observation-{index}.json"))
        .collect::<Vec<_>>();
    let reproduction = output::reproduction_json(records);

    write_json_file(out.join("reproduction.json"), &reproduction)?;
    write_json_file(
        out.join("manifest.json"),
        &json!({
            "manifest_version": 1,
            "schema": "origin-observer.reproduction.v1",
            "artifact_kind": "reproduction_observation",
            "subject": records
                .first()
                .map(|(_, record)| record.plan().subject())
                .unwrap_or_default(),
            "method": method,
            "params_digest": output::params_digest(params_json),
            "provider_count": records.len(),
            "providers": records
                .iter()
                .map(|(provider, _)| provider)
                .collect::<Vec<_>>(),
            "reproduction": reproduction["reproduction"],
            "files": {
                "observations": observation_files,
                "reproduction": "reproduction.json",
            },
        }),
    )
}

fn write_strategy_artifacts(
    out: &Path,
    records: &[(String, oo_observer::InvestigationRecord)],
    specs: &[ObservationSpec],
) -> CliResult<()> {
    std::fs::create_dir_all(out)?;

    for (index, (_, record)) in records.iter().enumerate() {
        let filename = format!("observation-{}.json", index + 1);
        write_json_file(out.join(filename), &output::investigation_json(record))?;
    }

    let observation_files = (1..=records.len())
        .map(|index| format!("observation-{index}.json"))
        .collect::<Vec<_>>();
    let strategy = output::strategy_json(records);

    write_json_file(out.join("strategy.json"), &strategy)?;
    write_json_file(
        out.join("manifest.json"),
        &json!({
            "manifest_version": 1,
            "schema": "origin-observer.strategy.v1",
            "artifact_kind": "strategy_observation",
            "decision": strategy["strategy"]["decision"],
            "findings": strategy["strategy"]["findings"],
            "strategy": {
                "subjects": specs.iter().map(|spec| &spec.subject).collect::<Vec<_>>(),
                "methods": specs.iter().map(|spec| &spec.method).collect::<Vec<_>>(),
                "params_digests": specs
                    .iter()
                    .map(|spec| output::params_digest(&spec.params_json))
                    .collect::<Vec<_>>(),
            },
            "provider_count": records
                .iter()
                .map(|(provider, _)| provider)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            "providers": records
                .iter()
                .map(|(provider, _)| provider)
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            "files": {
                "observations": observation_files,
                "strategy": "strategy.json",
            },
        }),
    )
}

fn write_proxy_artifacts(
    out: &Path,
    records: &[(String, oo_observer::InvestigationRecord)],
    resolution: &oo_proxy::ProxyResolution,
) -> CliResult<()> {
    std::fs::create_dir_all(out)?;

    for (index, (_, record)) in records.iter().enumerate() {
        let filename = format!("observation-{}.json", index + 1);
        write_json_file(out.join(filename), &output::investigation_json(record))?;
    }

    let observation_files = (1..=records.len())
        .map(|index| format!("observation-{index}.json"))
        .collect::<Vec<_>>();
    let proxy = output::proxy_resolution_json(resolution);

    write_json_file(out.join("proxy.json"), &proxy)?;
    write_json_file(
        out.join("manifest.json"),
        &json!({
            "manifest_version": 1,
            "schema": "origin-observer.proxy.v1",
            "artifact_kind": "proxy_classification",
            "kind": proxy["proxy"]["kind"],
            "files": {
                "observations": observation_files,
                "proxy": "proxy.json",
            },
        }),
    )
}

fn write_json_file(path: impl AsRef<Path>, value: &serde_json::Value) -> CliResult<()> {
    let text = serde_json::to_string_pretty(value)?;
    std::fs::write(path, format!("{text}\n"))?;
    Ok(())
}

fn write_observation(format: ObserveFormat, record: &oo_observer::InvestigationRecord) {
    match format {
        ObserveFormat::InvestigationJson => {
            output::write_stdout(&output::render_investigation(record));
        }
        ObserveFormat::ReportJson => {
            let report = output::build_report(record);
            output::write_stdout(&output::render_report_json(&report));
        }
        ObserveFormat::Human => {
            let report = output::build_report(record);
            output::write_stdout(&output::render_report_human(&report));
        }
    }
}

fn build_provider_registry(
    providers: Vec<String>,
    rpc_urls: Vec<String>,
) -> CliResult<ProviderRegistry> {
    let mut registry = ProviderRegistry::default();

    for provider in providers {
        let (name, endpoint) = provider
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("invalid --provider {provider}; expected name=url"))?;
        registry.push(ProviderIdentity::new(name, endpoint));
    }

    for (index, rpc_url) in rpc_urls.into_iter().enumerate() {
        registry.push(ProviderIdentity::new(format!("rpc-{}", index + 1), rpc_url));
    }

    Ok(registry)
}

fn apply_reproduction_status_by_subject(
    records: &mut [(String, oo_observer::InvestigationRecord)],
) {
    let subjects = records
        .iter()
        .map(|(_, record)| record.plan().subject().to_owned())
        .collect::<std::collections::BTreeSet<_>>();

    for subject in subjects {
        let grouped = records
            .iter()
            .filter(|(_, record)| record.plan().subject() == subject)
            .cloned()
            .collect::<Vec<_>>();
        let report = output::build_reproduction_report(&grouped);
        let reproduction = match report.status() {
            ReportReproductionStatus::Insufficient => ReproductionStatus::Observed,
            ReportReproductionStatus::Reproduced => ReproductionStatus::Reproduced,
            ReportReproductionStatus::Contradicted => ReproductionStatus::Contradicted,
        };

        for (_, record) in records
            .iter_mut()
            .filter(|(_, record)| record.plan().subject() == subject)
        {
            record.set_reproduction(reproduction);
        }
    }
}
