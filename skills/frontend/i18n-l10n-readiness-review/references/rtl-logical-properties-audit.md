# RTL and Logical-Property Layout Audit

Use this reference only when reviewing `lang`/`dir` attribute propagation, CSS logical
properties, or directional-iconography readiness for RTL markets.

> Version note: CSS logical-property support and the exact `dir="auto"` / `unicode-bidi`
> interaction are spec/browser-support-sensitive. Verify current support and behavior
> against the W3C Internationalization Techniques guidance and current MDN CSS logical
> properties documentation before asserting a given property is safe to rely on.

## What people get wrong

The common bad assumption is:

> "RTL support means flipping the whole page with `dir="rtl"` on `<html>`, and CSS
> `transform: scaleX(-1)` or a mirrored stylesheet handles the rest."

That is incomplete in two directions:

1. **Physical CSS properties don't flip automatically.** `margin-left`,
   `padding-right`, `left`, `text-align: left` are all physical — they do not
   reverse when `dir` changes. A layout built entirely from physical properties will
   have `dir="rtl"` applied but visually remain LTR-positioned (padding stays on the
   same physical side), which is often worse than doing nothing, because it looks
   half-implemented.
2. **Not everything should mirror.** Numerals, code snippets, logos, and some icon
   families (e.g., a play button) are directionally invariant and should *not* flip;
   only directionally-meaningful content (arrows indicating navigation direction,
   "next/back" chevrons, progress bars implying forward motion) should mirror. A
   blanket `scaleX(-1)` on the whole page over-mirrors and under-mirrors
   simultaneously (it flips text glyphs and logos it shouldn't, while CSS engines
   already handle text shaping/bidi separately from that transform).

## Officially grounded shape (W3C Internationalization guidance)

- The `dir` attribute belongs on `<html>` at minimum, and should be set from the
  active locale's script directionality, not hardcoded — the W3C guidance on the
  `dir` attribute and the CSS `direction` property covers this split: `dir` is the
  HTML-level semantic direction; `direction`/logical properties are the CSS-level
  mechanism that responds to it.
- CSS logical properties (`margin-inline-start`, `margin-inline-end`,
  `padding-block-start`, `inset-inline-start`, `border-inline-end`, `text-align:
  start`/`end`) are defined relative to the *flow direction*, so a single stylesheet
  written in logical properties adapts to `dir="ltr"` or `dir="rtl"` without a mirrored
  stylesheet or a JS-driven layout flip.
- Bidi (bidirectional text) isolation matters even in an LTR-primary UI that embeds
  RTL user content (e.g., a name or comment in Arabic inside an English UI) —
  unisolated bidi runs can visually corrupt adjacent punctuation/numbers. Use `dir`
  scoping or Unicode bidi-isolation characters/CSS `unicode-bidi: isolate` around
  user-generated content of unknown directionality, not just at the page root.

## Non-negotiable review rules

1. **Verify `dir` is set dynamically from the active locale's script, and actually
   reaches the rendered `<html>` element** — not just passed as a prop to a component
   that never forwards it to the DOM root. A `dir` value trapped in application state
   without reaching `<html dir>` does nothing for native browser bidi/text-alignment
   behavior.
2. **Grep for physical CSS properties in layout-critical rules** (`margin-left`,
   `margin-right`, `padding-left`, `padding-right`, `left`, `right`, `text-align: left`,
   `text-align: right`, `float: left`, `float: right`, `border-left`, `border-right`) —
   each is a candidate defect if RTL is in scope or roadmapped. Flag with file:line;
   do not silently "fix" — this skill reviews, it doesn't rewrite the codebase.
3. **Distinguish directionally-meaningful icons from directionally-invariant ones.**
   Flag navigation/progression icons (back/forward chevrons, "next step" arrows) that
   lack any RTL-mirroring mechanism (a `dir`-aware icon variant, CSS `transform:
   scaleX(-1)` scoped narrowly to that icon under `[dir="rtl"]`, or an icon library
   with built-in bidi awareness). Do not flag logos, brand marks, or numerals as
   needing mirroring.
4. **Check bidi isolation for embedded user-generated or mixed-direction content**
   (usernames, free-text fields, search queries) rendered inside a primarily
   single-direction UI — flag missing isolation (`unicode-bidi: isolate`, `dir="auto"`
   on the specific element, or equivalent) as a defect distinct from the page-level
   `dir` question.
5. **Do not accept a single manually-mirrored RTL stylesheet as a substitute for
   logical properties** if the codebase is under active development — a parallel
   mirrored stylesheet requires ongoing double-maintenance and drifts; flag it as a
   maintainability/regression risk even if it currently renders correctly, and
   recommend migration to logical properties as the structural fix.

## Minimal safe audit flow

1. Confirm where `dir` is set (locale config, i18n library, manual state) and trace
   whether it reaches `<html dir>` in the actual rendered output (grep component tree /
   root layout, not just the i18n config).
2. Grep CSS/CSS-in-JS/Tailwind classes for physical directional properties in
   layout-critical files; note density (a handful of instances vs. pervasive use)
   since that changes remediation scope and urgency.
3. Grep for directional icon usage (chevron/arrow component names, icon library
   imports for "next"/"back"/"forward"/"previous") and check for any `dir`-aware
   handling.
4. Grep for free-text/user-generated-content rendering sites and check for bidi
   isolation handling.
5. Record findings as file:line, the specific physical-property or missing-isolation
   instance, and whether RTL is currently shipped, roadmapped, or out of scope (which
   changes finding severity but not whether it's recorded).

## Adversarial checklist

Before clearing layout as "RTL-ready," answer these:

- Does `dir` actually reach `<html>`, or only live in component props/i18n state?
- What fraction of layout-critical CSS uses physical vs. logical properties?
- Which icons are directionally meaningful, and do any of them lack a mirroring
  mechanism?
- Is there any user-generated or mixed-direction text rendered without bidi isolation?
- Is RTL support implemented via a parallel mirrored stylesheet (maintenance/drift
  risk) or via logical properties (structural fix)?

If you cannot answer these, the RTL audit is incomplete — say so rather than declaring
readiness.

## When to push back

Push back if the user asks to:

- "just flip the whole page with `transform: scaleX(-1)` and call it RTL support" —
  that over-mirrors text/logos and under-addresses logical layout flow; it is not a
  substitute for `dir` + logical properties.
- defer `dir`/logical-property work indefinitely because "we're not launching RTL
  markets yet" while simultaneously claiming full i18n/l10n readiness — readiness
  claims should state RTL scope explicitly (in scope, roadmapped, or explicitly out of
  scope) rather than being silently omitted.
- treat a manually-mirrored RTL stylesheet as a permanent solution rather than a
  stopgap, when the codebase is under active development and will keep drifting.
