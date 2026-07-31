<!--
-----------------------------------------------------------------------------
Project : Origin Observer
File    : RELEASE_READINESS.md
Purpose : Record the Part 13 validation and scientific-release readiness pass.
Author  : İrfan Gedik
Year    : 2026
-----------------------------------------------------------------------------
-->

# Release Readiness — Part 13

This records the validation pass required by [ROADMAP.md](ROADMAP.md)'s
Part 13, "Validation and Scientific Release." Part 13 has no primary crates
of its own: its job is to check the other thirteen parts against the
roadmap's validation scenarios and release criteria, not to add new
functionality. Following WDRP's own rule, this document states what was
checked and how, not just a pass/fail verdict — an unsupported conclusion
would violate the project it is reporting on.

## Method

Checks were static (code review, `grep` across the workspace) and dynamic
(`cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D
warnings`, `cargo fmt --all --check`), run against commit `c03da4e` and the
defect fix below. No live network, wallet or provider was reached during
this pass — the project has no live-network test tier yet, so anything
claimed below rests on fixture- and unit-level evidence only, named per
item.

## Defect found and fixed

- **`oo_model::confidence::ConfidenceSnapshot::new` called `SystemTime::now()`
  directly**, bypassing the `oo_core::Clock` abstraction every other
  timestamped type in the workspace uses. This is exactly the
  non-determinism WDRP's "Experiments must be repeatable without hidden
  network calls" and this project's own "deterministic time via `Clock`"
  convention exist to prevent. The type was never called from anywhere else
  in the workspace, so it shipped untested and unused since it was written.
  Fixed by taking `clock: &dyn oo_core::Clock` as a parameter, matching the
  pattern already used in `oo-provider::attribution`. Two tests were added
  (`crates/oo-model/src/confidence.rs`, `snapshot_tests` module) exercising
  `ConfidenceSnapshot` and `ConfidenceTimeline` for the first time, using
  `ManualClock` to prove the timestamp is now caller-controlled.
- A full-workspace `grep` for `SystemTime::now()`, `Utc::now()` and
  `Instant::now()` outside `oo_core::clock` now returns nothing.

## Release criteria

