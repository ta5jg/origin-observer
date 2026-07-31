// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/trust.rs
// Purpose : Aggregate trust-relevant signals gathered for an asset.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Aggregate trust-relevant signals gathered for an asset.
//!
//! "Trust" here means only what this project can actually observe: whether
//! source code is verified and whether an indexer reports adoption activity.
//! It is not a judgment about legitimacy — a new, unverified, low-activity
//! asset can be entirely legitimate, and a verified, high-activity one can
//! still be a scam. The signal is reported as-is for a later confidence
//! decision to weigh, not pre-interpreted here.

use oo_provider::{IndexerSnapshot, VerificationResult};

/// Trust-relevant signals gathered for one asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrustSignal {
    /// Whether source code verification was observed, when an explorer was
    /// consulted at all.
    pub verified: Option<bool>,
    /// Whether an indexer reported any adoption activity, when one was
    /// consulted at all.
    pub has_activity: Option<bool>,
}

impl TrustSignal {
    /// Evaluates the signal from an optional explorer result and an optional
    /// indexer snapshot.
    #[must_use]
    pub fn evaluate(
        verification: Option<&VerificationResult>,
        indexer: Option<&IndexerSnapshot>,
    ) -> Self {
        Self {
            verified: verification.map(|result| result.verified),
            has_activity: indexer.map(IndexerSnapshot::has_activity_signal),
        }
    }

    /// Returns whether every consulted source reported a positive signal.
    ///
    /// A source that was never consulted does not count against the asset:
    /// this reports what was actually checked, not what was assumed.
    #[must_use]
    pub fn every_consulted_source_is_positive(self) -> bool {
        self.verified.unwrap_or(true) && self.has_activity.unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_sources_consulted_is_neither_positive_nor_negative() {
        let signal = TrustSignal::evaluate(None, None);
        assert_eq!(signal.verified, None);
        assert!(signal.every_consulted_source_is_positive());
    }

    #[test]
    fn an_unverified_source_is_reported_and_counted() {
        let signal = TrustSignal {
            verified: Some(false),
            has_activity: None,
        };
        assert!(!signal.every_consulted_source_is_positive());
    }
}
