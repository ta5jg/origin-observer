// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/trust_wallet.rs
// Purpose : Trust Wallet adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Trust Wallet adapter.
//!
//! Trust Wallet is a mobile-first wallet that also ships a browser extension
//! injecting an EIP-1193-compatible provider, per its public developer
//! documentation.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// Trust Wallet adapter.
pub struct TrustWallet;

impl WalletAdapter for TrustWallet {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("trust-wallet", "Trust Wallet")
    }

    fn capability(&self) -> WalletApiCapability {
        WalletApiCapability {
            platforms: vec![WalletPlatform::Mobile, WalletPlatform::Extension],
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
    fn trust_wallet_ships_on_mobile_first() {
        assert_eq!(
            TrustWallet.capability().platforms.first(),
            Some(&WalletPlatform::Mobile)
        );
    }
}
