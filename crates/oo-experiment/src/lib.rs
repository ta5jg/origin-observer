// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/lib.rs
// Purpose : Define and execute repeatable scientific experiments.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Define and execute repeatable scientific experiments.

pub mod control;
pub mod execution;
pub mod export;
pub mod hypothesis;
pub mod manifest;
pub mod model;
pub mod procedure;
pub mod registry;
pub mod repetition;
pub mod reproduction;
pub mod result;
pub mod validation;
pub mod variable;
pub mod verification;

pub use control::ExperimentControl;
pub use execution::{run, StepExecutor, StepOutcome};
pub use export::export_json;
pub use hypothesis::Hypothesis;
pub use manifest::{ExperimentManifest, ManifestEntry};
pub use model::ExperimentDesign;
pub use procedure::{Procedure, ProcedureStep};
pub use registry::ExperimentRegistry;
pub use repetition::{RepetitionRun, RepetitionSet};
pub use reproduction::derive_status;
pub use result::{ActualOutcome, ExpectedOutcome, ExperimentResult};
pub use validation::validate_design;
pub use variable::{ExperimentVariable, VariableRole};
pub use verification::{verify, Verdict};
