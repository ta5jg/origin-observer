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

### Notes

- Part 01 of the roadmap is complete. `oo-utils` and `oo-config` were the two
  remaining scaffold crates in that part.
- Confidence is represented twice in the workspace: `oo-config::WdrpConfidence`
  follows the constitution's six levels (L0–L5), while `oo-model::ConfidenceLevel`
  carries an older seven-variant scale. Configuration and reports use the
  constitutional form. Reconciling the two belongs to Part 10, where confidence
  is the subject rather than a side effect.
- Bitcoin is declared and disabled: no public JSON-RPC endpoint accepts
  anonymous reads, so it cannot be observed through the RPC transport yet.
  Declaring it keeps its absence from a comparison explicit.
