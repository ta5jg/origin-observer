// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/wallet_view.rs
// Purpose : Interpret a discovery outcome from one wallet's point of view.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Interpret a discovery outcome from one wallet's point of view.
//!
//! Discovery logic itself does not branch on which wallet is asking — that
//! is the roadmap's "no unexplained wallet-specific hacks" rule. What a
//! wallet adapter contributes here is only its identity and its documented
//! capability: whether it can even be observed from a page context, which
//! bears on whether a "would display" claim can be attributed to that
//! specific client at all versus to generic discovery evidence.

use oo_discovery::DiscoveryDecision;
use oo_model::cache::CacheState;
use oo_wallet::WalletAdapter;

/// How a discovery outcome reads for one specific wallet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletDisplayView {
    /// The wallet's stable configuration identifier.
    pub wallet_config_id: String,
    /// The wallet's human-readable display name.
    pub wallet_display_name: String,
    /// Whether the discovery decision implies the asset would be displayed.
    pub would_display: bool,
    /// Whether this view can be cited as evidence about this specific
    /// wallet, rather than only about discovery logic in general.
    pub citable_for_this_wallet: bool,
    /// Plain-language explanation of the `citable_for_this_wallet` verdict.
    pub rationale: String,
}

/// Builds a wallet-specific view of a discovery decision.
///
/// `cache_state` reflects whether the *investigation's own* observation was
/// cache-attributable (see [`crate::investigation::InvestigationRecord`]);
/// it is a property of the run, not of the wallet, since discovery evidence
/// is gathered independently of any particular wallet client.
#[must_use]
pub fn evaluate(
    adapter: &dyn WalletAdapter,
    decision: DiscoveryDecision,
    cache_state: CacheState,
) -> WalletDisplayView {
    let identity = adapter.identity();
    let would_display = matches!(decision, DiscoveryDecision::Accept);

    let (citable_for_this_wallet, rationale) =
        if matches!(cache_state, CacheState::Warm | CacheState::Stale) {
            (
            false,
            "the observation's cache state was warm or stale, so this result may reflect a prior \
             session rather than fresh discovery evidence"
                .to_owned(),
        )
        } else if !adapter.capability().is_page_observable() {
            (
                false,
                format!(
                "{} is not page-observable (no window.ethereum-shaped injection); this view comes \
                 from generic discovery evidence, not a client-specific check",
                identity.display_name
            ),
            )
        } else {
            (
                true,
                format!("derived from generic discovery evidence: {decision:?}"),
            )
        };

    WalletDisplayView {
        wallet_config_id: identity.config_id,
        wallet_display_name: identity.display_name,
        would_display,
        citable_for_this_wallet,
        rationale,
    }
}

#[cfg(test)]
mod tests {
    use oo_wallet::find_adapter;

    use super::*;

    #[test]
    fn an_accepted_decision_for_a_page_observable_wallet_is_citable_and_displays() {
        let adapter = find_adapter("metamask").expect("metamask is built in");
        let view = evaluate(
            adapter.as_ref(),
            DiscoveryDecision::Accept,
            CacheState::Empty,
        );
        assert!(view.would_display);
        assert!(view.citable_for_this_wallet);
        assert_eq!(view.wallet_config_id, "metamask");
    }

    #[test]
    fn a_rejected_decision_does_not_display_regardless_of_cache_state() {
        let adapter = find_adapter("metamask").expect("metamask is built in");
        let view = evaluate(
            adapter.as_ref(),
            DiscoveryDecision::Reject,
            CacheState::Empty,
        );
        assert!(!view.would_display);
    }

    #[test]
    fn a_warm_cache_observation_is_not_citable_even_for_a_page_observable_wallet() {
        let adapter = find_adapter("metamask").expect("metamask is built in");
        let view = evaluate(
            adapter.as_ref(),
            DiscoveryDecision::Accept,
            CacheState::Warm,
        );
        assert!(!view.citable_for_this_wallet);
    }

    #[test]
    fn a_non_page_observable_wallet_view_is_not_citable_for_that_wallet_specifically() {
        let adapter = find_adapter("ledger-live").expect("ledger-live is built in");
        assert!(!adapter.capability().is_page_observable());
        let view = evaluate(
            adapter.as_ref(),
            DiscoveryDecision::Accept,
            CacheState::Empty,
        );
        assert!(!view.citable_for_this_wallet);
    }
}
