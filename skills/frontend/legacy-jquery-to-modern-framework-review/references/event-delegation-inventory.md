# Event Delegation Inventory

Use this reference when cataloguing jQuery/Backbone-era event-binding patterns before a framework migration — specifically `$(document).on(...)`-style global delegation, `$.fn` plugin custom events, and Backbone view `events` maps.

## What people get wrong

The common bad assumption is:

> "Events are just `addEventListener` with extra syntax. I'll find every `.on(` call, rewrite it as an `onClick` prop, and move on."

That is incomplete, and it is the single most common source of silent regressions in jQuery-to-framework ports. jQuery delegation and framework event handlers solve different problems:

- **Delegation exists because elements come and go.** `$(document).on('click', '.tab', handler)` was written that way *because* `.tab` elements are added/removed dynamically (AJAX-loaded content, plugin-rendered markup) and a direct binding would miss elements that did not exist at bind time. A framework component only needs this pattern if it renders variable, dynamically-appearing children with no stable parent to attach the handler to — which is rare inside a component tree where the framework already re-renders and re-attaches handlers on every update.
- **The delegation selector is not the same as ownership.** `$(document).on('click', '.modal-close', ...)` is bound once, globally, but the handler's *logic* may belong to a specific feature. Naively porting this to a single global `document.addEventListener` in the new app (instead of scoping it to the owning component) reproduces the original code smell instead of fixing it.
- **Custom/synthetic events are easy to miss.** jQuery plugins frequently `.trigger('customEventName')` and other code listens with `.on('customEventName', ...)`. Grepping only for DOM event names (`click`, `submit`, `change`) misses this entire category of implicit coupling between unrelated-looking modules.
- **Backbone view `events` maps use a different delegation model** (`{'click .save': 'onSave'}`), scoped to `this.el`, and typically calling `this.render()` internally — the "component" boundary is the Backbone view, not the DOM node the handler is nominally attached to.

## Non-negotiable design rules

### 1. Classify every handler by binding scope, not by event name

For each match, record:

- **Global** (`$(document).on(...)`, `$(window).on(...)`, `$('body').on(...)`) — highest risk; likely couples unrelated features.
- **Container-scoped** (`$('#widget-container').on(...)`) — bound once to a stable ancestor; probably maps to a single component boundary.
- **Element-direct** (`$('.button').click(...)` or `.on()` with no delegate selector) — bound at render time; will break silently if the plugin re-renders the DOM node without rebinding (a classic jQuery bug this inventory should surface, not silently port forward).
- **Backbone view `events` map** — scoped to `this.el`; note the view class name and whether `render()` is called inside the handler.

### 2. Do not assume the delegated selector's specificity survives the port

A delegated selector like `.on('click', '.item.active', ...)` depends on class-toggling logic living somewhere else in the codebase. Trace where `.active` is added/removed before assuming the target framework's conditional rendering (`className` binding, `:class`, `[class.active]`) is an equivalent replacement — confirm the toggle logic itself is inventoried, not just the handler.

### 3. Treat custom/synthesized events as first-class inventory items

Grep for `.trigger(`, `.triggerHandler(`, and Backbone's `.trigger(` (models/collections/views all mix in Backbone.Events) alongside the corresponding `.on('eventName', ...)` listeners. Record the event name, the emitting module, and every listening module — this is often the only documentation of a cross-module dependency that exists.

### 4. Distinguish "will the target framework's re-render make this delegation unnecessary" from "must this delegation pattern be explicitly reproduced"

Framework component trees re-attach handlers on every re-render by design (React re-runs the component function; Vue/Angular re-bind via their own reactivity), which naturally replaces *element-direct* handlers that needed delegation only to survive jQuery's manual DOM churn. But a *genuinely* dynamic, framework-external DOM region (e.g., markup injected by a third-party widget the framework does not control) still needs an explicit delegation strategy — do not assume the framework's reactivity makes all delegation moot.

## Minimal safe inventory progression

1. Grep for `.on(`, `.bind(`, `.delegate(`, `.live(` (legacy jQuery <1.9 API — flag its presence as an age/version signal), `.click(`, `.trigger(`, `.triggerHandler(`, and Backbone `events:` object literals.
2. For each match, classify by binding scope (see rule 1) and record the file, the selector/event name, and the handler body's actual side effect (state mutation, network call, DOM mutation, navigation).
3. Group matches by the DOM region or feature they affect, not by file — a single feature's delegation logic is often split across an initializer file and a separate handler file.
4. For every custom/triggered event, find both the emitter and every listener before marking the entry complete; an event with only an emitter found (no listener located) is an open item, not a non-issue.
5. Produce the ownership mapping: each inventoried handler maps to a proposed owning component in the target architecture, or is marked "unresolved — requires product/design input" if no natural owner exists.

## Verification targets

- Confirm handler classification against actual runtime behavior where feasible (e.g., does the container the handler is bound to ever get replaced/re-rendered by innerHTML replacement elsewhere in the codebase — that would mean an element-direct binding silently stops firing, a bug worth flagging even before migration).
- Cross-check every `.trigger('name', ...)` call against every `.on('name', ...)` listener; an emitter with zero listeners found in-repo may indicate a listener registered dynamically (e.g., via a plugin option callback) — mark as `inference, needs runtime confirmation` rather than silently dropping it.

## When to push back

Push back if the user asks to:

- port event handlers file-by-file without first producing the ownership/ scope classification — this guarantees global delegation gets flattened into ad hoc per-component listeners that reintroduce the same "who owns this click" ambiguity the review exists to resolve,
- skip the custom/triggered-event grep because "we only care about DOM events" — cross-module `.trigger()`/`.on()` coupling is frequently the least-visible and highest-risk-to-drop behavior in the entire codebase,
- treat "the new framework re-renders automatically, so delegation is a non-issue" as true everywhere, without checking whether any DOM region is still externally/plugin-controlled.
