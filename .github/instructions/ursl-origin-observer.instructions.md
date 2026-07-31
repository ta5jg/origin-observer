---
description: URSL origin-observer project instructions
applyTo: "**"
---

# URSL Project Instructions

These instructions are generated for **Visual Studio Code** from the canonical URSL library. Profile: `origin-observer`. Preserve user-authored project instructions outside the URSL-managed block. Do not claim a check, benchmark, review, or deployment succeeded unless its evidence is available.

---

## Source: Always


# Always

- Inspect the current repository, relevant instructions, and acceptance criteria before changing code or configuration.
- Keep identity, authorization, privacy, data integrity, and safety boundaries explicit.
- Use the smallest coherent change; preserve user-owned work outside the requested scope.
- Validate with the narrowest relevant automated and manual checks, then report the commands actually run and their results.
- Distinguish verified facts, assumptions, limitations, and recommendations in every consequential result.
- Add or update regression evidence when correcting a defect, security issue, or behavioral contract.

## Enforcement

A change that violates an applicable item is not ready for acceptance until the violation is corrected or an authorized exception is recorded.


---

## Source: Never


# Never

- Never expose, commit, log, echo, or fabricate secrets, personal data, private keys, tokens, or production credentials.
- Never claim a test, benchmark, visual result, deployment, audit, or external action succeeded without direct evidence.
- Never weaken security controls, validation, authorization, backup, or rollback solely to make a change easier or faster.
- Never overwrite user-authored instructions or unrelated project files without explicit authorization.
- Never extend a security test beyond written scope, exfiltrate data, establish persistence, or intentionally degrade an authorized target.
- Never silently change a public contract, migration, financial rule, canonical simulation rule, or persistent data format.

## Enforcement

Any breach blocks publication and requires remediation plus an impact assessment.


---

## Source: Preferred


# Preferred

- Prefer explicit contracts, typed boundaries, deterministic inputs, reversible migrations, and independently testable components.
- Prefer standard library and well-maintained dependencies over bespoke infrastructure; document the trade-off when choosing otherwise.
- Prefer additive, backward-compatible evolution with a migration path over breaking changes.
- Prefer property, invariant, integration, and end-to-end tests where unit tests cannot prove the relevant behavior.
- Prefer measured performance decisions with a representative workload over intuition or micro-optimization.
- Prefer compact project instructions that reference canonical URSL sources over duplicated, diverging rules.

## Enforcement

Departures are allowed when justified by concrete constraints, documented trade-offs, and an appropriate verification plan.


---

## Source: Forbidden


# Forbidden

The following practices are prohibited in URSL-managed work:

- Hard-coded credentials, insecure secret fallbacks, or secret-bearing examples.
- Undocumented destructive commands, irreversible production mutations, or broad cleanup actions without explicit approval.
- Hidden network calls, telemetry, data collection, or external publishing.
- Suppressing errors, swallowing failed checks, disabling TLS verification, or bypassing authorization to make a workflow pass.
- Unsubstantiated security claims such as “secure,” “production-ready,” or “fully tested” without the associated evidence.
- Copying vendor-specific instruction text into canonical URSL documents when an adapter can express it at installation time.

## Enforcement

Detection is a blocking failure unless an authorized exception explicitly records the scope, owner, expiry, and compensating controls.


---

## Source: Origin Observer Project Rules

# Origin Observer Project Rules

1. Evidence precedes conclusion; an opinion never enters a report as a finding.
2. Every accepted claim identifies its source, timestamp, network, integrity
   digest, confidence level, and reproduction procedure.
3. Observation, inference, hypothesis, and conclusion are labelled separately and
   never merged into a single statement.
4. A result is accepted only when another engineer can reproduce it from the same
   inputs, procedure, and source material.
5. Unknown, contradicted, and inconclusive outcomes are preserved with their
   evidence rather than discarded or overwritten.
6. Confidence is explainable and falsifiable: state the level, what supports it,
   and what observation would lower it.
7. Experiments are repeatable without hidden network calls; every provider
   request is declared, recorded, and replayable from captured responses.
8. A wallet-specific observation must be explained before it is implemented; an
   unexplained wallet-specific special case is not accepted.
9. No component may impersonate an asset, forge trust, mislead a user, bypass a
   security control, or serve token marketing.
10. Provider, registry, and explorer responses are untrusted input: validate,
    record, and attribute them before any decision depends on them.
