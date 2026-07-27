// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-discovery/src/event.rs
// Purpose : Implement the event module for oo-discovery.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Discovery event model.

use oo_evidence::{EvidenceRecord, ReproductionStatus};

/// A normalized event derived from one evidence record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryEvent {
    subject: String,
    digest_hex: String,
    reproduction: ReproductionStatus,
}

impl DiscoveryEvent {
    /// Creates a discovery event.
    #[must_use]
    pub fn new(
        subject: impl Into<String>,
        digest_hex: impl Into<String>,
        reproduction: ReproductionStatus,
    ) -> Self {
        Self {
            subject: subject.into(),
            digest_hex: digest_hex.into(),
            reproduction,
        }
    }

    /// Creates an event from an evidence record.
    #[must_use]
    pub fn from_evidence(record: &EvidenceRecord) -> Self {
        Self::new(
            record.subject(),
            record.digest().to_hex(),
            record.reproduction(),
        )
    }

    /// Returns the event subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the evidence digest in hex.
    #[must_use]
    pub fn digest_hex(&self) -> &str {
        &self.digest_hex
    }

    /// Returns the reproduction status.
    #[must_use]
    pub const fn reproduction(&self) -> ReproductionStatus {
        self.reproduction
    }
}
