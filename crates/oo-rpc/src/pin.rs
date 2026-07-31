// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/pin.rs
// Purpose : Require observations to name the chain state they read.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Require observations to name the chain state they read.
//!
//! An observation taken against `latest` cannot be reproduced: by the time
//! anyone repeats it the chain has moved, and a disagreement between the two
//! runs cannot be attributed to the thing under study. Requests that carry a
//! block reference must therefore pin it to a concrete block, unless the
//! operator has explicitly allowed unpinned reads for an exploratory run.

use serde_json::Value;

use crate::{
    error::{RpcError, RpcResult},
    request::RpcRequest,
};

/// Block tags that name moving state rather than a fixed block.
pub const UNPINNED_TAGS: &[&str] = &["latest", "pending", "safe", "finalized", "earliest"];

/// Methods whose final parameter is a block reference.
///
/// The list is explicit rather than inferred, because treating an unknown
/// method's last parameter as a block reference would reject valid requests.
pub const BLOCK_TAGGED_METHODS: &[&str] = &[
    "eth_call",
    "eth_getBalance",
    "eth_getCode",
    "eth_getStorageAt",
    "eth_getTransactionCount",
    "eth_getProof",
];

/// A pinned block reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockPin {
    number: u64,
}

impl BlockPin {
    /// Creates a pin at a block number.
    #[must_use]
    pub const fn new(number: u64) -> Self {
        Self { number }
    }

    /// Returns the block number.
    #[must_use]
    pub const fn number(self) -> u64 {
        self.number
    }

    /// Returns the hexadecimal form used by Ethereum JSON-RPC.
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("0x{:x}", self.number)
    }

    /// Parses a hexadecimal or decimal block number.
    ///
    /// A moving tag such as `latest` is not a pin and returns `None`.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        let trimmed = value.trim();
        if let Some(hex) = trimmed.strip_prefix("0x") {
            return u64::from_str_radix(hex, 16).ok().map(Self::new);
        }
        trimmed.parse::<u64>().ok().map(Self::new)
    }
}

/// Policy deciding whether a request may read unpinned state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PinPolicy {
    /// Reject any request that reads moving state.
    #[default]
    Required,
    /// Permit unpinned reads, for exploratory runs whose results are not
    /// citable as reproduced evidence.
    AllowUnpinned,
}

impl PinPolicy {
    /// Creates a policy from the configuration flag.
    #[must_use]
    pub const fn from_allow_unpinned(allow: bool) -> Self {
        if allow {
            Self::AllowUnpinned
        } else {
            Self::Required
        }
    }

    /// Returns whether unpinned reads are permitted.
    #[must_use]
    pub const fn allows_unpinned(self) -> bool {
        matches!(self, Self::AllowUnpinned)
    }
}

/// Returns the block reference a request carries, if it has one.
#[must_use]
pub fn block_reference(request: &RpcRequest) -> Option<String> {
    if !BLOCK_TAGGED_METHODS.contains(&request.method()) {
        return None;
    }
    let params = request.params().as_array()?;
    let last = params.last()?;
    match last {
        Value::String(value) => Some(value.clone()),
        Value::Object(map) => map
            .get("blockNumber")
            .and_then(Value::as_str)
            .map(str::to_owned),
        _ => None,
    }
}

/// Returns whether a block reference names moving state.
#[must_use]
pub fn is_unpinned(reference: &str) -> bool {
    UNPINNED_TAGS.contains(&reference.trim().to_ascii_lowercase().as_str())
}

/// Rejects a request that would read unpinned state under a strict policy.
pub fn enforce(request: &RpcRequest, policy: PinPolicy) -> RpcResult<()> {
    if policy.allows_unpinned() {
        return Ok(());
    }
    let Some(reference) = block_reference(request) else {
        return Ok(());
    };
    if is_unpinned(&reference) {
        return Err(RpcError::UnpinnedBlock {
            method: request.method().to_owned(),
            block: reference,
        });
    }
    Ok(())
}

