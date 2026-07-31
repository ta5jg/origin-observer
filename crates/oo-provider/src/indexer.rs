// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/indexer.rs
// Purpose : Aggregate on-chain statistics from a chain indexer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Aggregate on-chain statistics from a chain indexer.
//!
//! Indexer APIs vary far more than explorer or price APIs do, with no shared
//! response shape worth standardizing on here. This module keeps the result
//! to the attributes discovery actually needs — holder and transfer counts —
//! and leaves the specific request shape for whichever indexer is consulted
//! to a caller, rather than committing this crate to one vendor's schema.

use crate::attribution::Attribution;

/// Aggregate statistics for one asset as reported by an indexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexerSnapshot {
    /// Attribution for this answer.
    pub attribution: Attribution,
    /// Distinct holder count, when reported.
    pub holder_count: Option<u64>,
    /// Total transfer count, when reported.
    pub transfer_count: Option<u64>,
}

impl IndexerSnapshot {
    /// Returns whether the indexer reports any adoption signal at all.
    ///
    /// An indexer that answers with no holders and no transfers has not
    /// necessarily failed; it may genuinely describe an asset nobody has
    /// touched. This distinguishes "reported zero" from "reported nothing."
    #[must_use]
    pub const fn has_activity_signal(&self) -> bool {
        self.holder_count.is_some() || self.transfer_count.is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};

    use super::*;
    use crate::capability::ProviderCategory;

    fn attribution() -> Attribution {
        Attribution::new(
            ProviderId::new(),
            ProviderCategory::Indexer,
            "test",
            &ManualClock::new(UNIX_EPOCH),
        )
    }

    #[test]
    fn a_reported_zero_still_counts_as_a_signal() {
        let snapshot = IndexerSnapshot {
            attribution: attribution(),
            holder_count: Some(0),
            transfer_count: Some(0),
        };
        assert!(snapshot.has_activity_signal());
    }

    #[test]
    fn nothing_reported_has_no_signal() {
        let snapshot = IndexerSnapshot {
            attribution: attribution(),
            holder_count: None,
            transfer_count: None,
        };
        assert!(!snapshot.has_activity_signal());
    }
}
