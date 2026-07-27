<!--
-----------------------------------------------------------------------------
Project : Origin Observer
File    : WDRP.md
Purpose : Reserve the Wallet Discovery Research Project constitution.
Author  : İrfan Gedik
Year    : 2026
-----------------------------------------------------------------------------
-->

# Wallet Discovery Research Project

## Constitution

The Wallet Discovery Research Project exists to understand, reproduce and
document the mechanisms that cause cryptocurrency wallets to recognize,
present, hide, ignore or distrust blockchain assets.

This project is research-first. It does not seek preferential treatment from
wallets, registries, explorers, markets or listing services. It seeks
evidence-backed explanations that can be reproduced by another engineer using
the same inputs, procedure and source material.

## Mission

WDRP investigates the discovery path from an on-chain object to a wallet user
interface:

```text
Chain state
  -> provider response
  -> descriptor extraction
  -> metadata and registry lookup
  -> wallet cache and policy
  -> confidence decision
  -> displayed, hidden or unknown asset
```

The final product is accepted only when it can explain why an asset is
recognized, predict whether a standards-compliant asset will be recognized,
identify the contributing discovery sources and reproduce the result under
controlled conditions.

## Research Commitments

- Evidence must precede conclusions.
- Every accepted claim must identify its source and reproducibility status.
- Unknown, contradicted and inconclusive outcomes must be preserved.
- Wallet-specific observations must not become unexplained wallet-specific
  hacks.
- Confidence must be explainable and falsifiable.
- Experiments must be repeatable without hidden network calls.
- Reports must separate observation, inference, hypothesis and conclusion.
- No project component may impersonate assets, mislead users, forge trust,
  bypass security controls or optimize token marketing.

## Required Evidence Fields

Every accepted observation should identify:

- evidence ID
- timestamp
- blockchain and network
- wallet and wallet version when applicable
- provider or registry source
- subject under observation
- raw or normalized source material
- integrity digest
- confidence level
- verification status
- reproduction procedure

## Confidence Contract

WDRP uses six confidence levels:

| Level | Meaning |
| --- | --- |
| L0 | Unknown |
| L1 | Hypothesis |
| L2 | Observed |
| L3 | Reproduced |
| L4 | Verified |
| L5 | Independently verified |

Only L5 findings may become accepted project knowledge. Lower levels remain
useful, but must be labelled as provisional.

## Release Rule

Origin Observer may publish a finding only when the finding can be traced back
to evidence, reproduced from documented inputs and rejected or revised by future
counter-evidence.
