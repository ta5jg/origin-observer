// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-provider/src/image.rs
// Purpose : Asset image and logo references.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Asset image and logo references.
//!
//! Wallets frequently decide "does this asset look legitimate" from whether a
//! logo is present at all, which makes the image URI's scheme part of the
//! evidence: an `ipfs://` reference resolves differently than an `https://`
//! one, and a wallet's willingness to fetch either is itself a discovery
//! signal, not a display detail.

use crate::attribution::Attribution;

/// Scheme an image reference uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageScheme {
    /// `https://`.
    Https,
    /// `ipfs://`.
    Ipfs,
    /// A scheme this module does not classify further.
    Other,
}

/// One provider's answer for an asset's image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageReference {
    /// Attribution for this answer.
    pub attribution: Attribution,
    /// The reference URI as reported.
    pub uri: String,
}

impl ImageReference {
    /// Creates an image reference.
    #[must_use]
    pub fn new(attribution: Attribution, uri: impl Into<String>) -> Self {
        Self {
            attribution,
            uri: uri.into(),
        }
    }

    /// Classifies the reference's scheme.
    #[must_use]
    pub fn scheme(&self) -> ImageScheme {
        if self.uri.starts_with("https://") {
            ImageScheme::Https
        } else if self.uri.starts_with("ipfs://") {
            ImageScheme::Ipfs
        } else {
            ImageScheme::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use oo_core::{ManualClock, ProviderId};

    use super::*;
    use crate::capability::ProviderCategory;

    fn attribution() -> Attribution {
        Attribution::new(
            ProviderId::new(),
            ProviderCategory::Image,
            "test",
            &ManualClock::new(UNIX_EPOCH),
        )
    }

    #[test]
    fn schemes_are_classified() {
        assert_eq!(
            ImageReference::new(attribution(), "https://example.org/logo.png").scheme(),
            ImageScheme::Https
        );
        assert_eq!(
            ImageReference::new(attribution(), "ipfs://Qm...").scheme(),
            ImageScheme::Ipfs
        );
        assert_eq!(
            ImageReference::new(attribution(), "data:image/png;base64,...").scheme(),
            ImageScheme::Other
        );
    }
}
