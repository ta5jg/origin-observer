---
name: perform-dependency-audit
description: Apply the Perform Dependency Audit workflow for relevant perform-dependency-audit work; use it before proposing or validating a change.
---

# Perform Dependency Audit

## Trigger

Use **Perform Dependency Audit** when the task requires an explicitly authorized operational assessment and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Verify lockfiles, provenance, maintainer/release signals, vulnerable and malicious packages, scripts, transitive reachability, licenses, integrity hashes, CI permissions, and a tested update/removal path. Never silently rewrite dependency locks.

1. Resolve the full direct and transitive graph from the lockfile, with versions, licenses, and install-time scripts.
2. Cross-reference known vulnerabilities and assess whether the vulnerable code is actually reachable in this application.
3. Evaluate maintenance signals and integrity controls: release cadence, maintainer count, signatures, and pinned hashes.
4. Deliver a prioritized update or removal path with the tested upgrade, never rewriting lockfiles silently.

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

A Perform Dependency Audit result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
