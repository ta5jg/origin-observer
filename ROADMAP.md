<!--
-----------------------------------------------------------------------------
Project : Origin Observer
File    : ROADMAP.md
Purpose : Define the permanent implementation order for Origin Observer.
Author  : İrfan Gedik
Year    : 2026
-----------------------------------------------------------------------------
-->

# Origin Observer Roadmap

## Roadmap Rule

A part is complete only when its behavior is documented, types and invariants
are defined, tests pass, outputs identify their origin, failures are explicit
and no unsupported conclusion is produced.

No later part may silently compensate for an incomplete earlier part.

## Part 00 — Workspace Foundation

**Goal:** Establish the permanent repository structure, headers, manifests,
configuration, test support, documentation and continuous integration.

**Primary crates:** all scaffold crates.

**Completion:** workspace builds, formats, lints and tests successfully.

## Part 01 — Core Engine

**Goal:** Define execution context, identifiers, clocks, errors, result
envelopes, deterministic serialization, integrity digests and runtime metadata.

**Primary crates:** `oo-core`, `oo-model`, `oo-utils`, `oo-config`.

## Part 02 — RPC Transport

**Goal:** Produce deterministic, attributable and replayable blockchain RPC
observations.

**Modules:** endpoints, requests, responses, transport, retries, rate limits,
batches, block pinning, chain validation, tracing, fixtures and replay.

**Primary crate:** `oo-rpc`.

## Part 03 — Descriptor Engine

**Goal:** Extract and validate chain, network, address, contract, asset,
standard, interface and metadata descriptors.

**Primary crates:** `oo-descriptor`, `oo-model`.

## Part 04 — Snapshot and Evidence Layer

**Goal:** Convert observations into reproducible snapshots and traceable
evidence.

**Modules:** snapshot request, collector, manifest, normalization, integrity,
evidence object, source, registry, relationships, validation, export and
reproduction record.

**Primary crates:** `oo-snapshot`, `oo-evidence`.

## Part 05 — Contract Intelligence

**Goal:** Explain executable and persistent smart-contract structure.

**Modules:** EIP-1167, EIP-1967, transparent and UUPS proxies, beacons, diamonds,
implementation resolution, storage slots and layouts, bytecode normalization,
opcodes, selectors, ABI acquisition, validation and decoding.

**Primary crates:** `oo-proxy`, `oo-storage`, `oo-bytecode`, `oo-abi`.

## Part 06 — Provider and Registry Intelligence

**Goal:** Identify every external component that may contribute to wallet
discovery.

**Modules:** registries, explorers, metadata, images, prices, DEXs, aggregators,
indexers, provider priority, conflict handling and attribution.

**Primary crate:** `oo-provider`.

## Part 07 — Wallet Intelligence

**Goal:** Model wallet behavior without unexplained wallet-specific hacks.

**Modules:** wallet identity, version, platform, installation state, traces,
cache state, capabilities and adapters for MetaMask, Trust Wallet, Rabby,
Coinbase Wallet, SafePal, Ledger Live, OKX, Phantom and generic wallets.

**Primary crate:** `oo-wallet`.

## Part 08 — Discovery Engine

**Goal:** Explain how an unknown blockchain object becomes a presented wallet
asset.

**Modules:** events, stages, paths, timelines, identity, metadata, logo, price,
trust, cache influence, provider priority, conflict resolution, recognition,
ignore decisions, comparisons, prediction inputs and discovery score.

**Primary crate:** `oo-discovery`.

## Part 09 — Experiment Engine

**Goal:** Execute controlled, repeatable and falsifiable experiments.

**Modules:** questions, hypotheses, variables, controls, procedures,
preconditions, expected and actual outcomes, repetition, reproduction,
verification, rejection, registry, runner, manifest and export.

**Primary crate:** `oo-experiment`.

## Part 10 — Confidence and Comparative Intelligence

**Goal:** Assign explainable confidence and compare observations without hiding
uncertainty.

**Modules:** evidence strength, verification, reproducibility, independence,
confidence explanation, wallet, asset, provider, network and temporal
comparison, contradictions and unknown propagation.

**Primary crate:** `oo-confidence`.

## Part 11 — Dataset, History and Cache Intelligence

**Goal:** Preserve historical and temporal context.

**Modules:** dataset schemas and manifests, versioning, deterministic
import/export, historical case studies, wallet and provider timelines, cache
observations, invalidation experiments and historical confidence.

**Primary crates:** `oo-dataset`, `oo-history`, `oo-cache`.

## Part 12 — Report and Orchestration Engine

**Goal:** Produce complete, inspectable and reproducible investigations.

**Modules:** investigation plans, orchestration, findings, conclusions, unknowns,
report manifests, machine and human reports, appendices, visualisation data,
reproduction instructions, CLI commands and exit codes.

**Primary crates:** `oo-observer`, `oo-report`, `oo-cli`.

## Part 13 — Validation and Scientific Release

**Goal:** Validate the complete system against native assets, known tokens,
undiscovered tokens, proxy and non-proxy contracts, assets with and without
metadata and liquidity, conflicting providers, cold and warm caches, desktop
and mobile wallets, multiple chains and multiple wallets.

**Release criteria:** no hidden network calls, unexplained confidence values or
conclusions without evidence; deterministic exports; reproducible reference
experiments; security, performance, documentation and WDRP reviews.

## Permanent Research Questions

- `RQ-0001` Why is Bitcoin discovered?
- `RQ-0002` Why is Ethereum discovered?
- `RQ-0003` Why is BNB discovered?
- `RQ-0004` Why is TRON discovered?
- `RQ-0005` Why is USDT discovered?
- `RQ-0006` Why is our asset not discovered?
- `RQ-0007` What is the minimum condition set required for discovery?
- `RQ-0008` Can discovery be reproduced?
- `RQ-0009` Can discovery be predicted?
- `RQ-0010` Can discovery confidence be measured?

## Definition of Done

Origin Observer succeeds only when it can explain why an asset is recognized,
predict whether a new asset will be recognized, identify wallet discovery
sources, reproduce discovery under controlled conditions and improve legitimate
standards-compliant discoverability through documented wallet mechanisms.

Anything less is an intermediate milestone.
