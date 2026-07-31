// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/ratelimit.rs
// Purpose : Bound the request rate an observation places on a public endpoint.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Bound the request rate an observation places on a public endpoint.
//!
//! The project observes public infrastructure it does not own. A rate limit is
//! therefore an obligation rather than a tuning knob: an experiment that
//! exhausts a shared endpoint degrades the service for everyone and produces
//! throttled responses that look like chain state.
//!
//! The limiter is a fixed-window counter driven by a caller-supplied clock, so
//! its behavior is deterministic in tests rather than dependent on wall time.

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Decision returned by the limiter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateDecision {
    /// The request may proceed.
    Permitted {
        /// Requests remaining in the current window.
        remaining: u32,
    },
    /// The request must wait.
    Limited {
        /// Milliseconds until the window resets.
        retry_after_ms: u64,
    },
}

impl RateDecision {
    /// Returns whether the request may proceed.
    #[must_use]
    pub const fn is_permitted(self) -> bool {
        matches!(self, Self::Permitted { .. })
    }
}

/// Requests permitted per window for one endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    permitted: u32,
    window_ms: u64,
}

impl RateLimit {
    /// Creates a rate limit.
    ///
    /// A limit of zero permitted requests would make the endpoint unusable, so
    /// it is raised to one: the caller intended a limit, not a block.
    #[must_use]
    pub const fn new(permitted: u32, window_ms: u64) -> Self {
        Self {
            permitted: if permitted == 0 { 1 } else { permitted },
            window_ms: if window_ms == 0 { 1 } else { window_ms },
        }
    }

    /// Creates a limit expressed as requests per second.
    #[must_use]
    pub const fn per_second(permitted: u32) -> Self {
        Self::new(permitted, 1_000)
    }

    /// Returns the permitted request count per window.
    #[must_use]
    pub const fn permitted(self) -> u32 {
        self.permitted
    }

    /// Returns the window length in milliseconds.
    #[must_use]
    pub const fn window_ms(self) -> u64 {
        self.window_ms
    }
}

impl Default for RateLimit {
    /// Five requests per second: enough for an observation run, low enough to
    /// stay a polite consumer of a public endpoint.
    fn default() -> Self {
        Self::per_second(5)
    }
}

#[derive(Debug, Clone, Copy)]
struct Window {
    started_at_ms: u64,
    used: u32,
}

/// Fixed-window rate limiter, keyed by endpoint.
///
/// The limiter is shared between clients through [`Clone`], so several clients
/// observing the same endpoint respect one budget rather than one each.
#[derive(Debug, Clone, Default)]
pub struct RateLimiter {
    limits: Arc<Mutex<BTreeMap<String, (RateLimit, Window)>>>,
    default_limit: RateLimit,
}

impl RateLimiter {
    /// Creates a limiter that applies `default_limit` to every endpoint.
    #[must_use]
    pub fn new(default_limit: RateLimit) -> Self {
        Self {
            limits: Arc::new(Mutex::new(BTreeMap::new())),
            default_limit,
        }
    }

    /// Sets a specific limit for one endpoint.
    pub fn set_limit(&self, endpoint: &str, limit: RateLimit) {
        let mut limits = self
            .limits
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let entry = limits.entry(endpoint.to_owned()).or_insert((
            limit,
            Window {
                started_at_ms: 0,
                used: 0,
            },
        ));
        entry.0 = limit;
    }

    /// Returns the limit that applies to an endpoint.
    #[must_use]
    pub fn limit_for(&self, endpoint: &str) -> RateLimit {
        let limits = self
            .limits
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        limits
            .get(endpoint)
            .map_or(self.default_limit, |(limit, _)| *limit)
    }

    /// Records an attempt at `now_ms` and reports whether it may proceed.
    ///
    /// The clock is supplied by the caller so a test can advance time exactly.
    pub fn check(&self, endpoint: &str, now_ms: u64) -> RateDecision {
        let mut limits = self
            .limits
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let entry = limits.entry(endpoint.to_owned()).or_insert((
            self.default_limit,
            Window {
                started_at_ms: now_ms,
                used: 0,
            },
        ));
        let (limit, window) = entry;

        let elapsed = now_ms.saturating_sub(window.started_at_ms);
        if elapsed >= limit.window_ms() {
            window.started_at_ms = now_ms;
            window.used = 0;
        }

        if window.used < limit.permitted() {
            window.used += 1;
            return RateDecision::Permitted {
                remaining: limit.permitted() - window.used,
            };
        }

        RateDecision::Limited {
            retry_after_ms: limit
                .window_ms()
                .saturating_sub(now_ms.saturating_sub(window.started_at_ms)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_within_the_limit_are_permitted() {
        let limiter = RateLimiter::new(RateLimit::new(2, 1_000));
        assert!(limiter.check("a", 0).is_permitted());
        assert!(limiter.check("a", 10).is_permitted());
    }

    #[test]
    fn exceeding_the_limit_reports_when_a_slot_frees() {
        let limiter = RateLimiter::new(RateLimit::new(2, 1_000));
        limiter.check("a", 0);
        limiter.check("a", 0);
        match limiter.check("a", 400) {
            RateDecision::Limited { retry_after_ms } => assert_eq!(retry_after_ms, 600),
            other => panic!("expected a limit, got {other:?}"),
        }
    }

    #[test]
    fn the_window_resets_once_it_elapses() {
        let limiter = RateLimiter::new(RateLimit::new(1, 1_000));
        assert!(limiter.check("a", 0).is_permitted());
        assert!(!limiter.check("a", 999).is_permitted());
        assert!(limiter.check("a", 1_000).is_permitted());
    }

    #[test]
    fn endpoints_hold_separate_budgets() {
        let limiter = RateLimiter::new(RateLimit::new(1, 1_000));
        assert!(limiter.check("a", 0).is_permitted());
        assert!(limiter.check("b", 0).is_permitted());
        assert!(!limiter.check("a", 0).is_permitted());
    }

    #[test]
    fn a_per_endpoint_limit_overrides_the_default() {
        let limiter = RateLimiter::new(RateLimit::new(1, 1_000));
        limiter.set_limit("generous", RateLimit::new(3, 1_000));
        assert_eq!(limiter.limit_for("generous").permitted(), 3);
        assert_eq!(limiter.limit_for("other").permitted(), 1);

        assert!(limiter.check("generous", 0).is_permitted());
        assert!(limiter.check("generous", 0).is_permitted());
        assert!(limiter.check("generous", 0).is_permitted());
        assert!(!limiter.check("generous", 0).is_permitted());
    }

    #[test]
    fn a_shared_limiter_is_one_budget_not_one_per_clone() {
        let limiter = RateLimiter::new(RateLimit::new(1, 1_000));
        let clone = limiter.clone();
        assert!(limiter.check("a", 0).is_permitted());
        assert!(!clone.check("a", 0).is_permitted());
    }

    #[test]
    fn a_zero_limit_is_treated_as_one_rather_than_a_block() {
        let limit = RateLimit::new(0, 0);
        assert_eq!(limit.permitted(), 1);
        assert_eq!(limit.window_ms(), 1);
    }
}
