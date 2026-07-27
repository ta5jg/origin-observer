// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/model.rs
// Purpose : Evidence model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence model.

use oo_core::{Digest, EvidenceId};

use crate::reproduction::ReproductionStatus;
use crate::source::EvidenceSource;

/// Research evidence record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    id: EvidenceId,
    source: EvidenceSource,
    subject: String,
    digest: Digest,
    reproduction: ReproductionStatus,
}

impl EvidenceRecord {
    /// Creates an evidence record.
    #[must_use]
    pub fn new(source: EvidenceSource, subject: impl Into<String>, digest: Digest) -> Self {
        Self {
            id: EvidenceId::new(),
            source,
            subject: subject.into(),
            digest,
            reproduction: ReproductionStatus::Observed,
        }
    }

    /// Returns the evidence identifier.
    #[must_use]
    pub const fn id(&self) -> EvidenceId {
        self.id
    }

    /// Returns the evidence source.
    #[must_use]
    pub const fn source(&self) -> &EvidenceSource {
        &self.source
    }

    /// Returns the evidence subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the integrity digest.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.digest
    }

    /// Returns the reproduction status.
    #[must_use]
    pub const fn reproduction(&self) -> ReproductionStatus {
        self.reproduction
    }

    /// Changes the reproduction status.
    pub const fn set_reproduction(&mut self, reproduction: ReproductionStatus) {
        self.reproduction = reproduction;
    }
}
