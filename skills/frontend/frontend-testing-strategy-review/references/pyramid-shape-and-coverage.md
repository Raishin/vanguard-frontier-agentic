# Test Pyramid Shape and Critical-Path Coverage

Use this reference when classifying the unit:component:E2E ratio of a suite, deciding which layer a given assertion belongs at, or auditing whether critical user journeys actually have coverage.

## What people get wrong

The naive story is:

> "We have 2,000 tests and 85% coverage, so the suite is solid."

That number says nothing about shape or what those tests actually assert. Two failure patterns hide behind a healthy-looking coverage number:

1. **Ice-cream-cone shape** — few unit tests, a moderate component layer, and a huge, slow, brittle E2E layer that re-proves logic the unit layer should own. This is slow CI, high flake surface, and expensive maintenance for marginal confidence gain.
2. **Hollow pyramid** — lots of unit tests, but they assert on implementation details (internal state, private methods, snapshot diffs of unrelated markup) instead of behavior, so a real regression in user-facing behavior can still ship green.

Coverage percentage cannot distinguish either failure from a healthy suite. Only reading the tests can.

## Classifying the shape

Count actual test files/cases per layer, don't accept a verbal estimate:

- **Unit** — pure functions, reducers, utility/hook logic in isolation, no DOM rendering.
- **Component** — a single component rendered with Testing Library (or framework-equivalent), asserting on rendered output/accessibility tree and user interaction, network mocked.
- **Integration** — multiple components/modules wired together (e.g. a form + its validation + a mocked API layer), still no real browser.
- **E2E** — real (or real-enough) browser, real routing, hits a running app (locally or staging), via Playwright/Cypress.

A commonly cited healthy shape is unit-heavy, E2E-light — many more fast, isolated tests than slow, full-stack ones — but do not apply a fixed ratio as a hard gate; the right shape depends on the codebase's actual risk surface (a form-heavy app has a legitimately larger component layer than a data-pipeline-heavy one). Flag a shape as a *finding* when:

- E2E test count is a large fraction of total tests but the app's core logic is stateless/computational (should be unit-tested instead), or
- there are numerous integration/E2E tests re-asserting something a unit test already covers with no additional confidence gained, or
- the unit layer is large but assertion inspection shows most assertions target internal state/props rather than rendered/observable behavior.

## Deciding which layer an assertion belongs at

- **Business logic, formatting, validation rules, reducers/selectors** → unit. If it doesn't need a DOM, don't give it one.
- **"Does this component render correctly given these props/state, and can a user interact with it?"** → component, with network/API mocked.
- **"Do these pieces work together" (form + validation + submit handler + mocked API)** → integration.
- **"Does the real critical journey work end-to-end through real routing/auth/persistence?"** → E2E, reserved for a *small number* of the highest-value flows, not every page.

Push back when a user proposes writing an E2E test for something a unit or component test can cover — the E2E test costs more (runtime, flakiness surface, infra) for the same assertion.

## Critical-user-journey coverage audit

A "critical user journey" is a flow whose failure has direct revenue, compliance, or trust impact — auth (login/signup/logout/password reset), checkout/payment, core conversion action (the thing the product exists to let users do), and any flow with a regulatory obligation (consent capture, data export/deletion).

For each critical journey, verify presence of assertions on:

- **the happy path**, obviously, but that alone is not sufficient;
- **error states** — network failure, validation rejection, expired session, rate limiting — does the suite prove the user sees a recoverable error, or does it stop at the happy path and assume errors "probably work"?
- **loading/pending states** — is there an assertion the UI shows a loading indicator and doesn't allow a double-submit, or is this untested?
- **accessibility tree for the interactive path** — for a critical flow, are the interactive elements queried via role/label (proving they're actually reachable via assistive tech), or only via test-id/class (which proves nothing about usability)?

A journey with only a happy-path E2E test and no error-state coverage is a **coverage gap**, not "covered." State this explicitly in the finding rather than crediting partial coverage as complete.

## Response discipline

When reporting pyramid shape or coverage gaps, cite the evidence:

- file/directory counts (e.g. "14 `*.spec.ts` under `e2e/`, 6 under `src/**/*.test.ts`"), or
- a CI coverage/test-report artifact if the user provided one.

Do not report a ratio or gap you inferred from the user's description alone without inspecting the actual test files — label such a claim `inference` and say so.
