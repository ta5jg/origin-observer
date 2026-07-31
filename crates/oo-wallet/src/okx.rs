// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/okx.rs
// Purpose : OKX Wallet adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! OKX Wallet adapter.
//!
//! OKX Wallet ships a browser extension and mobile application, both
//! injecting an EIP-1193-compatible provider, per its public documentation.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// OKX Wallet adapter.
pub struct OkxWallet;

impl WalletAdapter for OkxWallet {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("okx-wallet", "OKX Wallet")
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
    fn okx_wallet_identifies_itself() {
        assert_eq!(OkxWallet.identity().display_name, "OKX Wallet");
    }
}
