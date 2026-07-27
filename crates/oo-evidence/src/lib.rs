// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/lib.rs
// Purpose : Create, validate, relate and store research evidence.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Create, validate, relate and store research evidence.

pub mod builder;
pub mod export;
pub mod integrity;
pub mod model;
pub mod registry;
pub mod relationship;
pub mod reproduction;
pub mod source;
pub mod validation;

pub use builder::EvidenceBuilder;
pub use export::export_json;
pub use integrity::evidence_digest;
pub use model::EvidenceRecord;
pub use registry::EvidenceRegistry;
pub use relationship::{EvidenceRelation, EvidenceRelationKind};
pub use reproduction::ReproductionStatus;
pub use source::{EvidenceSource, EvidenceSourceKind};
pub use validation::validate_evidence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_and_registers_evidence() {
        let evidence =
            EvidenceBuilder::new(EvidenceSourceKind::Rpc, "fixture://chain-id", "eth_chainId")
                .bytes(br#"{"result":"0x1"}"#.to_vec())
                .build();

        assert!(validate_evidence(&evidence));

        let mut registry = EvidenceRegistry::default();
        registry.insert(evidence.clone());

        assert!(registry.get(evidence.id()).is_some());
        assert_eq!(export_json(&evidence)["subject"], "eth_chainId");
    }
}
