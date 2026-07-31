// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-confidence/src/factor.rs
// Purpose : Named factors contributing to a confidence assessment.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Named factors contributing to a confidence assessment.
//!
//! These are the four factors WDRP's own confidence contract names: evidence
//! strength, verification, reproducibility, and independence. Each is
//! reported separately so a confidence explanation can say which of the four
//! is missing, rather than only a combined number.

/// One factor in a confidence assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceFactorKind {
    /// Whether the underlying evidence itself is strong (a real digest, a
    /// named source) rather than an inference.
    EvidenceStrength,
    /// Whether the claim has been verified against its own stated
    /// expectation.
    Verification,
    /// Whether the same result was obtained more than once.
    Reproducibility,
    /// Whether a second, independent observer confirmed the result.
    Independence,
}

impl ConfidenceFactorKind {
    /// Returns a short human-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::EvidenceStrength => "evidence strength",
            Self::Verification => "verification",
            Self::Reproducibility => "reproducibility",
            Self::Independence => "independence",
        }
    }
}

/// One factor's evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfidenceFactor {
    /// Which factor this is.
    pub kind: ConfidenceFactorKind,
    /// Whether it was satisfied.
    pub satisfied: bool,
}

impl ConfidenceFactor {
    /// Creates a factor evaluation.
    #[must_use]
    pub const fn new(kind: ConfidenceFactorKind, satisfied: bool) -> Self {
        Self { kind, satisfied }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_factor_has_a_readable_name() {
        for kind in [
            ConfidenceFactorKind::EvidenceStrength,
            ConfidenceFactorKind::Verification,
            ConfidenceFactorKind::Reproducibility,
            ConfidenceFactorKind::Independence,
        ] {
            assert!(!kind.name().is_empty());
        }
    }
}
