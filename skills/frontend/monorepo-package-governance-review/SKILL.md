---
name: monorepo-package-governance-review
description: Reviews monorepo task-graph configuration (Turborepo tasks/caching, Nx task pipelines) alongside dependency and lockfile governance (pnpm catalogs, npm overrides, lifecycle-script risk) to prevent false-green CI from stale cache reuse and unpinned supply-chain exposure.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.2.0"
  updated: "2026-07-03"
  category: platform
---

# Monorepo & Package Governance Review

## Purpose

Monorepo tooling and dependency governance fail together more often than separately: a missing task-graph edge lets a stale build pass CI, and an unpinned dependency range lets that same task silently resolve a different, possibly compromised, package version next time. This skill reviews both the task graph (Turborepo/Nx) and the dependency/lockfile layer (pnpm catalogs, npm overrides, lifecycle scripts) as one governance surface, without stuffing bundler internals, framework-specific build guidance, or CI-vendor setup into every prompt.

## When to use

Use this skill when the user asks to:

- review `turbo.json` (`tasks`, `dependsOn`, `inputs`, `outputs`, `env`) or `nx.json` (`targetDefaults`, `namedInputs`) task-graph configuration,
- diagnose a false-green CI result where a task passed despite a real upstream code change (stale cache reuse),
- review `package.json`/lockfile version-pin policy, pnpm `catalog`/`catalogs`, or npm `overrides`/`resolutions`,
- audit lifecycle scripts (`postinstall`/`preinstall`/`prepare`) introduced by a new or bumped dependency,
- consolidate duplicate/drifted dependency versions across monorepo packages,
- review remote-cache (Turborepo Remote Cache / Nx Cloud) token handling in CI config,
- audit `.npmrc` scope-to-registry mappings for scoped internal packages (dependency-confusion exposure), verify `package-lock.json` is committed and CI enforces a frozen install (`npm ci`, not `npm install`), or review lifecycle-script (`postinstall`/`preinstall`/`prepare`) allowlisting for a new dependency.

## Lean operating rules

- Classify the monorepo tool in scope first (Turborepo vs Nx vs plain npm/pnpm workspaces) from actual config files present (`turbo.json`, `nx.json`, `pnpm-workspace.yaml`) — do not assume tooling from the user's phrasing alone.
- Trace every cross-package build/test task dependency to its actual `dependsOn` (Turborepo) or `targetDefaults`/`dependsOn` (Nx) edge; a task that consumes another package's build output without a graph edge is a false-green risk, not a style nit.
- Check that cache `inputs` (Nx `namedInputs`) and `env`/`globalEnv` (Turborepo) include every file and environment variable that actually affects a task's output; an incomplete cache key is a correctness bug that manifests as a stale pass, not merely a performance issue.
- Note that Turborepo's legacy `pipeline` key was renamed to `tasks`; flag `pipeline` usage as needing a version check, not silent modernization, since the correct key depends on the installed Turborepo major version.
- Treat any lockfile change unaccompanied by a matching `package.json`/`pnpm-workspace.yaml` change (or vice versa) as suspicious and requiring explanation before approval.
- Require pinning or `catalog:`/npm `overrides` consolidation for security-sensitive or high-blast-radius dependencies; do not blanket-require exact pins if the team runs a working Renovate/Dependabot auto-merge policy with CI gating — ask before assuming the policy is absent.
- Flag any new dependency's `postinstall`/`preinstall`/`prepare` script by quoting its actual script content from `package.json`, not by assuming intent from the package name alone.
- Verify current pnpm `catalog:`/`catalogs:` syntax and Turborepo/Nx task-config key names against Context7-sourced docs before recommending a diff, since these keys and defaults have changed across major versions (see Context7 Documentation Protocol below).
- Load only the reference needed for the component in scope; do not dump both the task-graph and dependency-governance reference into a single-focus review.

## Context7 Documentation Protocol

Before making any version-specific claim about Turborepo (`tasks` vs legacy `pipeline`, `dependsOn` semantics, `inputs`/`outputs`/`env`/`globalEnv` behavior, Remote Cache auth), Nx (`targetDefaults`, `namedInputs`, `dependsOn`, `nx.json` vs per-project `project.json`/`package.json` `nx` block), pnpm (`catalog`/`catalogs` syntax in `pnpm-workspace.yaml`, the `catalog:` protocol in `overrides`), or npm registry/supply-chain behavior (`.npmrc` scope-to-registry mapping, `npm ci` vs `npm install` frozen-install semantics, `allowScripts`/`npm approve-scripts` lifecycle-script gating):

1. Call `mcp__Context7__resolve-library-id` for the library in scope (`Turborepo`, `Nx`, `pnpm`, or `npm`) if not already resolved in this session.
2. Call `mcp__Context7__query-docs` with a specific query naming the exact config key or behavior in question (for example: "turbo.json tasks dependsOn env cache hash", "nx.json targetDefaults namedInputs", "pnpm-workspace.yaml catalog catalogs syntax", "npmrc scope registry mapping allowScripts npm ci").
3. Prefer the result over training-data recall — config key names and defaults have changed across major versions of all three tools.
4. If Context7 is unavailable or returns no relevant result, state the claim as `documentation-based (unverified this session)` and recommend the user confirm against the installed tool version's own docs before applying any diff.
5. Never invent a config key, CLI flag, or default value that Context7 did not return or that is not directly visible in the repo's own config file.

## References

Load these only when needed:

- [Task-graph review](references/task-graph-review.md) — use for `turbo.json`/`nx.json` false-green diagnosis: missing `dependsOn` edges, incomplete cache `inputs`/`env`, and legacy `pipeline` vs `tasks` drift.
- [Dependency and lockfile governance](references/dependency-lockfile-governance.md) — use for pnpm `catalog`/`catalogs`, npm `overrides`, lockfile/package.json mismatch, and lifecycle-script (`postinstall`) risk review.
- [npm supply-chain governance](references/npm-supply-chain-governance.md) — use for dependency-confusion review of `.npmrc` scope-to-registry mappings, unscoped registry auth tokens, `package-lock.json` commitment/frozen-install (`npm ci`) enforcement, and `allowScripts` lifecycle-script gating for new/bumped dependencies.

## Response minimum

Return, at minimum:

- whether the finding is task-graph (false-green risk), dependency-version (supply-chain/reproducibility risk), or both,
- exact missing edge / cache-key gap / unpinned range with file and line citation,
- evidence level (`live repo evidence`, `documentation-based`, or `inference`) and the Turborepo/Nx/pnpm version the guidance targets,
- proposed config diff (not applied — this skill is static-review-only),
- security caveat on any lifecycle script or committed token found, and a rollback/verification note (for example, which command to run locally to confirm the cache-key fix produces a miss on the affected change).
