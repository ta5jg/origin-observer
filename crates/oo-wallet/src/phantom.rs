// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/phantom.rs
// Purpose : Phantom adapter.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Phantom adapter.
//!
//! Phantom began as a Solana-only wallet and later added Ethereum support,
//! injecting an EIP-1193-compatible provider for EVM chains alongside its
//! Solana provider, per its public documentation. Only its EVM-facing
//! capability is modeled here; this project's configured chains are EVM and
//! TRON, so Phantom's Solana provider is out of scope for what this crate
//! observes.

use crate::adapter::WalletAdapter;
use crate::capability::WalletApiCapability;
use crate::model::WalletIdentity;
use crate::platform::WalletPlatform;

/// Phantom adapter.
pub struct Phantom;

impl WalletAdapter for Phantom {
    fn identity(&self) -> WalletIdentity {
        WalletIdentity::new("phantom", "Phantom")
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
    fn phantom_reports_evm_capability() {
        assert!(Phantom.capability().supports_eip1193);
    }
}
