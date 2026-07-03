---
name: html-semantics-accessibility-review
description: Review HTML markup and rendered DOM structure for correct native-element usage, valid heading/landmark hierarchy, and WAI-ARIA APG-conformant custom-widget patterns; produce a WCAG 2.2-grounded verdict with APG pattern citations for every custom interactive control, flagging anything that needs live screen-reader verification beyond static review.
allowed-tools: Read Grep Glob Bash(git diff:*) WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: compliance
---

# HTML Semantics & Accessibility Review

## Purpose

Automated a11y linters (axe, Lighthouse) catch roughly 30-50% of WCAG issues by design — they cannot verify whether a custom widget's keyboard interaction matches its intended pattern, whether ARIA correctly reflects (or wrongly overrides) native semantics, or whether a heading/landmark outline actually helps a screen-reader user navigate. This skill performs the manual, spec-grounded review that closes that gap: matching every custom interactive element against a specific WAI-ARIA APG pattern, verifying heading/landmark structure, and catching redundant or conflicting ARIA before it ships.

## When to use

Use this skill when the user asks to:

- review a pull request or component for HTML semantics or accessibility correctness,
- verify a custom widget (modal, dropdown, tabs, combobox, accordion, etc.) against WAI-ARIA APG patterns,
- audit heading level and landmark structure on a page or component tree,
- check whether ARIA roles/states/properties are correctly applied or redundant/conflicting with native semantics,
- assess WCAG 2.2 conformance risk for markup changes ahead of a compliance audit.

## Context7 Documentation Protocol

Element semantics, implicit ARIA-role mappings, and attribute-support tables change as the HTML and ARIA specs evolve — never assert them from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if they are not already loaded in this session.
2. Call `mcp__Context7__resolve-library-id` for the relevant documentation set — for this skill that is almost always MDN (`/mdn/content`, or `/websites/developer_mozilla_en-us` if the former lacks coverage for the query). Resolve the WAI-ARIA APG or WCAG spec source too if the review needs primary-spec wording rather than MDN's paraphrase.
3. Call `mcp__Context7__query-docs` for the specific element, role, or attribute in question — e.g. "implicit ARIA role for `<nav>`", "APG combobox keyboard pattern", "tabindex focus-order behavior" — before ruling on it. Do this per review, not once from memory of a prior session.
4. Prefer the official spec/MDN wording over this skill's own paraphrase when the two could be read to disagree; cite the resolved doc URL in the finding.
5. If Context7 is unavailable or returns no relevant match, fall back to the URLs in `official_docs` / `references/apg-pattern-index.md`, and explicitly mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
6. Never invent an ARIA role, state, property, or implicit-semantics mapping that no queried source confirms.

## Lean operating rules

- First rule of ARIA: no ARIA is better than bad ARIA. Prefer a native element (`<button>`, `<dialog>`, `<details>`, `<nav>`) over a JS-reimplemented equivalent with ARIA bolted on, unless the platform genuinely has no equivalent.
- Never accept 'axe/Lighthouse passed' as sufficient evidence for a custom interactive widget — manually match it against a specific APG pattern URL, including its full keyboard model (not just Tab/Enter — Escape, Home, End, Arrow keys per the pattern).
- Treat heading-level skips (h1→h3) and missing/duplicate landmarks as blocking findings, not style notes — they break screen-reader page-navigation shortcuts.
- Never approve positive `tabindex` values; require `tabindex="0"`/`"-1"` plus correct DOM order instead.
- Flag `aria-hidden="true"` on any ancestor of a focusable/interactive descendant — this creates an unreachable-but-focusable or reachable-but-hidden trap.
- Query current MDN/WAI-ARIA APG docs (see Context7 Documentation Protocol) for the specific element/role in question before ruling; element semantics and ARIA support tables change with spec updates — never assert support/behavior from memory.
- Anything requiring live screen-reader verification (actual NVDA/JAWS/VoiceOver behavior) gets flagged as a residual risk, not asserted as verified from static review alone.
- Label every claim as `live evidence`, `spec-cited`, `documentation-based`, or `inference` so the reviewer knows what's actually been verified vs. reasoned about.

## References

Load these only when needed:

- [WCAG success-criterion mapping](references/wcag-criterion-mapping.md) — use when a finding needs to be tied to a specific WCAG 2.2 success criterion for a compliance-audit deliverable.
- [APG pattern quick-index](references/apg-pattern-index.md) — use when identifying which APG pattern matches a given custom widget (modal, combobox, tabs, tree, menu, disclosure, etc.) before doing the deep keyboard-model comparison.
- [Heading and landmark structure rules](references/heading-landmark-rules.md) — use when auditing page-level or component-tree-level outline structure, including nested-landmark and multiple-landmark-of-same-type edge cases.

## Response minimum

Return, at minimum:

- the element/component in scope and its semantic verdict (native-element compliant / needs ARIA / needs restructure),
- for any custom interactive widget: the specific APG pattern URL cited, or an explicit flag that none matches and a native element should be used instead,
- the heading/landmark outline diff (before/after) when structure changed,
- every ARIA attribute added or removed, with its WAI-ARIA 1.2 role/state justification,
- residual risk notes for anything requiring live assistive-technology verification beyond this static review.
