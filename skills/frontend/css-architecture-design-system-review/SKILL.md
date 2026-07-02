---
name: css-architecture-design-system-review
description: Review CSS for specificity and cascade-layer discipline, design-token (custom-property) conformance, and responsive strategy correctness (container queries vs. media queries), catching specificity wars, hardcoded-value token drift, and non-reflowing layouts that fail WCAG 1.4.10/1.4.4 before they compound into unmaintainable stylesheets.
allowed-tools: Read Grep Glob Bash(git diff:*) WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# CSS Architecture & Design System Review

## Purpose

CSS at scale degrades through two silent mechanisms: specificity creep (engineers reaching for `!important` or ID selectors to win cascade fights instead of fixing the underlying layer structure) and design-token drift (hardcoded values reappearing next to a token system, diverging visual output over time). Neither shows up as a compile error or a failing test — they show up eighteen months later as an unmaintainable stylesheet nobody wants to touch. This skill catches both at review time, plus the responsive-strategy and WCAG visual-presentation failures that are CSS's specific accessibility responsibility.

## When to use

Use this skill when the user asks to:

- review a CSS/component-styling diff for specificity, cascade, or maintainability issues,
- audit a codebase's design-token (CSS custom-property) conformance,
- decide between container queries and media queries for a responsive pattern,
- establish or enforce a cascade-layer strategy (`@layer reset, base, tokens, components, utilities, overrides`),
- check layout resilience against WCAG 2.2 reflow (1.4.10) and resize-text (1.4.4) criteria.

Do not use this skill for:

- JavaScript/framework component architecture — use the matching framework-specific review skill instead,
- live visual-regression testing or contrast-ratio measurement — that requires a runtime tool, not static review,
- choosing a CSS methodology from scratch with no existing code — that is a greenfield design conversation, not a review.

## Context7 Documentation Protocol

- Resolve the MDN Web Docs library ID with `resolve-library-id` (matched result: `/mdn/content`) before citing any cascade-layer, container-query, or custom-property specificity claim.
- Before asserting cascade-layer precedence, `!important` interaction, or container-query browser-support behavior, call `query-docs` against `/mdn/content` and cite the section — do not assert from memory. Cascade-layer `!important` precedence is inverted relative to normal-declaration precedence and is easy to misstate without checking.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Never assume a browser-support baseline (e.g., "container queries are safe everywhere now") without checking current docs; treat support claims as version/date-sensitive, not fixed facts.

## Lean operating rules

- Never approve new `!important` without a documented cascade-layer justification — it is a symptom that the layer/selector structure should be fixed, not a valid quick fix. Note: `!important` precedence is inverted across layers (earlier-declared layers win for important declarations), so verify actual winning behavior against the declared `@layer` order rather than assuming a simple override.
- Never approve new ID-selector styling for components; treat it as a specificity-escalation risk that will require `!important` or worse to override later.
- In any codebase with an established design-token system, flag hardcoded color/spacing/typography values and name the nearest token substitute — do not silently allow token drift. Distinguish token tiers (primitive → semantic → component) when recommending a substitute; do not point a component-level override at a raw primitive if a semantic token exists.
- Do not bikeshed methodology (BEM vs. utility-first vs. CSS Modules vs. cascade layers) when a convention is already established; enforce consistency with what exists over personal preference.
- Query current MDN/W3C docs for `@layer`/`@container` support and semantics before ruling — container queries only reached broad Baseline status in 2023 and cascade-layer support nuances (especially `!important` interaction) vary; never assert current support from memory.
- Distinguish container queries (component depends on its container's size) from media queries (component depends on the viewport) — recommending the wrong one for the actual dependency is a correctness bug, not a style preference. A component intended for reuse across differently-sized containers (sidebar, main content, modal) needs a container query; a page-level layout shift needs a media query.
- Never treat CSS as an access-control boundary; `display:none`/`visibility:hidden` hiding an affordance is not a substitute for server-side authorization. Flag this as a HIGH-severity finding, not a style note, whenever hidden-but-present markup carries privileged data or actions.
- Flag anything needing live visual-regression or contrast-ratio measurement as a residual risk requiring live-runtime verification, not asserted from static analysis alone.
- Flag attribute-selector-plus-background-image patterns (e.g. `input[value^="a"] { background: url(...) }`) as a potential CSS-based data-exfiltration vector — they can leak user-controlled attribute values via network requests.
- Flag third-party `@import` rules with no accompanying Subresource Integrity or CSP `style-src` consideration as a supply-chain risk, not a performance nitpick.
- Never execute, build, or run application code, and never fetch live pages to "see how it renders" without the user's explicit ask; this is a static-review skill augmented only by documentation lookups (Read/Grep/Glob plus scoped `git diff` and `WebFetch` for docs/spec grounding).

## References

Load these only when needed:

- [Cascade layer strategy](references/cascade-layer-strategy.md) — use when establishing or auditing a `@layer` order for a codebase, or resolving a specificity conflict between two legitimate rules.
- [Design token conformance patterns](references/token-conformance.md) — use when auditing hardcoded-value drift against an existing custom-property token system, including token-tier (primitive vs. semantic vs. component) conventions.
- [Responsive strategy decision guide](references/responsive-strategy.md) — use when deciding container query vs. media query, or auditing reflow/resize-text compliance at 400%/200% zoom.

## Response minimum

Return, at minimum:

- the specificity/cascade verdict per flagged selector, with its computed specificity value when relevant,
- a token-conformance report distinguishing token-referenced values from hardcoded ones,
- the responsive-strategy verdict (container query vs. media query appropriateness) for any new responsive rule,
- the recommended cascade-layer placement for new rules,
- residual risk notes for anything requiring live visual-regression or contrast-checker verification beyond this static review.
