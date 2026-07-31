// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/attribution.rs
// Purpose : Attribute one answer to the provider that gave it.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Attribute one answer to the provider that gave it.
//!
//! WDRP's required evidence fields include the provider or registry source
//! and a timestamp for every observation. This is the one record type that
//! satisfies both: which provider answered, what was asked, and when — using
//! the project's [`oo_core::Clock`] rather than a direct wall-clock read, so a
//! test can supply a fixed instant.

use oo_core::{Clock, ProviderId};

use crate::capability::ProviderCategory;

/// Attribution for one provider answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribution {
    /// Provider that answered.
    pub provider_id: ProviderId,
    /// Category the provider was consulted as.
    pub category: ProviderCategory,
    /// What was queried: a URL, a method name, or another locator.
    pub locator: String,
    /// UNIX timestamp, in seconds, the answer was recorded.
    pub observed_unix_seconds: u64,
}

impl Attribution {
    /// Records an attribution using the given clock.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
        category: ProviderCategory,
        locator: impl Into<String>,
        clock: &dyn Clock,
    ) -> Self {
        Self {
            provider_id,
            category,
            locator: locator.into(),
            observed_unix_seconds: clock.unix_seconds(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use oo_core::ManualClock;

    use super::*;

    #[test]
    fn attribution_records_the_clocks_time() {
        let clock = ManualClock::new(UNIX_EPOCH + Duration::from_secs(1_000));
        let attribution = Attribution::new(
            ProviderId::new(),
            ProviderCategory::Price,
            "https://api.example.org/price",
            &clock,
        );
        assert_eq!(attribution.observed_unix_seconds, 1_000);
        assert_eq!(attribution.category, ProviderCategory::Price);
    }
}
