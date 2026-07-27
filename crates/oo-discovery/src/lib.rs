// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/lib.rs
// Purpose : Model discovery paths, timelines and decisions.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Model discovery paths, timelines and decisions.

pub mod comparison;
pub mod decision;
pub mod engine;
pub mod event;
pub mod identity;
pub mod logo;
pub mod metadata;
pub mod path;
pub mod prediction;
pub mod price;
pub mod resolution;
pub mod score;
pub mod stage;
pub mod timeline;
pub mod trust;
pub mod validation;

pub use decision::DiscoveryDecision;
pub use engine::{DiscoveryEngine, DiscoveryOutcome};
pub use event::DiscoveryEvent;
pub use score::{score_timeline, DiscoveryScore};
pub use timeline::DiscoveryTimeline;
pub use validation::validate_timeline;

#[cfg(test)]
mod tests {
    use oo_evidence::{EvidenceBuilder, EvidenceSourceKind};

    use super::*;

    #[test]
    fn evaluates_observed_evidence_as_review() {
        let evidence =
            EvidenceBuilder::new(EvidenceSourceKind::Rpc, "fixture://rpc", "eth_getBalance")
                .bytes(br#"{"result":"0x10"}"#.to_vec())
                .build();

        let outcome = DiscoveryEngine.evaluate([&evidence]);

        assert_eq!(outcome.timeline().len(), 1);
        assert_eq!(outcome.decision(), DiscoveryDecision::NeedsReview);
        assert!(outcome.score().value() > 0.0);
    }
}
