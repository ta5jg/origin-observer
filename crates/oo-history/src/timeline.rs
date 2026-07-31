// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/timeline.rs
// Purpose : A generic, time-ordered sequence of historical events.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A generic, time-ordered sequence of historical events.
//!
//! [`recognition_timeline`](crate::recognition_timeline) and
//! [`provider_timeline`](crate::provider_timeline) both need the same thing —
//! entries ordered by when they happened, with a way to check that order and
//! find the earliest or latest entry — so that behavior lives here once
//! rather than being duplicated per timeline kind.

use chrono::{DateTime, Utc};

/// One entry in a [`Timeline`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineEntry<T> {
    timestamp: DateTime<Utc>,
    detail: T,
}

impl<T> TimelineEntry<T> {
    /// Creates a timeline entry.
    #[must_use]
    pub const fn new(timestamp: DateTime<Utc>, detail: T) -> Self {
        Self { timestamp, detail }
    }

    /// Returns when this entry occurred.
    #[must_use]
    pub const fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Returns the entry's detail.
    #[must_use]
    pub const fn detail(&self) -> &T {
        &self.detail
    }
}

/// A sequence of historical entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timeline<T> {
    entries: Vec<TimelineEntry<T>>,
}

impl<T> Timeline<T> {
    /// Creates an empty timeline.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Appends an entry.
    pub fn push(&mut self, entry: TimelineEntry<T>) {
        self.entries.push(entry);
    }

    /// Returns the timeline's entries, in insertion order.
    #[must_use]
    pub fn entries(&self) -> &[TimelineEntry<T>] {
        &self.entries
    }

    /// Returns whether entries are ordered by non-decreasing timestamp.
    ///
    /// An out-of-order timeline usually means an entry was appended with a
    /// timestamp from before the last recorded event, which is worth
    /// rejecting explicitly rather than silently accepting a scrambled
    /// history.
    #[must_use]
    pub fn is_chronological(&self) -> bool {
        self.entries
            .windows(2)
            .all(|pair| pair[0].timestamp <= pair[1].timestamp)
    }

    /// Returns the earliest entry, by insertion order among ties.
    #[must_use]
    pub fn earliest(&self) -> Option<&TimelineEntry<T>> {
        self.entries.iter().min_by_key(|entry| entry.timestamp)
    }

    /// Returns the latest entry, by insertion order among ties.
    #[must_use]
    pub fn latest(&self) -> Option<&TimelineEntry<T>> {
        self.entries.iter().max_by_key(|entry| entry.timestamp)
    }
}

impl<T> Default for Timeline<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn at(seconds: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(seconds, 0).unwrap()
    }

    #[test]
    fn an_empty_timeline_is_trivially_chronological() {
        assert!(Timeline::<u32>::new().is_chronological());
    }

    #[test]
    fn entries_added_in_increasing_time_order_are_chronological() {
        let mut timeline = Timeline::new();
        timeline.push(TimelineEntry::new(at(1), 1));
        timeline.push(TimelineEntry::new(at(2), 2));
        assert!(timeline.is_chronological());
    }

    #[test]
    fn a_later_entry_appended_before_an_earlier_one_is_not_chronological() {
        let mut timeline = Timeline::new();
        timeline.push(TimelineEntry::new(at(2), 2));
        timeline.push(TimelineEntry::new(at(1), 1));
        assert!(!timeline.is_chronological());
    }

    #[test]
    fn earliest_and_latest_are_found_regardless_of_insertion_order() {
        let mut timeline = Timeline::new();
        timeline.push(TimelineEntry::new(at(5), "middle"));
        timeline.push(TimelineEntry::new(at(1), "first"));
        timeline.push(TimelineEntry::new(at(9), "last"));
        assert_eq!(*timeline.earliest().unwrap().detail(), "first");
        assert_eq!(*timeline.latest().unwrap().detail(), "last");
    }
}
