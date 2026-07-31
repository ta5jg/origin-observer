// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/prediction.rs
// Purpose : Predict discoverability from measured signals, with an explanation.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Predict discoverability from measured signals, with an explanation.
//!
//! This answers `RQ-0009`: "Can discovery be predicted?" The prediction is a
//! deterministic weighted sum over signals this crate already measures, never
//! a trained model — a black-box prediction with no stated reasoning would be
//! exactly the kind of unexplained wallet-specific hack the roadmap forbids,
//! and WDRP requires every finding to name its evidence. Each factor's
//! contribution is reported alongside the total, so the prediction can be
//! checked term by term rather than trusted as a single number.

use crate::comparison::DiscoverySignals;

/// One factor's contribution to a prediction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PredictionFactor {
    /// Factor name.
    pub name: &'static str,
    /// Weight assigned to this factor, from 0.0 to 1.0.
    pub weight: f64,
    /// Whether the observed signal satisfied this factor.
    pub satisfied: bool,
}

impl PredictionFactor {
    /// Returns this factor's contribution to the total score.
    #[must_use]
    pub fn contribution(self) -> f64 {
        if self.satisfied {
            self.weight
        } else {
            0.0
        }
    }
}

/// A discoverability prediction with its full factor breakdown.
#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverabilityPrediction {
    /// Every factor considered, in evaluation order.
    pub factors: Vec<PredictionFactor>,
}

impl DiscoverabilityPrediction {
    /// Predicts discoverability from measured signals.
    ///
    /// Weights are fixed and documented here rather than tuned against a
    /// dataset: without a labelled dataset of confirmed discovery outcomes to
    /// validate weights against, any specific numeric tuning would be an
    /// unverified claim. These weights encode a stated, arguable prior —
    /// metadata and price matter most because several wallets gate default
    /// display on them — and are meant to be revisited once
    /// `oo-dataset`/`oo-history` can supply real outcomes to check them
    /// against.
    #[must_use]
    pub fn predict(signals: &DiscoverySignals) -> Self {
        let factors = vec![
            PredictionFactor {
                name: "metadata complete",
                weight: 0.30,
                satisfied: signals.metadata.is_complete(),
            },
            PredictionFactor {
                name: "logo present",
                weight: 0.15,
                satisfied: signals.logo.present,
            },
            PredictionFactor {
                name: "price available",
                weight: 0.25,
                satisfied: signals.price.present,
            },
            PredictionFactor {
                name: "trust signals positive",
                weight: 0.15,
                satisfied: signals.trust.every_consulted_source_is_positive(),
            },
            PredictionFactor {
                name: "recognized by at least one other wallet",
                weight: 0.15,
                satisfied: signals.wallets_recognizing > 0,
            },
        ];
        Self { factors }
    }

    /// Returns the total predicted score, from 0.0 to 1.0.
    #[must_use]
    pub fn score(&self) -> f64 {
        self.factors
            .iter()
            .map(|factor| factor.contribution())
            .sum()
    }

    /// Returns the factors that were not satisfied, in evaluation order —
    /// the concrete list of what would need to change to raise the score.
    #[must_use]
    pub fn unmet_factors(&self) -> Vec<&PredictionFactor> {
        self.factors
            .iter()
            .filter(|factor| !factor.satisfied)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logo::LogoSignal;
    use crate::metadata::MetadataCompleteness;
    use crate::price::PriceSignal;
    use crate::trust::TrustSignal;

    fn signals(complete: bool, logo: bool, price: bool, recognizing: usize) -> DiscoverySignals {
        DiscoverySignals {
            metadata: MetadataCompleteness {
                has_name: complete,
                has_symbol: complete,
                has_decimals: complete,
                has_conflict: false,
            },
            logo: LogoSignal {
                present: logo,
                resolvable_scheme: logo,
            },
            price: PriceSignal {
                present: price,
                quote_count: usize::from(price),
                disagreement: false,
            },
            trust: TrustSignal {
                verified: None,
                has_activity: None,
            },
            wallets_recognizing: recognizing,
            wallets_observed: 1,
        }
    }

    #[test]
    fn every_factor_satisfied_scores_at_the_top() {
        let prediction = DiscoverabilityPrediction::predict(&signals(true, true, true, 1));
        assert!((prediction.score() - 1.0).abs() < f64::EPSILON);
        assert!(prediction.unmet_factors().is_empty());
    }

    #[test]
    fn no_observable_factor_satisfied_scores_only_the_default_trust_weight() {
        // Trust defaults to satisfied when no source was consulted (None is
        // not a negative finding), so the floor is that weight, not zero.
        let prediction = DiscoverabilityPrediction::predict(&signals(false, false, false, 0));
        assert!((prediction.score() - 0.15).abs() < 1e-9);
        assert_eq!(
            prediction.unmet_factors().len(),
            prediction.factors.len() - 1
        );
    }

    #[test]
    fn the_score_is_the_sum_of_satisfied_weights() {
        let prediction = DiscoverabilityPrediction::predict(&signals(true, false, true, 0));
        // metadata (0.30) + price (0.25) + trust default-satisfied (0.15).
        let expected = 0.30 + 0.25 + 0.15;
        assert!((prediction.score() - expected).abs() < 1e-9);
    }

    #[test]
    fn unmet_factors_name_exactly_what_would_need_to_change() {
        // Trust defaults to satisfied here because no source was consulted
        // (None counts as no negative finding); only the missing logo and the
        // absence of any other recognizing wallet remain unmet.
        let prediction = DiscoverabilityPrediction::predict(&signals(true, false, true, 0));
        let names: Vec<&str> = prediction
            .unmet_factors()
            .iter()
            .map(|factor| factor.name)
            .collect();
        assert_eq!(
            names,
            vec!["logo present", "recognized by at least one other wallet"]
        );
    }
}
