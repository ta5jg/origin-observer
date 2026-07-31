// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/generic.rs
// Purpose : Generic standards-only wallet: the control case.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Generic standards-only wallet: the control case.
//!
//! `config/wallets.toml` calls this entry the control case: a client with no
//! curated list, no cache and no vendor-specific behavior. A difference
//! between a named wallet and this baseline is the named wallet's own
//! contribution, which is the reason WDRP records this adapter at all rather
//! than treating "no adapter" as equivalent to "generic."

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// Generic standards-only wallet adapter.
pub struct GenericWallet;

impl WalletAdapter for GenericWallet {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("generic", "Generic Standards-Only Client")
    }

    fn capability(&self) -> WalletApiCapability {
        WalletApiCapability {
            platforms: vec![WalletPlatform::Web],
            injects_window_ethereum: true,
            supports_eip1193: true,
            supports_eip6963: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_generic_client_supports_only_the_base_standard() {
        let capability = GenericWallet.capability();
        assert!(capability.supports_eip1193);
        assert!(
            !capability.supports_eip6963,
            "no vendor-specific extensions"
        );
    }
}
