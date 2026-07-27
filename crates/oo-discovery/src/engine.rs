// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/engine.rs
// Purpose : Implement the engine module for oo-discovery.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Discovery engine.

use oo_evidence::EvidenceRecord;

use crate::decision::DiscoveryDecision;
use crate::event::DiscoveryEvent;
use crate::score::{score_timeline, DiscoveryScore};
use crate::timeline::DiscoveryTimeline;
use crate::validation::validate_timeline;

/// Result of evaluating discovery evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveryOutcome {
    timeline: DiscoveryTimeline,
    score: DiscoveryScore,
    decision: DiscoveryDecision,
}

impl DiscoveryOutcome {
    /// Returns the evaluated timeline.
    #[must_use]
    pub const fn timeline(&self) -> &DiscoveryTimeline {
        &self.timeline
    }

    /// Returns the score.
    #[must_use]
    pub const fn score(&self) -> DiscoveryScore {
        self.score
    }

    /// Returns the decision.
    #[must_use]
    pub const fn decision(&self) -> DiscoveryDecision {
        self.decision
    }
}

/// Deterministic engine that turns evidence into a discovery decision.
#[derive(Debug, Default, Clone, Copy)]
pub struct DiscoveryEngine;

impl DiscoveryEngine {
    /// Evaluates evidence records.
    #[must_use]
    pub fn evaluate<'a>(
        &self,
        records: impl IntoIterator<Item = &'a EvidenceRecord>,
    ) -> DiscoveryOutcome {
        let mut timeline = DiscoveryTimeline::default();

        for record in records {
            timeline.push(DiscoveryEvent::from_evidence(record));
        }

        let score = if validate_timeline(&timeline) {
            score_timeline(&timeline)
        } else {
            DiscoveryScore::new(0.0)
        };

        let decision = DiscoveryDecision::from_score(score);

        DiscoveryOutcome {
            timeline,
            score,
            decision,
        }
    }
}
