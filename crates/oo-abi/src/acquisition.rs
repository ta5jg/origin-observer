// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-abi/src/acquisition.rs
// Purpose : Record how an ABI fragment was obtained.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Record how an ABI fragment was obtained.
//!
//! Not every ABI is equally trustworthy. A verified source file published by
//! the deployer is a fact; a selector recovered from bytecode is a hypothesis
//! about what that selector might mean. Reporting a function name without
//! saying which of these it came from would let an inference read like a
//! verified fact, which is exactly what the WDRP constitution forbids.

/// How an ABI fragment's meaning was established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AcquisitionKind {
    /// Source code verified and published by an explorer, matched to the
    /// deployed bytecode.
    VerifiedSource,
    /// Metadata embedded in the deployment (Solidity's CBOR trailer) or
    /// published separately by the deployer.
    EmbeddedMetadata,
    /// A well-known standard interface (ERC-20, ERC-721, ...) matched by
    /// selector against a fixed catalog.
    StandardInterfaceMatch,
    /// A selector recovered from bytecode with no known meaning attached.
    UnknownSelector,
    /// Entered by a researcher from manual inspection.
    Manual,
}

impl AcquisitionKind {
    /// Returns whether this acquisition method establishes the fragment's
    /// meaning as a verified fact rather than a hypothesis.
    ///
    /// Only a source match against deployed bytecode, or metadata the
    /// deployer themselves published, counts as verified. A standard-interface
    /// match is a strong hypothesis — the selector matches, but nothing
    /// confirms the implementation actually behaves as the standard requires.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::VerifiedSource | Self::EmbeddedMetadata)
    }
}

/// Provenance of one ABI fragment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiAcquisition {
    /// How the fragment was obtained.
    pub kind: AcquisitionKind,
    /// Where it came from: an explorer URL, `"bytecode"`, or a researcher
    /// name for manual entries.
    pub locator: String,
}

impl AbiAcquisition {
    /// Creates an acquisition record.
    #[must_use]
    pub fn new(kind: AcquisitionKind, locator: impl Into<String>) -> Self {
        Self {
            kind,
            locator: locator.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_source_and_metadata_are_verified() {
        assert!(AcquisitionKind::VerifiedSource.is_verified());
        assert!(AcquisitionKind::EmbeddedMetadata.is_verified());
        assert!(!AcquisitionKind::StandardInterfaceMatch.is_verified());
        assert!(!AcquisitionKind::UnknownSelector.is_verified());
        assert!(!AcquisitionKind::Manual.is_verified());
    }
}
