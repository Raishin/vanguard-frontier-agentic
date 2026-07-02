# ACT Rules Detection Boundary

Use this reference when a finding needs an explicit machine-testable-vs-manual-only determination — i.e. when someone (a developer, a manager, a compliance owner) asks "can't a tool just catch this?" and the honest answer needs a citation, not a shrug.

## What people get wrong

The naive story is:

> "We ran the scanner and it's clean, so we're WCAG conformant."

Wrong, and documented as wrong. W3C's own Accessibility Conformance Testing (ACT) Rules Format exists precisely because only a subset of WCAG success criteria have rules that can be fully automated; many ACT rules are explicitly scoped as semi-automated (tool narrows candidates, human confirms) or cannot be automated at all. Context7-grounded testing-methodology sources (web.dev's accessibility-testing curriculum) list this as a documented pro/con of automated tooling, not a vendor limitation of any one scanner — every axe-core/Lighthouse-class tool inherits the same ceiling because the underlying criteria themselves are not all mechanically decidable.

## Officially grounded shape

ACT Rules (`https://www.w3.org/WAI/standards-guidelines/act/rules/`) each declare:

1. **Applicability** — the exact DOM/CSSOM condition the rule fires on.
2. **Expectation** — what must be true for the applicable element to pass.
3. **Accessibility Support** — known AT/browser caveats affecting the rule's validity.
4. **Test cases** — passed/failed/inapplicable examples used to validate rule implementations across tools.

Each ACT rule maps to one or more WCAG 2.x SC. A rule existing does not mean full SC coverage — most SC have partial automation at best (see the Detect column in `references/wcag22-sc-index.md`).

## Non-negotiable determination rule

For any finding, before closing it as "automated-clean" or escalating it as "needs manual review," answer:

1. Does a published ACT rule exist for this exact condition? If yes, cite its rule id/URL.
2. Does the ACT rule's *Applicability* actually match this element/pattern, or only a superficially similar one? (Common false-negative source: a rule scoped to `<img>` does not cover a CSS `background-image` used as content.)
3. Does the ACT rule's *Expectation* fully capture the SC's intent, or only a necessary-but-not-sufficient subset? (Example: a rule confirming an `alt` attribute exists does not confirm the alt text is accurate — that remains manual.)
4. If no ACT rule exists or covers only part of the SC, label the remainder explicitly `manual-required` rather than silently treating tool silence as a pass.

## Machine-testable vs manual-only quick reference

**Reliably rule-coverable (cite the ACT rule id when available):**
- Missing `alt` attribute presence (not correctness)
- Empty/duplicate `id` attributes
- `<html>` missing/invalid `lang`
- Color contrast ratio computation (given final rendered colors)
- Missing accessible name on form controls (presence, not adequacy)
- `<meta name="viewport">` blocking zoom (`user-scalable=no`, restrictive `maximum-scale`)
- Duplicate landmark of the same type with no distinguishing label (structural detection only)

**Semi-automated (tool flags candidate, human confirms):**
- Heading-level skips (tool detects the skip; human confirms whether it's a real structural break or acceptable visual-only heading)
- `tabindex` positive values (tool detects; human confirms actual focus-order impact)
- `aria-hidden` on focusable descendants (tool detects the conflict; human confirms it's not an intentional, correctly-managed dynamic pattern)
- Redundant/conflicting ARIA role vs native semantics

**Manual-only (no meaningful automation exists):**
- Whether alt text, link text, or error messages are actually meaningful/accurate
- Keyboard operability of a custom widget's full interaction model (arrow keys, Escape, Home/End per its APG pattern)
- Focus order matching visual/logical reading order
- Whether status messages are announced without stealing focus (4.1.3)
- Cognitive-load and plain-language adequacy (3.1.5-adjacent, readability)
- Actual screen-reader announcement correctness across NVDA/JAWS/VoiceOver

## When to push back

Push back if the user asks to:

- close a finding as "resolved" solely because a scanner stopped flagging it, when the underlying SC has no full-automation ACT rule for that condition,
- ship a compliance deliverable that cites "0 automated findings" as equivalent to "0 findings" — state the manual-only gap explicitly in the same sentence,
- treat every tool-flagged item as guaranteed-true without checking Applicability match — false positives are a documented characteristic of automated accessibility tooling, not an edge case.
