---
name: review-rust
description: Apply the Rust Review workflow for relevant review-rust work; use it before proposing or validating a change.
---

# Rust Review

Read `RULE-000100` and the Rust standard before reviewing. Treat compilation as
necessary evidence, not proof of runtime correctness.

## Procedure

1. Identify the public contract, invariants, ownership model, hot paths, and
   failure boundaries before proposing a change.
2. Check moves, borrows, lifetimes, partial initialization, integer conversion,
   overflow behavior, indexing, panics, cancellation, and error propagation.
3. For shared or async state, inspect `Send`/`Sync` assumptions, lock ordering,
   lock scope, blocking calls in async code, task shutdown, and atomic ordering.
4. For hot paths, identify per-iteration allocation, cloning, formatting,
   hashing, dynamic dispatch, cache-unfriendly layout, and unnecessary
   synchronization. Do not claim an optimization without a measurement plan.
5. For procedural or replicated simulation, reject implicit randomness,
   unordered iteration, wall-clock state, and renderer-owned canonical data.
6. For every `unsafe` block, require a local `SAFETY:` explanation, name the
   invariant, and verify pointer validity, alignment, initialization, aliasing,
   provenance, drop behavior, and panic safety. Prefer a safe abstraction.
7. Return findings by severity with location, exploit or failure path, minimal
   correction, behavior impact, and a regression test.

## Required Verification

Run the narrowest applicable commands first, then broaden only when relevant:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Use property tests for invariants, loom or controlled interleavings for
concurrency where justified, and deterministic seed replay for simulation.

## Completion Gate

Do not report a review as clean while a memory-safety, data-loss, deadlock,
authorization, determinism, or unbounded-resource risk remains unresolved.

## Trigger

Use **Rust Review** when the task requires a bounded engineering review and the affected artifact, acceptance criteria, and authorization boundary are known.

## Scope Boundary

This skill reports findings and remediation evidence; it does not claim the complete system is defect-free or approve unrelated components.

## Deliverable

A Rust Review finding set with scope, severity or priority, affected contract, evidence, minimal remediation, and verification status.
