---
name: review-dependencies
description: Apply the Review Dependencies workflow for relevant review-dependencies work; use it before proposing or validating a change.
---

# Review Dependencies

## Trigger

Use **Review Dependencies** when the task requires a bounded engineering review and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Inventory direct and transitive dependencies, versions, licenses, provenance, maintainer/release health, known vulnerabilities, lockfile integrity, update path, and runtime exposure. Do not remove or upgrade blindly.

1. Resolve the actual dependency graph from the lockfile rather than the declared manifest.
2. Check known vulnerabilities and reachability, licenses, install scripts, and integrity hashes.
3. Assess maintenance health and the replacement cost of anything deeply coupled.
4. Deliver a prioritized, tested update path, and record accepted risk with an owner and a review date.

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

A Review Dependencies finding set with scope, severity or priority, affected contract, evidence, minimal remediation, and verification status.