11. Chain and wallet observations are made against owned or public read-only
    interfaces; no action touches third-party funds, accounts, or credentials.
12. Any observation involving an address, balance, or user-linked identifier
    carries the minimum data the finding requires and no more.


---

## Source: Research Core Rules


# Research Core Rules

## Scope

Apply this rule whenever the change affects **research**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Rust Safety and Correctness Rules


# Rust Safety and Correctness Rules

1. Production code MUST use safe Rust by default. Each `unsafe` operation MUST
   have a local `SAFETY:` proof and a testable invariant.
2. Public APIs MUST make ownership, mutation, fallibility, cancellation, and
   concurrency expectations explicit.
3. Code MUST NOT use `unwrap` or `expect` on externally influenced or
   recoverable paths without a documented invariant.
4. Shared mutable state MUST have one clearly defined synchronization owner.
   Lock acquisition order and async blocking boundaries MUST be explicit.
5. A hot path MUST NOT allocate, clone, format, or synchronize per iteration
   without measured justification.
6. Deterministic systems MUST use explicit seed, time, ordering, and versioned
   ruleset inputs; hash-map iteration is not canonical ordering.
7. Tests and diagnostics MUST NOT expose credentials or personal data.

## Scope

Apply this rule to rust work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Mandatory Controls

Prove ownership and lifetime boundaries; inspect Result and panic paths, integer conversions, Send/Sync assumptions, lock scope, cancellation, unsafe invariants, FFI, allocations and clones in hot paths, and targeted Cargo checks. Treat unverified assumptions as risk, retain the smallest safe change, and do not accept a result without reproducible evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Cli Core Rules


# Cli Core Rules

## Scope

Apply this rule to every command-line program, developer tool, and script that
another person or an automated system will invoke. A command line is a public
interface: its flags, output, and exit codes are a contract, not presentation.

## Mandatory Controls

1. Exit codes are meaningful: zero only on success, and a distinct non-zero code
   for a usage error versus a runtime failure. Never exit zero after a failure.
2. Diagnostics go to standard error and results to standard output, so the tool
   composes in a pipeline without its own messages corrupting the data.
3. Destructive, irreversible, or remote-affecting operations require an explicit
   flag or confirmation, and offer a dry run that reports exactly what would
   change without changing it.
4. Flags are stable. Renaming or removing a flag, changing its default, or
   changing output structure that scripts parse is a breaking change and
   requires a version bump and a deprecation period.
5. Every error names what failed, the input that caused it, and the next action.
   A stack trace is not an error message.
6. Secrets are never accepted as command-line arguments, since arguments are
   visible in process listings and shell history; read them from a file,
   environment, or prompt.
7. The tool works non-interactively: when standard input is not a terminal, it
   must not block on a prompt, and it must respect a machine-readable output
   mode where one is offered.
8. Long operations report progress to standard error and handle interruption by
   leaving the system in a valid state.

## Verification

- Run the command with no arguments, with `--help`, and with an invalid flag,
  and confirm the exit code and stream for each.
- Pipe the output into another command and confirm nothing but results appear
  on standard output.
- Exercise the destructive path with the dry run first, then confirm the real
  run matches what the dry run reported.
- Run under a non-interactive shell with input redirected from an empty file and
  confirm the command terminates rather than waiting.
- Interrupt a long operation and verify the resulting state is valid and the
  exit code reflects the interruption.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control,
bounded expiry, approval record, and review date. A breaking flag change without
a deprecation period requires explicit approval from the interface owner and a
migration note in the release. An expired exception fails this rule.


---

## Source: Shell Core Rules


# Shell Core Rules

## Scope

Apply this rule whenever the change affects **shell**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Correctness Core Rules


# Correctness Core Rules

## Scope

Apply this rule whenever the change affects **correctness**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Documentation Core Rules


# Documentation Core Rules

## Scope

Apply this rule whenever the change affects **documentation**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Dependency Management Core Rules


# Dependency Management Core Rules

## Scope

Apply this rule whenever the change affects **dependency-management**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Supply Chain Core Rules


# Supply Chain Core Rules

## Scope

Apply this rule whenever the change affects **supply-chain**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Blockchain Core Rules


# Blockchain Core Rules

## Scope

Apply this rule whenever the change affects **blockchain**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Model hostile callers; verify role initialization, accounting invariants, external calls and callbacks, reentrancy, rounding, signatures, oracle assumptions, proxy storage and upgrade authority, and invariant or fuzz tests.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Security Core Rules


