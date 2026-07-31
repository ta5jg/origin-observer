---
name: compare-options
description: Apply the Compare Options workflow for relevant compare-options work; use it before proposing or validating a change.
---

# Compare Options

## Trigger

Use **Compare Options** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Start from the decision and constraints, prefer primary sources and reproducible experiments, record version/date, compare alternatives against explicit criteria, and label confidence and unknowns.

1. State the decision, the constraints that bound it, and the criteria with their relative weight before examining any option.
2. Describe each option on its own terms, including the case its advocates make and the conditions under which it fails.
3. Score against the stated criteria with evidence, and mark every score that rests on assumption rather than measurement.
4. Recommend one option, state what would change the recommendation, and record the reversal cost of choosing it.

## Required Evidence

- A source-level explanation of the affected contract and invariant
- At least one focused automated or reproducible verification appropriate to the capability
- Explicit treatment of boundary, failure, and unauthorized/invalid paths where applicable
- A measurement rather than intuition for every material performance claim

## Scope Boundary

This skill is limited to its named capability and must not absorb adjacent work that has a separate contract, owner, or verification path.

## Guardrails

Never expose secrets, claim unrun checks passed, broaden authorized scope, hide a breaking change, or perform destructive/production actions without explicit approval.

## Deliverable

A Compare Options result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
