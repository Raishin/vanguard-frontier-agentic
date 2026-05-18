---
name: playwright-e2e-suite-review
description: Use this skill when reviewing a Playwright end-to-end test suite for flakiness, selector brittleness, isolation defects, and CI reliability. Trigger when a user provides Playwright spec files, a playwright.config.ts/js, a CI workflow that runs Playwright, or asks why their E2E suite is flaky, slow, or fails intermittently in CI but passes locally. This skill reviews test artifacts statically; it does not execute the suite or launch browsers.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: delivery
  lifecycle: experimental
---

# Playwright E2E Suite Review

## Purpose
This skill reviews a Playwright end-to-end test suite for the defects that destroy CI trust at scale: flakiness, brittle selectors, broken test isolation, and unreliable CI configuration. A flaky E2E suite is worse than no suite — engineers learn to re-run failures instead of reading them, real regressions ship behind a green-after-retry checkmark, and the suite stops gating anything. The review catches hard waits, manual non-retrying assertions, implementation-coupled selectors, shared mutable state across tests, and retry/sharding misconfiguration before they erode confidence in the deploy pipeline.

## Lean operating rules
- Treat any use of `page.waitForTimeout()` / `waitForTimeout` in a spec (not a debugging branch) as HIGH — fixed sleeps are the single largest source of Playwright flakiness; they either race the app or pad every run.
- Treat manual non-retrying assertions (`expect(await locator.isVisible()).toBe(true)`, `expect(await locator.textContent()).toBe(...)`) as HIGH — they snapshot a single instant and lose Playwright's auto-retry; use web-first assertions (`await expect(locator).toBeVisible()`).
- Treat selectors bound to implementation detail — deep CSS chains, nth-child indexes, generated/hashed class names, raw XPath — as HIGH for brittleness; prefer role-, label-, text-, or `data-testid`-based locators.
- Treat tests that depend on ordering or share mutable state (module-level variables mutated across `test()` blocks, a record created in test A read in test B) as HIGH — they break under parallelism, sharding, and `--shuffle`, and produce non-reproducible failures.
- Treat `retries` set greater than 0 in CI with no flaky-test surfacing (no trace-on-retry, no flaky reporter, no quarantine) as HIGH — retries then silently mask real flakiness instead of buying time to fix it.
- Treat `trace`/`screenshot`/`video` all disabled in the CI project as HIGH — a CI-only failure with no trace is undebuggable and forces blind re-runs.
- Treat absolute waits on network (`waitForLoadState('networkidle')`) used as a general synchronization crutch as MEDIUM — it is fragile under analytics/polling; wait on the specific element or response instead.
- Treat shared `storageState` / auth fixtures mutated by tests, or login performed inside every test instead of via a setup project, as MEDIUM — slow and a cross-test contamination risk.
- Treat a single un-sharded CI job for a large suite, or `fullyParallel: false` without a stated reason, as MEDIUM — wall-clock time blocks every deploy.
- Treat `expect` timeouts or global `timeout` raised well above default to make a suite "pass" as MEDIUM — masks a real slow path or race.
- Do not recommend deleting or `.skip()`-ing a flaky test as the fix without a root-cause category and a quarantine/tracking path.
- Label every finding with evidence basis: spec/config text provided, documentation-based, or inference from absent configuration.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Flakiness findings (hard waits, manual assertions, network-idle crutches)
- Selector brittleness assessment (locator strategy per spec)
- Test isolation findings (shared state, ordering dependence, auth contamination)
- Retry and observability assessment (retries vs. trace/flaky surfacing)
- CI configuration findings (sharding, parallelism, artifact capture, timeouts)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
