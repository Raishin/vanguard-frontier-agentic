# Legacy Accessibility Shim Audit

Use this reference when checking whether a legacy jQuery/Backbone-era widget provides accessibility behavior that the replacement component must explicitly reproduce before being declared equivalent.

## What people get wrong

The common bad assumption is:

> "The markup for this widget has no `role`, no `aria-*` attributes, and no `tabindex` in the HTML template — so it must have no accessibility behavior, and the replacement doesn't need any either."

That is frequently false, and it is the single most common way a framework migration silently regresses keyboard and screen-reader support. Many jQuery UI-era plugins (jQuery UI's own dialog/tabs/autocomplete/datepicker widgets, and third-party clones of them) inject ARIA attributes, manage `tabindex`, and bind keyboard handlers **at runtime**, via JavaScript, into the DOM the plugin controls — none of it visible in the static HTML template or component markup a reviewer would normally read. Auditing only the markup and concluding "no accessibility behavior found" is a false negative, not a confirmed gap.

Conversely, the opposite mistake also happens: assuming a plugin "must" handle accessibility because it is a mature, popular widget — some plugins (and especially most custom/homegrown ones) genuinely do nothing beyond visual behavior, and the accessibility gap being ported forward is real and already present, not a migration regression.

Both mistakes are resolved the same way: verify, do not assume, in either direction.

## Officially grounded shape (what the ARIA APG actually specifies)

The W3C ARIA Authoring Practices Guide (APG) documents, per widget pattern (dialog, tabs, combobox/autocomplete, disclosure, menu, slider, etc.), the specific roles, states, properties, and keyboard interactions an accessible implementation of that pattern requires — e.g., a tabs widget needs `role="tablist"`/`role="tab"`/`role="tabpanel"`, `aria-selected`, and arrow-key navigation between tabs with roving `tabindex`; a modal dialog needs `role="dialog"`, `aria-modal="true"`, focus moved into the dialog on open, focus trapped within it, and focus returned to the triggering element on close. Use the APG pattern for the specific widget type as the checklist for what the *replacement* component must implement — regardless of what the legacy plugin did or did not do.

## Non-negotiable design rules

### 1. Check the plugin's actual runtime output, not the call-site markup

For jQuery UI and comparable plugins, the accessibility-relevant code lives in the plugin source, in functions that run on initialization and on state change (e.g., open/close, select/deselect). Grep the plugin source for `.attr('role'`, `.attr('aria-`, `.attr('tabindex'`, `setAttribute('aria-`, and keydown/keyup handlers that check `event.which`/`event.key` against arrow keys, `Escape`, `Enter`, `Space`, `Tab`. Their presence means the plugin manages this at runtime; their absence (confirmed by actually reading the source, not by absence in the call-site HTML) means it does not.

### 2. Separately verify focus management, which is easy to miss even when ARIA attributes are present

A widget can have textbook-correct `role`/`aria-*` attributes and still fail if focus is never moved: a modal that sets `role="dialog"` but never calls `.focus()` on an element inside it when opened, or never restores focus to the trigger on close, is not accessible despite "having ARIA." Grep specifically for `.focus()` calls (or their absence) at the widget's open/close or activate/deactivate transitions, independent of the ARIA-attribute check in rule 1.

### 3. Map each finding to the specific APG pattern for that widget type, not a generic "is it accessible" judgment

"Accessible" is not a yes/no property; it is per-interaction-pattern. A datepicker's requirements (grid navigation, `aria-live` announcement of the selected date) are different from a dropdown menu's (`aria-expanded`, `aria-haspopup`, arrow-key/Escape handling) or a tooltip's (`aria-describedby`, hover *and* focus triggering, Escape dismissal). Identify which APG pattern the widget corresponds to before building the parity checklist — do not reuse one pattern's checklist for a structurally different widget.

### 4. Treat "no accessibility behavior found after reading the source" as a real, reportable gap — not a reason to skip the checklist

If the plugin genuinely implements nothing (common for homegrown carousels, custom tooltips, and simple show/hide toggles built directly on top of jQuery with no ARIA/keyboard code at all), report this explicitly as a pre-existing gap the migration inherits, and still produce the APG-pattern-based parity checklist so the replacement component can *close* the gap rather than silently reproduce it. Do not conflate "the legacy version was already inaccessible" with "so the new version doesn't need to be."

## Minimal safe audit progression

1. Identify each interactive widget being migrated and classify it against the closest W3C ARIA APG pattern (dialog, tabs, tablist, menu, menubar, combobox, listbox, slider, tooltip, disclosure, accordion, carousel, etc.).
2. For each widget, locate the actual plugin/implementation source and grep for ARIA attribute writes, `tabindex` management, and keyboard event handlers (rule 1).
3. Separately check focus-management calls at the widget's key state transitions (rule 2).
4. Build a per-widget parity checklist directly from the matched APG pattern's required roles/states/properties/keyboard interactions (rule 3), marking each item as "present in legacy (verified)", "absent in legacy (confirmed gap)", or "inference — plugin source not fully readable, needs runtime confirmation".
5. Flag any widget where the legacy implementation provides real accessibility behavior that the currently-planned replacement component does not yet account for — this is the actual migration risk this reference exists to catch.

## Verification targets

- Where feasible, confirm the plugin's runtime-injected ARIA/keyboard behavior against the plugin's own official documentation or changelog (version-specific — accessibility support has been added/changed across major versions of common plugins) rather than relying solely on reading one version of vendored source.
- For focus-trap claims, confirm whether the trap covers `Tab` cycling in both directions (forward from the last focusable element wraps to the first, `Shift+Tab` from the first wraps to the last) — a common incomplete implementation traps only one direction.

## When to push back

Push back if the user asks to:

- skip the accessibility audit for a widget because "the markup has no ARIA attributes so there's nothing to check" — this is exactly the false-negative pattern this reference exists to prevent for runtime-injected behavior,
- declare a replacement component "equivalent" based on visual/behavioral parity alone, with no APG-pattern-based keyboard/focus/ARIA checklist,
- treat a confirmed pre-existing accessibility gap in the legacy widget as acceptable to carry forward silently without at least flagging it as a known, inherited gap in the migration output.
