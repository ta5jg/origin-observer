// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/model.rs
// Purpose : Wallet client identity.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Wallet client identity.

/// Identity of one wallet client under observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletIdentity {
    /// Stable identifier matching `config/wallets.toml`, e.g. `"metamask"`.
    pub config_id: String,
    /// Human-readable display name, e.g. `"MetaMask"`.
    pub display_name: String,
}

impl WalletIdentity {
    /// Creates a wallet identity.
    #[must_use]
    pub fn new(config_id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            config_id: config_id.into(),
            display_name: display_name.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_carries_both_the_config_id_and_the_display_name() {
        let identity = WalletIdentity::new("metamask", "MetaMask");
        assert_eq!(identity.config_id, "metamask");
        assert_eq!(identity.display_name, "MetaMask");
    }
}
