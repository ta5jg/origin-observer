// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/lib.rs
// Purpose : Orchestrate complete Origin Observer investigations.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Orchestrate complete Origin Observer investigations.

pub mod dataset;
pub mod history;
pub mod investigation;
pub mod orchestrator;
pub mod plan;
pub mod proxy;
pub mod service;
pub mod validation;
pub mod wallet_view;

pub use dataset::{export as export_dataset, InvestigationRow};
pub use history::record_recognition;
pub use investigation::InvestigationRecord;
pub use orchestrator::ObserverOrchestrator;
pub use plan::ObservationPlan;
pub use proxy::classify_offline as classify_proxy_offline;
pub use service::ObserverService;
pub use validation::{validate_investigation, validate_plan};
pub use wallet_view::{evaluate as evaluate_wallet_view, WalletDisplayView};

#[cfg(test)]
mod tests {
    use oo_core::{NetworkId, ProviderId};
    use oo_discovery::DiscoveryDecision;
    use serde_json::json;

    use super::*;

    #[test]
    fn observes_payload_into_investigation() {
        let plan = ObservationPlan::new(NetworkId::new(), ProviderId::new(), "eth_chainId");
        let record = ObserverService::default()
            .observe(plan, json!({"result": "0x1"}))
            .expect("valid investigation");

        assert_eq!(record.snapshot().subject(), "eth_chainId");
        assert_eq!(record.evidence().subject(), "eth_chainId");
        assert_eq!(record.outcome().decision(), DiscoveryDecision::NeedsReview);
        assert!(validate_investigation(&record));
    }
}
