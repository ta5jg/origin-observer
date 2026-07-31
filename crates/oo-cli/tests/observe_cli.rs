// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cli/tests/observe_cli.rs
// Purpose : Verify the observe CLI vertical slice.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

use std::path::PathBuf;
use std::process::Command;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_oo-cli"))
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/rpc")
        .join(name)
}

#[test]
fn observe_report_json_from_payload_file() {
    let output = cli()
        .args([
            "observe",
            "--subject",
            "eth_getBalance",
            "--payload-file",
            fixture_path("eth_get_balance.json")
                .to_str()
                .expect("fixture path is valid UTF-8"),
            "--format",
            "report-json",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains(r#""subject": "eth_getBalance""#));
    assert!(stdout.contains(r#""conclusion": "NeedsReview""#));
}

#[test]
fn observe_human_from_payload_json() {
    let output = cli()
        .args([
            "observe",
            "--subject",
            "eth_chainId",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x1"}"#,
            "--format",
            "human",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains("Origin Observer finding"));
    assert!(stdout.contains("subject: eth_chainId"));
    assert!(stdout.contains("conclusion: NeedsReview"));
}

#[test]
fn observe_rejects_invalid_payload_json() {
    let output = cli()
        .args([
            "observe",
            "--payload-json",
            "{bad",
            "--format",
            "report-json",
        ])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("invalid --payload-json"));
}

#[test]
fn observe_rejects_invalid_rpc_url() {
    let output = cli()
        .args([
            "observe",
            "--rpc-url",
            "ftp://example.invalid",
            "--format",
            "report-json",
        ])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("invalid provider endpoint"));
}

#[test]
fn observe_proxy_classification_requires_an_address() {
    let output = cli()
        .args(["observe", "--strategy", "proxy-classification"])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("proxy-classification requires --address"));
}

#[test]
fn observe_proxy_classification_requires_a_provider_or_rpc_url() {
    let output = cli()
        .args([
            "observe",
            "--strategy",
            "proxy-classification",
            "--address",
            "0xdac17f958d2ee523a2206206994597c13d831ec7",
        ])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("multi-call strategies require --provider or --rpc-url"));
}

#[test]
fn observe_chain_id_strategy_uses_builtin_subject() {
    let output = cli()
        .args([
            "observe",
            "--strategy",
            "chain-id",
            "--format",
            "report-json",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains(r#""subject": "eth_chainId""#));
}

#[test]
fn observe_balance_strategy_requires_address() {
    let output = cli()
        .args([
            "observe",
            "--strategy",
            "balance",
            "--format",
            "report-json",
        ])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("--strategy balance requires --address"));
}

#[test]
fn observe_contract_code_strategy_requires_address() {
    let output = cli()
        .args([
            "observe",
            "--strategy",
            "contract-code",
            "--format",
            "report-json",
        ])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("--strategy contract-code requires --address"));
}

#[test]
fn observe_balance_payload_includes_semantic_summary() {
    let output = cli()
        .args([
            "observe",
            "--subject",
            "eth_getBalance",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x0"}"#,
            "--format",
            "investigation-json",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains(r#""kind": "native_balance""#));
    assert!(stdout.contains(r#""is_zero": true"#));
}

#[test]
fn observe_erc20_symbol_payload_decodes_abi_string() {
    let output = cli()
        .args([
            "observe",
            "--subject",
            "erc20.symbol",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000045445535400000000000000000000000000000000000000000000000000000000"}"#,
            "--format",
            "investigation-json",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains(r#""field": "symbol""#));
    assert!(stdout.contains(r#""value": "TEST""#));
}

#[test]
fn observe_rpc_error_payload_includes_semantic_error() {
    let output = cli()
        .args([
            "observe",
            "--subject",
            "eth_getCode",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32603,"message":"Internal error"}}"#,
            "--format",
            "investigation-json",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains(r#""kind": "rpc_error""#));
    assert!(stdout.contains(r#""message": "Internal error""#));
}

#[test]
fn observe_cache_state_defaults_to_empty_and_is_reflected_in_output() {
    let output = cli()
        .args([
            "observe",
            "--subject",
            "eth_chainId",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x1"}"#,
            "--format",
            "investigation-json",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains(r#""state": "Empty""#));
    assert!(stdout.contains(r#""attributable_to_live_discovery": true"#));
}

#[test]
fn observe_declared_warm_cache_state_is_not_attributable_to_live_discovery() {
    let output = cli()
        .args([
            "observe",
            "--subject",
            "eth_chainId",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x1"}"#,
            "--format",
            "investigation-json",
            "--cache-state",
            "warm",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains(r#""state": "Warm""#));
    assert!(stdout.contains(r#""attributable_to_live_discovery": false"#));
}

#[test]
fn observe_wallet_view_lists_every_built_in_wallet_by_default() {
    let output = cli()
        .args([
            "observe",
            "--strategy",
            "wallet-view",
            "--address",
            "0xdac17f958d2ee523a2206206994597c13d831ec7",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x6001"}"#,
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains("\"wallet_config_id\": \"metamask\""));
    assert!(stdout.contains("\"wallet_config_id\": \"ledger-live\""));
}

#[test]
fn observe_wallet_view_can_be_filtered_to_one_wallet() {
    let output = cli()
        .args([
            "observe",
            "--strategy",
            "wallet-view",
            "--address",
            "0xdac17f958d2ee523a2206206994597c13d831ec7",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x6001"}"#,
            "--wallet",
            "metamask",
        ])
        .output()
        .expect("cli runs");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(stdout.contains("\"wallet_config_id\": \"metamask\""));
    assert!(!stdout.contains("\"wallet_config_id\": \"ledger-live\""));
}

#[test]
fn observe_wallet_view_rejects_an_unknown_wallet() {
    let output = cli()
        .args([
            "observe",
            "--strategy",
            "wallet-view",
            "--address",
            "0xdac17f958d2ee523a2206206994597c13d831ec7",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x6001"}"#,
            "--wallet",
            "not-a-real-wallet",
        ])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("unknown wallet"));
}

#[test]
fn observe_record_history_requires_a_question_id() {
    let path = std::env::temp_dir().join("oo-cli-test-history-missing-question-id.json");
    let _ = std::fs::remove_file(&path);

    let output = cli()
        .args([
            "observe",
            "--subject",
            "eth_chainId",
            "--payload-json",
            r#"{"jsonrpc":"2.0","id":1,"result":"0x1"}"#,
            "--record-history",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("cli runs");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("--record-history requires --question-id"));
}

#[test]
fn observe_record_history_creates_and_then_appends_to_a_case_study() {
    let path = std::env::temp_dir().join("oo-cli-test-history-case-study.json");
    let _ = std::fs::remove_file(&path);

    let args = [
        "observe".to_owned(),
        "--subject".to_owned(),
        "eth_chainId".to_owned(),
        "--payload-json".to_owned(),
        r#"{"jsonrpc":"2.0","id":1,"result":"0x1"}"#.to_owned(),
        "--record-history".to_owned(),
        path.to_str().unwrap().to_owned(),
        "--question-id".to_owned(),
        "RQ-0006".to_owned(),
        "--wallet".to_owned(),
        "metamask".to_owned(),
    ];

    let first = cli().args(&args).output().expect("cli runs");
    assert!(first.status.success());

    let after_first: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(after_first["question_id"], "RQ-0006");
    assert_eq!(
        after_first["recognition_timeline"]["entries"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let second = cli().args(&args).output().expect("cli runs");
    assert!(second.status.success());

    let after_second: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        after_second["recognition_timeline"]["entries"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn observe_record_history_rejects_a_mismatched_question_id() {
    let path = std::env::temp_dir().join("oo-cli-test-history-mismatch.json");
    let _ = std::fs::remove_file(&path);

    let base_args = |question_id: &str| {
        vec![
            "observe".to_owned(),
            "--subject".to_owned(),
            "eth_chainId".to_owned(),
            "--payload-json".to_owned(),
            r#"{"jsonrpc":"2.0","id":1,"result":"0x1"}"#.to_owned(),
            "--record-history".to_owned(),
            path.to_str().unwrap().to_owned(),
            "--question-id".to_owned(),
            question_id.to_owned(),
        ]
    };

    let first = cli().args(base_args("RQ-0006")).output().expect("cli runs");
    assert!(first.status.success());

    let second = cli().args(base_args("RQ-0007")).output().expect("cli runs");
    assert!(!second.status.success());
    let stderr = String::from_utf8(second.stderr).expect("stderr is UTF-8");
    assert!(stderr.contains("addresses 'RQ-0006', not 'RQ-0007'"));

    let _ = std::fs::remove_file(&path);
}
