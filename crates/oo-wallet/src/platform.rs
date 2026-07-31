// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/platform.rs
// Purpose : Platform a wallet client runs on.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Platform a wallet client runs on.
//!
//! The same wallet can present a different asset list on an extension and on
//! mobile, so platform is part of every observation's identity, not a display
//! detail. This mirrors `config/wallets.toml`'s platform list without
//! depending on `oo-config`, for the same reason `oo-provider` does not
//! depend on it: the two crates serve different layers and coupling them
//! would make a config schema change ripple into every adapter.

/// Platform a wallet client runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WalletPlatform {
    /// Browser extension.
    Extension,
    /// Mobile application.
    Mobile,
    /// Desktop application.
    Desktop,
    /// Hardware wallet companion application.
    Hardware,
    /// Web application.
    Web,
}

impl WalletPlatform {
    /// Returns the lowercase name used in configuration and reports.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::Mobile => "mobile",
            Self::Desktop => "desktop",
            Self::Hardware => "hardware",
            Self::Web => "web",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_names_are_lowercase() {
        assert_eq!(WalletPlatform::Extension.as_str(), "extension");
        assert_eq!(WalletPlatform::Hardware.as_str(), "hardware");
    }
}
