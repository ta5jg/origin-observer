---
name: conduct-research
description: Apply the Conduct Research workflow for relevant conduct-research work; use it before proposing or validating a change.
---

# Conduct Research

## Trigger

Use **Conduct Research** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Start from the decision and constraints, prefer primary sources and reproducible experiments, record version/date, compare alternatives against explicit criteria, and label confidence and unknowns.

1. Frame the question as the decision it must inform, and state what an answer would have to look like to be useful.
2. Prefer primary sources and reproducible experiments, recording version, date, and scope for every source consulted.
3. Separate what is measured, what is documented, and what is inferred, and label confidence on each conclusion.
4. Report findings with the evidence gaps that remain and the conditions that would invalidate the conclusion.

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

A Conduct Research result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
