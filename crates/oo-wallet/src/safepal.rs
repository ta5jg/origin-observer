// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/safepal.rs
// Purpose : SafePal adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! SafePal adapter.
//!
//! SafePal ships mobile and extension software wallets and a hardware
//! device, with the extension injecting an EIP-1193-compatible provider, per
//! its public documentation.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// SafePal adapter.
pub struct SafePal;

impl WalletAdapter for SafePal {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("safepal", "SafePal")
    }

    fn capability(&self) -> WalletApiCapability {
        WalletApiCapability {
            platforms: vec![
                WalletPlatform::Mobile,
                WalletPlatform::Extension,
                WalletPlatform::Hardware,
            ],
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
    fn safepal_covers_software_and_hardware_platforms() {
        assert!(SafePal
            .capability()
            .platforms
            .contains(&WalletPlatform::Hardware));
    }
}
