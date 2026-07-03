# Heading and Landmark Structure Rules

Use this reference when auditing the document outline of a page or component tree — heading hierarchy, landmark regions, and how components compose into a whole page's navigation structure.

## What people get wrong

The common bad assumption is:

> "Headings are for visual size; I'll pick whichever `<h*>` looks right at this font size, and CSS will fix the rest."

Wrong. Screen-reader users navigate primarily by heading level and landmark region — jumping "next heading," "next level-2 heading," or "next landmark" without reading surrounding text. A heading chosen for visual weight instead of document structure breaks that navigation model even when the page looks fine visually.

## Non-negotiable design rules

### 1. Heading levels must not skip

`<h1>` → `<h3>` with no `<h2>` between them is a structural break, not a style inconsistency. It reads to a screen-reader user as "I jumped past something — did I miss content?" Every component that renders a heading must accept its heading level as a prop/parameter from its parent context rather than hardcoding a level, so composed pages stay contiguous.

### 2. Exactly one `<h1>` per page (not per component)

A component library that hardcodes `<h1>` internally breaks the instant it's composed twice on one page, or nested inside a page that already has its own `<h1>`. Components should default to accepting a level and render the semantically-correct `<h1>`–`<h6>` element for that level (not just visually restyle a `<div>`).

### 3. Landmarks must be unique-per-page or explicitly labeled

A page may have only one unlabeled `<main>`, one unlabeled `<banner>` (`<header>` at the top level, not nested in `<article>`/`<section>`), and one unlabeled `<contentinfo>` (`<footer>` at the top level). Multiple instances of the same landmark type (e.g., two `<nav>` regions — primary nav and breadcrumb nav) MUST each carry a distinct accessible name via `aria-label` or `aria-labelledby`, or AT users hear "navigation, navigation" with no way to distinguish them.

### 4. Prefer semantic elements over `role=` on `<div>`

`<nav>`, `<main>`, `<header>`, `<footer>`, `<aside>`, `<section>` (with an accessible name) carry implicit landmark roles. Adding `role="navigation"` to a `<div>` works but is strictly worse than using `<nav>` — it adds a maintenance burden (the role can drift out of sync with a renamed/refactored element) for no benefit. Flag `role=` landmark attributes on generic elements as a downgrade unless there's a documented reason (e.g., legacy browser support matrix) — see MDN ARIA landmark-role guidance via the Context7 Documentation Protocol in SKILL.md for the "semantic HTML first" rule.

### 5. `<section>` and `<aside>` need an accessible name to count as landmarks

A bare `<section>` with no `aria-label`/`aria-labelledby`/heading-derived name does not expose as a landmark to most assistive technology — it is a generic grouping only. Do not report "landmark added" unless the accessible-name requirement is also met.

## Minimal safe audit flow

1. Extract the full heading tree (level + text) for the page or component in isolation.
2. Extract the full landmark tree (type + accessible name, or "unlabeled" if none).
3. Diff both trees against the pre-change version when reviewing a delta; against W3C rules from scratch when reviewing a new page.
4. Flag: any level skip, any second unlabeled instance of a unique-landmark type, any component that hardcodes a heading level instead of accepting one from its parent.
5. For component-tree-only reviews (no full-page context available), state explicitly that heading-level correctness cannot be fully verified without knowing the mount context, and flag it as a residual risk rather than asserting compliance.

## Verification targets

- Render (or read) the full page and extract the accessibility tree / heading outline via a real DOM inspection when live evidence is available; note this as `live evidence`.
- When only source is available (no live render), derive the outline from JSX/template structure and label the finding `inference` — a dynamically-computed heading level (e.g., `level={depth + 1}`) cannot be fully verified statically without knowing all call sites.

## When to push back

Push back if the user says:

- "just use `<div class="h2">`, we'll style it to look right" — that produces zero navigable structure for AT users regardless of visual styling,
- "we don't need `aria-label` on the second nav, screen-reader users will figure it out from context" — they cannot; landmark lists present regions out of visual context,
- "heading levels don't matter as long as visually it looks like a hierarchy" — visual hierarchy and semantic hierarchy are independent axes; both must be correct.
