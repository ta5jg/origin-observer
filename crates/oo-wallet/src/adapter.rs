// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/adapter.rs
// Purpose : Common interface every wallet-specific module implements.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Common interface every wallet-specific module implements.
//!
//! An adapter supplies only identity and documented capability — never
//! discovery logic. Discovery itself lives in `oo-discovery` and works the
//! same way regardless of which wallet is being observed; an adapter's entire
//! job is to say which wallet this is and what it is documented to support,
//! so that generic discovery logic can be applied without a per-wallet
//! branch.

use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;

/// A wallet client's identity and documented capability.
pub trait WalletAdapter {
    /// Returns the wallet's identity.
    fn identity(&self) -> WalletIdentity;

    /// Returns the wallet's documented API capability.
    fn capability(&self) -> WalletApiCapability;
}

/// Returns every built-in adapter, in the order declared.
#[must_use]
pub fn built_in_adapters() -> Vec<Box<dyn WalletAdapter>> {
    vec![
        Box::new(crate::metamask::MetaMask),
        Box::new(crate::trust_wallet::TrustWallet),
        Box::new(crate::rabby::Rabby),
        Box::new(crate::coinbase::CoinbaseWallet),
        Box::new(crate::safepal::SafePal),
        Box::new(crate::okx::OkxWallet),
        Box::new(crate::ledger_live::LedgerLive),
        Box::new(crate::phantom::Phantom),
        Box::new(crate::generic::GenericWallet),
    ]
}

/// Finds a built-in adapter by its configuration identifier.
#[must_use]
pub fn find_adapter(config_id: &str) -> Option<Box<dyn WalletAdapter>> {
    built_in_adapters()
        .into_iter()
        .find(|adapter| adapter.identity().config_id == config_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_built_in_adapter_has_a_unique_config_id() {
        let adapters = built_in_adapters();
        let mut ids: Vec<String> = adapters
            .iter()
            .map(|adapter| adapter.identity().config_id)
            .collect();
        let before = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), before);
    }

    #[test]
    fn find_adapter_locates_a_known_wallet() {
        let adapter = find_adapter("metamask").expect("metamask must be built in");
        assert_eq!(adapter.identity().display_name, "MetaMask");
    }

    #[test]
    fn find_adapter_returns_none_for_an_unknown_wallet() {
        assert!(find_adapter("not-a-real-wallet").is_none());
    }

    #[test]
    fn every_adapter_matches_the_declared_wallet_count() {
        // The nine wallets the roadmap names, plus the generic control case.
        assert_eq!(built_in_adapters().len(), 9);
    }
}
