// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/coinbase.rs
// Purpose : Coinbase Wallet adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Coinbase Wallet adapter.
//!
//! Coinbase Wallet ships a browser extension and mobile application, both
//! injecting an EIP-1193-compatible provider, per its public documentation.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// Coinbase Wallet adapter.
pub struct CoinbaseWallet;

impl WalletAdapter for CoinbaseWallet {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("coinbase-wallet", "Coinbase Wallet")
    }

    fn capability(&self) -> WalletApiCapability {
        WalletApiCapability {
            platforms: vec![WalletPlatform::Extension, WalletPlatform::Mobile],
            injects_window_ethereum: true,
            supports_eip1193: true,
            supports_eip6963: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coinbase_wallet_identifies_itself() {
        assert_eq!(CoinbaseWallet.identity().config_id, "coinbase-wallet");
    }
}
