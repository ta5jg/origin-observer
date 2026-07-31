// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/variable.rs
// Purpose : Variables an experiment manipulates or measures.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Variables an experiment manipulates or measures.

/// Role a variable plays in an experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableRole {
    /// Deliberately changed between runs to observe its effect.
    Independent,
    /// Measured as the outcome.
    Dependent,
    /// Held fixed so it cannot explain a difference in the outcome.
    Controlled,
}

/// One variable in an experiment's design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentVariable {
    /// Variable name.
    pub name: String,
    /// Role this variable plays.
    pub role: VariableRole,
    /// Value or range, described in text.
    pub value: String,
}

impl ExperimentVariable {
    /// Creates a variable.
    #[must_use]
    pub fn new(name: impl Into<String>, role: VariableRole, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            role,
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_variable_carries_its_role_and_value() {
        let variable = ExperimentVariable::new("wallet", VariableRole::Independent, "MetaMask");
        assert_eq!(variable.role, VariableRole::Independent);
        assert_eq!(variable.value, "MetaMask");
    }
}
