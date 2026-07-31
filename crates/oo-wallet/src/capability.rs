// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/capability.rs
// Purpose : Publicly documented API surface a wallet client exposes.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Publicly documented API surface a wallet client exposes.
//!
//! Every field here is a documented standards-compliance fact about the
//! software, not an inferred or reverse-engineered behavior: whether a client
//! injects an EIP-1193 provider, whether it announces itself under EIP-6963,
//! which platforms it ships on. This is the boundary the roadmap's "without
//! unexplained wallet-specific hacks" rule draws: declaring what a wallet
//! publicly claims to support is data; branching research logic on a wallet's
//! identity without a stated reason would be a hack.

use crate::platform::WalletPlatform;

/// Documented API surface for one wallet client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletApiCapability {
    /// Platforms this client ships on.
    pub platforms: Vec<WalletPlatform>,
    /// Whether the client injects an `window.ethereum`-shaped provider into
    /// the page on the extension/web platforms.
    pub injects_window_ethereum: bool,
    /// Whether the injected provider follows EIP-1193 (the standard request
    /// method every modern browser wallet is expected to implement).
    pub supports_eip1193: bool,
    /// Whether the client announces itself via EIP-6963 (multi-wallet
    /// discovery), letting a page detect several installed wallets at once
    /// instead of only the one that claimed `window.ethereum` first.
    pub supports_eip6963: bool,
}

impl WalletApiCapability {
    /// Returns whether the client can be observed at all from a page context
    /// (that is, it injects something a page-level script can query), as
    /// opposed to a companion application reached only through a native
    /// bridge or WalletConnect.
    #[must_use]
    pub const fn is_page_observable(&self) -> bool {
        self.injects_window_ethereum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_injecting_client_is_page_observable() {
        let capability = WalletApiCapability {
            platforms: vec![WalletPlatform::Extension],
            injects_window_ethereum: true,
            supports_eip1193: true,
            supports_eip6963: true,
        };
        assert!(capability.is_page_observable());
    }

    #[test]
    fn a_companion_app_is_not_page_observable() {
        let capability = WalletApiCapability {
            platforms: vec![WalletPlatform::Desktop, WalletPlatform::Hardware],
            injects_window_ethereum: false,
            supports_eip1193: false,
            supports_eip6963: false,
        };
        assert!(!capability.is_page_observable());
    }
}
