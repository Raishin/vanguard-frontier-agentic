---
name: frontend-testing-strategy-review
description: Reviews frontend test-pyramid shape, critical-path coverage, and flaky-test governance across unit, component, integration, and E2E layers (Vitest/Jest, Testing Library, Playwright/Cypress), loading framework references only when the task needs them.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: delivery
---

# Frontend Testing Strategy Review

## Purpose

A green CI badge does not prove a critical user journey works. This skill exists to separate "tests exist" from "tests exercise the actual failure modes that matter" — test-pyramid shape, critical-path coverage, and flaky-test governance — without dumping every testing-library's full API surface into every review.

## When to use

Use this skill when the user asks to:

- review or design a frontend test strategy across unit, component, integration, and E2E layers,
- diagnose why a test suite is slow, flaky, or not catching regressions,
- decide whether a given assertion belongs at the unit, component, or E2E layer,
- audit critical-user-journey coverage (checkout, auth, forms) before a release,
- evaluate a proposed test-framework migration (Jest to Vitest, Cypress to Playwright).

## Context7 Documentation Protocol

Test-runner and testing-library APIs (retry semantics, coverage-threshold config, query priority, selector strategy) change across majors and are documented, not folklore — never assert a flag, config shape, or "best practice" from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded in this session.
2. Call `mcp__Context7__resolve-library-id` for the framework in question: `/microsoft/playwright` for Playwright, `/vitest-dev/vitest` for Vitest, `/testing-library/testing-library-docs` for Testing Library, `/cypress-io/cypress-documentation` for Cypress. Prefer these resolved IDs over guessing a library name.
3. Call `mcp__Context7__query-docs` for the specific claim in question — e.g. "coverage threshold configuration", "web-first assertion retry behavior", "getByRole query priority", "avoid fixed waits" — before stating it as fact. Do this per review, not once from a prior session's memory.
4. Prefer the official docs URLs in `official_docs` for primary normative statements (e.g. exact CLI flags, exact config shape); use Context7 to ground and cross-check the claim before writing it into a finding.
5. If Context7 is unavailable or returns no relevant match, fall back to the `official_docs` URLs and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
6. Never invent a config key, CLI flag, matcher name, or API method that no queried source confirms.

## Lean operating rules

- Classify the pyramid shape first (unit:component:E2E ratio, counted from actual test files/CI artifacts, not the user's description of it) before recommending any specific test; a shape problem needs a rebalancing plan, not one more test.
- Treat coverage percentage as a weak signal; require evidence the suite asserts on error states, loading states, and accessibility tree for critical paths, not just the happy-path DOM. A file at 100% line coverage with no error-path assertion is not "well tested."
- Treat flaky-test quarantine (`.skip`, `test.fixme`, `it.skip`, `describe.skip`, Cypress `{retries: N}` used to paper over root cause) as a tracked liability with an owner and expiry, never a silent, permanent state.
- Prefer official, version-specific docs (via Context7) over memory for any framework API claim — Playwright, Vitest, Cypress, and Testing Library APIs and retry/wait semantics change across majors.
- Never request or accept real user credentials, session cookies, or production API keys as test fixtures; require synthetic data or network mocking (MSW, `cy.intercept`, Playwright route interception).
- Do not recommend a framework migration (Jest→Vitest, Cypress→Playwright) without a measured baseline (suite duration, ESM/native-TS support gap, current flake rate) and a rollback path; framework preference alone is not a migration justification.
- Distinguish "flaky because of the test" (missing await, race on network mock, unstable selector) from "flaky because of the app" (real timing bug, unhandled async state) — the fix differs and misdiagnosis just hides a production bug behind a retry.
- Load references only for the layer/framework in scope; do not load the Playwright reference for a pure Vitest unit-coverage question, and do not load the pyramid-shape reference for a single flaky-test triage.

## References

Load these only when needed:

- [Test pyramid shape and critical-path coverage](references/pyramid-shape-and-coverage.md) — use when classifying unit:component:E2E ratio, deciding which layer an assertion belongs at, or auditing critical-user-journey coverage.
- [Flaky-test diagnosis and governance](references/flaky-test-governance.md) — use when triaging a flaky or slow suite, reviewing quarantine (`.skip`/`fixme`) usage, or setting a flake-tracking policy.
- [Playwright and Cypress E2E review](references/e2e-framework-review.md) — use for E2E-layer specifics: locator/selector strategy, web-first assertions and retry semantics, test isolation, and Jest/Cypress-to-Playwright migration evidence requirements.
- [Vitest and Testing Library unit/component review](references/unit-component-framework-review.md) — use for unit/component-layer specifics: query priority (`getByRole` over test IDs), mocking boundaries, and coverage-threshold configuration.

## Response minimum

Return, at minimum:

- the test-pyramid shape observed (unit/component/E2E counts or ratio, with the file/CI-artifact evidence it came from, not an estimate),
- critical-user-journey coverage gaps, cited to a specific file/line or CI-artifact,
- flaky-test inventory status for any `.skip`/`fixme`/retry-suppressed test found (owner, expiry, or "untracked liability"),
- a minimal, prioritized diff-level test plan (not a full-suite rewrite),
- evidence level (`live evidence`, `repo evidence`, `documentation-based`, `inference`) for every claim, and explicit flag when a claim is `documentation-based (Context7 unavailable)`.
