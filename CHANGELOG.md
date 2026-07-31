<!--
-----------------------------------------------------------------------------
Project : Origin Observer
File    : CHANGELOG.md
Purpose : Track notable project changes by release.
Author  : İrfan Gedik
Year    : 2026
-----------------------------------------------------------------------------
-->

# Changelog

## [Unreleased]

### Added

- Complete Rust workspace scaffold.
- Permanent README and ROADMAP.
- Standard file headers and research directories.
- `oo-utils`: integrity digests over length-prefixed parts, atomic writes,
  deterministic text normalization with invisible-character detection, and field
  validators that name what failed.
- `oo-config`: typed configuration for chains, providers and wallets, layered
  loading with per-file integrity digests, environment overrides recorded by
  name, redacted credential handling and validation of the WDRP research
  thresholds.
- `config/chains.toml`, `config/providers.toml` and `config/wallets.toml`:
  the networks, external components and wallets the project observes.
- `oo-cli config` command, and a `status` command that reports the loaded
  configuration and its digest.

- `oo-rpc`: block pinning, chain validation, endpoint rate limiting, recorded
  replay fixtures, deterministic exponential backoff, and an exchange digest
  that covers the endpoint, method, parameters and response body.
- `oo-cli observe` now applies the configured observation policy: pinned reads,
  retry count and a shared endpoint rate limit.

### Fixed

- The RPC trace digest covered only the request and response identifiers, so two
  entirely different exchanges produced the same digest. It now covers the
  endpoint, method, parameters and response body.
- Retries applied to every failure, including malformed requests and errors the
  node itself returned, and the computed backoff was discarded rather than
  awaited. Only transport failures and rate limits are retried now, with a
  bounded exponential delay.
- Replay fixtures were keyed by request id, which is chosen by the caller and
  carries no meaning: two different questions with the same id answered each
  other. Fixtures are keyed by method and parameters.

- `oo-bytecode`: hexadecimal normalization, an EVM opcode decoder that walks
  `PUSH` immediates instead of scanning raw bytes, a content digest, a
  structural fingerprint that ignores embedded constants, and `PUSH4`-based
  selector recovery from compiled dispatchers.
- `oo-abi`: canonical function and event signatures with Keccak-256 selectors
  and event topics, a decoder for the single-value return shapes ERC-20 and
  ERC-721 actually use (fixed-width types plus one dynamic string/bytes),
  acquisition provenance that distinguishes a verified source from a
  bytecode-recovered guess, and standard-interface matching against a
  selector-derived catalog.
- `oo-storage`: EVM storage-slot arithmetic for mappings and dynamic arrays,
  the EIP-1967/1822 and legacy OpenZeppelin proxy slots derived from their
  published preimages rather than hardcoded, a pinned `eth_getStorageAt`
  reader built on `oo-rpc`, and a raw-word decoder.
- `oo-proxy`: end-to-end proxy resolution — EIP-1167 minimal proxy detection
  by exact bytecode template, EIP-1967 transparent/UUPS/beacon classification
  from storage, the legacy OpenZeppelin layout, and EIP-2535 diamond detection
  via `supportsInterface`. Every classification records the checks performed
  and what they observed, and a beacon result names the beacon rather than
  guessing the final implementation until a caller follows up.

- `oo-provider`: provider capability and priority-ordered selection, metadata
  merging that reports a disagreement between registries instead of silently
  preferring one, price-quote divergence detection, explorer source
  verification parsing, and attribution carrying a clock-derived timestamp.
- `oo-wallet`: adapters for MetaMask, Trust Wallet, Rabby, Coinbase Wallet,
  SafePal, OKX Wallet, Ledger Live, Phantom and the generic standards-only
  control case, each declaring only publicly documented API capability
  (EIP-1193/EIP-6963 support, injected provider, platforms) rather than
  inferred behavior; cache-state tracking so a warm-cache recognition is never
  reported as live discovery evidence.
- `oo-discovery`: the seven-stage mission diagram from WDRP's own constitution,
  per-asset metadata/logo/price/trust signal scoring, cross-wallet recognition
  resolution, asset-vs-reference-asset comparison (directly serving RQ-0006),
  and a discoverability prediction (RQ-0009) that is a documented weighted sum
  over measured signals, never a trained model, with every factor's
  contribution reported alongside the total.
