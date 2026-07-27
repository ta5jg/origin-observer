// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/conclusion.rs
// Purpose : Implement the conclusion module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Report conclusion model.

use oo_discovery::DiscoveryDecision;

/// Human-meaningful report conclusion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportConclusion {
    /// Finding is not supported by releasable evidence.
    NotSupported,
    /// Finding needs review or reproduction.
    NeedsReview,
    /// Finding is supported enough for publication.
    Supported,
}

impl ReportConclusion {
    /// Maps discovery decisions into report conclusions.
    #[must_use]
    pub const fn from_decision(decision: DiscoveryDecision) -> Self {
        match decision {
            DiscoveryDecision::Reject => Self::NotSupported,
            DiscoveryDecision::NeedsReview => Self::NeedsReview,
            DiscoveryDecision::Accept => Self::Supported,
        }
    }
}
