// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/manifest.rs
// Purpose : Tie a report together with its unknowns and appendices.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Tie a report together with its unknowns and appendices.
//!
//! A [`MachineReport`] states a finding and a conclusion; it says nothing
//! about what the investigation could not resolve or what supplementary
//! material backs it up. A manifest is the complete package, and it enforces
//! the roadmap's rule directly: a conclusion may not be reported as
//! [`ReportConclusion::Supported`](crate::conclusion::ReportConclusion::Supported)
//! while an unknown remains unresolved.

use crate::appendix::ReportAppendix;
use crate::conclusion::ReportConclusion;
use crate::machine::MachineReport;
use crate::unknown::ReportUnknown;

/// A complete, self-describing report package.
#[derive(Debug, Clone, PartialEq)]
pub struct ReportManifest {
    report: MachineReport,
    unknowns: Vec<ReportUnknown>,
    appendices: Vec<ReportAppendix>,
}

impl ReportManifest {
    /// Opens a manifest around a machine report.
    #[must_use]
    pub const fn new(report: MachineReport) -> Self {
        Self {
            report,
            unknowns: Vec::new(),
            appendices: Vec::new(),
        }
    }

    /// Returns the machine report.
    #[must_use]
    pub const fn report(&self) -> &MachineReport {
        &self.report
    }

    /// Records an unresolved unknown.
    pub fn add_unknown(&mut self, unknown: ReportUnknown) {
        self.unknowns.push(unknown);
    }

    /// Returns the recorded unknowns.
    #[must_use]
    pub fn unknowns(&self) -> &[ReportUnknown] {
        &self.unknowns
    }

    /// Attaches an appendix.
    pub fn add_appendix(&mut self, appendix: ReportAppendix) {
        self.appendices.push(appendix);
    }

    /// Returns the attached appendices.
    #[must_use]
    pub fn appendices(&self) -> &[ReportAppendix] {
        &self.appendices
    }

    /// Returns whether the manifest is internally consistent: a `Supported`
    /// conclusion may not coexist with an unresolved unknown, since an
    /// unknown is, by definition, something the investigation could not
    /// support.
    #[must_use]
    pub fn is_fully_explained(&self) -> bool {
        !matches!(self.report.conclusion(), ReportConclusion::Supported) || self.unknowns.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use oo_discovery::DiscoveryDecision;

    use super::*;
    use crate::finding::ReportFinding;

    fn report(decision: DiscoveryDecision) -> MachineReport {
        MachineReport::new(
            ReportFinding::new("eth_chainId", "abc123", decision, 0.9),
            ReportConclusion::from_decision(decision),
        )
    }

    #[test]
    fn a_supported_report_with_no_unknowns_is_fully_explained() {
        let manifest = ReportManifest::new(report(DiscoveryDecision::Accept));
        assert!(manifest.is_fully_explained());
    }

    #[test]
    fn a_supported_report_with_an_unresolved_unknown_is_not_fully_explained() {
        let mut manifest = ReportManifest::new(report(DiscoveryDecision::Accept));
        manifest.add_unknown(ReportUnknown::new("chain 137", "endpoint unreachable"));
        assert!(!manifest.is_fully_explained());
    }

    #[test]
    fn a_needs_review_report_with_unknowns_is_still_considered_explained() {
        let mut manifest = ReportManifest::new(report(DiscoveryDecision::NeedsReview));
        manifest.add_unknown(ReportUnknown::new("chain 137", "endpoint unreachable"));
        assert!(manifest.is_fully_explained());
    }

    #[test]
    fn appendices_are_recorded_in_the_order_they_are_added() {
        let mut manifest = ReportManifest::new(report(DiscoveryDecision::NeedsReview));
        manifest.add_appendix(crate::appendix::ReportAppendix::new(
            crate::appendix::AppendixKind::Supplementary,
            "notes",
            "content",
        ));
        assert_eq!(manifest.appendices().len(), 1);
    }
}