# Security Core Rules

## Scope

Apply this rule whenever the change affects **security**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Require explicit authorization and bounded scope; map assets and trust boundaries; prefer non-destructive evidence; assess authentication, authorization, input processing, secrets, dependencies, logging, remediation ownership, and retest criteria.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Authorized Security Work Only


# Authorized Security Work Only

1. Security testing MUST have explicit authorization identifying owner, systems,
   permitted techniques, time window, data rules, emergency contact, and stop
   conditions.
2. Testing MUST remain in scope. Discovery of a new asset does not authorize
   testing it.
3. Non-destructive validation and minimum necessary evidence are mandatory.
   Persistence, data exfiltration, production disruption, credential use beyond
   scope, social engineering, and lateral movement require separate permission.
4. Findings MUST preserve reproducible evidence without retaining secrets or
   unnecessary personal data.
5. Remediation claims MUST be retested within the same authorized scope.

## Scope

Apply this rule to ethical-security work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Mandatory Controls

Require explicit authorization and bounded scope; map assets and trust boundaries; prefer non-destructive evidence; assess authentication, authorization, input processing, secrets, dependencies, logging, remediation ownership, and retest criteria. Treat unverified assumptions as risk, retain the smallest safe change, and do not accept a result without reproducible evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Non Destructive First


# Non Destructive First

Security assessment is permitted only for systems, applications, contracts,
devices, accounts, networks, or data that the user owns or is explicitly
authorized to test.

## Requirements

- Document scope, authorization, exclusions, timing, and emergency contact.
- Prefer static analysis, configuration review, and isolated reproduction first.
- Minimize impact, traffic, persistence, and data exposure.
- Do not access unrelated user data.
- Stop when scope is uncertain or unexpected harm becomes possible.
- Preserve evidence without exposing secrets.
- Provide remediation and verification steps.
- Never use findings to gain unauthorized access or persistence.

## Scope

Apply this rule to ethical-security work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Evidence Handling


# Evidence Handling

Security assessment is permitted only for systems, applications, contracts,
devices, accounts, networks, or data that the user owns or is explicitly
authorized to test.

## Requirements

- Document scope, authorization, exclusions, timing, and emergency contact.
- Prefer static analysis, configuration review, and isolated reproduction first.
- Minimize impact, traffic, persistence, and data exposure.
- Do not access unrelated user data.
- Stop when scope is uncertain or unexpected harm becomes possible.
- Preserve evidence without exposing secrets.
- Provide remediation and verification steps.
- Never use findings to gain unauthorized access or persistence.

## Scope

Apply this rule to ethical-security work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Privacy Preservation


# Privacy Preservation

Security assessment is permitted only for systems, applications, contracts,
devices, accounts, networks, or data that the user owns or is explicitly
authorized to test.

## Requirements

- Document scope, authorization, exclusions, timing, and emergency contact.
- Prefer static analysis, configuration review, and isolated reproduction first.
- Minimize impact, traffic, persistence, and data exposure.
- Do not access unrelated user data.
- Stop when scope is uncertain or unexpected harm becomes possible.
- Preserve evidence without exposing secrets.
- Provide remediation and verification steps.
- Never use findings to gain unauthorized access or persistence.

## Scope

Apply this rule to ethical-security work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Production Testing


# Production Testing

Security assessment is permitted only for systems, applications, contracts,
devices, accounts, networks, or data that the user owns or is explicitly
authorized to test.

## Requirements

- Document scope, authorization, exclusions, timing, and emergency contact.
- Prefer static analysis, configuration review, and isolated reproduction first.
- Minimize impact, traffic, persistence, and data exposure.
- Do not access unrelated user data.
- Stop when scope is uncertain or unexpected harm becomes possible.
- Preserve evidence without exposing secrets.
- Provide remediation and verification steps.
- Never use findings to gain unauthorized access or persistence.

## Scope

Apply this rule to ethical-security work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Reverse Engineering Boundaries


# Reverse Engineering Boundaries

Security assessment is permitted only for systems, applications, contracts,
devices, accounts, networks, or data that the user owns or is explicitly
authorized to test.

## Requirements

- Document scope, authorization, exclusions, timing, and emergency contact.
- Prefer static analysis, configuration review, and isolated reproduction first.
- Minimize impact, traffic, persistence, and data exposure.
- Do not access unrelated user data.
- Stop when scope is uncertain or unexpected harm becomes possible.
- Preserve evidence without exposing secrets.
- Provide remediation and verification steps.
- Never use findings to gain unauthorized access or persistence.

