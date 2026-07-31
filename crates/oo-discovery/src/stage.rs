// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/stage.rs
// Purpose : The stages an asset passes through on the way to wallet display.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! The stages an asset passes through on the way to wallet display.
//!
//! These are the seven stages named in WDRP's own mission statement: chain
//! state, provider response, descriptor extraction, metadata and registry
//! lookup, wallet cache and policy, confidence decision, and the final
//! displayed/hidden/unknown outcome. Every other module in this crate reports
//! evidence attached to one of these stages, so a finding can always answer
//! "at which stage did this happen."

/// One stage of the discovery mission diagram, in the order WDRP defines it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiscoveryStage {
    /// The raw state on-chain, before any provider is consulted.
    ChainState,
    /// A provider's answer to a query about that state.
    ProviderResponse,
    /// Structured descriptors extracted from the response.
    DescriptorExtraction,
    /// Metadata and registry lookups enriching the descriptor.
    MetadataLookup,
    /// The wallet's own cache and display policy.
    WalletCachePolicy,
    /// The confidence decision made from everything gathered.
    ConfidenceDecision,
    /// The final outcome: displayed, hidden, or unknown to the user.
    FinalPresentation,
}

impl DiscoveryStage {
    /// Every stage, in mission-diagram order.
    pub const ALL: [Self; 7] = [
        Self::ChainState,
        Self::ProviderResponse,
        Self::DescriptorExtraction,
        Self::MetadataLookup,
        Self::WalletCachePolicy,
        Self::ConfidenceDecision,
        Self::FinalPresentation,
    ];

    /// Returns this stage's position in the mission diagram, starting at 0.
    #[must_use]
    pub fn order(self) -> usize {
        Self::ALL
            .iter()
            .position(|stage| *stage == self)
            .expect("DiscoveryStage::ALL contains every variant")
    }

    /// Returns the stage immediately following this one, if any.
    #[must_use]
    pub fn next(self) -> Option<Self> {
        Self::ALL.get(self.order() + 1).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stages_are_ordered_as_the_mission_diagram_defines_them() {
        assert_eq!(DiscoveryStage::ChainState.order(), 0);
        assert_eq!(DiscoveryStage::FinalPresentation.order(), 6);
        assert!(DiscoveryStage::ChainState.order() < DiscoveryStage::ProviderResponse.order());
    }

    #[test]
    fn next_walks_forward_through_the_diagram() {
        assert_eq!(
            DiscoveryStage::ChainState.next(),
            Some(DiscoveryStage::ProviderResponse)
        );
        assert_eq!(DiscoveryStage::FinalPresentation.next(), None);
    }

    #[test]
    fn every_declared_stage_appears_exactly_once_in_all() {
        let mut orders: Vec<usize> = DiscoveryStage::ALL
            .iter()
            .map(|stage| stage.order())
            .collect();
        orders.sort_unstable();
        orders.dedup();
        assert_eq!(orders.len(), DiscoveryStage::ALL.len());
    }
}
