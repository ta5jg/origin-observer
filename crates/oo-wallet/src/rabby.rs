// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/rabby.rs
// Purpose : Rabby Wallet adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Rabby Wallet adapter.
//!
//! Rabby is an EIP-1193 browser extension with a desktop application, per its
//! public documentation.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// Rabby wallet adapter.
pub struct Rabby;

impl WalletAdapter for Rabby {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("rabby", "Rabby Wallet")
    }

    fn capability(&self) -> WalletApiCapability {
        WalletApiCapability {
            platforms: vec![WalletPlatform::Extension, WalletPlatform::Desktop],
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
    fn rabby_supports_eip1193() {
        assert!(Rabby.capability().supports_eip1193);
    }
}