## Scope

Apply this rule to ethical-security work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Supply Chain Security


# Supply Chain Security

Security assessment is permitted only for systems, applications, contracts,
devices, accounts, networks, or data that the user owns or is explicitly
authorized to test.

## Requirements

- Document scope, authorization, exclusions, timing, and emergency contact.
- Prefer static analysis, configuration review, and isolated reproduction first.
- Minimize impact, traffic, persistence, and data exposure.
- Do not access unrelated user data.
- Stop when scope is uncertain or unexpected harm becomes possible.
- Preserve evidence without exposing secrets.
- Provide remediation and verification steps.
- Never use findings to gain unauthorized access or persistence.

## Scope

Apply this rule to ethical-security work and to every change that can affect its stated contract, trust boundary, or release evidence.

## Verification

Run the narrowest relevant automated or reproducible check, exercise normal and boundary behavior, and record commands, observed output, and checks that could not run.

## Exception Process

An exception requires a named owner, concrete rationale, compensating control, expiry, approval record, and review date. An expired exception fails this rule.


---

## Source: Privacy Core Rules


# Privacy Core Rules

## Scope

Apply this rule whenever the change affects **privacy**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Observability Core Rules


# Observability Core Rules

## Scope

Apply this rule whenever the change affects **observability**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Release Engineering Core Rules


# Release Engineering Core Rules

## Scope

Apply this rule whenever the change affects **release-engineering**. It is a release gate, not optional advice.

## Mandatory Controls

1. Define the affected contract, owner, inputs, state transition, failure mode, and compatibility boundary before implementation.
2. Preserve security, privacy, data integrity, and explicit authority boundaries; record assumptions and unresolved risk.
3. Use the smallest coherent change and add regression evidence for a corrected defect or changed contract.
4. **Domain-specific control:** Identify correctness, security, privacy, reliability, performance, maintainability, operational, and compatibility implications. Define measurable acceptance evidence and explicit rollback or recovery boundaries.
5. Do not accept a claimed correctness, security, visual, or performance result without reproducible evidence.

## Verification

- Identify changed paths and the runtime scenario to which this rule applies.
- Run focused tests or a reproducible inspection that exercises normal, boundary, failure, and unauthorized paths where relevant.
- Record commands, tool versions when material, observed output, and checks that could not be executed.

## Exception Process

An exception requires a named owner, concrete rationale, bounded expiry, compensating control, approval record, and review date. Expired exceptions fail this rule.


---

## Source: Developer Tool Zero-to-Production


# Developer Tool Zero-to-Production

## Mission

Take the project from validated idea to an evidence-based production-ready release.
"Complete" means the agreed scope and release gates passed; it does not mean the
software will never need maintenance.

## Phase 0 — Authorization and Scope

- Define owner, users, platforms, budget, deadline, and constraints.
- Define in-scope and out-of-scope systems.
- Record legal, licensing, privacy, and security boundaries.
- Define measurable acceptance criteria and stop conditions.

## Phase 1 — Discovery

- User problem and product goals
- Functional requirements
- Non-functional requirements
- Threat model and data classification
- Performance budgets
- Accessibility and localization targets
- Maintenance and support model

## Phase 2 — Architecture

- Context, modules, data flow, and trust boundaries
- APIs, persistence, dependencies, and failure modes
- Build, deployment, rollback, observability, backup, and recovery
- Architecture decision records for consequential choices

## Phase 3 — Repository Bootstrap

- Toolchain pinning
- Formatting, linting, testing, and CI
- Secret handling and dependency policy
- Documentation baseline
- Reproducible local environment

## Phase 4 — Vertical Slice

Build one narrow end-to-end path proving architecture, data flow, user interaction,
testing, security controls, observability, deployment, and rollback.

## Phase 5 — Incremental Implementation

For every feature:

1. Define acceptance tests.
2. Define security and failure cases.
3. Implement the smallest coherent behavior.
4. Add unit, integration, and end-to-end tests.
5. Review performance, accessibility, and compatibility.
6. Update documentation.

## Phase 6 — Verification

- Functional and regression tests
- Authorized ethical security assessment
- Dependency and supply-chain audit
- Performance and load tests
- Accessibility and localization review
- Recovery, migration, backup, and rollback rehearsals

