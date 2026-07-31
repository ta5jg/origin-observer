// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/ledger_live.rs
// Purpose : Ledger Live adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Ledger Live adapter.
//!
//! Ledger Live is a desktop and mobile companion application for Ledger
//! hardware devices. Unlike a browser extension, it does not inject a
//! page-level provider; dApp connections go through WalletConnect or a native
//! bridge instead, per Ledger's public documentation. This is why its
//! capability differs structurally from every browser-extension wallet in
//! this module, not a special case invented for it.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// Ledger Live adapter.
pub struct LedgerLive;

impl WalletAdapter for LedgerLive {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("ledger-live", "Ledger Live")
    }

    fn capability(&self) -> WalletApiCapability {
        WalletApiCapability {
            platforms: vec![
                WalletPlatform::Desktop,
                WalletPlatform::Mobile,
                WalletPlatform::Hardware,
            ],
            injects_window_ethereum: false,
            supports_eip1193: false,
            supports_eip6963: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ledger_live_is_not_page_observable() {
        // A companion application reached through WalletConnect or a native
        // bridge has nothing for a page-level script to query directly.
        assert!(!LedgerLive.capability().is_page_observable());
    }
}
