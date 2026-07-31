// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/validation.rs
// Purpose : Implement the validation module for oo-report.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Report validation.

use crate::machine::MachineReport;
use crate::manifest::ReportManifest;

/// Validates report invariants.
#[must_use]
pub fn validate_report(report: &MachineReport) -> bool {
    !report.finding().subject().trim().is_empty()
        && !report.finding().evidence_digest().trim().is_empty()
}

/// Validates a report manifest: its underlying report must be valid, and it
/// must not report a `Supported` conclusion while an unknown remains
/// unresolved.
#[must_use]
pub fn validate_manifest(manifest: &ReportManifest) -> bool {
    validate_report(manifest.report()) && manifest.is_fully_explained()
}

#[cfg(test)]
mod tests {
    use oo_discovery::DiscoveryDecision;

    use super::*;
    use crate::conclusion::ReportConclusion;
    use crate::finding::ReportFinding;
    use crate::unknown::ReportUnknown;

    #[test]
    fn a_manifest_with_no_unknowns_is_valid() {
        let report = MachineReport::new(
            ReportFinding::new("eth_chainId", "abc123", DiscoveryDecision::Accept, 0.9),
            ReportConclusion::Supported,
        );
        assert!(validate_manifest(&ReportManifest::new(report)));
    }

    #[test]
    fn a_supported_manifest_with_an_unknown_is_invalid() {
        let report = MachineReport::new(
            ReportFinding::new("eth_chainId", "abc123", DiscoveryDecision::Accept, 0.9),
            ReportConclusion::Supported,
        );
        let mut manifest = ReportManifest::new(report);
        manifest.add_unknown(ReportUnknown::new("chain 137", "endpoint unreachable"));
        assert!(!validate_manifest(&manifest));
    }
}
