// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/client.rs
// Purpose : Attributable RPC client.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Attributable RPC client.
//!
//! The client is the point where an observation becomes evidence. It refuses
//! requests that could not be reproduced, keeps the load on public endpoints
//! bounded, retries only failures that could plausibly succeed on a second
//! attempt, and digests the exchange so a later reader can confirm the recorded
//! request and response are the ones that happened.

use std::sync::Arc;

use oo_core::{Clock, Digest as CoreDigest, ProviderId, SystemClock};
use oo_utils::Digest;

use crate::{
    endpoint::RpcEndpoint,
    error::{RpcError, RpcResult},
    pin::{self, PinPolicy},
    ratelimit::{RateDecision, RateLimiter},
    request::RpcRequest,
    response::RpcResponse,
    retry::RetryPolicy,
    trace::RpcTrace,
    transport::RpcTransport,
};

/// RPC client that records attribution and integrity trace data.
pub struct RpcClient<T> {
    provider_id: ProviderId,
    endpoint: RpcEndpoint,
    transport: T,
    retry: RetryPolicy,
    pin_policy: PinPolicy,
    rate_limiter: Option<RateLimiter>,
    clock: Arc<dyn Clock>,
}

impl<T> RpcClient<T>
where
    T: RpcTransport,
{
    /// Creates an RPC client.
    ///
    /// Rate-limit bookkeeping reads [`SystemClock`] by default, so a live
    /// caller's requests are timed correctly without any extra setup; a test
    /// wanting deterministic timing should override it with [`Self::with_clock`].
    #[must_use]
    pub fn new(provider_id: ProviderId, endpoint: RpcEndpoint, transport: T) -> Self {
        Self {
            provider_id,
            endpoint,
            transport,
            retry: RetryPolicy::new(1, 0),
            pin_policy: PinPolicy::Required,
            rate_limiter: None,
            clock: Arc::new(SystemClock),
        }
    }

    /// Sets the retry policy.
    #[must_use]
    pub const fn with_retry(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    /// Sets the block-pinning policy.
    #[must_use]
    pub const fn with_pin_policy(mut self, policy: PinPolicy) -> Self {
        self.pin_policy = policy;
        self
    }

    /// Attaches a rate limiter shared with other clients.
    #[must_use]
    pub fn with_rate_limiter(mut self, limiter: RateLimiter) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// Overrides the clock used for rate-limit bookkeeping.
    ///
    /// Defaults to [`SystemClock`]; a test should inject a
    /// [`oo_core::ManualClock`] so it can advance time exactly rather than
    /// depending on how fast the test itself runs.
    #[must_use]
    pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }

    /// Returns the endpoint this client observes.
    #[must_use]
    pub const fn endpoint(&self) -> &RpcEndpoint {
        &self.endpoint
    }

    /// Sends a request and returns an attributable trace.
    pub async fn observe(&self, request: RpcRequest) -> RpcResult<RpcTrace> {
        // A request that reads moving state is refused before it is sent: the
        // observation would be unreproducible, and discovering that afterwards
        // costs an endpoint call and produces an unusable record.
        pin::enforce(&request, self.pin_policy)?;

        let mut attempt = 1;
        loop {
            if let Err(error) = self.check_rate_limit() {
                if attempt >= self.retry.max_attempts() {
                    return Err(error);
                }
                attempt += 1;
                self.wait(attempt).await;
                continue;
            }

            match self.transport.send(&self.endpoint, &request).await {
                Ok(response) => {
                    let digest = exchange_digest(&self.endpoint, &request, &response);
                    return Ok(RpcTrace::new(
                        self.provider_id,
                        self.endpoint.clone(),
                        request,
                        response,
                        attempt,
                        digest,
                    ));
                }
                Err(error) if attempt < self.retry.max_attempts() && error.is_retryable() => {
                    attempt += 1;
                    self.wait(attempt).await;
                }
                Err(error) => return Err(error),
            }
        }
    }

    fn check_rate_limit(&self) -> RpcResult<()> {
        let Some(limiter) = &self.rate_limiter else {
            return Ok(());
        };
        let endpoint = self.endpoint.url().to_string();
        let now_ms = u64::try_from(self.clock.unix_millis()).unwrap_or(u64::MAX);
        match limiter.check(&endpoint, now_ms) {
            RateDecision::Permitted { .. } => Ok(()),
            RateDecision::Limited { retry_after_ms } => {
                let limit = limiter.limit_for(&endpoint);
                Err(RpcError::RateLimited {
                    endpoint,
                    permitted: limit.permitted(),
                    window_ms: limit.window_ms(),
                    retry_after_ms,
                })
            }
        }
    }

    async fn wait(&self, attempt: u32) {
        let delay = self.retry.backoff_ms_for(attempt);
        if delay > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        }
    }
}

