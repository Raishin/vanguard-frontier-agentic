# Playwright and Cypress E2E Review

Use this reference for E2E-layer specifics: locator/selector strategy, web-first assertions and retry semantics, test isolation, and evidence requirements for a Cypress↔Playwright migration decision. For where an assertion belongs in the pyramid at all, see `pyramid-shape-and-coverage.md`. For root-causing an already-flaky E2E test, see `flaky-test-governance.md`.

> Version note: Playwright and Cypress config APIs and CLI flags evolve across majors. Verify exact option names/defaults against the installed version via Context7 (`/microsoft/playwright`, `/cypress-io/cypress-documentation`) or the official docs before citing a config shape in a report.

## Officially grounded design points

**Playwright:**
- Web-first assertions (`expect(locator).toHaveText(...)`, `.toBeVisible()`, etc.) auto-retry against the locator until the condition is met or a timeout is reached — this is the documented mechanism that removes the need for manual waits/sleeps. A test using a bare `page.locator(...)` value comparison instead of an `expect(locator).to...` assertion is not benefiting from this retry behavior.
- `TestConfig.retries` (default `0`) controls CI-level retry-on-failure; `TestConfig.repeatEach` reruns a test N times and is documented specifically as a flaky-test debugging aid, not a production resilience setting.
- Test isolation is a core design property — each test gets a fresh browser context by default; a suite that shares page/context state across tests (e.g. via a module-level `let page` reused without a `beforeEach` reset) is fighting the framework's isolation model and is a common source of order-dependent flake.

**Cypress:**
- `cy.get(...)` and other commands automatically retry/query until the element appears or a timeout is hit — this is native, not something the test author needs to add.
- Cypress's own best-practices docs explicitly name `cy.wait(<fixed-ms>)` before an assertion as an anti-pattern to avoid; the documented fix is to let the trailing `.should(...)`/assertion retry instead of inserting a fixed delay.
- Cypress's own best-practices docs recommend a dedicated `data-cy` (or equivalent `data-*`) attribute for element selection over tag/class/id selectors, specifically because tag/class/id are coupled to styling/behavior and churn independently of test intent.

## Non-negotiable design rules

1. **Selector strategy is accessibility-first, then a stable test hook — never CSS structure.** Prefer role/label/text queries (see `unit-component-framework-review.md` for the Testing Library query-priority list, which applies to E2E-adjacent component checks too); for pure E2E flows where role/label queries are impractical, a dedicated, styling-independent test attribute (`data-testid` for Playwright, `data-cy` for Cypress, per each tool's own convention) is the documented fallback — not `nth-child`, generated class names, or brittle text matches on copy that changes with content/locale.
2. **No fixed waits before an assertion.** Any `page.waitForTimeout(...)` or `cy.wait(<ms>)` placed immediately before an assertion (rather than waiting on a genuine external event — a specific network response, a route change) is a flake-risk anti-pattern per both tools' own guidance. Replace with the framework's auto-retrying assertion.
3. **Mock or intercept the network at a defined boundary; do not silently mix real and mocked calls.** `cy.intercept()` / Playwright route interception should be applied consistently for a given test — a test that mocks the primary API call but lets a background analytics/tracking call hit the real network (or vice versa) introduces nondeterminism unrelated to the behavior under test.
4. **Reserve E2E for the small set of genuinely cross-system critical journeys.** Do not let E2E become the default place to add "one more assertion" because it's easiest to write against a running app — see `pyramid-shape-and-coverage.md` for the layer-selection rule.
5. **State isolation between tests is the framework's job — don't fight it.** Don't reuse a `page`/session across tests for speed in a way that reintroduces order dependency; use the framework's fixture/hook lifecycle (Playwright fixtures, Cypress `beforeEach`) for setup/teardown instead of manual sharing.

## Cypress → Playwright (or the reverse) migration evidence bar

Do not recommend a framework switch on preference alone. Require, at minimum:

- a **measured baseline**: current suite wall-clock duration in CI, current flake rate (failures not reproducible on rerun, over a stated window), and any hard blocker (e.g. multi-tab/multi-origin testing needs, which the tools support differently) driving the request,
- an explicit statement of **what does not migrate automatically** — custom commands/plugins, CI parallelization config, visual-regression tooling integration, and existing flaky-test workarounds all need to be re-authored, not copy-pasted,
- a **rollback path** — can the old suite keep running in parallel (even read-only, non-blocking) until the new one has proven equivalent coverage over some number of CI runs, or is this a hard cutover with no fallback if the new suite has gaps.

A migration proposal missing a measured baseline or a rollback path is a rewrite driven by framework fanboyism, not evidence — push back and ask for the baseline before endorsing it.

## Response discipline

When reviewing E2E tests, cite the specific selector/wait/isolation pattern found (with file reference) rather than a general "selectors could be better" statement, and label whether the claim about framework retry/isolation behavior is `documentation-based` (grounded via Context7/official docs this session) or `inference`.
