// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/metadata.rs
// Purpose : Score how complete an asset's metadata is for discovery purposes.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Score how complete an asset's metadata is for discovery purposes.
//!
//! This directly serves `RQ-0007`: "What is the minimum condition set
//! required for discovery?" A completeness signal over `oo-provider`'s merged
//! metadata is one measurable input toward answering it, tracked separately
//! from raw provider merging so discovery-specific weighting stays out of
//! `oo-provider`, which has no opinion about what "enough" means.

use oo_provider::MergedMetadata;

/// How complete an asset's metadata is, from a discovery point of view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetadataCompleteness {
    /// Whether a name was resolved without conflict.
    pub has_name: bool,
    /// Whether a symbol was resolved without conflict.
    pub has_symbol: bool,
    /// Whether decimals were resolved without conflict.
    pub has_decimals: bool,
    /// Whether any field had a provider disagreement.
    pub has_conflict: bool,
}

impl MetadataCompleteness {
    /// Evaluates completeness from merged provider metadata.
    #[must_use]
    pub fn evaluate(merged: &MergedMetadata) -> Self {
        Self {
            has_name: merged.name.is_some(),
            has_symbol: merged.symbol.is_some(),
            has_decimals: merged.decimals.is_some(),
            has_conflict: !merged.conflicts.is_empty(),
        }
    }

    /// Returns the count of the three core fields present, from 0 to 3.
    #[must_use]
    pub const fn field_count(self) -> u8 {
        self.has_name as u8 + self.has_symbol as u8 + self.has_decimals as u8
    }

    /// Returns whether every core field is present and none conflicted.
    ///
    /// This is the strongest metadata signal this module can report; it does
    /// not by itself mean the asset will be discovered; wallets weigh other
    /// stages too.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        self.field_count() == 3 && !self.has_conflict
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merged(name: bool, symbol: bool, decimals: bool, conflicts: usize) -> MergedMetadata {
        MergedMetadata {
            name: name.then(|| "Tether USD".to_owned()),
            symbol: symbol.then(|| "USDT".to_owned()),
            decimals: decimals.then_some(6),
            conflicts: vec![oo_provider::MetadataConflict {
                field: "name",
                values: Vec::new(),
            }]
            .into_iter()
            .take(conflicts)
            .collect(),
        }
    }

    #[test]
    fn full_metadata_with_no_conflict_is_complete() {
        let completeness = MetadataCompleteness::evaluate(&merged(true, true, true, 0));
        assert_eq!(completeness.field_count(), 3);
        assert!(completeness.is_complete());
    }

    #[test]
    fn a_conflict_prevents_completeness_even_with_every_field_present() {
        let completeness = MetadataCompleteness::evaluate(&merged(true, true, true, 1));
        assert_eq!(completeness.field_count(), 3);
        assert!(!completeness.is_complete());
        assert!(completeness.has_conflict);
    }

    #[test]
    fn partial_metadata_is_not_complete() {
        let completeness = MetadataCompleteness::evaluate(&merged(true, false, false, 0));
        assert_eq!(completeness.field_count(), 1);
        assert!(!completeness.is_complete());
    }
}
