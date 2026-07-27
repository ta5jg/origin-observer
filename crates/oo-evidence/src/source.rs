// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/source.rs
// Purpose : Evidence source model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence source model.

/// Evidence source classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EvidenceSourceKind {
    /// Source kind is unknown.
    #[default]
    Unknown,
    /// Source came from RPC.
    Rpc,
    /// Source came from a snapshot.
    Snapshot,
    /// Source came from a descriptor.
    Descriptor,
    /// Source came from manual research notes.
    Manual,
}

/// Evidence source descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceSource {
    kind: EvidenceSourceKind,
    locator: String,
}

impl EvidenceSource {
    /// Creates an evidence source.
    #[must_use]
    pub fn new(kind: EvidenceSourceKind, locator: impl Into<String>) -> Self {
        Self {
            kind,
            locator: locator.into(),
        }
    }

    /// Returns the source kind.
    #[must_use]
    pub const fn kind(&self) -> EvidenceSourceKind {
        self.kind
    }

    /// Returns the source locator.
    #[must_use]
    pub fn locator(&self) -> &str {
        &self.locator
    }
}
