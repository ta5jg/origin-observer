// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-evidence/src/reproduction.rs
// Purpose : Evidence reproduction model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Evidence reproduction model.

/// Reproduction status for evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ReproductionStatus {
    /// Reproduction status is unknown.
    #[default]
    Unknown,
    /// Evidence has been observed once.
    Observed,
    /// Evidence has been reproduced by the same observer.
    Reproduced,
    /// Evidence has been independently verified.
    IndependentlyVerified,
    /// Evidence conflicts with another observation.
    Contradicted,
}
