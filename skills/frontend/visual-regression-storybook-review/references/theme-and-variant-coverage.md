# Theme and Variant Coverage

Use this reference when auditing whether dark mode, RTL, and `prefers-reduced-motion` story variants have visual and accessibility coverage, or when non-deterministic content needs masking/mocking before a pixel diff is trustworthy. For the mechanics of the test-runner and Chromatic themselves, see `test-runner-and-chromatic-wiring.md`. For axe-core gating specifics, see `accessibility-addon-gating.md`.

## What people get wrong

The common bad assumption is:

> If the default-theme story passes visual and a11y checks, the component is covered.

That is incomplete. A component's rendered DOM, computed contrast ratios, and layout can all differ meaningfully across:

- **color scheme** (light vs. dark, or any custom theme the design system supports) -- contrast ratios computed by axe-core in a light-theme snapshot say nothing about the dark-theme token pairing, which is a distinct set of color values,
- **text direction** (LTR vs. RTL) -- logical layout bugs (icon placement, padding asymmetry, text truncation direction) frequently only appear in RTL and are invisible in an LTR-only snapshot,
- **motion preference** (`prefers-reduced-motion: reduce`) -- a component that only has an animated entrance story has no coverage confirming the reduced-motion fallback renders correctly, not just "does not animate."

A visual-regression suite that only ever renders the default light/LTR/motion-enabled variant of each component has verified one of potentially four-plus meaningfully different rendered states, and will not catch a regression introduced in any of the others.

## Officially grounded shape

Storybook does not automatically render every theme/direction/motion-preference permutation of a story -- coverage of these dimensions is a function of how many story variants exist and whether decorators or globals feed them into each render. Confirm, as repo evidence rather than assumption, whether:

- a dark-mode (or multi-theme) decorator/global exists and is exercised by dedicated stories, not just toggle-able in the manual Storybook UI,
- RTL stories exist using the project's actual RTL mechanism (e.g. a `dir="rtl"` wrapper decorator), not just documented as "supported" in a design doc,
- a `prefers-reduced-motion` variant exists, typically via a decorator that sets the media feature for the story's iframe, or via a dedicated reduced-motion story.

`parameters.a11y.context` (see `accessibility-addon-gating.md`) determines what markup axe-core inspects per story -- confirm it is not silently scoped in a way that excludes the very wrapper (e.g. an RTL `dir` wrapper) whose correctness is under test.

## Non-negotiable design rules

1. **Do not count a single default-theme story as coverage for "the component supports dark mode."** Supporting a capability in code and having a regression-tested story for it are different claims; only the latter catches a regression.
2. **Mask or mock non-deterministic content before adding any new theme/variant story to a visual diff, rather than after a flaky failure appears.** Timestamps, live avatars, randomized placeholder data, and animated content are common sources of false-positive diffs; the fix is explicit masking (e.g. a `mask` option, disabled animations, or seeded fixture data) applied at story-authoring time, not a loosened global threshold applied later.
3. **Treat RTL coverage as a layout-logic concern, not a translation concern.** An RTL story does not need translated text to be useful -- it needs the same content rendered with `dir="rtl"` to catch physical-vs-logical CSS property bugs (e.g. `margin-left` instead of `margin-inline-start`).
4. **Treat `prefers-reduced-motion` coverage as a distinct assertion from "does it animate," not a subset of it.** A story confirming the *entrance* animation looks right says nothing about whether the reduced-motion fallback (often a hard cut or fade) is also correct; test both if the component is animated.
5. **Prioritize theme/variant coverage by actual usage risk, not exhaustive permutation.** A component used only in an admin-only, LTR-only internal tool does not need RTL coverage; a design-system primitive used across public, internationalized surfaces does. Recommend coverage proportional to blast radius, not maximal coverage for its own sake.

## Safe verification targets

- Story files for visually-critical components -- confirm the presence (or absence) of theme/direction/motion story variants, not just the default export.
- `.storybook/preview.ts` decorators and globals -- confirm a theme-switching or `dir`-setting decorator exists and is actually wired into the stories claimed to cover it.
- The test-runner's `postVisit` hook or Chromatic's captured snapshot list -- confirm the theme/variant stories are actually included in the set of stories being diffed, not excluded by a `tags` filter or story-level skip.

## When to push back

Push back if the user asks for:

- claiming "dark mode is covered" based on a single manually-toggled preview in local dev, with no corresponding story or CI snapshot,
- adding an RTL story with translated placeholder text but no `dir="rtl"` wrapper, which tests translation rendering but not the actual logical-CSS layout bug class RTL coverage exists to catch,
- deferring reduced-motion coverage indefinitely on an animation-heavy component while treating it as low priority, without characterizing the actual user population affected (motion-sensitivity accommodations are an accessibility requirement, not a nice-to-have).

Partial coverage presented as complete coverage is worse than an honestly documented gap.
