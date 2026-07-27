// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/retry.rs
// Purpose : RPC retry policy.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! RPC retry policy.

/// Deterministic retry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    max_attempts: u32,
    backoff_ms: u64,
}

impl RetryPolicy {
    /// Creates a retry policy.
    #[must_use]
    pub const fn new(max_attempts: u32, backoff_ms: u64) -> Self {
        Self {
            max_attempts,
            backoff_ms,
        }
    }

    /// Returns the maximum attempt count.
    #[must_use]
    pub const fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Returns the deterministic backoff for an attempt.
    #[must_use]
    pub const fn backoff_ms(&self) -> u64 {
        self.backoff_ms
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(1, 0)
    }
}
