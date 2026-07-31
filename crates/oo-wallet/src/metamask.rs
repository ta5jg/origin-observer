// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/metamask.rs
// Purpose : MetaMask adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! MetaMask adapter.
//!
//! MetaMask is the reference EIP-1193 implementation and added EIP-6963
//! multi-provider announcement support; both are publicly documented in
//! MetaMask's own developer documentation.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// MetaMask wallet adapter.
pub struct MetaMask;

impl WalletAdapter for MetaMask {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("metamask", "MetaMask")
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
    fn metamask_is_page_observable() {
        assert!(MetaMask.capability().is_page_observable());
    }
}
