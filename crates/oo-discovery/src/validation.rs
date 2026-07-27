// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/validation.rs
// Purpose : Implement the validation module for oo-discovery.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Discovery validation.

use crate::timeline::DiscoveryTimeline;

/// Validates timeline invariants.
#[must_use]
pub fn validate_timeline(timeline: &DiscoveryTimeline) -> bool {
    timeline
        .events()
        .iter()
        .all(|event| !event.subject().trim().is_empty() && !event.digest_hex().trim().is_empty())
}
