// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/asset_case_study.rs
// Purpose : A documented historical investigation of one asset.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A documented historical investigation of one asset.
//!
//! A case study ties a research question to the timelines that answer it —
//! how wallets came to recognize the asset, and how provider metadata
//! availability changed alongside it — and a narrative explaining what the
//! timelines show.

use crate::provider_timeline::ProviderTimeline;
use crate::recognition_timeline::RecognitionTimeline;

/// A historical case study for one asset.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AssetCaseStudy {
    question_id: String,
    narrative: String,
    recognition_timeline: RecognitionTimeline,
    provider_timeline: ProviderTimeline,
}

impl AssetCaseStudy {
    /// Opens a case study for a permanent research question.
    #[must_use]
    pub fn new(question_id: impl Into<String>) -> Self {
        Self {
            question_id: question_id.into(),
            narrative: String::new(),
            recognition_timeline: RecognitionTimeline::new(),
            provider_timeline: ProviderTimeline::new(),
        }
    }

    /// Returns the research question this case study addresses.
    #[must_use]
    pub fn question_id(&self) -> &str {
        &self.question_id
    }

    /// Sets the narrative explaining what the timelines show.
    pub fn set_narrative(&mut self, narrative: impl Into<String>) {
        self.narrative = narrative.into();
    }

    /// Returns the narrative.
    #[must_use]
    pub fn narrative(&self) -> &str {
        &self.narrative
    }

    /// Returns the wallet-recognition timeline.
    #[must_use]
    pub const fn recognition_timeline(&self) -> &RecognitionTimeline {
        &self.recognition_timeline
    }

    /// Returns a mutable reference to the wallet-recognition timeline.
    pub const fn recognition_timeline_mut(&mut self) -> &mut RecognitionTimeline {
        &mut self.recognition_timeline
    }

    /// Returns the provider metadata-availability timeline.
    #[must_use]
    pub const fn provider_timeline(&self) -> &ProviderTimeline {
        &self.provider_timeline
    }

    /// Returns a mutable reference to the provider metadata-availability
    /// timeline.
    pub const fn provider_timeline_mut(&mut self) -> &mut ProviderTimeline {
        &mut self.provider_timeline
    }

    /// Returns whether the case study has a research question and a
    /// narrative: a case study with an empty narrative documents nothing.
    #[must_use]
    pub fn is_documented(&self) -> bool {
        !self.question_id.trim().is_empty() && !self.narrative.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_case_study_is_not_yet_documented() {
        let case_study = AssetCaseStudy::new("RQ-0006");
        assert!(!case_study.is_documented());
    }

    #[test]
    fn a_case_study_with_a_narrative_is_documented() {
        let mut case_study = AssetCaseStudy::new("RQ-0006");
        case_study.set_narrative("USDT was recognized across five wallets by 2022.");
        assert!(case_study.is_documented());
    }

    #[test]
    fn timelines_can_be_populated_after_construction() {
        let mut case_study = AssetCaseStudy::new("RQ-0006");
        assert!(case_study.recognition_timeline().entries().is_empty());
        case_study.recognition_timeline_mut();
        assert!(case_study.provider_timeline().entries().is_empty());
    }
}
