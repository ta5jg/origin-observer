// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/reproduction.rs
// Purpose : Implement the reproduction module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Reproduction report model.

/// Result of comparing multiple observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportReproductionStatus {
    /// Fewer than two observations were supplied.
    Insufficient,
    /// Multiple observations agree on the evidence digest.
    Reproduced,
    /// Multiple observations disagree.
    Contradicted,
}

/// One observation in a reproduction report.
#[derive(Debug, Clone, PartialEq)]
pub struct ReproductionObservation {
    provider: String,
    subject: String,
    snapshot_digest: String,
    evidence_digest: String,
    decision: String,
    score: f64,
}

impl ReproductionObservation {
    /// Creates a reproduction observation.
    #[must_use]
    pub fn new(
        provider: impl Into<String>,
        subject: impl Into<String>,
        snapshot_digest: impl Into<String>,
        evidence_digest: impl Into<String>,
        decision: impl Into<String>,
        score: f64,
    ) -> Self {
        Self {
            provider: provider.into(),
            subject: subject.into(),
            snapshot_digest: snapshot_digest.into(),
            evidence_digest: evidence_digest.into(),
            decision: decision.into(),
            score: score.clamp(0.0, 1.0),
        }
    }

    /// Returns provider label or locator.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Returns subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns snapshot digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns evidence digest.
    #[must_use]
    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    /// Returns decision text.
    #[must_use]
    pub fn decision(&self) -> &str {
        &self.decision
    }

    /// Returns score.
    #[must_use]
    pub const fn score(&self) -> f64 {
        self.score
    }
}

/// Multi-observation reproduction report.
#[derive(Debug, Clone, PartialEq)]
pub struct ReproductionReport {
    observations: Vec<ReproductionObservation>,
    status: ReportReproductionStatus,
    consensus_digest: Option<String>,
}

impl ReproductionReport {
    /// Builds a reproduction report from observations.
    #[must_use]
    pub fn new(observations: Vec<ReproductionObservation>) -> Self {
        let first_digest = observations
            .first()
            .map(|observation| observation.evidence_digest().to_owned());
        let status = if observations.len() < 2 {
            ReportReproductionStatus::Insufficient
        } else if let Some(first_digest) = first_digest.as_deref() {
            if observations
                .iter()
                .all(|observation| observation.evidence_digest() == first_digest)
            {
                ReportReproductionStatus::Reproduced
            } else {
                ReportReproductionStatus::Contradicted
            }
        } else {
            ReportReproductionStatus::Insufficient
        };
        let consensus_digest = if status == ReportReproductionStatus::Reproduced {
            first_digest
        } else {
            None
        };

        Self {
            observations,
            status,
            consensus_digest,
        }
    }

    /// Returns observations.
    #[must_use]
    pub fn observations(&self) -> &[ReproductionObservation] {
        &self.observations
    }

    /// Returns reproduction status.
    #[must_use]
    pub const fn status(&self) -> ReportReproductionStatus {
        self.status
    }

    /// Returns consensus digest when reproduced.
    #[must_use]
    pub fn consensus_digest(&self) -> Option<&str> {
        self.consensus_digest.as_deref()
    }
}
