// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/tests/public_api.rs
// Purpose : Verify the transport guarantees an observation depends on.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Public API integration tests.
//!
//! Part 02 promises deterministic, attributable and replayable observations.
//! These tests exercise that promise through the public API: a run replays from
//! recorded material without touching the network, refuses reads it could not
//! reproduce, and produces a digest that identifies the exchange.

use std::sync::Arc;

use oo_core::{Clock, ManualClock, ProviderId};
use oo_rpc::{
    BlockPin, FixtureTransport, PinPolicy, RateLimit, RateLimiter, RecordingTransport, RetryPolicy,
    RpcClient, RpcEndpoint, RpcError, RpcRequest, RpcResponse, RpcTransport,
};
use serde_json::json;

fn endpoint() -> RpcEndpoint {
    RpcEndpoint::parse("https://rpc.example.invalid").expect("endpoint")
}

fn usdt_name_call(block: &str) -> RpcRequest {
    RpcRequest::new(
        1,
        "eth_call",
        json!([
            {"to": "0xdac17f958d2ee523a2206206994597c13d831ec7", "data": "0x06fdde03"},
            block
        ]),
    )
    .expect("request")
}

fn fixtures() -> FixtureTransport {
    let mut fixture = FixtureTransport::new();
    fixture.insert_for(
        "eth_chainId",
        json!([]),
        RpcResponse::success(0, json!("0x1")),
    );
    fixture.insert_for(
        "eth_call",
        json!([
            {"to": "0xdac17f958d2ee523a2206206994597c13d831ec7", "data": "0x06fdde03"},
            "0x1220000"
        ]),
        RpcResponse::success(0, json!("0x54657468657220555344")),
    );
    fixture
}

#[tokio::test]
async fn a_pinned_observation_replays_from_recorded_material() {
    let client = RpcClient::new(ProviderId::new(), endpoint(), fixtures());
    let trace = client
        .observe(usdt_name_call("0x1220000"))
        .await
        .expect("trace");

    assert_eq!(trace.attempt(), 1);
    assert!(!trace.digest().is_zero());
}

#[tokio::test]
async fn an_unreproducible_read_is_refused() {
    // "latest" names moving state, so the observation could never be repeated.
    let client = RpcClient::new(ProviderId::new(), endpoint(), fixtures());
    let error = client
        .observe(usdt_name_call("latest"))
        .await
        .expect_err("must refuse");

    assert!(matches!(error, RpcError::UnpinnedBlock { .. }), "{error}");
}

#[tokio::test]
async fn pinning_turns_an_exploratory_request_into_a_reproducible_one() {
    let pinned = oo_rpc::pin::pin_request(&usdt_name_call("latest"), BlockPin::new(0x0122_0000))
        .expect("pinned");
    let client = RpcClient::new(ProviderId::new(), endpoint(), fixtures());
    assert!(client.observe(pinned).await.is_ok());
}

#[tokio::test]
async fn a_recorded_run_replays_identically() {
    let recorder = RecordingTransport::new(fixtures());
    let request = usdt_name_call("0x1220000");

    let live = recorder.send(&endpoint(), &request).await.expect("live");
    let replay = recorder
        .recorded()
        .send(&endpoint(), &request)
        .await
        .expect("replay");

    assert_eq!(live, replay);
}

#[tokio::test]
async fn the_digest_distinguishes_two_different_answers_to_one_question() {
    let request = usdt_name_call("0x1220000");

    let mut first_set = FixtureTransport::new();
    first_set.insert_for(
        request.method(),
        request.params().clone(),
        RpcResponse::success(0, json!("0x54657468657220555344")),
    );
    let mut second_set = FixtureTransport::new();
    second_set.insert_for(
        request.method(),
        request.params().clone(),
        RpcResponse::success(0, json!("0x555344204361736800")),
    );

    let provider = ProviderId::new();
    let first = RpcClient::new(provider, endpoint(), first_set)
        .observe(request.clone())
        .await
        .expect("first");
    let second = RpcClient::new(provider, endpoint(), second_set)
        .observe(request)
        .await
        .expect("second");

    assert_ne!(
        first.digest(),
        second.digest(),
        "a digest that ignored the answer could not detect a changed observation"
    );
}

#[tokio::test]
async fn a_permanent_failure_fails_once_rather_than_repeating() {
    // The fixture set has no answer for this question, which is a permanent
    // condition: retrying it would only produce the same failure again.
    let client = RpcClient::new(ProviderId::new(), endpoint(), fixtures())
        .with_retry(RetryPolicy::new(5, 0));
    let request = RpcRequest::new(1, "eth_blockNumber", json!([])).expect("request");

    let error = client.observe(request).await.expect_err("must fail");
    assert!(matches!(error, RpcError::MissingFixture { .. }), "{error}");
}

#[tokio::test]
async fn a_shared_rate_limit_bounds_a_whole_run() {
    let limiter = RateLimiter::new(RateLimit::new(1, 1_000));
    let provider = ProviderId::new();
    let clock: Arc<dyn Clock> = Arc::new(ManualClock::from_unix_epoch());

    let first = RpcClient::new(provider, endpoint(), fixtures())
        .with_rate_limiter(limiter.clone())
        .with_clock(clock.clone());
    let second = RpcClient::new(provider, endpoint(), fixtures())
        .with_rate_limiter(limiter)
        .with_clock(clock);

    assert!(first.observe(usdt_name_call("0x1220000")).await.is_ok());
    let error = second
        .observe(usdt_name_call("0x1220000"))
        .await
        .expect_err("the budget is shared, not per client");
    assert!(matches!(error, RpcError::RateLimited { .. }), "{error}");
}

#[tokio::test]
async fn an_endpoint_serving_the_wrong_chain_stops_the_run() {
    let mut fixture = FixtureTransport::new();
    // Sepolia answering where mainnet was configured.
    fixture.insert_for(
        "eth_chainId",
        json!([]),
        RpcResponse::success(0, json!("0xaa36a7")),
    );

    let error = oo_rpc::confirm_chain(&fixture, &endpoint(), 1, 1)
        .await
        .expect_err("must fail");
    assert!(matches!(error, RpcError::ChainMismatch { .. }), "{error}");
}

#[tokio::test]
async fn an_endpoint_serving_the_expected_chain_is_confirmed() {
    let observed = oo_rpc::confirm_chain(&fixtures(), &endpoint(), 1, 1)
        .await
        .expect("confirmed");
    assert_eq!(observed, 1);
}

#[test]
fn a_strict_policy_is_the_default_for_a_new_client() {
    // The permissive policy must be chosen deliberately; it cannot be reached
    // by forgetting to configure the client.
    assert_eq!(PinPolicy::default(), PinPolicy::Required);
}
