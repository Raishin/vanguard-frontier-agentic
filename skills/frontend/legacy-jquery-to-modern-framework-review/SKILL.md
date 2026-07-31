---
name: legacy-jquery-to-modern-framework-review
description: Inventory the hidden behaviors in a legacy jQuery/Backbone-era codebase — implicit global event delegation, direct DOM mutation outside any render cycle, plugin side effects, ad-hoc accessibility shims, and unsanitized HTML string building — that a mechanical framework port would silently drop or need to explicitly reproduce.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Legacy jQuery to Modern Framework Review

## Purpose

A mechanical 1:1 port of jQuery code into a modern component framework routinely loses behavior that was never written down: event delegation bound at `document` level, DOM mutations performed by third-party plugins outside any framework render cycle, and accessibility behavior (focus trapping, ARIA attribute toggling) implemented ad hoc in a plugin nobody remembers the internals of. A migration plan that skips this inventory ships a component tree that *looks* equivalent and silently regresses on events, side effects, or accessibility the day it reaches production. This skill exists to make that inventory explicit — component by component, handler by handler — before a migration plan commits to a strangler boundary.

## When to use

Use this skill when the user asks to:

- inventory jQuery/Backbone-era DOM manipulation and event-binding patterns before a framework migration,
- identify which jQuery plugins have undocumented side effects (DOM mutation, global state, timers) that must be explicitly reproduced in the replacement,
- find unsanitized HTML string-building (`.html()`, string concatenation into `innerHTML`) that a port must not carry forward unchanged — or worse, upgrade into an unguarded `dangerouslySetInnerHTML`/`v-html`,
- check whether legacy widgets provide accessibility behavior (keyboard support, focus management, ARIA toggling) that the replacement component must match before being declared equivalent.

Do not use this skill for:

- authoring the replacement framework's component code itself — this skill produces the inventory a migration plan consumes, it does not design the target architecture (pair with `frontend-migration-modernization-plan` or the target framework's architecture-review skill for that),
- general DOM XSS/CSP review with no legacy-migration angle — use `frontend-dom-xss-csp-review` for that,
- reviewing a codebase that has no jQuery/Backbone-era code at all.

## Context7 Documentation Protocol

- Resolve the target framework's Context7 library ID (`resolve-library-id`) before citing any claim about how the replacement component's API is expected to behave — e.g. React: `/reactjs/react.dev`; Vue: `/websites/vuejs_guide`; Angular: `/websites/angular_dev`. Read `package.json` first to confirm the actual target framework and version; do not assume one from the ticket title.
- For every `dangerouslySetInnerHTML`/`v-html`-shaped replacement candidate, ground the security claim in Context7 React docs before writing it: `dangerouslySetInnerHTML` accepts an untrusted string only if the caller has already sanitized it — passing raw legacy `.html()` input straight through creates the same "Security Hole with dangerouslySetInnerHTML" pattern the official React docs warn about explicitly (a `post.content` string with an `onerror` payload rendered unsanitized). Label this claim `documentation-based (Context7: /reactjs/react.dev)`.
- For claims about refs, `useEffect`, and "escape hatch" DOM access as the sanctioned place to reproduce legacy imperative DOM code, ground them in the official React docs' "Manipulating the DOM with Refs" material (refs are an escape hatch for stepping outside React; manually manipulating DOM nodes React also renders causes conflicts, e.g. calling `ref.current.remove()` outside of `setState` crashes on the next render). Do not invent a different "sanctioned" location for imperative code.
- This skill does not by itself certify the new implementation is correct — it inventories legacy behavior and flags what the target implementation must account for. Do not recommend a specific target-framework API as the finished solution without verifying the claim against Context7/official docs first.
- If Context7 is unavailable for the target framework, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- Treat every `$(document).on('click', '.selector', ...)`-style delegated handler as a routing/ownership question, not just an inventory line: in the ported code, which component owns this event, does the DOM structure still exist for delegation to make sense, and is event delegation still needed once the framework's synthetic/native event system replaces manual `$(document)` binding?
- Treat every `.html()`, `.append()`, `.prepend()`, or `.after()` call fed by non-literal data (template strings, concatenation, server response, `.val()`/`.text()` of another element) as a security finding, not just a style note. Flag it as an unsanitized-injection candidate for the migration plan's risk register, and flag doubly if the proposed replacement is `dangerouslySetInnerHTML`/`v-html` with no sanitizer in between.
- Do not assume a jQuery UI/plugin widget (datepicker, autocomplete, modal, tabs, slider, tooltip) has zero accessibility behavior just because the markup around it looks bare. Check the plugin's own documentation or bundled source for ARIA attribute toggling, `role` assignment, and keyboard handlers before declaring an accessibility gap — a missing declaration in the call-site markup does not mean the plugin injects nothing at runtime.
- Do not classify a jQuery plugin as side-effect-free because its public API looks simple. Grep the plugin source itself (not just call sites) for global variable writes, `setInterval`/`setTimeout` registration, direct `document`/`window` event binding, and DOM nodes created outside the element the plugin was called on — these are exactly the behaviors a component-scoped framework render cycle will not reproduce automatically.
- Do not recommend a specific target-framework API as "the" replacement without first grounding the claim via the Context7 Documentation Protocol above.
- Load `references/event-delegation-inventory.md` only when cataloguing event-binding patterns.
- Load `references/dom-mutation-and-plugin-side-effects.md` only when auditing third-party plugin behavior.
- Load `references/legacy-a11y-shim-audit.md` only when checking accessibility parity requirements.

## References

Load these only when needed:

- [Event delegation inventory](references/event-delegation-inventory.md) — use to catalogue `$(document).on(...)`-style global delegation, `$.fn` custom-event patterns, and Backbone view event maps, and map each handler to an owning component in the target architecture.
- [DOM mutation and plugin side effects](references/dom-mutation-and-plugin-side-effects.md) — use to find third-party jQuery plugins performing direct DOM mutation, timers, global namespace writes, or singleton state outside any render cycle, and to flag unsanitized HTML string-building sites.
- [Legacy accessibility shim audit](references/legacy-a11y-shim-audit.md) — use to verify what keyboard/focus/ARIA behavior existing widgets provide before their replacement is declared equivalent.

## Response minimum

Return, at minimum:

- the inventory of delegated event handlers with proposed component ownership in the target framework,
- the list of plugins/handlers with undocumented side effects requiring explicit reproduction,
- unsanitized HTML-construction sites flagged as security findings, with an explicit call-out if the proposed replacement is `dangerouslySetInnerHTML`/`v-html` without a sanitizer,
- an accessibility-parity checklist per replaced widget (keyboard support, focus management, ARIA attributes/roles),
- evidence level per finding (source-code-verified vs. plugin-docs-based vs. inference), and Context7/official-docs grounding for any target-framework API claim.
