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

Offline tests are deterministic and do not require live RPC access.

## Format and Lint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Live Smoke

Live RPC checks are opt-in and intentionally kept out of CI:

```bash
sh scripts/live_smoke.sh
```

Optional endpoints and output directory:

```bash
OO_LIVE_RPC_A=https://cloudflare-eth.com \
OO_LIVE_RPC_B=https://cloudflare-eth.com \
OO_LIVE_SMOKE_OUT=/tmp/origin-observer-live-smoke \
sh scripts/live_smoke.sh
```

## Run

```bash
cargo run -p oo-cli -- --help
```

## Current Capabilities

| Capability | What it does | Evidence produced |
| --- | --- | --- |
| Deterministic fixture observation | Runs a local JSON-RPC-style payload through the observation pipeline without network access. | Investigation JSON, machine report or human report |
| Live JSON-RPC observation | Calls one or more HTTP JSON-RPC providers and records the observed response. | Snapshot digest, evidence digest, provider attribution |
| Named provider comparison | Runs the same observation against named providers using `--provider name=url`. | Reproduction status: `Observed`, `Reproduced` or `Contradicted` |
| Built-in discovery strategies | Converts common wallet-discovery questions into RPC calls. | Strategy report, per-observation artifacts and manifest |
| Semantic decoding | Converts raw RPC results into first-level discovery meaning. | Chain identity, native balance, code presence, ERC-20 metadata or RPC error |
| Integrity artifacts | Writes stable artifact directories with digests and manifest metadata. | `manifest.json`, observation files, report files |
| Offline verification gate | Runs deterministic format, lint and test checks. | `scripts/check.sh` pass/fail result |
| Optional live smoke | Verifies live RPC behavior outside CI. | Live artifact directory and provider comparison output |

## Command Reference

| Command | Purpose | Network | Example |
| --- | --- | --- | --- |
| `cargo run -p oo-cli -- --help` | Show CLI help. | No | `cargo run -p oo-cli -- --help` |
| `cargo run -p oo-cli -- status` | Print workspace status. | No | `cargo run -p oo-cli -- status` |
| `cargo run -p oo-cli -- roadmap` | Print embedded roadmap. | No | `cargo run -p oo-cli -- roadmap` |
| `cargo run -p oo-cli -- wdrp` | Print embedded WDRP constitution. | No | `cargo run -p oo-cli -- wdrp` |
| `cargo run -p oo-cli -- observe` | Observe default `eth_chainId` fixture result. | No | `cargo run -p oo-cli -- observe` |
| `cargo run -p oo-cli -- observe --payload-json ...` | Observe a full JSON payload supplied inline. | No | `cargo run -p oo-cli -- observe --payload-json '{"jsonrpc":"2.0","id":1,"result":"0x1"}'` |
| `cargo run -p oo-cli -- observe --payload-file ...` | Observe a JSON payload from a fixture file. | No | `cargo run -p oo-cli -- observe --payload-file fixtures/rpc/eth_get_balance.json` |
| `cargo run -p oo-cli -- observe --rpc-url ...` | Run a live observation against an unnamed RPC endpoint. | Yes | `cargo run -p oo-cli -- observe --rpc-url https://ethereum-rpc.publicnode.com` |
| `cargo run -p oo-cli -- observe --provider name=url` | Run live named-provider observation or comparison. | Yes | `cargo run -p oo-cli -- observe --provider public=https://ethereum-rpc.publicnode.com` |
| `sh scripts/check.sh` | Run the deterministic local quality gate. | No | `sh scripts/check.sh` |
| `sh scripts/live_smoke.sh` | Run opt-in live RPC smoke checks. | Yes | `sh scripts/live_smoke.sh` |

## Output Formats

| Format | Flag | Use when |
| --- | --- | --- |
| Investigation JSON | `--format investigation-json` | You need the direct observation, snapshot digest, evidence digest and semantic summary. |
| Report JSON | `--format report-json` | You need machine-readable report output for tools or automation. |
| Human report | `--format human` | You need a short terminal-readable finding. |

## Observation Strategies

Origin Observer can run a direct JSON-RPC observation or a built-in discovery
strategy. Strategies turn wallet-discovery questions into reproducible RPC
calls, provider comparisons and evidence artifacts.

| Strategy | Required input | RPC calls | Semantic result |
| --- | --- | --- | --- |
| `chain-id` | None | `eth_chainId` | Chain id and known network name when recognized |
| `balance` | `--address` | `eth_getBalance` | Native balance in wei and zero/non-zero state |
| `contract-code` | `--address` | `eth_getCode` | Contract-code presence and byte length |
| `erc20-metadata` | `--address` | `eth_call` for `name`, `symbol`, `decimals` | ERC-20 metadata fields decoded from ABI return data |
| `wallet-overview` | `--address` | `eth_getBalance`, `eth_getCode` | Address classification plus native balance state |

```bash
cargo run -p oo-cli -- observe --strategy chain-id --format investigation-json
cargo run -p oo-cli -- observe --strategy balance --address 0x0000000000000000000000000000000000000000
cargo run -p oo-cli -- observe --strategy contract-code --address 0x0000000000000000000000000000000000000000
cargo run -p oo-cli -- observe --strategy erc20-metadata --address 0x0000000000000000000000000000000000000000
cargo run -p oo-cli -- observe --strategy wallet-overview --address 0x0000000000000000000000000000000000000000
```

Named providers make reproduction explicit:

```bash
cargo run -p oo-cli -- observe \
  --strategy wallet-overview \
  --address 0x0000000000000000000000000000000000000000 \
  --provider a=https://cloudflare-eth.com \
  --provider b=https://cloudflare-eth.com \
  --out /tmp/origin-observer-wallet-overview
```

Strategy artifacts use manifest version 1 and include per-observation JSON,
strategy-level decision, semantic findings, provider list, method list and
params digests. Semantic output decodes the first wallet-discovery primitives:
chain identity, native balance, contract-code presence and ERC-20 metadata
fields.

## Artifact Files

| File | Produced by | Contents |
| --- | --- | --- |
| `investigation.json` | Single observation with `--out` | Subject, semantic summary, snapshot payload, evidence and discovery outcome |
| `report.json` | Single observation with `--out` | Machine-readable report for one investigation |
| `observation-N.json` | Provider comparison or strategy with `--out` | One provider/subject observation per file |
| `reproduction.json` | Multi-provider single-subject observation | Provider comparison and reproduction status |
| `strategy.json` | Multi-subject strategy observation | Strategy decision, findings and all provider observations |
| `manifest.json` | Every artifact run with `--out` | Manifest version, schema, provider list, params digests and file roles |

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
