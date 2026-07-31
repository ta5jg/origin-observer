// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/source.rs
// Purpose : Name the provenance of a historical record.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Name the provenance of a historical record.
//!
//! WDRP does not accept an unsourced claim as evidence, and a historical case
//! study is no exception: every timeline entry and case study carries the
//! source it rests on, even when that source is only a short description
//! rather than a URL.

/// The provenance of a historical claim.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HistoricalSource {
    description: String,
    url: Option<String>,
}

impl HistoricalSource {
    /// Names a source by description.
    #[must_use]
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            url: None,
        }
    }

    /// Attaches a URL to the source.
    #[must_use]
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Returns the source's description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the source's URL, when recorded.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// Returns whether the source names anything at all.
    #[must_use]
    pub fn is_named(&self) -> bool {
        !self.description.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_source_with_only_whitespace_is_not_named() {
        assert!(!HistoricalSource::new("   ").is_named());
    }

    #[test]
    fn a_source_with_a_description_is_named() {
        assert!(HistoricalSource::new("project changelog").is_named());
    }

    #[test]
    fn a_url_can_be_attached_after_construction() {
        let source = HistoricalSource::new("explorer").with_url("https://example.invalid");
        assert_eq!(source.url(), Some("https://example.invalid"));
    }
}
