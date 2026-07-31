// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/dex.rs
// Purpose : Liquidity and price observations from decentralized exchanges.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Liquidity and price observations from decentralized exchanges.
//!
//! A DEX quote is fundamentally different evidence from a centralized price
//! feed: it is derived from an on-chain pool an observer could in principle
//! re-derive from the same block, and it carries a liquidity figure that says
//! how much weight the price deserves. A quote from a pool with negligible
//! liquidity is not comparable evidence to one from a deep pool, so this
//! module keeps liquidity attached to every quote rather than discarding it.

use crate::attribution::Attribution;

/// One DEX pool's quote for an asset pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DexQuote {
    /// Attribution for this quote.
    pub attribution: Attribution,
    /// Pool contract address, when known.
    pub pool_address: Option<String>,
    /// Base asset symbol or address.
    pub base_asset: String,
    /// Quote asset symbol or address.
    pub quote_asset: String,
    /// Raw decimal price of one base unit in quote units.
    pub raw_price: String,
    /// Raw decimal liquidity figure, in the quote asset, when reported.
    pub raw_liquidity: Option<String>,
}

impl DexQuote {
    /// Returns whether the pool's liquidity meets a minimum threshold.
    ///
    /// A quote with unknown liquidity does not meet any threshold: absence of
    /// evidence is not evidence of sufficiency.
    #[must_use]
    pub fn meets_liquidity_threshold(&self, minimum: f64) -> bool {
        self.raw_liquidity
            .as_deref()
            .and_then(|value| value.parse::<f64>().ok())
            .is_some_and(|liquidity| liquidity >= minimum)
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};

    use super::*;
    use crate::capability::ProviderCategory;

    fn quote(liquidity: Option<&str>) -> DexQuote {
        DexQuote {
            attribution: Attribution::new(
                ProviderId::new(),
                ProviderCategory::Dex,
                "test",
                &ManualClock::new(UNIX_EPOCH),
            ),
            pool_address: Some("0xpool".to_owned()),
            base_asset: "USDT".to_owned(),
            quote_asset: "WETH".to_owned(),
            raw_price: "0.0003".to_owned(),
            raw_liquidity: liquidity.map(str::to_owned),
        }
    }

    #[test]
    fn sufficient_liquidity_meets_the_threshold() {
        assert!(quote(Some("1000000")).meets_liquidity_threshold(500_000.0));
    }

    #[test]
    fn insufficient_liquidity_does_not_meet_the_threshold() {
        assert!(!quote(Some("100")).meets_liquidity_threshold(500_000.0));
    }

    #[test]
    fn unknown_liquidity_never_meets_a_threshold() {
        assert!(!quote(None).meets_liquidity_threshold(0.0));
    }
}
