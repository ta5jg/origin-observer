// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-utils/tests/public_api.rs
// Purpose : Verify the utility guarantees other crates depend on.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Public API integration tests.
//!
//! Every crate that records evidence relies on these guarantees: a digest that
//! identifies content, a write that is never half-finished and a comparison
//! that does not report a difference where none exists.

use std::{fs, path::PathBuf};

use oo_utils::{fs as utils_fs, text, validation, Digest};

fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("oo-utils-api-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("scratch directory");
    path
}

#[test]
fn a_digest_identifies_its_content_across_calls() {
    let first = Digest::of_str("observed material");
    let second = Digest::of_str("observed material");
    assert_eq!(first, second);
    assert!(first.qualified().starts_with("sha256:"));
    assert!(first.verifies(b"observed material"));
    assert!(!first.verifies(b"observed materia"));
}

#[test]
fn an_evidence_record_survives_being_rewritten() {
    let root = scratch("evidence");
    let record = root.join("evidence").join("record.json");

    utils_fs::write_atomic_str(&record, "{\"confidence\":\"L2\"}").expect("first write");
    utils_fs::write_atomic_str(&record, "{\"confidence\":\"L3\"}").expect("second write");

    let contents = utils_fs::read_to_string(&record).expect("read");
    assert_eq!(contents, "{\"confidence\":\"L3\"}");

    let stray: Vec<_> = utils_fs::list_files(record.parent().expect("parent"))
        .expect("list")
        .into_iter()
        .filter(|path| path.to_string_lossy().contains(".tmp"))
        .collect();
    assert!(
        stray.is_empty(),
        "a partial write must leave nothing behind"
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn provider_spellings_of_one_asset_compare_equal() {
    assert!(text::equivalent("Tether USD", "  tether   usd "));
    assert!(!text::equivalent("Tether USD", "Tether USDC"));
}

#[test]
fn a_deceptive_asset_name_is_detectable_rather_than_silently_normalized() {
    // Two names that render identically must still be distinguishable, because
    // the difference is the finding.
    let honest = "Tether USD";
    let deceptive = "Tether\u{200B}USD";
    assert!(text::contains_invisible(deceptive));
    assert!(!text::contains_invisible(honest));
    assert_ne!(honest, deceptive);
}

#[test]
fn validators_name_the_field_that_failed() {
    let error = validation::require_identifier("chains.<id>", "Ethereum Mainnet")
        .expect_err("uppercase and spaces are rejected");
    assert!(error.to_string().contains("chains.<id>"), "{error}");
}

#[test]
fn only_http_endpoints_are_accepted_for_observation() {
    assert!(validation::require_http_url("endpoint", "https://rpc.example.org").is_ok());
    assert!(validation::require_http_url("endpoint", "file:///etc/passwd").is_err());
}
