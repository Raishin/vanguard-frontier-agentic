# Generated Accessibility Gap Audit

Use this reference when a diff introduces or modifies an interactive component — a menu, modal/dialog, tabs, combobox, disclosure, tooltip, accordion, or custom form control — and the component's origin is AI/LLM-generated or suspected to be.

## What people get wrong

The common bad assumption is:

> "It looks and behaves correctly in a mouse-driven click-through, so it's accessible."

That is incomplete, and it is the specific gap generated frontend code falls into systematically. Models trained heavily on visual/structural patterns reproduce a component's DOM shape and CSS with high fidelity — divs, click handlers, conditional rendering, styling — because that is the dense part of the training signal. Keyboard operability, focus management, and ARIA state are comparatively sparse and easy to omit while still producing something that *looks* like a working component in a quick visual check. A generated `<div onClick={...}>` styled to look exactly like a button, with no `role="button"`, no `tabIndex`, no `onKeyDown` for Enter/Space, and no focus-visible state, passes every visual QA pass and fails every keyboard/screen-reader user.

## Officially grounded shape (W3C ARIA Authoring Practices Guide)

The APG defines, per widget pattern, the required:

- **role** — the ARIA role (or correct native HTML element) that identifies the widget's semantic type to assistive technology,
- **keyboard interaction** — the specific key bindings required for that pattern (e.g., a menu requires Arrow Up/Down to move focus among items, Escape to close, Enter/Space to activate; a tabs widget requires Arrow Left/Right to move between tabs and typically automatic or manual activation),
- **focus management** — where focus moves on open/close/activation (e.g., a modal must trap focus within itself while open and restore focus to the triggering element on close),
- **required states/properties** — the ARIA attributes that communicate current state (`aria-expanded`, `aria-selected`, `aria-checked`, `aria-haspopup`, `aria-controls`, `aria-activedescendant`, etc., depending on pattern).

Do not treat these as optional polish. A component missing any of the four is not "mostly accessible" — it is non-operable for keyboard-only and screen-reader users for that specific interaction.

## Non-negotiable audit rules

1. **Identify the correct APG pattern first.** Match the component to its specific APG pattern (menu vs. menubar vs. listbox vs. combobox are not interchangeable) before auditing — applying the wrong pattern's checklist produces false passes and false negatives.
2. **Test keyboard operability by tracing code, not by assuming it from visual markup.** Confirm there is an actual `onKeyDown`/`onKeyUp` handler (or native element providing it for free) implementing every key binding the APG pattern requires — a component with only `onClick` is not keyboard-operable regardless of how it looks.
3. **Verify focus management explicitly for overlay patterns.** For any modal, dialog, or popover pattern, confirm: focus moves into the overlay on open, focus is trapped within it while open, and focus returns to a sensible element (typically the trigger) on close. Absence of any of these three is a confirmed gap, not a style preference.
4. **Check that ARIA state attributes are wired to actual component state, not hardcoded.** `aria-expanded="false"` hardcoded in JSX/template that never updates when the component opens is a common generated-code pattern — flag it as equivalent to having no `aria-expanded` at all, since it actively communicates wrong state.
5. **Prefer native HTML elements over ARIA-patched divs wherever the pattern allows it**, per the APG's own guidance that native semantics are more robust than reconstructed ARIA — flag a custom `<div>`-based reconstruction of something a native `<button>`, `<dialog>`, or `<select>` already provides for free, unless there's a stated, real constraint the native element can't meet.
6. **Do not accept a passing automated accessibility linter (eslint-plugin-jsx-a11y, axe-core static rules) as sufficient evidence of full compliance.** Static linters catch a meaningful but partial subset (missing `alt`, some ARIA misuse); keyboard operability and focus-trap correctness generally require code tracing or runtime interaction testing beyond static lint rules — say so explicitly rather than treating a clean lint run as a full pass.

## Audit workflow

1. Identify each new/modified interactive component in the diff and match it to its specific APG pattern.
2. Pull the APG page for that exact pattern (`official_docs` URL in this skill's `metadata.json`) and extract its required role, keyboard bindings, focus-management behavior, and ARIA state attributes.
3. Trace the component's actual code against each of the four requirement categories; do not infer from the visual/CSS layer.
4. For each requirement category, classify as: `present and correctly wired`, `present but not wired to real state` (e.g., hardcoded ARIA attribute), or `missing`.
5. Prioritize `missing` keyboard-operability and focus-management findings above missing/incorrect ARIA attributes above missing native-element preference — a component nobody can reach by keyboard is a harder blocker than a suboptimal role choice.

## High-risk assumptions to kill

- "It has an ARIA role, so it's accessible." A role without the matching keyboard interaction and state management is worse than no role at all — it advertises a contract to assistive technology that the component does not fulfill.
- "The design system's base component is accessible, so anything built from AI-generated composition around it is too." Verify the generated wiring code didn't strip or override the base component's accessible behavior (e.g., replacing a native `<button>` wrapper with a styled `<div>` for layout convenience).
- "This passed a quick manual click-through, so keyboard/screen-reader behavior is fine." A mouse-driven manual check exercises none of the keyboard or AT-specific code paths that are exactly where generated code is most likely to have gaps.

## When to push back

Push back if the user asks you to:

- approve an interactive component with no traced keyboard-handler evidence because "it looks right,"
- treat a passing static a11y linter run as proof of full APG-pattern compliance,
- skip focus-management review on a modal/overlay pattern because "it's just a small popover."

Those are exactly the corners where generated code's semantic gaps hide.