/// Digests the whole exchange: endpoint, method, parameters and response body.
///
/// The digest exists so a stored observation can be checked against the record
/// it claims to be. A digest over identifiers alone would collide for two
/// completely different exchanges and prove nothing.
#[must_use]
pub fn exchange_digest(
    endpoint: &RpcEndpoint,
    request: &RpcRequest,
    response: &RpcResponse,
) -> CoreDigest {
    let params = serde_json::to_string(request.params()).unwrap_or_else(|_| "null".to_owned());
    let body =
        serde_json::to_string(&response.to_json_value()).unwrap_or_else(|_| "null".to_owned());
    let digest = Digest::of_str_parts([
        endpoint.url().as_str(),
        request.method(),
        params.as_str(),
        body.as_str(),
    ]);

    // The utility digest is SHA-256, so its hexadecimal form decodes into
    // exactly the 32 bytes the core digest carries.
    let mut bytes = [0u8; 32];
    for (index, chunk) in digest.hex().as_bytes().chunks(2).enumerate().take(32) {
        let high = hex_value(chunk[0]);
        let low = chunk.get(1).copied().map_or(0, hex_value);
        bytes[index] = (high << 4) | low;
    }
    CoreDigest::new(bytes)
}

const fn hex_value(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use serde_json::json;
    use std::sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    };

    use super::*;
    use crate::{fixture::FixtureTransport, ratelimit::RateLimit};

    fn endpoint() -> RpcEndpoint {
        RpcEndpoint::parse("https://rpc.example.invalid").expect("endpoint")
    }

    /// Fails a fixed number of times, then succeeds.
    struct FlakyTransport {
        failures: AtomicU32,
        calls: Arc<AtomicU32>,
        error: fn() -> RpcError,
    }

    #[async_trait]
    impl RpcTransport for FlakyTransport {
        async fn send(
            &self,
            _endpoint: &RpcEndpoint,
            request: &RpcRequest,
        ) -> RpcResult<RpcResponse> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.failures.load(Ordering::SeqCst) > 0 {
                self.failures.fetch_sub(1, Ordering::SeqCst);
                return Err((self.error)());
            }
            Ok(RpcResponse::success(request.id(), json!("0x1")))
        }
    }

    fn chain_id_request() -> RpcRequest {
        RpcRequest::new(1, "eth_chainId", json!([])).expect("request")
    }

    #[tokio::test]
    async fn a_successful_observation_records_its_attempt_and_digest() {
        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_chainId",
            json!([]),
            RpcResponse::success(0, json!("0x1")),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture);
        let trace = client.observe(chain_id_request()).await.expect("trace");

        assert_eq!(trace.attempt(), 1);
        assert!(!trace.digest().is_zero());
    }

    #[test]
    fn the_digest_covers_the_response_not_only_the_identifiers() {
        // Two exchanges that share request and response ids but differ in
        // content must not produce the same digest.
        let request = chain_id_request();
        let first = exchange_digest(
            &endpoint(),
            &request,
            &RpcResponse::success(1, json!("0x1")),
        );
        let second = exchange_digest(
            &endpoint(),
            &request,
            &RpcResponse::success(1, json!("0x38")),
        );
        assert_ne!(first, second);
    }

    #[test]
    fn the_digest_covers_the_request_parameters() {
        let response = RpcResponse::success(1, json!("0x0"));
        let first = exchange_digest(
            &endpoint(),
            &RpcRequest::new(1, "eth_getBalance", json!(["0xaaa", "0x1"])).expect("request"),
            &response,
        );
        let second = exchange_digest(
            &endpoint(),
            &RpcRequest::new(1, "eth_getBalance", json!(["0xbbb", "0x1"])).expect("request"),
            &response,
        );
        assert_ne!(first, second);
    }

    #[test]
    fn the_digest_covers_the_endpoint() {
        let request = chain_id_request();
        let response = RpcResponse::success(1, json!("0x1"));
        let first = exchange_digest(&endpoint(), &request, &response);
        let second = exchange_digest(
            &RpcEndpoint::parse("https://other.example.invalid").expect("endpoint"),
            &request,
            &response,
        );
        assert_ne!(first, second);
    }

    #[test]
    fn the_digest_is_stable_for_the_same_exchange() {
        let request = chain_id_request();
        let response = RpcResponse::success(1, json!("0x1"));
        assert_eq!(
            exchange_digest(&endpoint(), &request, &response),
            exchange_digest(&endpoint(), &request, &response)
        );
    }

    #[tokio::test]
    async fn a_transport_failure_is_retried_up_to_the_policy() {
        let calls = Arc::new(AtomicU32::new(0));
        let transport = FlakyTransport {
            failures: AtomicU32::new(2),
            calls: Arc::clone(&calls),
            error: || RpcError::Transport("connection reset".to_owned()),
        };

        let client = RpcClient::new(ProviderId::new(), endpoint(), transport)
            .with_retry(RetryPolicy::new(3, 0));
        let trace = client.observe(chain_id_request()).await.expect("trace");

        assert_eq!(trace.attempt(), 3);
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn a_node_error_is_not_retried() {
        // Repeating a request the node already answered would produce several
        // identical wrong observations instead of one honest failure.
        let calls = Arc::new(AtomicU32::new(0));
        let transport = FlakyTransport {
            failures: AtomicU32::new(5),
            calls: Arc::clone(&calls),
            error: || RpcError::Response {
                code: -32601,
                message: "method not found".to_owned(),
            },
        };

        let client = RpcClient::new(ProviderId::new(), endpoint(), transport)
            .with_retry(RetryPolicy::new(5, 0));
        let error = client
            .observe(chain_id_request())
            .await
            .expect_err("must fail");

        assert!(matches!(error, RpcError::Response { .. }), "{error}");
        assert_eq!(calls.load(Ordering::SeqCst), 1, "no retry was attempted");
    }

    #[tokio::test]
    async fn an_unpinned_read_is_refused_before_the_endpoint_is_called() {
        let calls = Arc::new(AtomicU32::new(0));
        let transport = FlakyTransport {
            failures: AtomicU32::new(0),
            calls: Arc::clone(&calls),
            error: || RpcError::Transport("unused".to_owned()),
        };

        let client = RpcClient::new(ProviderId::new(), endpoint(), transport);
        let request =
            RpcRequest::new(1, "eth_call", json!([{"to": "0xabc"}, "latest"])).expect("request");
        let error = client.observe(request).await.expect_err("must fail");

        assert!(matches!(error, RpcError::UnpinnedBlock { .. }), "{error}");
        assert_eq!(calls.load(Ordering::SeqCst), 0, "nothing was sent");
    }

    #[tokio::test]
    async fn an_exploratory_client_may_read_moving_state() {
        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_call",
            json!([{"to": "0xabc"}, "latest"]),
            RpcResponse::success(0, json!("0x1")),
        );

        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture)
            .with_pin_policy(PinPolicy::AllowUnpinned);
        let request =
            RpcRequest::new(1, "eth_call", json!([{"to": "0xabc"}, "latest"])).expect("request");
        assert!(client.observe(request).await.is_ok());
    }

    #[tokio::test]
    async fn the_rate_limit_stops_a_run_from_exhausting_an_endpoint() {
        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_chainId",
            json!([]),
            RpcResponse::success(0, json!("0x1")),
        );

        let limiter = RateLimiter::new(RateLimit::new(1, 1_000));
        let clock: Arc<dyn oo_core::Clock> = Arc::new(oo_core::ManualClock::from_unix_epoch());
        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture)
            .with_rate_limiter(limiter)
            .with_clock(clock);

        assert!(client.observe(chain_id_request()).await.is_ok());
        let error = client
            .observe(chain_id_request())
            .await
            .expect_err("must be limited");
        assert!(matches!(error, RpcError::RateLimited { .. }), "{error}");
    }

    #[tokio::test]
    async fn the_default_clock_lets_a_limited_window_actually_elapse() {
        // Regression test: `check_rate_limit` used to read a `clock_ms` value
        // frozen at client construction, so a real caller retrying under a
        // real `tokio::time::sleep` never saw the window as having elapsed —
        // once rate-limited, a client was rate-limited forever. This uses no
        // clock override at all (the default `SystemClock`) with a window
        // short enough for the test to actually wait out, proving live time
        // now reaches the limiter.
        let mut fixture = FixtureTransport::new();
        fixture.insert_for(
            "eth_chainId",
            json!([]),
            RpcResponse::success(0, json!("0x1")),
        );

        let limiter = RateLimiter::new(RateLimit::new(1, 50));
        let client = RpcClient::new(ProviderId::new(), endpoint(), fixture)
            .with_rate_limiter(limiter)
            .with_retry(RetryPolicy::new(5, 20));

        // Two calls back to back: the second must wait out the 50ms window
        // via the retry loop's real sleep, not fail immediately or forever.
        assert!(client.observe(chain_id_request()).await.is_ok());
        assert!(client.observe(chain_id_request()).await.is_ok());
    }
}
