// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/recognition_timeline.rs
// Purpose : Track how a wallet's recognition of an asset changed over time.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Track how a wallet's recognition of an asset changed over time.

use oo_core::WalletId;

use crate::source::HistoricalSource;
use crate::timeline::{Timeline, TimelineEntry};

/// One recorded recognition observation for a wallet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecognitionEvent {
    wallet_id: WalletId,
    recognized: bool,
    source: HistoricalSource,
}

impl RecognitionEvent {
    /// Records a recognition observation.
    #[must_use]
    pub const fn new(wallet_id: WalletId, recognized: bool, source: HistoricalSource) -> Self {
        Self {
            wallet_id,
            recognized,
            source,
        }
    }

    /// Returns the observed wallet.
    #[must_use]
    pub const fn wallet_id(&self) -> WalletId {
        self.wallet_id
    }

    /// Returns whether the wallet recognized the asset at this point.
    #[must_use]
    pub const fn recognized(&self) -> bool {
        self.recognized
    }

    /// Returns the source this observation rests on.
    #[must_use]
    pub const fn source(&self) -> &HistoricalSource {
        &self.source
    }
}

/// A wallet-recognition timeline.
pub type RecognitionTimeline = Timeline<RecognitionEvent>;

impl Timeline<RecognitionEvent> {
    /// Returns the first entry at which the wallet recognized the asset.
    #[must_use]
    pub fn first_recognition(&self) -> Option<&TimelineEntry<RecognitionEvent>> {
        self.entries()
            .iter()
            .find(|entry| entry.detail().recognized())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use oo_core::WalletId;

    use super::*;

    fn wallet() -> WalletId {
        WalletId::new()
    }

    fn at(seconds: i64) -> chrono::DateTime<Utc> {
        Utc.timestamp_opt(seconds, 0).unwrap()
    }

    #[test]
    fn first_recognition_skips_leading_negative_observations() {
        let mut timeline = RecognitionTimeline::new();
        timeline.push(TimelineEntry::new(
            at(1),
            RecognitionEvent::new(wallet(), false, HistoricalSource::new("session 1")),
        ));
        timeline.push(TimelineEntry::new(
            at(2),
            RecognitionEvent::new(wallet(), true, HistoricalSource::new("session 2")),
        ));
        let first = timeline.first_recognition().unwrap();
        assert_eq!(first.timestamp(), at(2));
    }

    #[test]
    fn a_timeline_with_no_recognition_has_none() {
        let mut timeline = RecognitionTimeline::new();
        timeline.push(TimelineEntry::new(
            at(1),
            RecognitionEvent::new(wallet(), false, HistoricalSource::new("session 1")),
        ));
        assert!(timeline.first_recognition().is_none());
    }
}
