# Accessibility Adversarial Checklist

Use this reference for spot-checking a Tier-1 accessibility verdict, specifically for WCAG 2.2 failures that automated tooling (axe-core, Lighthouse, WAVE) is structurally unable to detect. Automated scanners catch missing alt text, contrast ratios, and missing form labels reliably. They cannot reliably catch behavior that only manifests through actual interaction.

## What people get wrong

The naive story is:

> "axe-core reported zero violations, so the component is accessible."

Wrong. Automated tooling checks static DOM properties and computed styles. It cannot drive a keyboard through a component, cannot observe whether focus moves logically after a state change, and cannot tell whether an `aria-live` region actually announces what a screen reader user needs to hear at the right moment. A zero-violation scan result is evidence about a narrow slice of WCAG, not the whole standard.

## What automated tooling cannot catch — hunt here first

- **Keyboard traps (WCAG 2.1.2).** Tab and Shift+Tab through the actual component (mentally trace the DOM order and any `tabIndex`/focus-trap library configuration in the diff). A modal, combobox, or custom widget that captures focus and never returns it to a logical position on close is invisible to a static scanner.
- **Focus order and focus management on state change (WCAG 2.4.3, 3.2.2).** When a component opens (modal, drawer, toast), does focus move to it? When it closes, does focus return to the triggering element, or is it lost to `<body>`? Trace this in the actual event handlers — a scanner sees only the DOM snapshot, not the transition.
- **`aria-live` region correctness (WCAG 4.1.3).** Confirm the live region actually exists in the DOM *before* the content update fires (a region injected at the same time as its content will not announce in most screen readers), and that `aria-live="polite"` vs `"assertive"` matches the urgency of the message. A visually-correct toast with no live region, or one injected too late, passes automated scans while being silent to screen reader users.
- **Meaningful sequence and reading order (WCAG 1.3.2).** CSS-driven visual reordering (flexbox `order`, grid placement, absolute positioning) can produce a DOM reading order that diverges from the visual order. A scanner does not compare visual order to DOM order; you must.
- **Custom widget ARIA pattern conformance (WCAG 4.1.2).** For any custom combobox, tabs, accordion, tree, or menu, verify against the actual WAI-ARIA Authoring Practices Guide pattern for that widget type: required `role`, state attributes (`aria-expanded`, `aria-selected`, `aria-activedescendant`), and the specific keyboard interaction model (arrow keys, Home/End, typeahead). A widget that "looks like" a combobox visually but implements none of the APG's keyboard model will pass a static scan and fail every screen reader user.
- **Status messages without focus shift (WCAG 4.1.3 / APG Status Messages pattern).** Form validation errors or async status updates that update the DOM without moving focus and without an appropriate live region are invisible to sighted-mouse-user testing and to static scanners alike.
- **Target size and pointer gesture failures (WCAG 2.5.8, 2.5.1).** Small interactive targets or gesture-only interactions (drag-only reordering with no keyboard/button alternative) are layout facts a scanner does not evaluate against the 2.5.8 minimum, and functionality facts it cannot exercise at all.

## Non-negotiables

- Never accept "axe-core / Lighthouse reported no violations" as sufficient evidence for a CONFIRMED-clean verdict on any of the above categories — those tools do not test them.
- Every CONFIRMED accessibility finding must cite a specific WCAG 2.2 success criterion number (e.g., "2.4.3 Focus Order"), not a generic "accessibility issue" label.
- Ground every custom-widget ARIA pattern claim in the actual WAI-ARIA Authoring Practices Guide (APG) pattern page for that widget type — do not assert a keyboard model from memory.
- An accessibility finding that is a HARD gate (keyboard trap, missing focus management on a modal, broken custom-widget semantics that block operation) must be flagged as mandatory-block, not informational.

## When to push back

Push back if:

- the Tier-1 verdict's only accessibility evidence is an automated scanner result, with no mention of keyboard traversal, focus management, or live-region behavior,
- a custom interactive widget is present in the diff with no APG pattern citation for its role/state/keyboard model,
- "we'll test with a screen reader later" is offered as a substitute for tracing the actual live-region and focus-management code now.
