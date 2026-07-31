// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/retry.rs
// Purpose : RPC retry policy.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! RPC retry policy.
//!
//! Backoff is exponential and deterministic: no jitter. Jitter would make two
//! runs of the same experiment take different paths through the same failure,
//! and the timing of an observation is part of what a reproduction repeats.

/// Largest backoff any single attempt will wait, in milliseconds.
///
/// A retry that waits longer than this has stopped being a retry and become an
/// unbounded stall inside a procedure that claims to be repeatable.
pub const MAX_BACKOFF_MS: u64 = 30_000;

/// Deterministic retry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    max_attempts: u32,
    backoff_ms: u64,
}

impl RetryPolicy {
    /// Creates a retry policy.
    ///
    /// `max_attempts` counts the first attempt, so a value of one means no
    /// retry. Zero is raised to one: a policy that permits no attempt at all
    /// would silently produce no observation.
    #[must_use]
    pub const fn new(max_attempts: u32, backoff_ms: u64) -> Self {
        Self {
            max_attempts: if max_attempts == 0 { 1 } else { max_attempts },
            backoff_ms,
        }
    }

    /// Returns the maximum attempt count, including the first.
    #[must_use]
    pub const fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Returns the base backoff.
    #[must_use]
    pub const fn backoff_ms(&self) -> u64 {
        self.backoff_ms
    }

    /// Returns the delay to wait before `attempt`.
    ///
    /// The first attempt never waits. Each later attempt doubles the base
    /// delay, bounded by [`MAX_BACKOFF_MS`].
    #[must_use]
    pub const fn backoff_ms_for(&self, attempt: u32) -> u64 {
        if attempt <= 1 || self.backoff_ms == 0 {
            return 0;
        }
        let exponent = attempt - 2;
        if exponent >= 32 {
            return MAX_BACKOFF_MS;
        }
        match self.backoff_ms.checked_mul(1u64 << exponent) {
            Some(value) if value < MAX_BACKOFF_MS => value,
            _ => MAX_BACKOFF_MS,
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_attempt_never_waits() {
        let policy = RetryPolicy::new(3, 100);
        assert_eq!(policy.backoff_ms_for(1), 0);
    }

    #[test]
    fn backoff_doubles_with_each_attempt() {
        let policy = RetryPolicy::new(5, 100);
        assert_eq!(policy.backoff_ms_for(2), 100);
        assert_eq!(policy.backoff_ms_for(3), 200);
        assert_eq!(policy.backoff_ms_for(4), 400);
    }

    #[test]
    fn backoff_is_bounded() {
        let policy = RetryPolicy::new(64, 1_000);
        assert_eq!(policy.backoff_ms_for(40), MAX_BACKOFF_MS);
        assert!(policy.backoff_ms_for(20) <= MAX_BACKOFF_MS);
    }

    #[test]
    fn a_zero_backoff_stays_zero() {
        let policy = RetryPolicy::new(3, 0);
        assert_eq!(policy.backoff_ms_for(3), 0);
    }

    #[test]
    fn a_policy_always_permits_at_least_one_attempt() {
        assert_eq!(RetryPolicy::new(0, 0).max_attempts(), 1);
    }

    #[test]
    fn the_default_policy_does_not_retry() {
        assert_eq!(RetryPolicy::default().max_attempts(), 1);
    }
}
