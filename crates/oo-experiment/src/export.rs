// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-experiment/src/export.rs
// Purpose : Export an experiment design and its result as JSON.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Export an experiment design and its result as JSON.

use serde_json::json;

use crate::model::ExperimentDesign;
use crate::repetition::RepetitionSet;
use crate::result::ExpectedOutcome;

/// Exports an experiment design and its repetitions as one JSON document.
#[must_use]
pub fn export_json(
    design: &ExperimentDesign,
    expected: &ExpectedOutcome,
    repetitions: &RepetitionSet,
) -> serde_json::Value {
    json!({
        "question_id": design.experiment.question_id(),
        "hypothesis": {
            "statement": design.hypothesis.statement,
            "falsifying_condition": design.hypothesis.falsifying_condition,
            "is_falsifiable": design.hypothesis.is_falsifiable(),
        },
        "variables": design.variables.iter().map(|variable| json!({
            "name": variable.name,
            "role": format!("{:?}", variable.role),
            "value": variable.value,
        })).collect::<Vec<_>>(),
        "controls": design.controls.iter().map(|control| json!({
            "description": control.description,
            "rationale": control.rationale,
        })).collect::<Vec<_>>(),
        "preconditions": design.preconditions,
        "procedure": design.procedure.steps().iter().map(|step| json!({
            "order": step.order,
            "action": step.action,
        })).collect::<Vec<_>>(),
        "expected_outcome": expected.statement,
        "repetitions": repetitions.runs().iter().map(|run| json!({
            "index": run.index,
            "statement": run.outcome.statement,
            "evidence_digest": run.outcome.evidence_digest,
        })).collect::<Vec<_>>(),
    })
}

#[cfg(test)]
mod tests {
    use oo_model::experiment::Experiment;

    use super::*;
    use crate::hypothesis::Hypothesis;
    use crate::result::ActualOutcome;

    #[test]
    fn export_carries_the_research_question_and_repetition_count() {
        let design = ExperimentDesign::new(
            Experiment::new("RQ-0005", "USDT is recognized"),
            Hypothesis::new("USDT is recognized", "it is not recognized"),
        );
        let expected = ExpectedOutcome {
            statement: "recognized".to_owned(),
        };
        let mut repetitions = RepetitionSet::default();
        repetitions.record(ActualOutcome {
            statement: "recognized".to_owned(),
            evidence_digest: Some("sha256:abc".to_owned()),
        });

        let exported = export_json(&design, &expected, &repetitions);
        assert_eq!(exported["question_id"], "RQ-0005");
        assert_eq!(exported["repetitions"].as_array().unwrap().len(), 1);
        assert_eq!(exported["hypothesis"]["is_falsifiable"], true);
    }
}
