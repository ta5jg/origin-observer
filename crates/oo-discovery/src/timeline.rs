// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/timeline.rs
// Purpose : Implement the timeline module for oo-discovery.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Discovery timeline.

use crate::event::DiscoveryEvent;

/// Ordered set of normalized discovery events.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiscoveryTimeline {
    events: Vec<DiscoveryEvent>,
}

impl DiscoveryTimeline {
    /// Appends an event.
    pub fn push(&mut self, event: DiscoveryEvent) {
        self.events.push(event);
    }

    /// Returns all events.
    #[must_use]
    pub fn events(&self) -> &[DiscoveryEvent] {
        &self.events
    }

    /// Returns event count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns true when no events exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
