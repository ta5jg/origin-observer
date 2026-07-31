---
name: analyze-repository
description: Apply the Analyze Repository workflow for relevant analyze-repository work; use it before proposing or validating a change.
---

# Analyze Repository

## Trigger

Use **Analyze Repository** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Map entry points, build/test commands, dependency boundaries, architecture, data flow, operational surfaces, and uncommitted user changes before recommending work. Produce an evidence-backed inventory rather than inferring structure from names.

1. Inventory entry points, build and test commands, package boundaries, generated artifacts, and any uncommitted work already present.
2. Trace one representative request or execution path end to end to learn the real architecture rather than the one implied by directory names.
3. Record ownership boundaries, external dependencies, operational surfaces, and the checks that actually run in CI.
4. Report an evidence-backed inventory with the specific files that support each claim, and list what could not be determined.

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

A Analyze Repository result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