## Phase 7 — Release

- Version, changelog, artifacts, signing where applicable
- Production configuration and migration
- Monitoring, alerts, deployment, rollback, and post-release validation
- User and developer documentation

## Phase 8 — Operations

- SLOs, incident response, vulnerability management
- Dependency upgrades, backup verification, regression monitoring
- Feedback loop, ownership, and technical-debt register

## Definition of Done

- Acceptance criteria satisfied
- No unresolved critical/high security findings
- Required tests executed and evidenced
- Production configuration validated
- Rollback tested
- Documentation and ownership complete


---

## Source: Universal Project Zero-to-Production Master Prompt


# Universal Project Zero-to-Production Master Prompt

Act as the accountable engineering lead for the current project. Establish the
actual repository state before planning or changing it. Deliver only the agreed
scope through evidence-backed milestones; code generation alone is never a
completion criterion.

## Operating Contract

1. Identify the project type, users, owners, assets, trust boundaries, target
   platforms, dependencies, operating environment, and acceptance criteria.
2. Separate verified facts, assumptions, unknowns, decisions, and risks. Resolve
   repository evidence before asking a question that can be answered locally.
3. Define a thin vertical slice that proves the critical architecture, user or
   system outcome, safety boundaries, tests, observability, and rollback path.
4. Implement in small reversible increments. Preserve public compatibility and
   user-owned work unless an approved requirement changes them.
5. Apply the relevant URSL profile, rules, skills, threat model, and playbook.
   Use primary documentation for version-sensitive technical claims.
6. Run the narrowest relevant validation before broad validation; report actual
   commands, results, limitations, and residual risk.
7. Require explicit approval before destructive, irreversible, credential,
   financial, production, external communication, publishing, or deployment actions.

## Required Delivery Gates

Discovery → architecture and threat model → repository bootstrap → vertical
slice → incremental implementation → functional/security/reliability/performance
verification → documentation and operations → release readiness.

Each gate requires a defined input, observable deliverable, acceptance criteria,
evidence, owner, and rollback/recovery condition.

## Response Contract

For every material iteration return: verified current state; active gate;
affected contract and invariant; smallest safe next change; files changed;
validation run and result; security/privacy/performance impact; residual risks;
and approvals required.

## Execution Strategy

Inspect the current state; separate facts, assumptions, and unknowns; select the smallest safe next action; implement only within scope; then verify and report evidence.

## Required Response

Return verified current state, the next bounded action, affected files, validation actually run, residual risks, and approvals still required.


---

## Required Operational Workflows

Apply the relevant workflow below whenever its trigger matches the task. These workflows are mandatory for this profile even when the host agent does not support a separate skill directory.

### Conduct Research

# Conduct Research

## Trigger

Use **Conduct Research** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Start from the decision and constraints, prefer primary sources and reproducible experiments, record version/date, compare alternatives against explicit criteria, and label confidence and unknowns.

1. Frame the question as the decision it must inform, and state what an answer would have to look like to be useful.
2. Prefer primary sources and reproducible experiments, recording version, date, and scope for every source consulted.
3. Separate what is measured, what is documented, and what is inferred, and label confidence on each conclusion.
4. Report findings with the evidence gaps that remain and the conditions that would invalidate the conclusion.

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

A Conduct Research result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.

### Summarize Evidence

# Summarize Evidence

## Trigger

Use **Summarize Evidence** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Separate observed facts from hypotheses, identify owner and impact, link raw evidence, state priority rationale, and specify the next bounded action and unblocker.

1. Separate what was observed, what was measured, and what was inferred, and label each accordingly.
2. Attach the raw evidence to each claim so a reader can check it without repeating the work.
3. State the confidence and the specific gap that limits it, rather than smoothing over uncertainty.
4. Close with the decision the summary supports and the next action with an owner.

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

A Summarize Evidence result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.

### Compare Options

# Compare Options

## Trigger

Use **Compare Options** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Start from the decision and constraints, prefer primary sources and reproducible experiments, record version/date, compare alternatives against explicit criteria, and label confidence and unknowns.

1. State the decision, the constraints that bound it, and the criteria with their relative weight before examining any option.
2. Describe each option on its own terms, including the case its advocates make and the conditions under which it fails.
3. Score against the stated criteria with evidence, and mark every score that rests on assumption rather than measurement.
4. Recommend one option, state what would change the recommendation, and record the reversal cost of choosing it.

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

A Compare Options result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.

