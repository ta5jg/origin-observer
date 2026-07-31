---
name: perform-blockchain-protocol-assessment
description: Apply the Perform Blockchain Protocol Assessment workflow for relevant perform-blockchain-protocol-assessment work; use it before proposing or validating a change.
---

# Perform Blockchain Protocol Assessment

## Trigger

Use **Perform Blockchain Protocol Assessment** when the task requires an explicitly authorized operational assessment and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Model validators, relayers, bridges, governance, indexing, consensus/finality assumptions, replay paths, economic incentives, and key custody. Prefer simulation and local/testnet evidence; state residual systemic risk explicitly.

1. Map the protocol participants — validators, relayers, bridges, oracles, governance — and what each is trusted to do.
2. Model the economic incentives, including the profit available from delaying, reordering, or censoring a message.
3. Assess consensus and finality assumptions, cross-chain replay, key custody, and upgrade authority under adversarial conditions.
4. Deliver simulation or testnet evidence per finding, and state the residual systemic risk explicitly.

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

A Perform Blockchain Protocol Assessment result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
