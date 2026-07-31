// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/control.rs
// Purpose : Conditions held constant so they cannot explain a result.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Conditions held constant so they cannot explain a result.
//!
//! A control is what lets an experiment attribute a result to its independent
//! variable rather than to something incidental. `config/wallets.toml`'s
//! generic standards-only client exists specifically to serve as one: a
//! result that also occurs with the generic client did not come from a named
//! wallet's own behavior.

/// One condition held constant across an experiment's runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentControl {
    /// What is held constant.
    pub description: String,
    /// Why it must be held constant.
    pub rationale: String,
}

impl ExperimentControl {
    /// Creates a control.
    #[must_use]
    pub fn new(description: impl Into<String>, rationale: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            rationale: rationale.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_control_carries_a_rationale() {
        let control = ExperimentControl::new(
            "chain pinned to block 0x1220000",
            "an unpinned block would let chain state change between runs",
        );
        assert!(!control.rationale.is_empty());
    }
}