- `oo-experiment`: falsifiable hypothesis and experiment-design types, a
  reproduction-status derivation that requires at least two consistent runs
  before granting `Reproduced` or `IndependentlyVerified`, a verdict-producing
  verification step, a per-question experiment registry, a step-by-step
  executor that stops at the first failure, a manifest that tallies verdicts,
  and JSON export.
- `oo-confidence`: reconciles the workspace's three confidence
  representations — `oo_evidence::ReproductionStatus`, the WDRP `L0`–`L5`
  publication gate, and `oo_model::ConfidenceLevel`'s general scale — through
  one pair of conversion functions that return `None` for a contradicted
  status rather than mapping it to the lowest level; the four WDRP confidence
  factors (evidence strength, verification, reproducibility, independence)
  and an equal-weighted factor score; a `ConfidenceExplanation` that names
  which factors are unmet; an `aggregate` function combining several
  observations of one claim with contradiction and unknown propagation; a
  single dimension-labelled `compare` function serving wallet, asset,
  provider, network and temporal comparison as one operation; and
  consistency validation for a hand-built or deserialized explanation.
- `oo-dataset`: field-typed dataset schemas, major/minor dataset versioning
  with major-version compatibility, a manifest tying a schema, version,
  record count and content digest together, a length-prefixed digest over
  serialized records so regrouping them changes the digest, deterministic
  JSON import/export, and validation that a record set's count and digest
  match its manifest.
- `oo-history`: a generic chronological timeline used by both wallet
  recognition and provider metadata-availability history, each entry carrying
  a named source; a historical-claim type that is reliable only when it has a
  named source and was reproduced or independently verified, and refuted
  (never merely low-confidence) when contradicted; asset case studies tying a
  permanent research question to its timelines and narrative; and validation
  that a case study is documented and its timelines are chronological.
- `oo-cache`: timestamped cache observations built on `oo_model::cache`, state
  transitions that recognize warm/stale-to-empty/invalidated as a successful
  invalidation, before/after invalidation experiments and an aggregate success
  rate across a set of them, a single dimension-labelled comparison function,
  a per-key observation profile that derives its transition history in
  timestamp order regardless of insertion order, and validation that an
  invalidation experiment's before/after observations share a key and are
  chronologically ordered.

### Notes

- Parts 01, 02, 05, 06, 07, 08, 09, 10 and 11 of the roadmap are complete.
  `oo-utils` and `oo-config` were the two remaining scaffold crates in Part 01;
  Part 02 was missing rate limits, block pinning, chain validation and
  on-disk replay; Part 05 (`oo-bytecode`, `oo-abi`, `oo-storage`, `oo-proxy`)
  was entirely unimplemented scaffolding.
- Standard slot and topic hashes in `oo-storage` and `oo-abi` are derived from
  their published preimages at build time rather than hardcoded as hexadecimal
  literals. A hand-transcribed 32-byte hash is exactly the kind of unverifiable
  claim WDRP does not accept from its own findings, and the tooling holds
  itself to the same standard.
- `oo-abi`'s ABI decoder covers fixed-width types and a single dynamic
  string/bytes return; it does not decode arbitrary tuples or arrays. That
  subset answers `name()`, `symbol()`, `decimals()`, `balanceOf(address)` and
  their ERC-721 counterparts. A shape outside it is a refused
  `UnsupportedType`, not a guess.
- `oo-proxy`'s diamond detection relies on the `IDiamondLoupe` interface id, a
  fixed EIP-2535 constant this crate cannot re-derive from first principles
  (it is the XOR of several facet selectors). It is recorded as one named
  constant specifically so it is easy to re-check against the current EIP-2535
  text.
- Confidence was represented independently in three places —
  `oo_evidence::ReproductionStatus`'s raw reproduction fact,
  `oo-config::WdrpConfidence`'s L0–L5 publication gate, and
  `oo-model::ConfidenceLevel`'s general seven-variant scale — with no shared
  statement of how they relate. `oo-confidence` (Part 10) is that statement:
  it converts between them without merging the types, and treats
  `Contradicted` evidence as refuted rather than silently folding it into the
  lowest confidence level, since "refuted" and "not yet observed" are
  different claims.
- Bitcoin is declared and disabled: no public JSON-RPC endpoint accepts
  anonymous reads, so it cannot be observed through the RPC transport yet.
  Declaring it keeps its absence from a comparison explicit.
