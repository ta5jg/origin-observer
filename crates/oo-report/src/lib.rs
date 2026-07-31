// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-report/src/lib.rs
// Purpose : Generate human-readable and machine-readable reports.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Generate human-readable and machine-readable reports.

pub mod appendix;
pub mod builder;
pub mod conclusion;
pub mod export;
pub mod finding;
pub mod human;
pub mod machine;
pub mod manifest;
pub mod reproduction;
pub mod unknown;
pub mod validation;

pub use appendix::{AppendixKind, ReportAppendix};
pub use builder::ReportBuilder;
pub use conclusion::ReportConclusion;
pub use export::{export_json, export_manifest_json, export_reproduction_json};
pub use finding::ReportFinding;
pub use human::render_human;
pub use machine::MachineReport;
pub use manifest::ReportManifest;
pub use reproduction::{ReportReproductionStatus, ReproductionObservation, ReproductionReport};
pub use unknown::ReportUnknown;
pub use validation::{validate_manifest, validate_report};

#[cfg(test)]
mod tests {
    use oo_discovery::{DiscoveryDecision, DiscoveryEngine};
    use oo_evidence::{EvidenceBuilder, EvidenceSourceKind};

    use super::*;

    #[test]
    fn builds_machine_and_human_report() {
        let evidence =
            EvidenceBuilder::new(EvidenceSourceKind::Rpc, "fixture://rpc", "eth_chainId")
                .bytes(br#"{"result":"0x1"}"#.to_vec())
                .build();
        let outcome = DiscoveryEngine.evaluate([&evidence]);
        let report = ReportBuilder.build(&evidence, &outcome);

        assert_eq!(report.finding().decision(), DiscoveryDecision::NeedsReview);
        assert_eq!(report.conclusion(), ReportConclusion::NeedsReview);
        assert!(validate_report(&report));
        assert_eq!(export_json(&report)["finding"]["subject"], "eth_chainId");
        assert!(render_human(&report).contains("eth_chainId"));
    }

    #[test]
    fn builds_reproduction_report() {
        let observations = vec![
            ReproductionObservation::new(
                "provider-a",
                "eth_chainId",
                "abc",
                "abc",
                "NeedsReview",
                0.45,
            ),
            ReproductionObservation::new(
                "provider-b",
                "eth_chainId",
                "abc",
                "abc",
                "NeedsReview",
                0.45,
            ),
        ];
        let report = ReproductionReport::new(observations);

        assert_eq!(report.status(), ReportReproductionStatus::Reproduced);
        assert_eq!(report.consensus_digest(), Some("abc"));
        assert_eq!(
            export_reproduction_json(&report)["reproduction"]["provider_count"],
            2
        );
    }
}
