# Accessibility Addon (a11y) Gating

Use this reference when reviewing or configuring the `a11y` addon's `parameters.a11y.test` gating behavior, axe-core rule configuration, or diagnosing why accessibility violations aren't failing CI despite the addon being installed. For test-runner/Chromatic wiring, see `test-runner-and-chromatic-wiring.md`. For dark mode/RTL/reduced-motion coverage of the stories being checked, see `theme-and-variant-coverage.md`.

> Version note: `parameters.a11y.test` and the `initialGlobals`/`globals.a11y.manual` flag are current documented shapes; the addon's config surface has changed across Storybook majors (notably the move to `parameters.a11y.test` gating via the Vitest addon or test-runner). Verify the exact parameter names against the installed Storybook version via Context7 (`/storybookjs/storybook`) or the official docs before writing a config diff.

## What people get wrong

The common bad assumption is:

> The `a11y` addon is installed, so accessibility violations block merge.

That is false by default. The addon being installed only means violations are visible in the Storybook UI's a11y panel during manual browsing. Whether a violation actually fails a build depends entirely on the `parameters.a11y.test` value, which is `undefined` (no enforced test behavior) unless explicitly set.

## Officially grounded shape

The accessibility addon is built on `axe-core`; its configuration options largely map to axe-core's own API surface:

| Property | Default | Description |
|---|---|---|
| `parameters.a11y.context` | `'body'` | Context passed to `axe.run` -- which elements checks run against. |
| `parameters.a11y.config` | (empty) | Configuration passed to `axe.configure()` -- most commonly used to enable/disable individual rules. |
| `parameters.a11y.options` | `{}` | Options passed to `axe.run` -- can adjust which rulesets are checked. |
| `parameters.a11y.test` | `undefined` | Determines test behavior when run with the Vitest addon or the test-runner. |
| `globals.a11y.manual` | `undefined` | Set `true` to prevent a story from being automatically analyzed when visited. |

`parameters.a11y.test` accepts exactly three documented values:

- `'off'` -- do not run accessibility tests automatically (manual panel review is still possible).
- `'todo'` -- run accessibility tests; violations surface as a **warning** in the Storybook UI, not a failing test.
- `'error'` -- run accessibility tests; violations surface as a **failing test** in the Storybook UI and CLI/CI.

This parameter can be set at the project level (`.storybook/preview.ts`), the component level (a story file's meta/default export), or an individual story level -- so a project can have global `'error'` gating with a deliberate `'todo'` or `'off'` override on a specific known-noisy story, or the reverse (global `'todo'` with `'error'` only on a few critical components). Either shape is valid; the review's job is to confirm which shape actually exists and whether it matches intent.

For test-runner-based enforcement (as opposed to the Vitest addon), the documented pattern injects and configures axe via the `axe-playwright` package inside the `preVisit`/`postVisit` hooks:

```typescript
import type { TestRunnerConfig } from '@storybook/test-runner';
import { getStoryContext } from '@storybook/test-runner';
import { injectAxe, checkA11y, configureAxe } from 'axe-playwright';

const config: TestRunnerConfig = {
  async preVisit(page) {
    await injectAxe(page);
  },
  async postVisit(page, context) {
    const storyContext = await getStoryContext(page, context);
    await configureAxe(page, {
      rules: storyContext.parameters?.a11y?.config?.rules,
    });
    const element = storyContext.parameters?.a11y?.element ?? 'body';
    await checkA11y(page, element, {
      detailedReport: true,
      detailedReportOptions: { html: true },
    });
  },
};
export default config;
```

Note this is a distinct enforcement path from `parameters.a11y.test` (which governs the addon's own Vitest-addon/test-runner-integrated behavior) -- a project wiring axe manually through `axe-playwright` in `preVisit`/`postVisit` is not automatically respecting `parameters.a11y.test`, and the two should not be assumed to be reading the same configuration unless the `postVisit` hook explicitly reads `storyContext.parameters?.a11y?.config`.

## Non-negotiable design rules

1. **Never credit "a11y addon is installed" as accessibility gating without confirming `parameters.a11y.test` is `'error'`** (or that a manual `axe-playwright` wiring in `postVisit` calls `checkA11y` in a way that actually fails the test runner process, not just logs a report). `'todo'` and the addon's default unset state are advisory only.
2. **Confirm which enforcement path is in use** -- `parameters.a11y.test` via the Vitest addon/test-runner integration, or a manually wired `axe-playwright` `preVisit`/`postVisit` pair -- before writing a config diff; they are configured differently and a fix aimed at the wrong path silently does nothing.
3. **Do not recommend disabling a rule globally via `parameters.a11y.config.rules` to silence a violation** without first confirming the violation is a false positive for the actual DOM/ARIA semantics in question, not a real defect the team wants to suppress.
4. **Respect `globals.a11y.manual: true` as an intentional opt-out signal**, not dead config -- confirm with the team whether a story marked `manual` is deliberately excluded (e.g., because it renders non-deterministic third-party content) before treating its absence from automated coverage as a gap to fix by removing the flag.
5. **Scope `parameters.a11y.context` deliberately** when a story wraps content the team doesn't own (e.g., a third-party embed) -- running the full-page `axe.run` against unowned markup produces violations the team cannot fix and trains reviewers to ignore the panel.

## Safe verification targets

- `.storybook/preview.ts` -- confirm the project-level `parameters.a11y.test` value and any `initialGlobals.a11y.manual` default.
- Individual story files -- confirm component/story-level `parameters.a11y` overrides match documented intent (not accidental inheritance).
- `.storybook/test-runner.ts` -- if `axe-playwright` is used, confirm `preVisit` calls `injectAxe` and `postVisit` calls `checkA11y` with a real failure path (not a report-only call whose result is discarded).
- CI logs from a Storybook build with a known-injected violation -- the most reliable evidence that gating is real is a CI run that actually failed on a deliberate violation, not just config inspection.

## When to push back

Push back if the user asks for:

- treating `'todo'` as sufficient gating because "it shows up in the UI,"
- disabling a specific axe rule project-wide to unblock a single component's merge, as a permanent fix rather than a scoped, justified exception,
- adding the `a11y` addon without setting `parameters.a11y.test` anywhere, and calling that "accessibility testing is now covered."

Visibility in a panel is not the same as a merge-blocking test.
