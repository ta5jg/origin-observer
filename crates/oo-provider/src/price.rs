// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/price.rs
// Purpose : Price quotes from market-data providers.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Price quotes from market-data providers.
//!
//! A price is not consensus data; it is one provider's momentary claim. The
//! raw decimal string a provider returned is kept alongside any numeric
//! conversion, so a caller can always cite exactly what was reported rather
//! than a value that has already been through floating-point rounding.

/// One provider's price quote for an asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriceQuote {
    /// Attribution for this quote.
    pub attribution: crate::attribution::Attribution,
    /// Quote currency, e.g. `"usd"`.
    pub currency: String,
    /// Raw decimal value exactly as reported.
    pub raw_value: String,
}

impl PriceQuote {
    /// Creates a price quote.
    #[must_use]
    pub fn new(
        attribution: crate::attribution::Attribution,
        currency: impl Into<String>,
        raw_value: impl Into<String>,
    ) -> Self {
        Self {
            attribution,
            currency: currency.into(),
            raw_value: raw_value.into(),
        }
    }

    /// Parses the raw value as `f64`, when it is well-formed.
    ///
    /// The parsed value is for display and rough comparison only; it must
    /// never be treated as more precise than the raw string it came from.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        self.raw_value.parse().ok()
    }
}

/// Returns the quotes whose currency-normalized values disagree beyond a
/// stated tolerance, expressed as a fraction (`0.02` = 2%).
///
/// Returns quote pairs rather than a single "the price," because a genuine
/// disagreement between providers about an asset's price is itself
/// observable evidence about how well-covered that asset is.
#[must_use]
pub fn disagreements(quotes: &[PriceQuote], tolerance: f64) -> Vec<(usize, usize)> {
    let values: Vec<Option<f64>> = quotes.iter().map(PriceQuote::as_f64).collect();
    let mut pairs = Vec::new();
    for i in 0..quotes.len() {
        for j in (i + 1)..quotes.len() {
            if quotes[i].currency != quotes[j].currency {
                continue;
            }
            let (Some(a), Some(b)) = (values[i], values[j]) else {
                continue;
            };
            if a == 0.0 && b == 0.0 {
                continue;
            }
            let base = a.abs().max(b.abs());
            if base > 0.0 && (a - b).abs() / base > tolerance {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};

    use super::*;
    use crate::attribution::Attribution;
    use crate::capability::ProviderCategory;

    fn quote(currency: &str, value: &str) -> PriceQuote {
        PriceQuote::new(
            Attribution::new(
                ProviderId::new(),
                ProviderCategory::Price,
                "test",
                &ManualClock::new(UNIX_EPOCH),
            ),
            currency,
            value,
        )
    }

    #[test]
    fn a_well_formed_value_parses() {
        assert_eq!(quote("usd", "1.0005").as_f64(), Some(1.0005));
    }

    #[test]
    fn a_malformed_value_does_not_parse() {
        assert_eq!(quote("usd", "not-a-number").as_f64(), None);
    }

    #[test]
    fn agreeing_quotes_report_no_disagreement() {
        let quotes = vec![quote("usd", "1.00"), quote("usd", "1.001")];
        assert!(disagreements(&quotes, 0.05).is_empty());
    }

    #[test]
    fn a_large_divergence_is_reported() {
        let quotes = vec![quote("usd", "1.00"), quote("usd", "1.50")];
        assert_eq!(disagreements(&quotes, 0.05), vec![(0, 1)]);
    }

    #[test]
    fn quotes_in_different_currencies_are_not_compared() {
        let quotes = vec![quote("usd", "1.00"), quote("eur", "1.50")];
        assert!(disagreements(&quotes, 0.05).is_empty());
    }
}
