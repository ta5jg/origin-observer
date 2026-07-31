// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/logo.rs
// Purpose : Score whether an asset has an image signal available.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Score whether an asset has an image signal available.
//!
//! Several wallets gate default display on a logo being present in a
//! registry they trust; a missing logo is therefore not cosmetic, it is a
//! discovery input. This module reports presence and scheme rather than
//! fetching or validating the image itself, which is `oo-provider`'s
//! concern.

use oo_provider::{ImageReference, ImageScheme};

/// Image availability signal for one asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogoSignal {
    /// Whether any provider reported an image reference at all.
    pub present: bool,
    /// Whether at least one reference uses a scheme this module recognizes
    /// (`https://` or `ipfs://`), which most wallets can actually resolve.
    pub resolvable_scheme: bool,
}

impl LogoSignal {
    /// Evaluates the signal from every image reference gathered for an asset.
    #[must_use]
    pub fn evaluate(references: &[ImageReference]) -> Self {
        Self {
            present: !references.is_empty(),
            resolvable_scheme: references
                .iter()
                .any(|reference| !matches!(reference.scheme(), ImageScheme::Other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};
    use oo_provider::{Attribution, ProviderCategory};

    use super::*;

    fn reference(uri: &str) -> ImageReference {
        ImageReference::new(
            Attribution::new(
                ProviderId::new(),
                ProviderCategory::Image,
                "test",
                &ManualClock::new(UNIX_EPOCH),
            ),
            uri,
        )
    }

    #[test]
    fn no_references_means_no_signal() {
        let signal = LogoSignal::evaluate(&[]);
        assert!(!signal.present);
        assert!(!signal.resolvable_scheme);
    }

    #[test]
    fn a_resolvable_reference_is_recognized() {
        let signal = LogoSignal::evaluate(&[reference("https://example.org/logo.png")]);
        assert!(signal.present);
        assert!(signal.resolvable_scheme);
    }

    #[test]
    fn a_present_but_unresolvable_reference_is_still_present() {
        let signal = LogoSignal::evaluate(&[reference("data:image/png;base64,x")]);
        assert!(signal.present);
        assert!(!signal.resolvable_scheme);
    }
}
