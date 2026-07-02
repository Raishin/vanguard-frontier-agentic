# WCAG 2.2 Success-Criteria Index

Use this reference when scoping which success criteria (SC) apply to a review, or when a finding needs to be anchored to the correct SC id, Level, and POUR principle before it goes into a report.

## What people get wrong

The naive story is:

> "WCAG 2.2 is basically WCAG 2.1 plus a couple of new rules."

Incomplete. WCAG 2.2 added 9 new success criteria (all Level A or AA; no new AAA additions beyond what already existed) and formally deprecated 4.1.1 Parsing (obsolete now that most user agents and AT no longer rely on strict HTML parse-error handling). Auditing against a WCAG 2.1 checklist silently misses the new SC and may still flag 4.1.1 findings that no longer matter for 2.2 conformance. Always confirm which version the conformance target actually names.

## New in WCAG 2.2 (verify against `https://www.w3.org/TR/WCAG22/` before citing)

| SC | Level | Principle | One-line scope |
|---|---|---|---|
| 2.4.11 Focus Not Obscured (Minimum) | AA | Operable | Focused element must not be entirely hidden by author-created content (sticky headers, cookie banners) |
| 2.4.13 Focus Appearance | AAA | Operable | Minimum size/contrast for the focus indicator itself |
| 2.5.7 Dragging Movements | AA | Operable | Any drag-only interaction needs a single-pointer alternative |
| 2.5.8 Target Size (Minimum) | AA | Operable | Pointer targets at least 24x24 CSS px, with documented exceptions |
| 3.2.6 Consistent Help | A | Understandable | Help mechanisms (contact, chat, FAQ) must appear in the same relative order across pages |
| 3.3.7 Redundant Entry | A | Understandable | Don't force re-entry of information already provided in the same process |
| 3.3.8 Accessible Authentication (Minimum) | AA | Understandable | No cognitive-function test (e.g. puzzle solving) required for auth, unless an alternative exists |
| 3.3.9 Accessible Authentication (Enhanced) | AAA | Understandable | Stricter version of 3.3.8 — no object recognition/personal-content exception |
| 1.4.13 (carried from 2.1 but frequently missed) | AA | Perceivable | Content on hover/focus must be dismissible, hoverable, persistent |

Deprecated: 4.1.1 Parsing is removed as a 2.2-conformance requirement — do not cite it as a blocking finding for a WCAG 2.2 target, note that duplicate-ID/malformed-markup issues instead usually surface as 1.3.1 or 4.1.2 failures.

## Full A/AA index by principle (automated-vs-manual detectability)

Detectability legend: **A** = reliably automated-detectable by static/DOM analysis; **M** = requires manual judgment or live AT verification; **P** = partially automatable (tool flags candidates, human must confirm).

### Perceivable

| SC | Level | Detect |
|---|---|---|
| 1.1.1 Non-text Content | A | P — missing `alt` is automatable; *correctness* of alt text is manual |
| 1.2.1–1.2.5 Time-based Media (captions, audio description) | A/AA | M |
| 1.3.1 Info and Relationships | A | P — heading/landmark presence automatable; semantic *correctness* manual |
| 1.3.2 Meaningful Sequence | A | M |
| 1.3.3 Sensory Characteristics | A | M |
| 1.3.4 Orientation | AA | M |
| 1.3.5 Identify Input Purpose | AA | P — `autocomplete` attribute presence automatable |
| 1.4.1 Use of Color | A | M |
| 1.4.2 Audio Control | A | M |
| 1.4.3 Contrast (Minimum) | AA | A — contrast ratio is fully computable from rendered color values |
| 1.4.4 Resize Text | AA | M (requires zoom/reflow testing) |
| 1.4.5 Images of Text | AA | M |
| 1.4.10 Reflow | AA | M |
| 1.4.11 Non-text Contrast | AA | A — computable for UI-component/graphical-object boundaries |
| 1.4.12 Text Spacing | AA | M |
| 1.4.13 Content on Hover or Focus | AA | M |

### Operable

| SC | Level | Detect |
|---|---|---|
| 2.1.1 Keyboard | A | M |
| 2.1.2 No Keyboard Trap | A | M |
| 2.1.4 Character Key Shortcuts | A | M |
| 2.2.1 Timing Adjustable | A | M |
| 2.2.2 Pause, Stop, Hide | A | M |
| 2.3.1 Three Flashes or Below Threshold | A | M |
| 2.4.1 Bypass Blocks | A | P — skip-link/landmark presence automatable; effectiveness manual |
| 2.4.2 Page Titled | A | A — `<title>` presence/non-empty is automatable |
| 2.4.3 Focus Order | A | M |
| 2.4.4 Link Purpose (In Context) | A | P — empty/ambiguous link text flaggable; true purpose manual |
| 2.4.5 Multiple Ways | AA | M |
| 2.4.6 Headings and Labels | AA | M |
| 2.4.7 Focus Visible | AA | P — `outline: none` without replacement is flaggable; adequacy manual |
| 2.4.11 Focus Not Obscured (Minimum) | AA | M |
| 2.5.1 Pointer Gestures | A | M |
| 2.5.2 Pointer Cancellation | A | M |
| 2.5.3 Label in Name | A | P — accessible-name-vs-visible-label mismatch is automatable |
| 2.5.4 Motion Actuation | A | M |
| 2.5.7 Dragging Movements | AA | M |
| 2.5.8 Target Size (Minimum) | AA | A — computed target box size is measurable |

### Understandable

| SC | Level | Detect |
|---|---|---|
| 3.1.1 Language of Page | A | A — `<html lang>` presence/validity is automatable |
| 3.1.2 Language of Parts | AA | P — `lang` attribute presence automatable; correctness manual |
| 3.2.1 On Focus | A | M |
| 3.2.2 On Input | A | M |
| 3.2.3 Consistent Navigation | AA | M |
| 3.2.4 Consistent Identification | AA | M |
| 3.2.6 Consistent Help | A | M |
| 3.3.1 Error Identification | A | M |
| 3.3.2 Labels or Instructions | A | P — unlabeled-input detection automatable |
| 3.3.3 Error Suggestion | AA | M |
| 3.3.4 Error Prevention (Legal, Financial, Data) | AA | M |
| 3.3.7 Redundant Entry | A | M |
| 3.3.8 Accessible Authentication (Minimum) | AA | M |

### Robust

| SC | Level | Detect |
|---|---|---|
| 4.1.2 Name, Role, Value | A | P — missing role/name on custom widgets flaggable; full correctness manual |
| 4.1.3 Status Messages | AA | M |

## Non-negotiable rule for compliance-facing output

Never present an SC citation without its Level attached. Never mark a **P** (partial) or **M** (manual) row as fully resolved because an automated scanner reported zero findings for it — state explicitly which portion the scan covered and which portion still needs manual/AT verification.

## When to push back

Push back if the user asks to:

- treat a "0 issues" Lighthouse/axe report as WCAG 2.2 conformance — automated tools document their own coverage limits (see the Context7 Documentation Protocol grounding on automated-testing pros/cons); a clean scan is a floor, not a ceiling,
- skip the new 2.2 criteria because "we already did a WCAG 2.1 audit" — 2.2 added 9 new SC that a 2.1-era audit never evaluated,
- cite 4.1.1 Parsing as a blocking WCAG 2.2 finding — it is deprecated for 2.2 conformance targets.
