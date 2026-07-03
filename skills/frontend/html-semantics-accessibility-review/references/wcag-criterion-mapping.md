# WCAG 2.2 Success-Criterion Mapping

Use this reference when a finding from this skill needs to be tied to a specific WCAG 2.2 success criterion (SC) for a compliance-audit deliverable — e.g. when the output feeds an ADA / Section 508 / EN 301 549 conformance report and "looks wrong" is not an acceptable citation.

## Grounding rule

This file gives a starting map from common findings to likely SC numbers so review output cites *something* concrete instead of vague prose. It does not replace reading the actual SC "Understanding" doc for edge cases — verify exact wording and Level (A/AA/AAA) against `https://www.w3.org/TR/WCAG22/` or via the Context7 Documentation Protocol before finalizing a compliance-facing citation, since SC numbering and scope can be easy to misremember (e.g., confusing 4.1.2 Name/Role/Value with 1.3.1 Info and Relationships).

## Finding → likely success criterion

| Finding category | Likely SC (Level) | Why |
|---|---|---|
| Heading level skip / missing heading structure | 1.3.1 Info and Relationships (A) | Structure conveyed visually must also be conveyed programmatically |
| Duplicate unlabeled landmark of the same type | 1.3.1 Info and Relationships (A), 2.4.1 Bypass Blocks (A) | Landmarks are the mechanism for bypassing repeated content; ambiguous landmarks break that mechanism |
| Custom widget missing/incorrect role | 4.1.2 Name, Role, Value (A) | AT must be able to programmatically determine the role of every UI component |
| Custom widget state (`aria-expanded`, `aria-checked`, etc.) not kept in sync with actual UI state | 4.1.2 Name, Role, Value (A) | Same SC — "Value" includes states that change |
| Custom widget missing full keyboard operability (e.g. no `Escape`, no arrow-key navigation per its APG pattern) | 2.1.1 Keyboard (A) | All functionality must be operable through a keyboard interface |
| Positive `tabindex` values / illogical focus order | 2.4.3 Focus Order (A) | Focus order must preserve meaning and operability |
| `outline: none` / removed focus indicator with no replacement | 2.4.7 Focus Visible (AA) | Keyboard focus indicator must be visible |
| `aria-hidden="true"` trapping a focusable descendant | 4.1.2 Name, Role, Value (A), 2.1.1 Keyboard (A) | Element is reachable by keyboard but hidden from AT (or vice versa) — a dual failure |
| Toast/alert stealing focus or interrupting unexpectedly | 4.1.3 Status Messages (AA) | Status messages must be programmatically determinable without receiving focus |
| Native element replaced with unlabeled `<div>`/`<span>` + click handler (fake button/link) | 4.1.2 Name, Role, Value (A), 2.1.1 Keyboard (A) | No role exposed, no keyboard access by default |
| Missing/incorrect accessible name on interactive control (icon-only button, ambiguous link text) | 4.1.2 Name, Role, Value (A), 2.4.4 Link Purpose (In Context) (A) | Control lacks a determinable/discriminable accessible name |

## Non-negotiable rule for compliance-facing output

Never present an SC citation as authoritative without the Level (A/AA/AAA) attached — an auditor reading "1.3.1" without a level cannot assess conformance-target scope (most orgs target AA). If uncertain of the exact SC number for a novel finding, say so explicitly (`inference — needs SC verification`) rather than guessing a plausible-sounding number; a wrong citation in a compliance deliverable is worse than an honest gap.

## Verification targets

- Cross-check any SC number cited in this table against the live WCAG 2.2 "Understanding" page for that SC before it goes into a compliance-audit-facing deliverable (not required for a routine PR review comment, where this table's mapping is sufficient).
- For AAA-level findings, flag explicitly that AAA is rarely a required conformance target — do not imply a blocking failure at AAA carries the same severity as an A/AA failure unless the user has stated AAA is their target.

## When to push back

Push back if the user asks to:

- label every accessibility finding as a single generic "WCAG violation" for a compliance deliverable without SC-level granularity — auditors and legal teams need the specific criterion and level,
- downgrade a Level A finding to "nice to have" — Level A is the floor of any recognized conformance claim (WCAG A/AA/AAA, Section 508, EN 301 549 all build on Level A as mandatory),
- treat this skill's static-review findings as a substitute for a full conformance audit — this skill narrows and grounds findings but does not replace testing with real assistive technology across the criteria this file does not cover (e.g., color contrast, captions, timing).
