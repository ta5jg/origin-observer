// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/price.rs
// Purpose : Score whether an asset has a price signal available.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Score whether an asset has a price signal available.
//!
//! Many wallets query a price feed before showing an asset's balance in fiat
//! terms, and some suppress an asset entirely when no price is available.
//! Whether providers agree matters as much as whether a price exists at all:
//! reusing `oo-provider`'s own disagreement detection keeps this module from
//! re-deciding what counts as a meaningful divergence.

use oo_provider::{disagreements, PriceQuote};

/// Price availability signal for one asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceSignal {
    /// Whether any provider returned a parseable quote.
    pub present: bool,
    /// Number of providers that returned a parseable quote.
    pub quote_count: usize,
    /// Whether providers disagreed beyond the given tolerance.
    pub disagreement: bool,
}

impl PriceSignal {
    /// Evaluates the signal from every quote gathered for an asset.
    #[must_use]
    pub fn evaluate(quotes: &[PriceQuote], tolerance: f64) -> Self {
        let quote_count = quotes
            .iter()
            .filter(|quote| quote.as_f64().is_some())
            .count();
        Self {
            present: quote_count > 0,
            quote_count,
            disagreement: !disagreements(quotes, tolerance).is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};
    use oo_provider::{Attribution, ProviderCategory};

    use super::*;

    fn quote(value: &str) -> PriceQuote {
        PriceQuote::new(
            Attribution::new(
                ProviderId::new(),
                ProviderCategory::Price,
                "test",
                &ManualClock::new(UNIX_EPOCH),
            ),
            "usd",
            value,
        )
    }

    #[test]
    fn no_quotes_means_no_signal() {
        let signal = PriceSignal::evaluate(&[], 0.05);
        assert!(!signal.present);
        assert_eq!(signal.quote_count, 0);
    }

    #[test]
    fn agreeing_quotes_report_no_disagreement() {
        let signal = PriceSignal::evaluate(&[quote("1.00"), quote("1.001")], 0.05);
        assert!(signal.present);
        assert_eq!(signal.quote_count, 2);
        assert!(!signal.disagreement);
    }

    #[test]
    fn diverging_quotes_are_flagged() {
        let signal = PriceSignal::evaluate(&[quote("1.00"), quote("2.00")], 0.05);
        assert!(signal.disagreement);
    }
}
