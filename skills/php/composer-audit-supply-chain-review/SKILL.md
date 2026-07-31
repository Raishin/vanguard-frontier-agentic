---
name: composer-audit-supply-chain-review
description: Use this skill to review PHP Composer dependency supply-chain posture — whether composer audit is wired into CI with a failing exit-code gate on security advisories, whether config.policy.advisories.audit and config.policy.abandoned settings are configured to actually surface risk, whether abandoned packages have a tracked replacement plan, and whether composer.lock is present, current, and pins dependencies at security-sensitive boundaries. Use when a vulnerable or abandoned Packagist dependency could reach production because the audit gate is missing or non-blocking, an abandoned package has no owner, or the lock file is absent, drifted, or bypassed by unpinned constraints. Static review only; it never installs packages, runs Composer commands, or contacts Packagist.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-16"
  category: security
  lifecycle: experimental
---

# Composer Audit & Supply-Chain Review

## Purpose

Review a PHP project's Composer dependency supply chain so a vulnerable or abandoned Packagist package cannot reach production unnoticed. The dominant failure modes are `composer audit` missing from CI (or present but not gating on its exit code), advisory/abandoned policy configured to report instead of fail, abandoned packages with no tracked replacement, and `composer.lock` that is absent, drifted, or undermined by unpinned dependency constraints at security-sensitive boundaries.

## When to use

Use this skill when the user asks to:

- review whether `composer audit` actually gates CI on security advisories, rather than merely running and being ignored,
- review `config.policy.advisories.audit` / `config.policy.abandoned.audit` / `config.policy.abandoned.block` settings for a PHP project,
- assess abandoned Packagist dependencies for replacement-plan coverage,
- review whether `composer.lock` is present, current, and consistent with `composer.json`,
- review dependency pinning at security-sensitive boundaries (auth, crypto, serialization, payment) for unpinned or overly wide version constraints.

## When not to use

Do not use this skill for:

- installing, updating, or removing packages, or running `composer install`/`update`/`audit` against any live, sandbox, or CI environment — this skill is static review only.
- application architecture or dependency-injection redesign, or choosing a specific replacement package on the team's behalf — name the risk and hand the decision to the owning engineer.
- reviewing non-Composer package ecosystems (npm, pip, RubyGems, etc.) — this skill is Composer/Packagist-specific.
- asserting a specific deployment is free of vulnerable dependencies from configuration review alone; a clean-looking policy does not prove the last audit run was clean.

## Preconditions

- `composer.json` (including any `config.policy` block) and `composer.lock`, if one exists, for the project in scope.
- The CI pipeline definition(s) that build, test, or deploy the project, to confirm whether `composer audit` runs and whether its exit code gates the pipeline.
- Any existing `composer audit` output, report, or CI log excerpt available for the repository.
- The Composer version in use or its constraint, since `config.policy.abandoned.audit` defaults differ between Composer 2.6 (`report`) and Composer 2.7+ (`fail`).
- Any documented accepted-risk exceptions for specific advisories or abandoned packages, if the team maintains them.

## Lean operating rules

- Confirm `composer audit` is both present in CI and gating — its non-zero exit code must actually fail the job, not be discarded or logged past.
- Read `config.policy.advisories.audit` and the `config.policy.abandoned.*` keys from `composer.json`; where a key is unset, apply the documented default for the project's Composer version rather than assuming a value.
- Treat `composer.lock` absence for an application as a blocking finding on its own; the documented exception is that a library is not required to commit its lock file.
- Treat `composer.lock` drift from `composer.json` (stale-lock warnings, hash mismatches) as a distinct finding from lock absence.
- Flag unpinned or overly wide version constraints at security-sensitive boundaries even when a lock file exists, since the next `composer update` can still move far without review.
- Never fabricate an advisory ID, CVE number, or package name; report a gap in evidence rather than inventing an identifier.
- Label every claim `repo evidence`, `documentation-based`, or `inference`.

## Context7 documentation protocol

