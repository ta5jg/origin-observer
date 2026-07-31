// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/path.rs
// Purpose : Record which discovery stages an investigation actually reached.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Record which discovery stages an investigation actually reached.
//!
//! WDRP's mission is to identify the contributing discovery sources for an
//! asset, which means knowing not just the final outcome but which stages
//! were reached and which were not. A path that reaches
//! `DescriptorExtraction` but never `MetadataLookup` names a different
//! failure than one that reaches every stage but fails at
//! `WalletCachePolicy`.

use crate::stage::DiscoveryStage;

/// The stages one investigation reached, each with a note on what was found.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiscoveryPath {
    reached: Vec<(DiscoveryStage, String)>,
}

impl DiscoveryPath {
    /// Records that a stage was reached, with a short note on what happened.
    pub fn record(&mut self, stage: DiscoveryStage, note: impl Into<String>) {
        self.reached.push((stage, note.into()));
    }

    /// Returns whether a stage was reached.
    #[must_use]
    pub fn reached(&self, stage: DiscoveryStage) -> bool {
        self.reached.iter().any(|(recorded, _)| *recorded == stage)
    }

    /// Returns every stage reached, in the order they were recorded.
    #[must_use]
    pub fn stages(&self) -> Vec<DiscoveryStage> {
        self.reached.iter().map(|(stage, _)| *stage).collect()
    }

    /// Returns the furthest stage reached, by mission-diagram order rather
    /// than recording order.
    #[must_use]
    pub fn furthest_stage(&self) -> Option<DiscoveryStage> {
        self.reached
            .iter()
            .map(|(stage, _)| *stage)
            .max_by_key(|stage| stage.order())
    }

    /// Returns the first stage in the mission diagram that was never reached.
    ///
    /// This is the path's failure point: everything before it happened,
    /// nothing at or after it did.
    #[must_use]
    pub fn first_missing_stage(&self) -> Option<DiscoveryStage> {
        DiscoveryStage::ALL
            .into_iter()
            .find(|stage| !self.reached(*stage))
    }

    /// Returns whether the path reached every stage.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.first_missing_stage().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_path_reaches_nothing() {
        let path = DiscoveryPath::default();
        assert!(!path.reached(DiscoveryStage::ChainState));
        assert_eq!(path.first_missing_stage(), Some(DiscoveryStage::ChainState));
        assert!(!path.is_complete());
    }

    #[test]
    fn the_first_missing_stage_is_the_earliest_gap_not_the_furthest_reached() {
        let mut path = DiscoveryPath::default();
        path.record(DiscoveryStage::ChainState, "observed");
        path.record(DiscoveryStage::ProviderResponse, "observed");
        // DescriptorExtraction is skipped, then a later stage is recorded.
        path.record(DiscoveryStage::MetadataLookup, "observed anyway");

        assert_eq!(path.furthest_stage(), Some(DiscoveryStage::MetadataLookup));
        assert_eq!(
            path.first_missing_stage(),
            Some(DiscoveryStage::DescriptorExtraction),
            "the gap is the finding, not how far the path later reached"
        );
    }

    #[test]
    fn a_path_reaching_every_stage_is_complete() {
        let mut path = DiscoveryPath::default();
        for stage in DiscoveryStage::ALL {
            path.record(stage, "observed");
        }
        assert!(path.is_complete());
        assert_eq!(path.first_missing_stage(), None);
    }
}
