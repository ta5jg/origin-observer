// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/registry.rs
// Purpose : Evidence registry.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence registry.

use std::collections::BTreeMap;

use oo_core::EvidenceId;

use crate::model::EvidenceRecord;

/// In-memory evidence registry.
#[derive(Debug, Clone, Default)]
pub struct EvidenceRegistry {
    records: BTreeMap<EvidenceId, EvidenceRecord>,
}

impl EvidenceRegistry {
    /// Inserts an evidence record.
    pub fn insert(&mut self, record: EvidenceRecord) -> Option<EvidenceRecord> {
        self.records.insert(record.id(), record)
    }

    /// Returns an evidence record.
    #[must_use]
    pub fn get(&self, id: EvidenceId) -> Option<&EvidenceRecord> {
        self.records.get(&id)
    }

    /// Returns the number of stored records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true when no evidence is registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