This skill's allowed tools are Read, Grep, and Glob only — it does not call Context7 or any live documentation lookup at review time. Every Composer-specific behavioral claim (exit codes, `config.policy` key names and defaults, lockfile semantics) must instead be grounded in the bundled reference files, which are sourced from the official Composer documentation and dated at authoring time. If a claim cannot be traced to a bundled reference or to evidence in the repository, label it an assumption rather than asserting it as documented behavior. See [official sources](references/composer-audit-policy.md), [abandoned-and-advisory governance](references/abandoned-and-advisory-governance.md), and [lockfile integrity](references/lockfile-integrity.md).

## Workflow

Follow the review in this order: (1) locate every CI job that builds, tests, or deploys the project and check for a `composer audit` step; (2) confirm that step's exit code is not suppressed (no `|| true`, no `continue-on-error`, no output-only reporting); (3) read `config.policy.advisories.audit` and the `config.policy.abandoned.*` keys, comparing against documented defaults for the Composer version in use; (4) check for abandoned packages already flagged in any available audit output or lockfile metadata, and whether a replacement plan or accepted-risk exception exists; (5) confirm `composer.lock` is present and check for drift against `composer.json`; (6) scan dependency constraints for unpinned or overly wide ranges at security-sensitive boundaries; (7) emit findings with evidence tiers, concrete remediation, and ownership handoffs.

## Decision gates

- Block only when `composer audit` is not run in CI with a failing exit-code gate on advisories, when an abandoned package has no replacement plan, or when `composer.lock` is absent, drifted, or dependencies are floated unpinned at a security-sensitive boundary.
- Every Composer-specific claim traces to a bundled reference file or explicit repository evidence, never memory.
- No advisory ID, CVE, or package name is asserted without repository evidence backing it.
- Application architecture and replacement-package selection are handed to the owning engineer, never decided here.

## Evidence classification

Label each finding `repo evidence` (seen directly in `composer.json`, `composer.lock`, or CI configuration), `documentation-based` (a Composer-documented default or behavior from the bundled references), or `inference` (a reasonable conclusion not directly observed). A documented default never proves what a specific project's configuration actually is — check the file before asserting the default applies.

## Security and privacy constraints

Static review only. Never install packages, run `composer install`/`update`/`audit`, or make any network call to Packagist or any registry. Never fabricate advisory IDs, CVE numbers, or package names — report missing evidence instead. Treat any credential found in `auth.json` or Composer configuration as a redact-and-flag finding, never echoed in output.

## Escalation conditions

Escalate to incident response on any evidence a known-vulnerable or already-flagged-abandoned dependency is already reachable from a production deployment path (e.g. a deployment manifest pinning a version an existing audit output already flags). Escalate abandoned-package findings with no owner to the responsible engineering team as a tracked risk item rather than a routine comment.

## References

Load these only when needed:

- [Composer audit policy](references/composer-audit-policy.md) — `composer audit` behavior, exit codes, and `config.policy.advisories.audit` gating.
- [Abandoned and advisory governance](references/abandoned-and-advisory-governance.md) — `config.policy.abandoned.audit`/`config.policy.abandoned.block` and replacement-plan review criteria, with OSS-risk context.
- [Lockfile integrity](references/lockfile-integrity.md) — `composer.lock` presence, drift, and dependency-pinning review criteria.

## Response minimum

Return, at minimum:

- the CI audit-gating state (present and failing / present but non-gating / absent), labeled with evidence tier;
- the `config.policy.advisories.audit` and `config.policy.abandoned.*` configuration state, compared against documented defaults for the Composer version in use;
- abandoned-package findings with replacement-plan status;
- `composer.lock` presence/freshness state and any unpinned-constraint exposure at security-sensitive boundaries;
- concrete remediation and an exact verification step for each finding;
- ownership handoffs and any incident-response escalation.

## Anti-goals

- Do not install packages, run any Composer command, or contact Packagist or any registry.
- Do not fabricate advisory IDs, CVE numbers, or package names not present in repository evidence.
- Do not redesign application architecture or select a replacement package on the team's behalf.
- Do not present the general OSS-risk statistics used as motivation as a finding specific to the repository under review.
- Do not assert a Composer behavior from memory when it is not grounded in a bundled reference or repository evidence.
