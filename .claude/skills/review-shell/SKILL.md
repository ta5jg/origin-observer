---
name: review-shell
description: Apply the Review Shell workflow for relevant review-shell work; use it before proposing or validating a change.
---

# Review Shell

## Trigger

Use **Review Shell** when the task requires a bounded engineering review and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Use the language's compiler, formatter, linter, dependency tooling, and test runner. Trace input validation, type/ownership or memory boundaries, concurrency/async behavior, error handling, serialization, secret handling, and deployment/runtime compatibility.

1. Verify quoting of every expansion, since an unquoted variable is the most common defect class in shell.
2. Check error handling: failure modes of pipelines, unset variables, and commands whose exit status is ignored.
3. Inspect any path constructed from input, temporary file creation, and privilege used by the script.
4. Confirm with a static shell checker and a run against inputs containing spaces, newlines, and empty values.

## Required Evidence

- A source-level explanation of the affected contract and invariant
- At least one focused automated or reproducible verification appropriate to the capability
- Explicit treatment of boundary, failure, and unauthorized/invalid paths where applicable
- A measurement rather than intuition for every material performance claim

## Scope Boundary

This skill reports findings and remediation evidence; it does not claim the complete system is defect-free or approve unrelated components.

## Guardrails

Never expose secrets, claim unrun checks passed, broaden authorized scope, hide a breaking change, or perform destructive/production actions without explicit approval.

## Deliverable

A Review Shell finding set with scope, severity or priority, affected contract, evidence, minimal remediation, and verification status.
