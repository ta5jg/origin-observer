// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/hypothesis.rs
// Purpose : A falsifiable hypothesis under test.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A falsifiable hypothesis under test.
//!
//! WDRP's scientific principles require every hypothesis to be temporary and
//! every experiment to be able to fail. A statement with no way to fail it is
//! not a hypothesis; this type therefore requires a stated falsifying
//! condition, not only the claim itself.

/// A hypothesis with its own falsifying condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hypothesis {
    /// The claim under test.
    pub statement: String,
    /// What observation would prove this hypothesis false.
    pub falsifying_condition: String,
}

impl Hypothesis {
    /// Creates a hypothesis.
    #[must_use]
    pub fn new(statement: impl Into<String>, falsifying_condition: impl Into<String>) -> Self {
        Self {
            statement: statement.into(),
            falsifying_condition: falsifying_condition.into(),
        }
    }

    /// Returns whether this hypothesis is well-formed: both the claim and its
    /// falsifying condition are stated, and they are not the same text.
    #[must_use]
    pub fn is_falsifiable(&self) -> bool {
        !self.statement.trim().is_empty()
            && !self.falsifying_condition.trim().is_empty()
            && self.statement.trim() != self.falsifying_condition.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hypothesis_with_a_distinct_falsifying_condition_is_falsifiable() {
        let hypothesis = Hypothesis::new(
            "MetaMask recognizes USDT because it is in the default token list",
            "MetaMask fails to recognize USDT when the default token list is disabled",
        );
        assert!(hypothesis.is_falsifiable());
    }

    #[test]
    fn a_hypothesis_with_no_falsifying_condition_is_rejected() {
        let hypothesis = Hypothesis::new("MetaMask recognizes USDT", "");
        assert!(!hypothesis.is_falsifiable());
    }

    #[test]
    fn a_falsifying_condition_identical_to_the_claim_does_not_falsify_anything() {
        let hypothesis = Hypothesis::new("USDT is recognized", "USDT is recognized");
        assert!(!hypothesis.is_falsifiable());
    }
}
