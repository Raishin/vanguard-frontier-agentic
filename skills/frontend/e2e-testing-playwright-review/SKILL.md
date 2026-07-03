---
name: e2e-testing-playwright-review
description: Reviews Playwright end-to-end test configuration -- fixtures, storageState/auth setup, CI sharding and parallelism, and toHaveScreenshot visual-assertion options -- for reliability and correct gating, grounded in current, version-specific Playwright API docs.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: delivery
---

# E2E Testing (Playwright) Review

## Purpose

Playwright E2E suites fail in two directions: they're flaky enough that teams disable them, or they're so under-configured (no sharding, no masking, no stable waits) that they're slow and noisy without adding confidence. This skill reviews Playwright-specific configuration -- fixtures, auth state, parallelism, and screenshot assertions -- against current official API behavior rather than remembered API shapes that may be stale across majors.

## When to use

Use this skill when the user asks to:

- review or configure Playwright test fixtures, `storageState`, or auth setup,
- diagnose Playwright test flakiness (timing, animation, non-deterministic content),
- configure CI sharding/parallelism for a Playwright suite,
- review or tune `toHaveScreenshot` visual-assertion options (`maxDiffPixelRatio`, `mask`, `animations`).

## Context7 Documentation Protocol

Playwright's config shape, CLI flags, and assertion option names change across majors and are documented, not folklore -- never assert a flag, option, or "best practice" from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded in this session.
2. Call `mcp__Context7__resolve-library-id` with library name `Playwright` to obtain the current Context7-compatible ID (`/microsoft/playwright`); prefer the resolved ID over guessing.
3. Call `mcp__Context7__query-docs` for the specific claim in question -- e.g. "toHaveScreenshot maxDiffPixelRatio and mask options", "shard CLI flag and blob reporter merge", "storageState project dependencies setup" -- before stating it as fact. Do this per review, not once from a prior session's memory.
4. Prefer the official docs URLs in `official_docs` for primary normative statements (exact CLI flags, exact config shape); use Context7 to ground and cross-check the claim before writing it into a finding.
5. If Context7 is unavailable or returns no relevant match, fall back to the `official_docs` URLs and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
6. Never invent a config key, CLI flag, assertion option, or fixture API that no queried source confirms.

## Lean operating rules

- Always confirm the installed Playwright version before asserting on API option names; `toHaveScreenshot` options and CLI `--shard` syntax are stable but still verify against the project's `package.json` version, not assumption.
- Distinguish flakiness caused by real non-determinism (animation, dynamic content, network timing) from flakiness caused by weak locators or missing waits; the fix differs, and misdiagnosis just hides a real timing bug behind a wider tolerance.
- Recommend `animations: 'disabled'` and explicit `mask`/`stylePath` for non-deterministic regions before recommending a looser `maxDiffPixelRatio`/`maxDiffPixels`/`threshold` -- widening tolerance first papers over the actual source of visual noise.
- Treat `storageState.json` fixtures as sensitive: they must come from a dedicated test account, never a real user session, and should not be committed if they contain live tokens (see security notes below).
- Recommend CI sharding (`--shard=N/M` with a matrix strategy plus `blob` reporter and `merge-reports`) only after confirming the suite's actual wall-clock time in CI justifies the added job complexity and report-merge step; sharding without a merge step silently drops report coverage.
- Prefer the setup-project/`dependencies` pattern (a dedicated `setup` project producing `storageState`, consumed via `dependencies: ['setup']`) over ad hoc `globalSetup` for auth when the project already uses Playwright's project model; both are documented, but they compose differently with sharding and per-project storage state.
- Do not conflate `fullyParallel` (parallelizes tests within a single file, in addition to across files) with CI-level sharding (`--shard`, distributes files across separate CI jobs/machines) -- they solve different bottlenecks and a suite can need one, both, or neither.
- Load the design-token/visual-regression skill instead of this one when the question is about baseline-approval workflow or a third-party visual-review service (e.g. Chromatic), not Playwright's own `toHaveScreenshot` config.

## References

Load these only when needed:

- [Fixtures, auth setup, and storageState security](references/fixtures-and-auth-setup.md) -- use when reviewing or designing `storageState`/auth fixtures, `setup` project dependencies, `globalSetup`, or handling of `storageState.json`/HAR files as sensitive artifacts.
- [CI sharding and parallelism](references/ci-sharding-and-parallelism.md) -- use when configuring or reviewing `--shard`, matrix CI strategy, blob-reporter merge, `fullyParallel`, or `workers` tuning.
- [Visual assertion tuning (toHaveScreenshot)](references/visual-assertion-tuning.md) -- use when reviewing or tuning `toHaveScreenshot`/`toMatchSnapshot` options (`animations`, `mask`, `maxDiffPixelRatio`, `maxDiffPixels`, `threshold`, `stylePath`) or diagnosing visual-diff flakiness.

## Response minimum

Return, at minimum:

- the Playwright feature/config area in scope (fixtures, sharding, screenshot assertion),
- evidence level and the exact Playwright version the guidance targets,
- root cause of any flakiness identified (not just a threshold-widening patch),
- proposed config diff (not applied) with the option names verified against docs,
- security caveat on any `storageState`/HAR fixture reviewed.
