// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/relationship.rs
// Purpose : Evidence relationship model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence relationship model.

use oo_core::EvidenceId;

/// Relationship between evidence records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceRelationKind {
    /// One record supports another.
    Supports,
    /// One record contradicts another.
    Contradicts,
    /// One record reproduces another.
    Reproduces,
}

/// Evidence relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceRelation {
    from: EvidenceId,
    to: EvidenceId,
    kind: EvidenceRelationKind,
}

impl EvidenceRelation {
    /// Creates an evidence relationship.
    #[must_use]
    pub const fn new(from: EvidenceId, to: EvidenceId, kind: EvidenceRelationKind) -> Self {
        Self { from, to, kind }
    }

    /// Returns the source evidence id.
    #[must_use]
    pub const fn from(&self) -> EvidenceId {
        self.from
    }

    /// Returns the target evidence id.
    #[must_use]
    pub const fn to(&self) -> EvidenceId {
        self.to
    }

    /// Returns the relationship kind.
    #[must_use]
    pub const fn kind(&self) -> EvidenceRelationKind {
        self.kind
    }
}
