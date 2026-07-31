// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/explorer.rs
// Purpose : Contract verification results from block explorers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Contract verification results from block explorers.
//!
//! Whether source code is verified changes how much an ABI can be trusted: a
//! verified source is `AcquisitionKind::VerifiedSource` in `oo-abi`'s terms,
//! while an unverified contract offers nothing beyond bytecode inference.
//! This module parses the common explorer response shape (Etherscan and its
//! forks: BscScan, PolygonScan) rather than a specific vendor SDK, since
//! `config/providers.toml` already lists several explorers sharing this API.

use serde_json::Value;

use crate::attribution::Attribution;

/// Verification result for one contract address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationResult {
    /// Attribution for this answer.
    pub attribution: Attribution,
    /// Whether the explorer reports verified source.
    pub verified: bool,
    /// Contract name, when reported.
    pub contract_name: Option<String>,
    /// Compiler version, when reported.
    pub compiler_version: Option<String>,
    /// Whether an ABI was returned alongside the source.
    pub abi_available: bool,
}

/// Parses an Etherscan-family `getsourcecode` response.
///
/// Returns `None` when the response does not match the expected shape at
/// all — a genuinely different API, not a verification result to interpret
/// one way or the other.
#[must_use]
pub fn parse_source_code_response(
    attribution: Attribution,
    response: &Value,
) -> Option<VerificationResult> {
    let entry = response.get("result")?.as_array()?.first()?;

    let source_code = entry
        .get("SourceCode")
        .and_then(Value::as_str)
        .unwrap_or("");
    let abi = entry.get("ABI").and_then(Value::as_str).unwrap_or("");
    let contract_name = entry
        .get("ContractName")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    let compiler_version = entry
        .get("CompilerVersion")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    Some(VerificationResult {
        attribution,
        verified: !source_code.is_empty(),
        contract_name,
        compiler_version,
        // Unverified contracts commonly report the literal string
        // "Contract source code not verified" in the ABI field instead of an
        // empty value, so a JSON-parseable ABI is required, not just a
        // non-empty string.
        abi_available: !source_code.is_empty() && serde_json::from_str::<Value>(abi).is_ok(),
    })
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};
    use serde_json::json;

    use super::*;
    use crate::capability::ProviderCategory;

    fn attribution() -> Attribution {
        Attribution::new(
            ProviderId::new(),
            ProviderCategory::Explorer,
            "test",
            &ManualClock::new(UNIX_EPOCH),
        )
    }

    #[test]
    fn a_verified_contract_is_parsed() {
        let response = json!({
            "status": "1",
            "result": [{
                "SourceCode": "contract Token {}",
                "ABI": "[{\"type\":\"function\"}]",
                "ContractName": "Token",
                "CompilerVersion": "v0.8.20"
            }]
        });
        let result = parse_source_code_response(attribution(), &response).unwrap();
        assert!(result.verified);
        assert!(result.abi_available);
        assert_eq!(result.contract_name.as_deref(), Some("Token"));
    }

    #[test]
    fn an_unverified_contract_reports_no_abi() {
        let response = json!({
            "status": "0",
            "result": [{
                "SourceCode": "",
                "ABI": "Contract source code not verified"
            }]
        });
        let result = parse_source_code_response(attribution(), &response).unwrap();
        assert!(!result.verified);
        assert!(!result.abi_available);
    }

    #[test]
    fn an_unrecognized_shape_returns_none() {
        assert!(parse_source_code_response(attribution(), &json!({"unexpected": true})).is_none());
    }
}
