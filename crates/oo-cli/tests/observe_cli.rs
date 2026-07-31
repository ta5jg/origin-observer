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