### Analyze Repository

# Analyze Repository

## Trigger

Use **Analyze Repository** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Map entry points, build/test commands, dependency boundaries, architecture, data flow, operational surfaces, and uncommitted user changes before recommending work. Produce an evidence-backed inventory rather than inferring structure from names.

1. Inventory entry points, build and test commands, package boundaries, generated artifacts, and any uncommitted work already present.
2. Trace one representative request or execution path end to end to learn the real architecture rather than the one implied by directory names.
3. Record ownership boundaries, external dependencies, operational surfaces, and the checks that actually run in CI.
4. Report an evidence-backed inventory with the specific files that support each claim, and list what could not be determined.

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

A Analyze Repository result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.

### Rust Review

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

### Review Shell

# Review Shell

## Trigger

Use **Review Shell** when the task requires a bounded engineering review and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Use the language's compiler, formatter, linter, dependency tooling, and test runner. Trace input validation, type/ownership or memory boundaries, concurrency/async behavior, error handling, serialization, secret handling, and deployment/runtime compatibility.

1. Verify quoting of every expansion, since an unquoted variable is the most common defect class in shell.
2. Check error handling: failure modes of pipelines, unset variables, and commands whose exit status is ignored.
3. Inspect any path constructed from input, temporary file creation, and privilege used by the script.
4. Confirm with a static shell checker and a run against inputs containing spaces, newlines, and empty values.

## Required Evidence

- A source-level explanation of the affected contract and invariant
- At least one focused automated or reproducible verification appropriate to the capability
- Explicit treatment of boundary, failure, and unauthorized/invalid paths where applicable
- A measurement rather than intuition for every material performance claim

## Scope Boundary

This skill reports findings and remediation evidence; it does not claim the complete system is defect-free or approve unrelated components.

## Guardrails

Never expose secrets, claim unrun checks passed, broaden authorized scope, hide a breaking change, or perform destructive/production actions without explicit approval.

## Deliverable

A Review Shell finding set with scope, severity or priority, affected contract, evidence, minimal remediation, and verification status.

### Review Dependencies

# Review Dependencies

## Trigger

Use **Review Dependencies** when the task requires a bounded engineering review and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Inventory direct and transitive dependencies, versions, licenses, provenance, maintainer/release health, known vulnerabilities, lockfile integrity, update path, and runtime exposure. Do not remove or upgrade blindly.

1. Resolve the actual dependency graph from the lockfile rather than the declared manifest.
2. Check known vulnerabilities and reachability, licenses, install scripts, and integrity hashes.
3. Assess maintenance health and the replacement cost of anything deeply coupled.
4. Deliver a prioritized, tested update path, and record accepted risk with an owner and a review date.

## Required Evidence

- A source-level explanation of the affected contract and invariant
- At least one focused automated or reproducible verification appropriate to the capability
- Explicit treatment of boundary, failure, and unauthorized/invalid paths where applicable
- A measurement rather than intuition for every material performance claim

## Scope Boundary

This skill reports findings and remediation evidence; it does not claim the complete system is defect-free or approve unrelated components.

## Guardrails

Never expose secrets, claim unrun checks passed, broaden authorized scope, hide a breaking change, or perform destructive/production actions without explicit approval.

## Deliverable

A Review Dependencies finding set with scope, severity or priority, affected contract, evidence, minimal remediation, and verification status.

### Perform Blockchain Protocol Assessment

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

### Perform Dependency Audit

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

### Write Documentation

# Write Documentation

## Trigger

Use **Write Documentation** when the task requires the capability named by this skill and the affected artifact, acceptance criteria, and authorization boundary are known.

## Inputs

- Target files, affected runtime path, and explicit acceptance criteria
- Existing architecture, tests, deployment/rollback boundary, and applicable profile
- Authorization and stop conditions when the task has security, production, financial, or external effects

## Specialist Procedure

Keep documentation tied to executable behavior and versions. Include prerequisites, safe defaults, failure/rollback, examples that can be verified, ownership, and links to authoritative contracts.

1. Identify the reader and the task they are trying to complete, and write to that task rather than to the feature list.
2. State prerequisites, exact commands, and expected output, verifying each by executing it.
3. Document the failure paths, destructive steps, and rollback with the same care as the success path.
4. Record version applicability and ownership, and link to the contract that must change with the documentation.

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

A Write Documentation result with scope, inputs, outputs, evidence, remaining risks, and the next owner/action.
