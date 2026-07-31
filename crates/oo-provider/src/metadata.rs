// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/metadata.rs
// Purpose : Asset metadata from registries and metadata providers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Asset metadata from registries and metadata providers.
//!
//! A registry and a dedicated metadata service answer the same question —
//! name, symbol, decimals — through different transports, so they share one
//! result shape. When two providers disagree, [`merge`] reports the
//! disagreement instead of silently preferring one: a name conflict between
//! two registries is itself a discovery finding, not noise to average away.

use crate::attribution::Attribution;

/// Asset metadata as reported by one provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetMetadata {
    /// Attribution for this answer.
    pub attribution: Attribution,
    /// Asset display name, if reported.
    pub name: Option<String>,
    /// Asset symbol, if reported.
    pub symbol: Option<String>,
    /// Decimal precision, if reported.
    pub decimals: Option<u8>,
}

/// A field where two or more providers disagreed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataConflict {
    /// Field name: `"name"`, `"symbol"` or `"decimals"`.
    pub field: &'static str,
    /// Distinct values reported, each with the provider that reported it.
    pub values: Vec<(String, String)>,
}

/// Result of combining metadata from multiple providers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MergedMetadata {
    /// Name, when every provider that reported one agreed.
    pub name: Option<String>,
    /// Symbol, when every provider that reported one agreed.
    pub symbol: Option<String>,
    /// Decimals, when every provider that reported one agreed.
    pub decimals: Option<u8>,
    /// Every field where providers disagreed.
    pub conflicts: Vec<MetadataConflict>,
}

/// Merges metadata from multiple providers, reporting disagreement rather
/// than resolving it.
#[must_use]
pub fn merge(reports: &[AssetMetadata]) -> MergedMetadata {
    let mut merged = MergedMetadata::default();

    merged.name = merge_field(reports, "name", &mut merged.conflicts, |report| {
        report.name.clone()
    });
    merged.symbol = merge_field(reports, "symbol", &mut merged.conflicts, |report| {
        report.symbol.clone()
    });
    merged.decimals = merge_field(reports, "decimals", &mut merged.conflicts, |report| {
        report.decimals.map(|value| value.to_string())
    })
    .and_then(|value| value.parse().ok());

    merged
}

fn merge_field(
    reports: &[AssetMetadata],
    field: &'static str,
    conflicts: &mut Vec<MetadataConflict>,
    extract: impl Fn(&AssetMetadata) -> Option<String>,
) -> Option<String> {
    let mut values: Vec<(String, String)> = Vec::new();
    for report in reports {
        if let Some(value) = extract(report) {
            values.push((value, report.attribution.provider_id.to_string()));
        }
    }

    let mut distinct: Vec<&str> = values.iter().map(|(value, _)| value.as_str()).collect();
    distinct.sort_unstable();
    distinct.dedup();

    match distinct.len() {
        0 => None,
        1 => Some(distinct[0].to_owned()),
        _ => {
            conflicts.push(MetadataConflict { field, values });
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};

    use super::*;
    use crate::capability::ProviderCategory;

    fn report(name: &str, symbol: &str, decimals: u8) -> AssetMetadata {
        AssetMetadata {
            attribution: Attribution::new(
                ProviderId::new(),
                ProviderCategory::Registry,
                "test",
                &ManualClock::new(UNIX_EPOCH),
            ),
            name: Some(name.to_owned()),
            symbol: Some(symbol.to_owned()),
            decimals: Some(decimals),
        }
    }

    #[test]
    fn agreeing_providers_merge_cleanly() {
        let merged = merge(&[
            report("Tether USD", "USDT", 6),
            report("Tether USD", "USDT", 6),
        ]);
        assert_eq!(merged.name.as_deref(), Some("Tether USD"));
        assert_eq!(merged.symbol.as_deref(), Some("USDT"));
        assert_eq!(merged.decimals, Some(6));
        assert!(merged.conflicts.is_empty());
    }

    #[test]
    fn disagreeing_providers_report_a_conflict_rather_than_picking_one() {
        let merged = merge(&[
            report("Tether USD", "USDT", 6),
            report("Tether Gold", "USDT", 6),
        ]);
        assert!(merged.name.is_none());
        assert_eq!(merged.conflicts.len(), 1);
        assert_eq!(merged.conflicts[0].field, "name");
        assert_eq!(merged.conflicts[0].values.len(), 2);
    }

    #[test]
    fn an_empty_report_set_merges_to_nothing() {
        let merged = merge(&[]);
        assert!(merged.name.is_none());
        assert!(merged.conflicts.is_empty());
    }

    #[test]
    fn a_field_only_one_provider_reported_is_not_a_conflict() {
        let mut only_name = report("Tether USD", "USDT", 6);
        only_name.symbol = None;
        let merged = merge(&[only_name]);
        assert_eq!(merged.name.as_deref(), Some("Tether USD"));
        assert!(merged.conflicts.is_empty());
    }
}
