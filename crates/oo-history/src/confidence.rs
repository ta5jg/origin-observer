// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/confidence.rs
// Purpose : State how much a historical claim can be trusted.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! State how much a historical claim can be trusted.
//!
//! A case study narrative is only as good as its weakest claim. This module
//! does not recompute the full confidence machinery in
//! `oo-confidence` — that crate is for live evidence, and this crate is not
//! one of its dependents — it only states the two properties a historical
//! claim needs before a case study may lean on it: a named source, and a
//! reproduction status strong enough to not be "unknown" or "refuted."

use oo_evidence::ReproductionStatus;

use crate::source::HistoricalSource;

/// A single claim made within a historical case study.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalClaim {
    statement: String,
    status: ReproductionStatus,
    source: HistoricalSource,
}

impl HistoricalClaim {
    /// Records a historical claim.
    #[must_use]
    pub fn new(
        statement: impl Into<String>,
        status: ReproductionStatus,
        source: HistoricalSource,
    ) -> Self {
        Self {
            statement: statement.into(),
            status,
            source,
        }
    }

    /// Returns the claim's statement.
    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }

    /// Returns the claim's reproduction status.
    #[must_use]
    pub const fn status(&self) -> ReproductionStatus {
        self.status
    }

    /// Returns the claim's source.
    #[must_use]
    pub const fn source(&self) -> &HistoricalSource {
        &self.source
    }

    /// Returns whether a case study may rely on this claim: it must have a
    /// named source and have been reproduced at least once by the same
    /// observer, or independently verified.
    #[must_use]
    pub fn is_reliable(&self) -> bool {
        self.source.is_named()
            && matches!(
                self.status,
                ReproductionStatus::Reproduced | ReproductionStatus::IndependentlyVerified
            )
    }

    /// Returns whether a later observation contradicted this claim.
    #[must_use]
    pub const fn is_refuted(&self) -> bool {
        matches!(self.status, ReproductionStatus::Contradicted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reproduced_claim_with_a_named_source_is_reliable() {
        let claim = HistoricalClaim::new(
            "USDT was recognized by MetaMask in 2021",
            ReproductionStatus::Reproduced,
            HistoricalSource::new("archived changelog"),
        );
        assert!(claim.is_reliable());
    }

    #[test]
    fn an_unsourced_claim_is_not_reliable_even_if_reproduced() {
        let claim = HistoricalClaim::new(
            "USDT was recognized",
            ReproductionStatus::Reproduced,
            HistoricalSource::new(""),
        );
        assert!(!claim.is_reliable());
    }

    #[test]
    fn a_merely_observed_claim_is_not_yet_reliable() {
        let claim = HistoricalClaim::new(
            "USDT was recognized",
            ReproductionStatus::Observed,
            HistoricalSource::new("one session log"),
        );
        assert!(!claim.is_reliable());
    }

    #[test]
    fn a_contradicted_claim_is_refuted_and_never_reliable() {
        let claim = HistoricalClaim::new(
            "USDT was recognized",
            ReproductionStatus::Contradicted,
            HistoricalSource::new("archived changelog"),
        );
        assert!(claim.is_refuted());
        assert!(!claim.is_reliable());
    }
}
