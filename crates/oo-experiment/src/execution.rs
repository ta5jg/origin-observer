// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/execution.rs
// Purpose : Run a procedure's steps and collect their outcomes.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Run a procedure's steps and collect their outcomes.
//!
//! The runner is deliberately generic over how a step is actually executed:
//! this crate does not know how to call an RPC endpoint or query a wallet,
//! and should not — that coupling belongs to whichever crate performs the
//! observation (`oo-observer`, `oo-discovery`). A step executor is supplied
//! by the caller, so the same runner works whether a step means "issue an
//! eth_call" or "ask a human tester to check a wallet's UI."

use crate::procedure::{Procedure, ProcedureStep};

/// Outcome of running one procedure step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepOutcome {
    /// Step that was run.
    pub step: ProcedureStep,
    /// Whether the step completed without error.
    pub succeeded: bool,
    /// Free-text detail about what happened.
    pub detail: String,
}

/// Executes one procedure step and reports its outcome.
pub trait StepExecutor {
    /// Runs one step.
    fn execute(&mut self, step: &ProcedureStep) -> StepOutcome;
}

/// Runs every step of a procedure through an executor, in order.
///
/// Execution stops at the first failing step: a later step's meaning usually
/// depends on an earlier one having succeeded (a call depends on a pinned
/// block being resolved, for instance), so continuing past a failure would
/// produce outcomes for steps that never really ran under the intended
/// conditions.
pub fn run(procedure: &Procedure, executor: &mut dyn StepExecutor) -> Vec<StepOutcome> {
    let mut outcomes = Vec::with_capacity(procedure.steps().len());
    for step in procedure.steps() {
        let outcome = executor.execute(step);
        let failed = !outcome.succeeded;
        outcomes.push(outcome);
        if failed {
            break;
        }
    }
    outcomes
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysSucceeds;
    impl StepExecutor for AlwaysSucceeds {
        fn execute(&mut self, step: &ProcedureStep) -> StepOutcome {
            StepOutcome {
                step: step.clone(),
                succeeded: true,
                detail: "ok".to_owned(),
            }
        }
    }

    struct FailsAtStep(u32);
    impl StepExecutor for FailsAtStep {
        fn execute(&mut self, step: &ProcedureStep) -> StepOutcome {
            StepOutcome {
                step: step.clone(),
                succeeded: step.order != self.0,
                detail: if step.order == self.0 {
                    "failed".to_owned()
                } else {
                    "ok".to_owned()
                },
            }
        }
    }

    fn procedure() -> Procedure {
        let mut procedure = Procedure::default();
        procedure.push("first");
        procedure.push("second");
        procedure.push("third");
        procedure
    }

    #[test]
    fn every_step_runs_when_all_succeed() {
        let outcomes = run(&procedure(), &mut AlwaysSucceeds);
        assert_eq!(outcomes.len(), 3);
        assert!(outcomes.iter().all(|outcome| outcome.succeeded));
    }

    #[test]
    fn execution_stops_at_the_first_failure() {
        let outcomes = run(&procedure(), &mut FailsAtStep(2));
        assert_eq!(outcomes.len(), 2, "the third step must never run");
        assert!(outcomes[0].succeeded);
        assert!(!outcomes[1].succeeded);
    }

    #[test]
    fn an_empty_procedure_produces_no_outcomes() {
        assert!(run(&Procedure::default(), &mut AlwaysSucceeds).is_empty());
    }
}
