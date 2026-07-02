# APG Pattern Quick-Index

Use this reference when a component under review is a custom (JS-driven) interactive widget and you need to identify which WAI-ARIA Authoring Practices Guide (APG) pattern applies before comparing its actual keyboard/role/state implementation against the canonical model.

## What people get wrong

The common bad assumption is:

> "I added `role="button"` and a click handler, so it's accessible."

Wrong. A role only tells assistive technology what the element *is*; it does nothing for keyboard operability, focus management, or state exposure. Every non-trivial custom widget corresponds to exactly one (sometimes zero) APG pattern, and that pattern defines a full contract: role, required/optional states and properties, and a specific keyboard interaction model — not just "Enter activates it."

> Version note: WAI-ARIA and the APG evolve. Confirm the pattern's current keyboard model and required ARIA attributes via Context7 (`/mdn/content`) or the live `https://www.w3.org/WAI/ARIA/apg/patterns/` page before ruling — do not cite this file's summaries as the final word.

## Non-negotiable design rule: match first, judge second

Before writing any finding about a custom widget, identify its APG pattern by name. If no pattern fits, that itself is a finding: either the widget should not exist as a custom control (use a native element), or it needs a `role="group"`/`role="region"` treatment with no interactive-widget semantics at all. Never invent ad-hoc ARIA for a shape that doesn't match a documented pattern.

## Common widget → pattern map (starting point, not exhaustive)

| Widget seen in markup | Likely APG pattern | Minimum keyboard model to verify |
|---|---|---|
| Modal / overlay dialog | Dialog (Modal) | Focus moves into dialog on open; `Tab`/`Shift+Tab` trapped inside; `Escape` closes; focus returns to trigger on close |
| Non-modal panel toggle | Disclosure (Show/Hide) | `Enter`/`Space` toggles; `aria-expanded` reflects state; no focus trap |
| Dropdown menu (app-style, not `<select>`) | Menu / Menu Button | `Enter`/`Space`/`ArrowDown` opens; `ArrowUp`/`ArrowDown` moves; `Escape` closes and returns focus; `Home`/`End` jump to first/last |
| Custom text input with suggestions | Combobox | `ArrowDown` opens/moves listbox; `Escape` closes without selecting; `Enter` selects active option; `aria-activedescendant` or roving `tabindex` tracks active option |
| Tab strip | Tabs | Arrow keys move between tabs (auto- or manual-activation, pick one and document which); `Tab` key exits to panel; only active tab is in the sequential tab order |
| Accordion | Accordion (built from Disclosure) | Each header toggles its own panel independently; heading semantics preserved (see heading-landmark-rules.md) |
| Tree view (file explorer style) | Tree View | `ArrowUp`/`ArrowDown` moves; `ArrowRight`/`ArrowLeft` expands/collapses or moves to child/parent; `*` expands all siblings (optional) |
| Custom slider | Slider | `ArrowLeft`/`ArrowRight` (or `Up`/`Down`) adjusts by step; `Home`/`End` jump to min/max; `aria-valuenow`/`aria-valuemin`/`aria-valuemax` kept in sync |
| Toast / live status message | Alert or Status (live region, not a focusable widget) | No keyboard model — it must not steal focus; verify `role="alert"` (assertive, interrupting) vs `role="status"` (polite) is the deliberate choice, not a default |

## Verification targets

For each matched pattern, confirm in the markup/code:

- the root and child elements carry the pattern's required roles (native-implicit or explicit `role=`),
- every required `aria-*` state/property from the pattern is present and kept in sync with actual UI state (not just set once at mount),
- the full keyboard model is implemented, not a subset — a widget with only mouse/click handlers and no `keydown` logic fails regardless of correct roles,
- focus is visible (no `outline: none` without a replacement focus style) and moves predictably on open/close/select.

## High-risk assumptions to kill

- "It has the right `role`, so it's done" — role without keyboard model and state sync is theater.
- "Semantic-sounding class name (`.dropdown-menu`) means it follows the Menu pattern" — check the actual DOM/role/keydown handlers, not the class name.
- "We tested with the mouse and it felt fine" — APG conformance is a keyboard-first and AT-first contract; mouse-only QA proves nothing about it.
- "It matched a pattern last time we reviewed it" — widgets drift as code changes; re-verify against the current implementation every review, not from memory of a prior review.

## When to push back

Push back if the user asks to:

- add ARIA roles/states to make a widget "look accessible" without implementing the pattern's keyboard model,
- ship a custom widget where a native element (`<select>`, `<details>`, `<dialog>`) would satisfy the requirement with less code and less risk,
- treat "no pattern fits perfectly" as license to skip APG entirely rather than picking the closest pattern or reconsidering the widget design.
