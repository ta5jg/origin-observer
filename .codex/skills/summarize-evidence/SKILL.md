---
name: summarize-evidence
description: Apply the Summarize Evidence workflow for relevant summarize-evidence work; use it before proposing or validating a change.
---

# Summarize Evidence

## Trigger

Use **Summarize Evidence** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Separate observed facts from hypotheses, identify owner and impact, link raw evidence, state priority rationale, and specify the next bounded action and unblocker.

1. Separate what was observed, what was measured, and what was inferred, and label each accordingly.
2. Attach the raw evidence to each claim so a reader can check it without repeating the work.
3. State the confidence and the specific gap that limits it, rather than smoothing over uncertainty.
4. Close with the decision the summary supports and the next action with an owner.

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

A Summarize Evidence result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
