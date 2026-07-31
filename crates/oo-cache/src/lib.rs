// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/lib.rs
// Purpose : Model and investigate wallet and provider cache behavior.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Model and investigate wallet and provider cache behavior.

pub mod comparison;
pub mod experiment;
pub mod invalidation;
pub mod model;
pub mod observation;
pub mod state;
pub mod validation;

pub use comparison::{compare, CacheComparison};
pub use experiment::InvalidationExperimentSet;
pub use invalidation::InvalidationExperiment;
pub use model::CacheProfile;
pub use observation::TimedCacheObservation;
pub use state::CacheTransition;
pub use validation::{validate_invalidation_experiment, CacheValidationError};
