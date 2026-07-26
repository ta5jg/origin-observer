// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/runtime.rs
// Purpose : Runtime configuration and execution environment.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Runtime infrastructure shared across Origin Observer.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::clock::{Clock, SystemClock};
use crate::context::{Context, SharedContext};
use crate::id::RuntimeId;

/// Immutable runtime describing a single Origin Observer execution.
#[derive(Debug)]
pub struct Runtime {
    id: RuntimeId,
    context: SharedContext,
    name: String,
    version: String,
    metadata: BTreeMap<String, String>,
}

impl Runtime {
    /// Creates a runtime using the system clock.
    #[must_use]
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self::with_clock(name, version, &SystemClock)
    }

    /// Creates a runtime using a custom clock.
    #[must_use]
    pub fn with_clock<C>(name: impl Into<String>, version: impl Into<String>, clock: &C) -> Self
    where
        C: Clock,
    {
        Self {
            id: RuntimeId::new(),
            context: Arc::new(Context::with_clock(clock)),
            name: name.into(),
            version: version.into(),
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the runtime identifier.
    #[must_use]
    pub fn id(&self) -> RuntimeId {
        self.id
    }

    /// Returns the runtime name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the runtime version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the shared execution context.
    #[must_use]
    pub fn context(&self) -> &SharedContext {
        &self.context
    }

    /// Returns immutable runtime metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn metadata_value(&self, key: impl AsRef<str>) -> Option<&str> {
        self.metadata.get(key.as_ref()).map(String::as_str)
    }

    /// Inserts or replaces a metadata entry.
    pub fn insert_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Removes a metadata entry.
    pub fn remove_metadata(&mut self, key: impl AsRef<str>) -> Option<String> {
        self.metadata.remove(key.as_ref())
    }

    /// Returns whether a metadata key exists.
    #[must_use]
    pub fn contains_metadata(&self, key: impl AsRef<str>) -> bool {
        self.metadata.contains_key(key.as_ref())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new("origin-observer", env!("CARGO_PKG_VERSION"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::ManualClock;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn runtime_creates_unique_identifier() {
        let a = Runtime::default();
        let b = Runtime::default();

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn runtime_uses_supplied_clock() {
        let clock = ManualClock::new(UNIX_EPOCH + Duration::from_secs(100));

        let runtime = Runtime::with_clock("oo", "1.0.0", &clock);

        assert_eq!(runtime.context().created_at_unix_ms(), 100_000);
    }

    #[test]
    fn metadata_operations() {
        let mut runtime = Runtime::default();

        runtime.insert_metadata("network", "ethereum");

        assert!(runtime.contains_metadata("network"));
        assert_eq!(runtime.metadata_value("network"), Some("ethereum"));

        runtime.remove_metadata("network");

        assert!(!runtime.contains_metadata("network"));
    }

    #[test]
    fn runtime_name_and_version() {
        let runtime = Runtime::new("OriginObserver", "0.1.0");

        assert_eq!(runtime.name(), "OriginObserver");
        assert_eq!(runtime.version(), "0.1.0");
    }
}