| Criterion | Status | Evidence |
| --- | --- | --- |
| No hidden network calls | Met | `reqwest` is used in exactly one file, `crates/oo-rpc/src/http.rs`, behind `RpcTransport`/`HttpTransport`, which every caller reaches only through `RpcClient` with an explicit `PinPolicy`, `RetryPolicy` and `RateLimiter`. No other crate imports `reqwest`, `TcpStream` or `std::net`. |
| No unexplained confidence values | Met, with one documented gap | `oo-confidence::score` and `oo-discovery::prediction` both use equal-weighted or explicitly named factor weights with doc comments stating why (no labelled outcome dataset exists to justify unequal weights). `oo-confidence::level` reconciles `ReproductionStatus` into WDRP's L0–L5 scale, but only reaches L0, L2, L3 and L5 — `ReproductionStatus` has no variant corresponding to WDRP's L1 ("Hypothesis") or L4 ("Verified", as distinct from "Independently verified"), so those two levels are unreachable from evidence-derived confidence today. This is a scope limitation, not a defect: `oo-confidence` never claims L1 or L4, it simply cannot produce them. |
| No conclusions without evidence | Met | `oo-report::ReportConclusion::Supported` is only reachable via `DiscoveryDecision::Accept`, which `oo-discovery` derives from measured signals, never assigned directly. `oo-report::ReportManifest::is_fully_explained` additionally refuses to call a manifest fully explained if it reports `Supported` while an unresolved `ReportUnknown` remains attached. |
| Deterministic exports | Met | Every export function added or reviewed this pass (`oo-dataset::export_records`, `oo-report::export_json`/`export_manifest_json`) is covered by a same-input-twice-same-output test. No `HashMap`/`HashSet` field is reachable from a `Serialize` derive anywhere in the workspace (the one crate using them, `oo-model::confidence`'s graph module, is not serialized). |
| Reproducible reference experiments | Met, for the mechanism | `oo-experiment::reproduction::derive_status` requires at least two consistent runs before granting `Reproduced`, and independent verification requires a second observer on top of that — both gated by explicit tests. What this pass could not do is run the reference experiments named in `research/questions/PERMANENT.md` (RQ-0001–RQ-0010) against a live chain, since no live-network tier exists; the reproducibility *mechanism* is proven, its application to a specific asset is not yet exercised end-to-end. |
| Security review | Partial | `unsafe_code = "forbid"` is a workspace-wide lint (`Cargo.toml`), and `cargo clippy --workspace --all-targets -- -D warnings` is clean. This pass did not audit dependency advisories (`cargo audit` is not installed in this environment) or attempt fuzzing; both are recommended before a public release rather than completed here. |
| Performance review | Not done | Out of scope for this pass. No benchmark suite exists yet; nothing in the codebase indicates an obvious hot-path problem (no unbounded retries, no synchronous network calls off the async runtime), but this is an observation, not a measurement. |
| Documentation review | Partial | `CHANGELOG.md` is current through Part 12. `docs/architecture/README.md` is itself a table of contents for ~25 planned documents (dependency rules, data flow, error model, research model docs, specifications) that were never written — this predates this session and is a known, pre-existing gap, not something introduced by Parts 09–13. |
| WDRP review | Met | Every crate touched in Parts 09–13 follows WDRP's confidence contract (evidence before conclusion, `Contradicted` never silently downgraded to "unknown" — see `oo-confidence::level`, `oo-history::confidence`), and reproduction requires multiple consistent observations before being reported as such. |

## Validation scenarios

The roadmap names ten scenario categories to validate against. Coverage
below is per scenario, with whether it is reachable through an end-to-end
CLI run (`oo-cli observe`), through `oo-observer`'s library API, or only at
the crate/unit level:

| Scenario | Reachable via CLI today | Crate-level coverage |
| --- | --- | --- |
| Native assets, known tokens | Yes — `oo-cli observe --strategy erc20-metadata`, fixture-backed CLI tests in `crates/oo-cli/tests/observe_cli.rs` | `oo-abi`, `oo-discovery` |
| Undiscovered tokens | Partial — CLI reports `NeedsReview`/`Reject` for weak signals; no fixture specifically models an unknown token end-to-end | `oo-discovery::comparison`, `oo-discovery::prediction` |
| Proxy and non-proxy contracts | Yes — `oo-cli observe --strategy proxy-classification --address <addr> --rpc-url <url>` fetches bytecode plus the five known EIP-1967/1822/legacy-OZ slots and classifies offline via `oo_observer::classify_proxy_offline` | `oo-proxy` (full live resolution including diamond — unit-tested) |
| Assets with/without metadata and liquidity | No — not wired into the CLI | `oo-provider::metadata`, `oo-provider::dex` (divergence detection — unit-tested) |
| Conflicting providers | Partial — `oo-cli observe --provider name=url` repeated builds a reproduction report; no committed fixture models two providers actually disagreeing | `oo-provider::metadata` divergence detection, `oo-report::ReproductionReport` |
| Cold and warm caches | Yes — `oo-cli observe --cache-state <empty\|warm\|stale\|invalidated\|unknown>` attaches an `oo_cache::TimedCacheObservation` to every produced investigation via `InvestigationRecord::set_cache_observation`, and the declared state (always a caller assertion, never a live measurement — the CLI cannot observe a wallet's cache) is echoed in every investigation's `"cache"` output field | `oo-cache` (state transitions, invalidation experiments — unit-tested) |
| Desktop and mobile wallets | Yes — `oo-cli observe --strategy wallet-view --address <addr> [--wallet <config_id>]` reads the underlying observation's discovery decision through every built-in wallet adapter (or one, filtered), via `oo_observer::evaluate_wallet_view`, flagging non-page-observable (desktop/hardware) clients as not citable for a client-specific claim | `oo-wallet` (per-wallet adapters, cache-state tracking — unit-tested); `research/unknowns/REGISTER.md` UNK-0005 already tracks that desktop/mobile decision parity is unconfirmed |
| Multiple chains | Yes — `NetworkId` is threaded through `ObservationPlan`; CLI accepts arbitrary RPC endpoints per run | `oo-config::chains.toml` |
| Multiple wallets | Yes — `wallet-view` without `--wallet` evaluates every built-in adapter in one run | `oo-wallet` |
| (Research archival, not a named scenario but load-bearing for reproducibility) | Yes — `--out` on any `observe` run writes `dataset.json` (`oo_observer::export_dataset`, validated against its own rows); `--record-history <path> --question-id <id> [--wallet <label>]` appends a recognition entry to a persisted `oo_history::AssetCaseStudy` JSON file, creating it on first use | `oo-dataset`, `oo-history` |

**Update:** all five crates `RELEASE_READINESS.md` originally flagged as
implemented-but-unwired (`oo-proxy`, `oo-wallet`, `oo-cache`, `oo-history`,
`oo-dataset`) are now both direct dependencies of `oo-observer`
(`crates/oo-observer/src/{proxy,wallet_view,history,dataset}.rs`, plus a
`cache_observation` field on `InvestigationRecord`) *and* reachable through
`oo-cli`: `--strategy proxy-classification`, `--strategy wallet-view`,
`--cache-state`, `--out`'s `dataset.json`, and `--record-history` +
`--question-id`. Making case studies round-trip through JSON files required
adding `Serialize`/`Deserialize` to `oo-core`'s identifier macro and to
`oo-history`'s timeline/case-study types — both previously declared `serde`
as a dependency but never used it. Every new CLI path has both unit
coverage (`crates/oo-cli/src/output.rs`, no live RPC needed) and black-box
coverage (`crates/oo-cli/tests/observe_cli.rs`, invoking the compiled
binary). None of the ten named validation scenarios remain library-API-only;
the two still marked "Partial" (undiscovered tokens, conflicting providers)
are partial because no committed fixture models the specific case, not
because the mechanism is unreachable.

## Outcome

No blocking defect remains open. One real defect (the `SystemTime::now()`
call) was found and fixed with regression tests. All five crates flagged as
implemented-but-unwired during the original pass are now wired through to
the CLI, closing that finding. What remains recorded rather than hidden:
partial security/performance review (no `cargo audit`, no fuzzing, no
benchmark suite), unwritten architecture documentation predating this work,
and two scenarios (undiscovered tokens, conflicting providers) that are
mechanically reachable but lack a fixture demonstrating the specific case.
