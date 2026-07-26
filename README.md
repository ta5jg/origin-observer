<!--
-----------------------------------------------------------------------------
Project : Origin Observer
File    : README.md
Purpose : Introduce Origin Observer, its architecture and development workflow.
Author  : İrfan Gedik
Year    : 2026
-----------------------------------------------------------------------------
-->

# Origin Observer

**Origin Observer** is the engineering implementation of the
**Wallet Discovery Research Project (WDRP)**.

Its single primary goal is to understand the mechanisms that make blockchain
assets discoverable by cryptocurrency wallets.

Every module, experiment, line of code, report, document and hypothesis must
contribute directly to that goal.

## Final Objective

Understand why wallets automatically recognize Bitcoin, Ethereum, BNB, TRON,
USDT and similar assets, then reproduce those discovery mechanisms for
standards-compliant blockchain assets through evidence-based, documented and
reproducible engineering.

No shortcuts. No imitation. Only understanding and reproducibility.

## Primary Research Question

> Why does a wallet automatically recognize one blockchain asset while
> ignoring another?

## Project North Star

Whenever a design decision is required, ask:

> Does this help us understand or reproduce wallet discovery?

If the answer is yes, the work belongs to Origin Observer. If the answer is no,
it belongs elsewhere.

## Scientific Method

```text
Observe
  ↓
Collect Evidence
  ↓
Form Hypothesis
  ↓
Experiment
  ↓
Reproduce
  ↓
Verify
  ↓
Publish
```

## Scientific Principles

- Evidence before opinion.
- Observation before conclusion.
- Reproduction before publication.
- Verification before acceptance.
- Every hypothesis is temporary.
- Every conclusion identifies its evidence.
- Unknown is an acceptable scientific result.
- Every project conclusion remains falsifiable.

## Scope

Origin Observer investigates asset identity, RPC behavior, token descriptors,
proxy architectures, storage, bytecode, ABI behavior, events, indexers,
registries, metadata, logos, price sources, trust signals, caches, wallet logic,
discovery paths, timelines, comparisons and confidence.

## Non-Goals

Origin Observer does not impersonate assets, mislead users, bypass security,
exploit wallets, falsify metadata, forge trust, optimize token economics or
perform marketing and listing campaigns.

## Workspace Architecture

| Crate | Responsibility |
| --- | --- |
| `oo-cli` | Command-line interface |
| `oo-observer` | Top-level investigation orchestration |
| `oo-core` | Shared runtime contracts and execution services |
| `oo-model` | Stable domain models |
| `oo-config` | Configuration loading and validation |
| `oo-rpc` | Deterministic, attributable JSON-RPC transport |
| `oo-descriptor` | Asset and contract descriptor extraction |
| `oo-snapshot` | Reproducible state snapshots |
| `oo-evidence` | Evidence creation, integrity and registry |
| `oo-confidence` | Explainable confidence evaluation |
| `oo-provider` | Provider, registry and indexer attribution |
| `oo-proxy` | Proxy detection and implementation resolution |
| `oo-storage` | Storage-slot and layout analysis |
| `oo-bytecode` | Runtime bytecode analysis |
| `oo-abi` | ABI acquisition, validation and decoding |
| `oo-wallet` | Wallet-specific observation models |
| `oo-discovery` | Discovery path, timeline and decision modeling |
| `oo-experiment` | Controlled experiment execution |
| `oo-history` | Historical case-study support |
| `oo-dataset` | Dataset schemas and deterministic exports |
| `oo-cache` | Cache observation and experiment models |
| `oo-report` | Human- and machine-readable reports |
| `oo-utils` | Small dependency-light utilities |
| `oo-test-support` | Shared fixtures and test helpers |

## Repository Structure

```text
origin-observer/
├── Cargo.toml
├── README.md
├── ROADMAP.md
├── WDRP.md
├── crates/
├── config/
├── data/
├── datasets/
├── docs/
├── evidence/
├── experiments/
├── fixtures/
├── hypotheses/
├── reports/
├── research/
├── scripts/
├── snapshots/
├── tests/
└── tools/
```

## Build

```bash
cargo build --workspace
```

## Test

```bash
cargo test --workspace
```

## Format and Lint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Run

```bash
cargo run -p oo-cli -- --help
```

## Evidence Contract

Every accepted observation must eventually identify an evidence ID, timestamp,
blockchain, network, wallet, provider, subject, source, integrity digest,
confidence level, verification status and reproducibility status.

## Confidence Levels

| Level | Meaning |
| --- | --- |
| L0 | Unknown |
| L1 | Hypothesis |
| L2 | Observed |
| L3 | Reproduced |
| L4 | Verified |
| L5 | Independently verified |

Only independently verified results may become accepted project knowledge.

## Development Rule

Implementation follows `ROADMAP.md` in order. Empty modules are intentional:
the permanent architecture is created first, then each module is implemented and
tested in roadmap order.

## Author

İrfan Gedik — 2026
