---
name: wcag-22-accessibility-audit
description: Audit frontend markup, components, and design-system primitives against WCAG 2.2 Level A/AA success criteria and ARIA APG interaction patterns, separating automated-detectable violations from manual-verification-required items and flagging legal exposure, with reference material loaded progressively per success-criteria category.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: compliance
---

# WCAG 2.2 Accessibility Audit

## Purpose

Frontend teams routinely ship components that pass visual QA but fail assistive-technology usage — missing accessible names, keyboard traps, insufficient contrast, non-conformant custom widgets. This skill audits against WCAG 2.2's exact success criteria (all four principles: Perceivable, Operable, Understandable, Robust) and the ACT Rules' machine-testable/manual-only split, without dumping the full 87-criterion spec into every review; it loads only the success-criteria category and evidence tier relevant to the review in scope. It complements — and does not duplicate — `html-semantics-accessibility-review`, which owns deep native-element/ARIA-APG keyboard-pattern matching; this skill owns the full-spectrum SC conformance sweep, automated-vs-manual evidence separation, and legal-exposure triage.

## When to use

Use this skill when the user asks to:

- run a WCAG 2.2 conformance audit across a page, flow, or component library (not just a single widget's ARIA pattern),
- triage an accessibility bug report, audit finding, or demand letter against specific success criteria,
- prepare input for a VPAT / accessibility conformance statement,
- determine whether a finding is machine-testable (cite an ACT rule) or requires manual/assistive-technology verification before it can be closed as passing,
- assess legal/litigation exposure of a known or suspected accessibility gap.

## Context7 Documentation Protocol

WCAG 2.2 SC wording, technique/failure lists, and ACT rule coverage are living documents (WCAG 2.2 added SC in 2023; techniques and ACT rules are updated independently of the SC text) — never assert SC numbering, level, or technique applicability from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded in this session.
2. Call `mcp__Context7__resolve-library-id` for `web.dev` (prefer `/googlechrome/web.dev` or `/websites/web_dev_learn`) when grounding testing-methodology claims (automated vs manual coverage limits, tool selection factors) — these are documented, not folklore.
3. Call `mcp__Context7__query-docs` for the specific claim in question — e.g. "automated accessibility testing coverage limitations", "color contrast calculation formula", "focus not obscured success criterion" — before stating it as fact. Do this per audit, not once from a prior session's memory.
4. For primary normative text (exact SC wording, Level, technique/failure IDs, ACT rule text), prefer the W3C URLs in `official_docs` over Context7 paraphrase — Context7 is for grounding testing-methodology and coverage claims, not for replacing the normative spec text itself.
5. If Context7 is unavailable or returns no relevant match, fall back to the `official_docs` URLs and `references/` files, and mark the claim `documentation-based (Context7 unavailable)` instead of presenting it as freshly verified.
6. Never invent a WCAG 2.2 success-criterion number, Level, technique ID, or ACT rule ID that no queried source confirms.

## Lean operating rules

- First classify the review scope: full-conformance audit (all applicable SC) vs targeted-criteria review (e.g. "just check contrast and focus") vs single-finding triage. Do not run a full sweep when the user asked about one criterion.
- Separate every finding into exactly one of two evidence tiers — automated-detectable (contrast ratio, missing alt text, form-label association, duplicate IDs, missing document language, empty links/buttons) or manual-required (meaningful sequence, focus order intent, sensory characteristics, actual assistive-technology walkthrough, cognitive-load judgment calls) — and never present a manual-required item as closed by automated tooling alone.
- Automated tooling (axe-core-class scanners, Lighthouse) is documented as catching a minority of real-world WCAG issues and can produce false positives — treat a clean automated scan as a floor, not a conformance claim, and say so explicitly in the report.
- For any custom interactive widget, defer keyboard-pattern/ARIA-role correctness to `html-semantics-accessibility-review`'s APG matching rather than re-deriving it here; this skill's job is mapping the widget's *failure* to the correct SC and Level, and the litigation-exposure severity, not re-doing the pattern audit.
- Never recommend an accessibility overlay or "auto-remediation" widget script as a fix; recommend the underlying semantic-markup, contrast, or structural change and cite the SC it resolves.
- Cite the exact WCAG 2.2 success-criterion id and Level for every finding (e.g. "2.4.11 Focus Not Obscured (Minimum) — AA"), never a vague "accessibility issue" or bare SC number without Level.
- Flag SC with documented high litigation exposure (1.4.3 Contrast Minimum, 2.1.1/2.1.2 Keyboard, 2.4.7 Focus Visible, 2.4.11 Focus Not Obscured, 4.1.2 Name Role Value, 1.1.1 Non-text Content) with elevated severity and an explicit legal-exposure note.
- Load reference files only for the SC category and evidence question actually in scope; do not preload the full SC index for a single-criterion triage.

## References

Load these only when needed:

- [WCAG 2.2 success-criteria index](references/wcag22-sc-index.md) — use for the full A/AA success-criteria list grouped by POUR principle, with automated-vs-manual detectability and new-in-2.2 flags, when scoping which criteria apply to a review.
- [ACT Rules detection boundary](references/act-rules-detection-boundary.md) — use when a finding needs an explicit machine-testable-vs-manual-only determination, citing the relevant ACT rule id, before closing or escalating it.
- [Legal exposure and severity model](references/legal-exposure-severity.md) — use when triaging a finding's litigation risk, prioritizing a remediation backlog, or preparing input for a VPAT/conformance statement.

## Response minimum

Return, at minimum:

- the component/page/criteria category in scope,
- each finding mapped to an exact WCAG 2.2 SC id, Level (A/AA), and automated-vs-manual evidence label,
- evidence level for the finding itself (automated scan output, manual code review, live AT verification, or documentation-based inference),
- safest remediation with a technique/SC reference, not an overlay or workaround,
- explicit statement of what was NOT verified (manual checks not performed, AT combinations not tested) so no false conformance claim is implied,
- legal-exposure flag (elevated/standard) for any finding touching a high-litigation-risk SC.
