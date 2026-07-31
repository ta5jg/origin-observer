// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/unknown.rs
// Purpose : Record what an investigation could not determine.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Record what an investigation could not determine.
//!
//! WDRP requires failures to be explicit rather than silently absent from a
//! report. An unknown names the subject the investigation could not resolve
//! and why, so a reader sees the gap instead of assuming the investigation
//! covered everything it looked at.

/// One thing an investigation could not resolve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportUnknown {
    subject: String,
    reason: String,
}

impl ReportUnknown {
    /// Records an unresolved subject and why it could not be resolved.
    #[must_use]
    pub fn new(subject: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            reason: reason.into(),
        }
    }

    /// Returns the subject that could not be resolved.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns why the subject could not be resolved.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unknown_carries_both_its_subject_and_reason() {
        let unknown = ReportUnknown::new(
            "USDC on chain 137",
            "provider endpoint unreachable during the run",
        );
        assert_eq!(unknown.subject(), "USDC on chain 137");
        assert_eq!(
            unknown.reason(),
            "provider endpoint unreachable during the run"
        );
    }
}
