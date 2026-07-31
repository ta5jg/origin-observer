// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/comparison.rs
// Purpose : Compare the signals gathered for two assets.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Compare the signals gathered for two assets.
//!
//! This is the central operation behind `RQ-0006`: "Why is our asset not
//! discovered?" Comparison against a reference asset already known to be
//! discovered (USDT, for instance) turns that question into a list of named
//! differences instead of a single unexplained yes/no.

use crate::logo::LogoSignal;
use crate::metadata::MetadataCompleteness;
use crate::price::PriceSignal;
use crate::trust::TrustSignal;

/// Every signal gathered for one asset, ready for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscoverySignals {
    /// Metadata completeness signal.
    pub metadata: MetadataCompleteness,
    /// Logo availability signal.
    pub logo: LogoSignal,
    /// Price availability signal.
    pub price: PriceSignal,
    /// Trust signal.
    pub trust: TrustSignal,
    /// Number of observed wallets that recognized the asset.
    pub wallets_recognizing: usize,
    /// Number of wallets actually observed.
    pub wallets_observed: usize,
}

/// One named difference between a subject asset and a reference asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalDifference {
    /// Name of the differing field.
    pub field: &'static str,
    /// The subject asset's value, rendered for display.
    pub subject_value: String,
    /// The reference asset's value, rendered for display.
    pub reference_value: String,
}

/// Compares a subject asset's signals against a reference asset's, returning
/// every field where they differ.
///
/// An empty result does not mean the assets are discovered identically — it
/// means every signal this module tracks matched; a caller comparing final
/// wallet recognition should also check `wallets_recognizing` explicitly.
#[must_use]
pub fn compare(subject: &DiscoverySignals, reference: &DiscoverySignals) -> Vec<SignalDifference> {
    let mut differences = Vec::new();

    diff_bool(
        &mut differences,
        "metadata.is_complete",
        subject.metadata.is_complete(),
        reference.metadata.is_complete(),
    );
    diff_bool(
        &mut differences,
        "logo.present",
        subject.logo.present,
        reference.logo.present,
    );
    diff_bool(
        &mut differences,
        "price.present",
        subject.price.present,
        reference.price.present,
    );
    diff_option_bool(
        &mut differences,
        "trust.verified",
        subject.trust.verified,
        reference.trust.verified,
    );
    diff_option_bool(
        &mut differences,
        "trust.has_activity",
        subject.trust.has_activity,
        reference.trust.has_activity,
    );

    let subject_rate = recognition_rate(subject);
    let reference_rate = recognition_rate(reference);
    if (subject_rate - reference_rate).abs() > f64::EPSILON {
        differences.push(SignalDifference {
            field: "wallet_recognition_rate",
            subject_value: format!("{subject_rate:.2}"),
            reference_value: format!("{reference_rate:.2}"),
        });
    }

    differences
}

fn recognition_rate(signals: &DiscoverySignals) -> f64 {
    if signals.wallets_observed == 0 {
        return 0.0;
    }
    signals.wallets_recognizing as f64 / signals.wallets_observed as f64
}

fn diff_bool(
    differences: &mut Vec<SignalDifference>,
    field: &'static str,
    subject: bool,
    reference: bool,
) {
    if subject != reference {
        differences.push(SignalDifference {
            field,
            subject_value: subject.to_string(),
            reference_value: reference.to_string(),
        });
    }
}

fn diff_option_bool(
    differences: &mut Vec<SignalDifference>,
    field: &'static str,
    subject: Option<bool>,
    reference: Option<bool>,
) {
    if subject != reference {
        differences.push(SignalDifference {
            field,
            subject_value: format!("{subject:?}"),
            reference_value: format!("{reference:?}"),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signals(
        complete: bool,
        logo: bool,
        price: bool,
        recognizing: usize,
        observed: usize,
    ) -> DiscoverySignals {
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
            wallets_observed: observed,
        }
    }

    #[test]
    fn identical_signals_produce_no_differences() {
        let subject = signals(true, true, true, 5, 5);
        let reference = signals(true, true, true, 5, 5);
        assert!(compare(&subject, &reference).is_empty());
    }

    #[test]
    fn a_missing_logo_is_reported_by_name() {
        let subject = signals(true, false, true, 5, 5);
        let reference = signals(true, true, true, 5, 5);
        let differences = compare(&subject, &reference);
        assert!(differences
            .iter()
            .any(|difference| difference.field == "logo.present"));
    }

    #[test]
    fn a_lower_recognition_rate_is_reported() {
        let subject = signals(true, true, true, 1, 5);
        let reference = signals(true, true, true, 5, 5);
        let differences = compare(&subject, &reference);
        let recognition = differences
            .iter()
            .find(|difference| difference.field == "wallet_recognition_rate")
            .expect("recognition rate must differ");
        assert_eq!(recognition.subject_value, "0.20");
        assert_eq!(recognition.reference_value, "1.00");
    }

    #[test]
    fn an_asset_never_observed_by_any_wallet_has_a_zero_rate_not_a_panic() {
        let subject = signals(true, true, true, 0, 0);
        let reference = signals(true, true, true, 5, 5);
        assert!(!compare(&subject, &reference).is_empty());
    }
}
