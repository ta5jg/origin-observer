---
name: write-documentation
description: Apply the Write Documentation workflow for relevant write-documentation work; use it before proposing or validating a change.
---

# Write Documentation

## Trigger

Use **Write Documentation** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Keep documentation tied to executable behavior and versions. Include prerequisites, safe defaults, failure/rollback, examples that can be verified, ownership, and links to authoritative contracts.

1. Identify the reader and the task they are trying to complete, and write to that task rather than to the feature list.
2. State prerequisites, exact commands, and expected output, verifying each by executing it.
3. Document the failure paths, destructive steps, and rollback with the same care as the success path.
4. Record version applicability and ownership, and link to the contract that must change with the documentation.

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

A Write Documentation result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
