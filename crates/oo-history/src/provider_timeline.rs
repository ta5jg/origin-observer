// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/provider_timeline.rs
// Purpose : Track how a provider's metadata availability changed over time.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Track how a provider's metadata availability changed over time.

use oo_core::ProviderId;

use crate::source::HistoricalSource;
use crate::timeline::{Timeline, TimelineEntry};

/// One recorded metadata-availability observation for a provider.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProviderEvent {
    provider_id: ProviderId,
    metadata_available: bool,
    source: HistoricalSource,
}

impl ProviderEvent {
    /// Records a metadata-availability observation.
    #[must_use]
    pub const fn new(
        provider_id: ProviderId,
        metadata_available: bool,
        source: HistoricalSource,
    ) -> Self {
        Self {
            provider_id,
            metadata_available,
            source,
        }
    }

    /// Returns the observed provider.
    #[must_use]
    pub const fn provider_id(&self) -> ProviderId {
        self.provider_id
    }

    /// Returns whether metadata was available at this point.
    #[must_use]
    pub const fn metadata_available(&self) -> bool {
        self.metadata_available
    }

    /// Returns the source this observation rests on.
    #[must_use]
    pub const fn source(&self) -> &HistoricalSource {
        &self.source
    }
}

/// A provider metadata-availability timeline.
pub type ProviderTimeline = Timeline<ProviderEvent>;

impl Timeline<ProviderEvent> {
    /// Returns the first entry at which metadata became available.
    #[must_use]
    pub fn first_metadata_availability(&self) -> Option<&TimelineEntry<ProviderEvent>> {
        self.entries()
            .iter()
            .find(|entry| entry.detail().metadata_available())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use oo_core::ProviderId;

    use super::*;

    fn provider() -> ProviderId {
        ProviderId::new()
    }

    fn at(seconds: i64) -> chrono::DateTime<Utc> {
        Utc.timestamp_opt(seconds, 0).unwrap()
    }

    #[test]
    fn first_metadata_availability_skips_leading_unavailable_observations() {
        let mut timeline = ProviderTimeline::new();
        timeline.push(TimelineEntry::new(
            at(1),
            ProviderEvent::new(provider(), false, HistoricalSource::new("check 1")),
        ));
        timeline.push(TimelineEntry::new(
            at(2),
            ProviderEvent::new(provider(), true, HistoricalSource::new("check 2")),
        ));
        let first = timeline.first_metadata_availability().unwrap();
        assert_eq!(first.timestamp(), at(2));
    }

    #[test]
    fn a_timeline_with_no_availability_has_none() {
        let mut timeline = ProviderTimeline::new();
        timeline.push(TimelineEntry::new(
            at(1),
            ProviderEvent::new(provider(), false, HistoricalSource::new("check 1")),
        ));
        assert!(timeline.first_metadata_availability().is_none());
    }
}
