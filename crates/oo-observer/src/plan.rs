// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-observer/src/plan.rs
// Purpose : Implement the plan module for oo-observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Observation plan model.

use oo_core::{NetworkId, ProviderId};

/// Reproducible plan for one observation subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationPlan {
    network_id: NetworkId,
    provider_id: ProviderId,
    subject: String,
}

impl ObservationPlan {
    /// Creates an observation plan.
    #[must_use]
    pub fn new(network_id: NetworkId, provider_id: ProviderId, subject: impl Into<String>) -> Self {
        Self {
            network_id,
            provider_id,
            subject: subject.into(),
        }
    }

    /// Returns the network identifier.
    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    /// Returns the provider identifier.
    #[must_use]
    pub const fn provider_id(&self) -> ProviderId {
        self.provider_id
    }

    /// Returns the observation subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }
}
