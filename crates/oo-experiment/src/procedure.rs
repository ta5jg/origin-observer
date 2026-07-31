// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/procedure.rs
// Purpose : The ordered steps another engineer would repeat.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! The ordered steps another engineer would repeat.
//!
//! WDRP requires every accepted observation to name a reproduction procedure.
//! A procedure here is the literal ordered list of actions that constitutes
//! that reproduction, not a summary of them.

/// One step in a procedure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureStep {
    /// Position in the sequence, starting at 1.
    pub order: u32,
    /// The action to take, in imperative form.
    pub action: String,
}

/// An ordered, repeatable procedure.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Procedure {
    steps: Vec<ProcedureStep>,
}

impl Procedure {
    /// Appends a step, automatically numbering it.
    pub fn push(&mut self, action: impl Into<String>) {
        let order = self.steps.len() as u32 + 1;
        self.steps.push(ProcedureStep {
            order,
            action: action.into(),
        });
    }

    /// Returns the steps, in order.
    #[must_use]
    pub fn steps(&self) -> &[ProcedureStep] {
        &self.steps
    }

    /// Returns whether the procedure has at least one step.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_are_numbered_in_the_order_they_are_added() {
        let mut procedure = Procedure::default();
        procedure.push("pin the block");
        procedure.push("call eth_call");
        assert_eq!(procedure.steps()[0].order, 1);
        assert_eq!(procedure.steps()[1].order, 2);
        assert_eq!(procedure.steps()[1].action, "call eth_call");
    }

    #[test]
    fn an_empty_procedure_is_reported_as_such() {
        assert!(Procedure::default().is_empty());
    }
}
