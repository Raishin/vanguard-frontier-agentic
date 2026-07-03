# Vitest and Testing Library Unit/Component Review

Use this reference for unit/component-layer specifics: query priority (`getByRole` over test IDs), mocking boundaries, and coverage-threshold configuration. For where an assertion belongs in the pyramid at all, see `pyramid-shape-and-coverage.md`.

> Version note: Vitest and Testing Library config/query APIs evolve across majors (e.g. Vitest's coverage-threshold `perFile` inheritance behavior changed across versions). Verify exact option names/defaults against the installed version via Context7 (`/vitest-dev/vitest`, `/testing-library/testing-library-docs`) or the official docs before citing a config shape or migration-breaking change in a report.

## Officially grounded design points

**Testing Library query priority (documented, not a style opinion):**

Testing Library's own docs define an explicit priority order, and a review should measure component tests against it directly:

1. **Queries accessible to everyone** — top priority, because they reflect both visual and assistive-technology user experience: `getByRole` (the primary choice — it exposes name/role/state exactly as the accessibility tree does), `getByLabelText` (form fields), `getByPlaceholderText` (secondary, for inputs), `getByText` (non-interactive elements), `getByDisplayValue` (current form value).
2. **Semantic queries** — lower priority than accessible queries; use when an accessible query isn't practical.
3. **Test IDs** — documented as the *last resort*, specifically because a `data-testid` proves nothing about whether a real user (including one using assistive technology) can actually find/use the element.

A component test suite that predominantly uses `getByTestId`/`container.querySelector` where `getByRole`/`getByLabelText` would work is a **finding**, not a style nit — it means the tests are not verifying the thing that actually matters (can a user, including an AT user, interact with this), and it silently permits an accessibility regression (e.g. a missing accessible name) to pass a test that only checked a test-id existed.

Custom test-id queries built via `queryHelpers.queryByAttribute` are supported for a project's own attribute convention (e.g. `data-test-id` instead of `data-testid`), but the priority order still applies — a custom test-id query is still a last resort, not a replacement for role/label queries.

**Vitest coverage thresholds:**

Vitest's `coverage.thresholds` config supports global thresholds (`lines`, `functions`, `branches`, `statements`), per-glob-pattern thresholds (each pattern requiring `perFile: true` explicitly to apply per-file, since patterns do not inherit the top-level `perFile` setting), and negative-number thresholds meaning "no more than N uncovered items" rather than a percentage. A reviewed config claiming a threshold is enforced should be checked against these actual semantics — e.g. a glob-pattern threshold block without its own `perFile: true` will NOT enforce per-file coverage even if the top-level config has `perFile: true`, because glob patterns do not inherit it.

## Non-negotiable design rules

1. **Assert on rendered/observable behavior, not implementation detail.** A component test that reaches into instance state, calls a private method, or snapshot-diffs unrelated markup churn (e.g. a full-component snapshot that breaks on any unrelated style change) is brittle and doesn't prove user-facing correctness — see the "hollow pyramid" failure mode in `pyramid-shape-and-coverage.md`.
2. **Query by role/label first; test-id is the last resort, not the default.** Apply the Testing Library priority order above as a hard review criterion, not a suggestion.
3. **Mock at the network boundary, not inside application logic.** Component/integration tests should mock the API layer (MSW, or an injected fetch client) rather than mocking internal application functions, so the test still exercises real component logic/wiring and doesn't silently pass after a refactor that keeps the mocked function's shape but breaks real integration.
4. **Coverage thresholds are a floor, not a target.** A threshold catches *regression below a floor*; it does not prove the newly-added code at 100% coverage actually asserts on error/loading states (see `pyramid-shape-and-coverage.md`). Do not treat "coverage threshold met" as equivalent to "critical path covered."
5. **Watch-mode/local dev ergonomics should not leak into CI config.** Settings tuned for fast local iteration (e.g. skipping type-checking, relaxed isolation) need an explicit, separate CI config — verify the CI config path, don't assume local config is what runs in the pipeline.

## Response discipline

When reviewing unit/component tests, cite the specific query/mock/assertion pattern found (with file reference), state which Testing Library priority tier the primary queries fall into, and label whether a coverage-threshold or config-semantics claim is `documentation-based` (grounded via Context7/official docs this session) or `inference`.
