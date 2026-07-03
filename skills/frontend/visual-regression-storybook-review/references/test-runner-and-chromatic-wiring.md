# Test-Runner and Chromatic Wiring

Use this reference when reviewing or configuring the Storybook `test-runner`'s `preVisit`/`postVisit` hooks, its CI invocation, Chromatic project setup, or when a user is unclear on the difference between the two. For axe-core/a11y gating specifics, see `accessibility-addon-gating.md`. For theme/variant coverage, see `theme-and-variant-coverage.md`.

> Version note: hook signatures and helper exports (`getStoryContext`, `waitForPageReady`) live in `@storybook/test-runner` and have evolved across Storybook majors. Verify exact exports and signatures against the installed version via Context7 (`/storybookjs/storybook`) or the official docs before citing one as available.

## What people get wrong

The common bad assumption is:

> "We have Storybook tests" means visual regressions are covered.

That is not necessarily true. A project can have:

- a `test-runner` that only asserts `expect(canvas).toBeInTheDocument()`-style smoke checks, with no image snapshot step at all,
- Chromatic connected but running in "notify only" mode with no branch protection requiring the check,
- both tools installed, but neither one covering the components the team actually considers visually critical.

"Storybook tests exist" and "visual regressions are gated" are different claims. Confirm which one is actually true before crediting a project with coverage.

## Officially grounded shape

Per Storybook's own documentation, the `test-runner` and Chromatic are described as complementary, not redundant:

- **The `test-runner`** is a generic testing tool, built on Playwright, that can run locally or in CI and be configured or extended to run "all kinds of tests." It exposes a `preVisit`/`postVisit` hook API in `.storybook/test-runner.ts`:
  - `setup()` runs once before the test runner starts.
  - `preVisit(page, context)` runs before a story is rendered; `page` is Playwright's page object, `context` carries the story's id/title/name.
  - `postVisit(page, context)` runs after a story is fully rendered; this is where image-snapshot assertions and `getStoryContext(page, context)` (to read the story's full parameters/args/argTypes) typically go.
  - `waitForPageReady(page)` is a documented helper for image-snapshot testing that waits for the page -- including async items like images and fonts -- to finish loading before a screenshot is taken; skipping it is a common source of flaky screenshot diffs on slow-loading assets.
- **Chromatic** is a cloud-based service that runs visual and interaction tests without the team having to set up the test-runner itself. It syncs with the git provider and manages access control for private projects. Storybook's own docs describe using the test-runner locally and Chromatic in CI as a valid pairing: use Chromatic for visual and component tests, and the test-runner for other custom tests.
- Chromatic project configuration lives in `chromatic.config.json` (fields such as `projectId`, `buildScriptName`, `zip`, `debug`) rather than being hardcoded into CI invocation flags.

## Non-negotiable design rules

1. **Do not credit "Storybook tests pass" as visual coverage without confirming a diffing step exists.** A `test-runner` config with no image-snapshot assertion in `postVisit`, and no Chromatic connection, has zero visual-regression coverage regardless of how many stories exist.
2. **Confirm the check is required, not advisory.** A connected Chromatic project that is not set as a required status check on the default branch's protection rules can be bypassed by any merge; "connected" and "gating" are different claims and must be verified separately (repo evidence: branch protection config, not just Chromatic dashboard presence).
3. **Use `waitForPageReady` (or an equivalent explicit wait) before any screenshot assertion in `postVisit`.** Taking a screenshot before fonts/images finish loading produces non-deterministic diffs that erode trust in the whole suite and lead teams to loosen thresholds instead of fixing the real timing bug.
4. **Do not run the test-runner without `preVisit`/`postVisit` review when adding a11y or visual checks.** These hooks are the only place axe injection and screenshot assertions can run per-story with access to `getStoryContext`; bolting checks on outside this API produces inconsistent coverage across stories.
5. **When both the test-runner and Chromatic exist, confirm they aren't duplicating the same check with different tolerances.** Two visual-diff mechanisms with different pixel-diff thresholds on the same components is a maintenance and false-confidence problem, not defense in depth.

## Safe verification targets

- `.storybook/test-runner.ts` (or `.js`) -- confirm `preVisit`/`postVisit` hooks exist and what they assert.
- `chromatic.config.json` and the CI workflow step invoking `chromatic` -- confirm `projectId` is present and the step is wired into a required CI job, not an optional/manual trigger.
- Branch protection rules (repo settings, not just CI YAML) -- confirm the Chromatic and/or test-runner CI check is marked required before merge.
- `package.json` `test-storybook` script and its CI invocation flags (e.g. `--ci`, coverage flags) -- confirm it's actually called in CI, not just defined locally.

## When to push back

Push back if the user asks for:

- widening a `test-runner` image-snapshot diff threshold as the first response to flaky screenshots, before confirming `waitForPageReady` or explicit content masking is in place,
- connecting Chromatic without also making it a required CI check, calling that "visual regression coverage,"
- removing the test-runner because "Chromatic covers it," without confirming Chromatic's scope actually includes the custom assertions the test-runner was running.

Those are shortcuts that produce the appearance of coverage without the substance.
