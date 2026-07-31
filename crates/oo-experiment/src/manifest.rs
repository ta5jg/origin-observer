// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/manifest.rs
// Purpose : Summarize a set of experiments for publication.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Summarize a set of experiments for publication.
//!
//! A manifest is the citable index for a batch of experiments: which research
//! questions were addressed, how many designs, and how many reached each
//! verdict. It does not carry the full designs — those belong in the export
//! this manifest indexes.

use std::collections::BTreeMap;

use crate::verification::Verdict;

/// One entry in a manifest: a research question and its verdict counts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManifestEntry {
    /// Research question addressed.
    pub question_id: String,
    /// Number of designs reaching each verdict kind.
    pub supported: usize,
    /// Number of rejected designs.
    pub rejected: usize,
    /// Number of pending designs.
    pub pending: usize,
    /// Number of designs that could not be verified (unfalsifiable).
    pub unverifiable: usize,
}

/// A manifest summarizing verdicts across a batch of experiments.
#[derive(Debug, Clone, Default)]
pub struct ExperimentManifest {
    entries: BTreeMap<String, ManifestEntry>,
}

impl ExperimentManifest {
    /// Records one experiment's verdict against its research question.
    pub fn record(&mut self, question_id: impl Into<String>, verdict: Option<Verdict>) {
        let question_id = question_id.into();
        let entry = self
            .entries
            .entry(question_id.clone())
            .or_insert_with(|| ManifestEntry {
                question_id,
                ..ManifestEntry::default()
            });
        match verdict {
            Some(Verdict::Supported(_)) => entry.supported += 1,
            Some(Verdict::Rejected) => entry.rejected += 1,
            Some(Verdict::Pending) => entry.pending += 1,
            None => entry.unverifiable += 1,
        }
    }

    /// Returns every entry, ordered by research question.
    #[must_use]
    pub fn entries(&self) -> Vec<&ManifestEntry> {
        self.entries.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use oo_evidence::ReproductionStatus;

    use super::*;

    #[test]
    fn verdicts_are_tallied_per_question() {
        let mut manifest = ExperimentManifest::default();
        manifest.record(
            "RQ-0005",
            Some(Verdict::Supported(ReproductionStatus::Reproduced)),
        );
        manifest.record("RQ-0005", Some(Verdict::Rejected));
        manifest.record("RQ-0006", None);

        let entries = manifest.entries();
        let rq5 = entries
            .iter()
            .find(|entry| entry.question_id == "RQ-0005")
            .unwrap();
        assert_eq!(rq5.supported, 1);
        assert_eq!(rq5.rejected, 1);

        let rq6 = entries
            .iter()
            .find(|entry| entry.question_id == "RQ-0006")
            .unwrap();
        assert_eq!(rq6.unverifiable, 1);
    }
}
