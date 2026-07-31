// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-confidence/src/lib.rs
// Purpose : Compute explainable evidence and discovery confidence.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Compute explainable evidence and discovery confidence.

pub mod engine;
pub mod explanation;
pub mod factor;
pub mod level;
pub mod score;
pub mod validation;

pub use engine::{aggregate, compare, ConfidenceComparison};
pub use explanation::{explain, ConfidenceExplanation};
pub use factor::{ConfidenceFactor, ConfidenceFactorKind};
pub use level::{is_publishable, to_confidence_level, to_wdrp_code};
pub use score::{score_factors, ConfidenceScore};
pub use validation::{validate_explanation, ConfidenceValidationError};
