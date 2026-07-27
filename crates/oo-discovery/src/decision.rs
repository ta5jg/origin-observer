// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/decision.rs
// Purpose : Implement the decision module for oo-discovery.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Discovery decision model.

use crate::score::DiscoveryScore;

/// Decision produced from a discovery score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryDecision {
    /// No release-worthy evidence exists.
    Reject,
    /// Evidence exists, but it needs more reproduction.
    NeedsReview,
    /// Evidence is strong enough to publish as a research finding.
    Accept,
}

impl DiscoveryDecision {
    /// Creates a decision from score thresholds.
    #[must_use]
    pub fn from_score(score: DiscoveryScore) -> Self {
        let value = score.value();
        if value >= 0.80 {
            Self::Accept
        } else if value >= 0.40 {
            Self::NeedsReview
        } else {
            Self::Reject
        }
    }
}
