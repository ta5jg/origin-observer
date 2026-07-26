// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/context.rs
// Purpose : Shared execution context infrastructure.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Shared execution context.
//!
//! A `Context` carries immutable metadata describing a single execution of
//! Origin Observer. It is intentionally lightweight so it can be passed
//! throughout the system without owning heavyweight resources.

use std::collections::BTreeMap;

use crate::clock::{Clock, SystemClock};
use crate::id::{ExecutionId, SessionId};
use std::sync::Arc;

/// Immutable execution context shared between components.
#[derive(Clone, Debug)]
pub struct Context {
    execution_id: ExecutionId,
    session_id: SessionId,
    created_at_unix_ms: u128,
    labels: BTreeMap<String, String>,
}

impl Context {
    /// Creates a new context using the system clock.
    #[must_use]
    pub fn new() -> Self {
        Self::with_clock(&SystemClock)
    }

    /// Creates a new context using the supplied clock.
    #[must_use]
    pub fn with_clock<C>(clock: &C) -> Self
    where
        C: Clock,
    {
        Self {
            execution_id: ExecutionId::new(),
            session_id: SessionId::new(),
            created_at_unix_ms: clock.unix_millis(),
            labels: BTreeMap::new(),
        }
    }

    /// Returns the execution identifier.
    #[must_use]
    pub fn execution_id(&self) -> ExecutionId {
        self.execution_id
    }

    /// Returns the session identifier.
    #[must_use]
    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns the creation timestamp.
    #[must_use]
    pub fn created_at_unix_ms(&self) -> u128 {
        self.created_at_unix_ms
    }

    /// Returns an immutable view of all labels.
    #[must_use]
    pub fn labels(&self) -> &BTreeMap<String, String> {
        &self.labels
    }

    /// Returns a label by key.
    #[must_use]
    pub fn label(&self, key: impl AsRef<str>) -> Option<&str> {
        self.labels.get(key.as_ref()).map(String::as_str)
    }

    /// Inserts or replaces a label.
    pub fn insert_label(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.labels.insert(key.into(), value.into());
    }

    /// Removes a label.
    pub fn remove_label(&mut self, key: impl AsRef<str>) -> Option<String> {
        self.labels.remove(key.as_ref())
    }

    /// Returns true when the context contains the supplied label.
    #[must_use]
    pub fn contains_label(&self, key: impl AsRef<str>) -> bool {
        self.labels.contains_key(key.as_ref())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe shared context handle.
pub type SharedContext = Arc<Context>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::ManualClock;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn context_generates_ids() {
        let context = Context::new();

        assert_ne!(context.execution_id(), ExecutionId::new());

        assert_ne!(context.session_id(), SessionId::new());
    }

    #[test]
    fn context_uses_supplied_clock() {
        let clock = ManualClock::new(UNIX_EPOCH + Duration::from_secs(123));

        let context = Context::with_clock(&clock);

        assert_eq!(context.created_at_unix_ms(), 123_000);
    }

    #[test]
    fn labels_can_be_managed() {
        let mut context = Context::new();

        context.insert_label("network", "ethereum");
        context.insert_label("chain", "mainnet");

        assert!(context.contains_label("network"));
        assert_eq!(context.label("network"), Some("ethereum"));

        context.remove_label("network");

        assert!(!context.contains_label("network"));
    }

    #[test]
    fn shared_context_is_cloneable() {
        let shared = Arc::new(Context::new());

        let cloned = shared.clone();

        assert_eq!(shared.execution_id(), cloned.execution_id());
    }
}
