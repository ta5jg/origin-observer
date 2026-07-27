// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/score.rs
// Purpose : Implement the score module for oo-discovery.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Discovery scoring.

use oo_evidence::ReproductionStatus;

use crate::timeline::DiscoveryTimeline;

/// Deterministic discovery score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiscoveryScore {
    value: f64,
}

impl DiscoveryScore {
    /// Creates a clamped score.
    #[must_use]
    pub fn new(value: f64) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    /// Returns the score value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

/// Scores a timeline by evidence reproduction strength.
#[must_use]
pub fn score_timeline(timeline: &DiscoveryTimeline) -> DiscoveryScore {
    if timeline.is_empty() {
        return DiscoveryScore::new(0.0);
    }

    let total = timeline
        .events()
        .iter()
        .map(|event| match event.reproduction() {
            ReproductionStatus::Unknown => 0.0,
            ReproductionStatus::Observed => 0.45,
            ReproductionStatus::Reproduced => 0.75,
            ReproductionStatus::IndependentlyVerified => 1.0,
            ReproductionStatus::Contradicted => 0.0,
        })
        .sum::<f64>();

    DiscoveryScore::new(total / timeline.len() as f64)
}
