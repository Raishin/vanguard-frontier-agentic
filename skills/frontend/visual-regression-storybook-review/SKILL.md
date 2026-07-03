---
name: visual-regression-storybook-review
description: Reviews Storybook visual-testing setup -- test-runner wiring, Chromatic integration, and the a11y addon's axe-core gating -- to ensure visually-critical components have deterministic pixel-diff and accessibility coverage before merge, grounded in current Storybook docs.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: delivery
---

# Visual Regression (Storybook) Review

## Purpose

Storybook can run three different kinds of automated checks -- the generic `test-runner`, hosted Chromatic visual/interaction diffing, and the `a11y` addon's axe-core accessibility checks -- and teams frequently conflate them or wire only one when they need two. This skill reviews which checks are actually wired, whether they gate merge or are advisory-only, and whether visually-critical components and all theme variants are in scope, grounded in current, version-specific Storybook API behavior rather than remembered config shapes that may be stale across majors.

## When to use

Use this skill when the user asks to:

- review or configure Storybook's `test-runner`, Chromatic, or `a11y` addon setup,
- diagnose why a visual or accessibility regression shipped despite Storybook tests passing,
- decide whether a check should run locally, in the test-runner, or via Chromatic,
- audit whether dark mode/RTL/reduced-motion story variants have visual and a11y coverage.

## Context7 Documentation Protocol

Storybook's addon config shape (`preVisit`/`postVisit` hooks, `parameters.a11y.*`, `chromatic.config.json` fields) has changed across Storybook 8/9/10 and is documented, not folklore -- never assert a hook signature, parameter name, or default value from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded in this session.
2. Call `mcp__Context7__resolve-library-id` with library name `Storybook` to obtain the current Context7-compatible ID (`/storybookjs/storybook`); prefer the resolved ID over guessing.
3. Call `mcp__Context7__query-docs` for the specific claim in question -- e.g. "test-runner preVisit postVisit hooks configuration", "a11y addon parameters.a11y.test values", "Chromatic config.json fields and CI wiring" -- before stating it as fact. Do this per review, not once from a prior session's memory.
4. Prefer the official docs URLs in `official_docs` for primary normative statements (exact parameter names, exact config shape); use Context7 to ground and cross-check the claim before writing it into a finding.
5. If Context7 is unavailable or returns no relevant match, fall back to the `official_docs` URLs and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
6. Never invent a config key, addon parameter, hook name, or axe-core rule ID that no queried source confirms.

## Lean operating rules

- Distinguish the three tools explicitly: `test-runner` (generic CI test harness, runs locally or in CI via Playwright-driven `preVisit`/`postVisit` hooks), Chromatic (hosted visual + interaction diffing with a reviewer UI and git-provider sync), and the `a11y` addon (axe-core checks configured through `parameters.a11y.*`) -- do not treat them as interchangeable or assume one subsumes the others.
- Confirm whether `parameters.a11y.test` is set to `'error'` (fails the build on violations) rather than left unset or at `'todo'` (warns only) before crediting a project with enforced accessibility gating; `'off'` disables the check entirely except for manual panel review.
- Require that visually-critical stories include dark mode, RTL, and `prefers-reduced-motion` variants in the checked set, not just the default light theme -- a diff/a11y suite that only ever renders the default theme systematically misses regressions in every other supported mode.
- Verify the exact addon/config API shape (e.g., `preVisit`/`postVisit` hook signatures, `injectAxe`/`configureAxe`/`checkA11y` from `axe-playwright`, or `parameters.a11y.context`/`config`/`options`) against the installed Storybook major version before recommending a `.storybook/test-runner.ts` or `preview.ts` diff, since these shapes have changed across Storybook 8/9/10.
- Recommend masking or mocking non-deterministic content (timestamps, animations, randomized data, live network responses) before recommending a looser pixel-diff tolerance -- widening tolerance first hides the actual source of visual noise rather than fixing it.
- Treat the test-runner and Chromatic as complementary, not redundant: the test-runner can run custom assertions locally and in any CI, while Chromatic adds hosted visual/interaction diffing with reviewer approval and git sync; a project that has only one may still have a real coverage gap depending on what it needs.
- Load the design-token-governance skill instead of this one when the root cause is a token/contrast issue rather than a Storybook wiring gap; load `wcag-22-accessibility-audit` when the question is about accessibility compliance strategy broader than what the `a11y` addon checks.

## References

Load these only when needed:

- [Test-runner and Chromatic wiring](references/test-runner-and-chromatic-wiring.md) -- use when reviewing or configuring the `test-runner`'s `preVisit`/`postVisit` hooks, CI invocation, or Chromatic project setup and the distinction between the two.
- [Accessibility addon gating](references/accessibility-addon-gating.md) -- use when reviewing or configuring the `a11y` addon's `parameters.a11y.test` gating behavior, axe-core rule configuration, or diagnosing why violations aren't failing CI.
- [Theme and variant coverage](references/theme-and-variant-coverage.md) -- use when auditing whether dark mode, RTL, and reduced-motion story variants have visual/a11y coverage, or when non-deterministic content needs masking before diffing.

## Response minimum

Return, at minimum:

- which of the three tools (test-runner/Chromatic/a11y addon) is in scope and whether it currently gates merge,
- theme/variant coverage gap (dark mode, RTL, reduced-motion) if any,
- evidence level and the Storybook major version the guidance targets,
- proposed config diff (not applied),
- security caveat on any Chromatic/Percy token or PII-bearing baseline reviewed.
