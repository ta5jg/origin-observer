// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/appendix.rs
// Purpose : Supplementary report material: reproduction steps and data.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Supplementary report material: reproduction steps and data.
//!
//! The roadmap names three kinds of supplementary material — appendices,
//! visualisation data, and reproduction instructions — that are all the same
//! shape: a titled block of content attached to a report. [`AppendixKind`]
//! distinguishes them without three near-identical types.

/// What kind of supplementary material an appendix holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendixKind {
    /// Steps a reader can follow to reproduce the investigation.
    ReproductionInstructions,
    /// Data intended for charting or visualisation, not for reading directly.
    VisualizationData,
    /// Any other supplementary material.
    Supplementary,
}

/// One titled block of supplementary report material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportAppendix {
    kind: AppendixKind,
    title: String,
    content: String,
}

impl ReportAppendix {
    /// Creates an appendix.
    #[must_use]
    pub fn new(kind: AppendixKind, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            kind,
            title: title.into(),
            content: content.into(),
        }
    }

    /// Returns the appendix kind.
    #[must_use]
    pub const fn kind(&self) -> AppendixKind {
        self.kind
    }

    /// Returns the appendix title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the appendix content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_appendix_carries_its_kind_title_and_content() {
        let appendix = ReportAppendix::new(
            AppendixKind::ReproductionInstructions,
            "How to reproduce",
            "pin block 18000000 and call eth_getStorageAt",
        );
        assert_eq!(appendix.kind(), AppendixKind::ReproductionInstructions);
        assert_eq!(appendix.title(), "How to reproduce");
        assert!(appendix.content().contains("eth_getStorageAt"));
    }
}
