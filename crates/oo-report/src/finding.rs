// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/finding.rs
// Purpose : Implement the finding module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Report finding model.

use oo_discovery::DiscoveryDecision;

/// A publishable research finding derived from evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct ReportFinding {
    subject: String,
    evidence_digest: String,
    decision: DiscoveryDecision,
    score: f64,
}

impl ReportFinding {
    /// Creates a report finding.
    #[must_use]
    pub fn new(
        subject: impl Into<String>,
        evidence_digest: impl Into<String>,
        decision: DiscoveryDecision,
        score: f64,
    ) -> Self {
        Self {
            subject: subject.into(),
            evidence_digest: evidence_digest.into(),
            decision,
            score: score.clamp(0.0, 1.0),
        }
    }

    /// Returns the finding subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the evidence digest.
    #[must_use]
    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    /// Returns the discovery decision.
    #[must_use]
    pub const fn decision(&self) -> DiscoveryDecision {
        self.decision
    }

    /// Returns the discovery score.
    #[must_use]
    pub const fn score(&self) -> f64 {
        self.score
    }
}
