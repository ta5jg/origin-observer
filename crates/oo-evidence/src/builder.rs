// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/builder.rs
// Purpose : Evidence builder.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence builder.

use crate::integrity::evidence_digest;
use crate::model::EvidenceRecord;
use crate::source::{EvidenceSource, EvidenceSourceKind};

/// Builder for evidence records.
#[derive(Debug, Clone)]
pub struct EvidenceBuilder {
    source: EvidenceSource,
    subject: String,
    bytes: Vec<u8>,
}

impl EvidenceBuilder {
    /// Creates a builder for a source and subject.
    #[must_use]
    pub fn new(
        kind: EvidenceSourceKind,
        locator: impl Into<String>,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            source: EvidenceSource::new(kind, locator),
            subject: subject.into(),
            bytes: Vec::new(),
        }
    }

    /// Adds source bytes for integrity hashing.
    #[must_use]
    pub fn bytes(mut self, bytes: impl Into<Vec<u8>>) -> Self {
        self.bytes = bytes.into();
        self
    }

    /// Builds the evidence record.
    #[must_use]
    pub fn build(self) -> EvidenceRecord {
        let digest = evidence_digest(&self.bytes);
        EvidenceRecord::new(self.source, self.subject, digest)
    }
}