/// Rewrites a request's block reference to a pinned block.
///
/// This is how an exploratory request becomes reproducible: the caller resolves
/// the current head once, then pins every subsequent request in the run to it.
pub fn pin_request(request: &RpcRequest, pin: BlockPin) -> RpcResult<RpcRequest> {
    if !BLOCK_TAGGED_METHODS.contains(&request.method()) {
        return Ok(request.clone());
    }
    let Some(params) = request.params().as_array() else {
        return Ok(request.clone());
    };
    if params.is_empty() {
        return Ok(request.clone());
    }

    let mut params = params.clone();
    let last = params.len() - 1;
    match &params[last] {
        Value::String(_) => params[last] = Value::String(pin.to_hex()),
        Value::Object(map) => {
            let mut map = map.clone();
            map.insert("blockNumber".to_owned(), Value::String(pin.to_hex()));
            params[last] = Value::Object(map);
        }
        // A method in the list whose final parameter is neither a tag nor a
        // block object is left untouched rather than reshaped by guesswork.
        _ => return Ok(request.clone()),
    }

    RpcRequest::new(request.id(), request.method(), Value::Array(params))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn call(params: Value) -> RpcRequest {
        RpcRequest::new(1, "eth_call", params).expect("request")
    }

    #[test]
    fn a_pin_round_trips_through_its_hexadecimal_form() {
        let pin = BlockPin::new(19_000_000);
        assert_eq!(BlockPin::parse(&pin.to_hex()), Some(pin));
        assert_eq!(BlockPin::parse("19000000"), Some(pin));
        assert_eq!(BlockPin::parse("latest"), None);
    }

    #[test]
    fn moving_tags_are_recognized() {
        for tag in UNPINNED_TAGS {
            assert!(is_unpinned(tag), "{tag}");
        }
        assert!(is_unpinned("  LATEST "));
        assert!(!is_unpinned("0x1234"));
    }

    #[test]
    fn a_strict_policy_rejects_a_moving_read() {
        let request = call(json!([{"to": "0xabc"}, "latest"]));
        let error = enforce(&request, PinPolicy::Required).expect_err("must reject");
        assert!(error.to_string().contains("unpinned"), "{error}");
    }

    #[test]
    fn a_strict_policy_accepts_a_pinned_read() {
        let request = call(json!([{"to": "0xabc"}, "0x1220000"]));
        assert!(enforce(&request, PinPolicy::Required).is_ok());
    }

    #[test]
    fn an_exploratory_policy_permits_a_moving_read() {
        let request = call(json!([{"to": "0xabc"}, "latest"]));
        assert!(enforce(&request, PinPolicy::AllowUnpinned).is_ok());
    }

    #[test]
    fn methods_without_a_block_parameter_are_unaffected() {
        let request = RpcRequest::new(1, "eth_chainId", json!([])).expect("request");
        assert_eq!(block_reference(&request), None);
        assert!(enforce(&request, PinPolicy::Required).is_ok());
    }

    #[test]
    fn pinning_replaces_a_moving_tag() {
        let request = call(json!([{"to": "0xabc"}, "latest"]));
        let pinned = pin_request(&request, BlockPin::new(255)).expect("pinned");
        assert_eq!(block_reference(&pinned).as_deref(), Some("0xff"));
        assert!(enforce(&pinned, PinPolicy::Required).is_ok());
    }

    #[test]
    fn pinning_preserves_the_method_and_the_leading_parameters() {
        let request = call(json!([{"to": "0xabc", "data": "0x06fdde03"}, "latest"]));
        let pinned = pin_request(&request, BlockPin::new(1)).expect("pinned");
        assert_eq!(pinned.method(), "eth_call");
        assert_eq!(
            pinned.params().as_array().expect("array")[0],
            json!({"to": "0xabc", "data": "0x06fdde03"})
        );
    }

    #[test]
    fn pinning_a_method_without_a_block_parameter_changes_nothing() {
        let request = RpcRequest::new(7, "eth_chainId", json!([])).expect("request");
        let pinned = pin_request(&request, BlockPin::new(9)).expect("pinned");
        assert_eq!(pinned, request);
    }
}
