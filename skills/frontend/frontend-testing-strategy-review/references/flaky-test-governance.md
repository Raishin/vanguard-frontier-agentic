# Flaky-Test Diagnosis and Governance

Use this reference when triaging a flaky or slow suite, reviewing quarantine (`.skip`/`fixme`/retry) usage, or setting a flake-tracking policy.

## What people get wrong

The naive response to a flaky test is:

> "Add a retry / bump the timeout / skip it for now, we'll fix it later."

That treats the symptom and destroys the signal. A retry that makes a flaky test "pass" does not make the underlying condition go away — it either hides a real race condition in the app, or hides a bad test that will keep costing CI time and eroding trust in red builds ("it's probably just flaky" is how real regressions ship).

## Two root-cause categories — diagnose which one before prescribing a fix

**1. Flaky because of the test:**
- missing `await` on an async assertion or interaction,
- a fixed sleep/wait (`cy.wait(3000)`, `await new Promise(r => setTimeout(r, 500))`) racing against real timing instead of waiting on a condition,
- an unstable selector (nth-child, generated class name, text that changes with content/locale) that matches the wrong element or none, intermittently,
- test-order dependency (shared mutable fixture/module state leaking between tests),
- unmocked network call racing against a mocked one, or a mock that isn't reset between tests.

**Fix:** correct the test. Playwright and Cypress both provide auto-retrying, web-first assertions (e.g. `expect(locator).toHaveText(...)`, `cy.get(...).should(...)`) specifically so a fixed wait is never necessary — Cypress's own best-practices guidance explicitly frames `cy.wait(<ms>)` before an assertion as the anti-pattern to replace with a retrying assertion. Prefer a stable, semantic selector (role/label/text, or a dedicated test-id attribute intentionally isolated from styling/behavior, e.g. Cypress's documented `data-cy` convention) over structural/CSS selectors.

**2. Flaky because of the app:**
- a genuine race condition (UI renders before data resolves, double-submit possible, state update ordering depends on network timing),
- a timing-dependent bug that only manifests under CI load/latency, not locally.

**Fix:** this is a production bug wearing a test-flake costume. The test correctly caught it. Do not "fix" this by adding a retry to the test — that ships the bug and just makes the test stop catching it. Escalate as an app defect finding, not a test-infra finding.

Do not accept "just retry it" as a fix without first determining which category applies. A retry that happens to make a test-side-race pass is masking a bug in the test; a retry that makes an app-side race pass is masking a bug in the product.

## Quarantine governance

Treat every `.skip`, `it.skip`, `describe.skip`, `test.fixme`, `xit`, or Cypress config-level `retries` override used to suppress a known-flaky test as a **tracked liability**, not a resolved state. For each instance found, the finding must record:

- **owner** — who is responsible for un-quarantining it,
- **reason** — why it's flaky (from the root-cause categories above, if known; "unknown" is an acceptable but explicit answer),
- **expiry/tracking** — a linked issue or a stated re-review date; a skip with no linked issue and no date is an **untracked liability** and should be flagged as such, at elevated severity if the skipped test covers a critical user journey (see `pyramid-shape-and-coverage.md`).

A quarantine with none of the above is worse than an honestly-failing test: it produces false confidence in a currently-green suite.

## Playwright-specific retry configuration

Playwright's `TestConfig.retries` sets a maximum retry count for failed tests (default `0`, no retries) and `TestConfig.repeatEach` reruns each test N times, which the official docs frame explicitly as a *debugging* tool for flaky tests, not a permanent fix. A production CI config with `retries` set above 0 is a legitimate resilience buffer against genuine infra noise (network blips in a real CI runner) — but retries silently masking a *reproducible* failure (fails 100% of the time without the retry) is a different problem and should be flagged separately from noise-tolerance retries.

> Verify the exact current default and config shape for `retries`/`repeatEach` against the installed Playwright version via Context7 or `official_docs` before citing a specific number in a report — these are config surface, not folklore.

## Response discipline

When reporting a flaky-test finding, state:

- which root-cause category it falls into (test-side vs app-side), with the specific evidence (missing await, fixed wait, unstable selector, or a described race condition),
- current quarantine status and whether it is tracked (owner + expiry) or untracked,
- whether the flaky test covers a critical user journey (elevates severity if so — see `pyramid-shape-and-coverage.md`),
- the minimal fix (stabilize the test) vs the escalation (file an app-defect finding) — do not conflate the two into a single "add retries" recommendation.
