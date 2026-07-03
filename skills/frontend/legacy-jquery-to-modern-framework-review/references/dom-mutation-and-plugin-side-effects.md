# DOM Mutation and Plugin Side Effects

Use this reference when auditing third-party jQuery plugins and ad-hoc DOM-manipulation code for behavior that happens outside any framework render cycle — direct DOM mutation, global state writes, timers, and unsanitized HTML construction.

## What people get wrong

The common bad assumption is:

> "The plugin has a small public API (`$(el).fancyPlugin({ options })`), so its footprint is small too."

That is false for the overwhelming majority of jQuery-era UI plugins. A plugin's public API surface tells you nothing about:

- how many DOM nodes it creates and where it puts them (often *outside* the element it was called on — appended to `body`, injected as siblings, or moved elsewhere in the tree),
- whether it registers global listeners (`$(window).resize(...)`, `$(document).on('keydown', ...)`) that persist for the lifetime of the page regardless of whether the "widget" is still visible,
- whether it holds module-level or `$.fancyPlugin.instances`-style singleton state shared across every instantiation on the page,
- whether it starts timers (`setInterval` for polling/auto-rotation, `setTimeout` for debounce/animation) that are never cleaned up on the jQuery side either, but happen to not matter in a page-reload-based app the way they will matter in a long-lived SPA.

A framework component's render cycle (mount → update → unmount) has no way to automatically clean up any of this. If the plugin's real behavior is not inventoried, the "equivalent" component will leak listeners, leak timers, or silently stop working the first time the framework unmounts and remounts the component (which jQuery-era pages never did — they reloaded instead).

## Non-negotiable design rules

### 1. Read the plugin's source, not just its call sites

A call site like `$('.carousel').slick({ autoplay: true })` gives zero information about side effects. Locate the actual plugin source (vendored file, node_modules, or inlined script) and grep it directly for:

- `document.body.append`/`appendChild`, `$('body').append(`, or any DOM insertion target that is not `this`/the plugin's own root element,
- `setInterval`, `setTimeout` with no matching `clearInterval`/`clearTimeout` in a documented teardown/`destroy` method,
- `$(window)`, `$(document)` listener registration,
- module-level `var`/`let` outside any function scope, or properties attached to the jQuery plugin namespace itself (`$.fn.pluginName.defaults`, a shared cache object) — these persist across every instance and every page-lifecycle event.

### 2. Distinguish "has a destroy/teardown method" from "is actually called"

Many plugins document a `.pluginName('destroy')` API. Grep the *call sites*, not just the plugin source, for whether teardown is ever invoked. A legacy app that never unmounts widgets (because it never removes DOM nodes without a full page reload) commonly never calls teardown — meaning the migration is the first time this code path's absence becomes an observable bug (memory growth, duplicate global listeners after client-side navigation).

### 3. Treat every `.html()`, `.append()`, `.prepend()`, `.after()`, `.before()`, `.replaceWith()`, or raw `.innerHTML =` assignment fed by non-literal data as a security finding

"Non-literal" means anything built from a variable, template string, concatenation, `.val()`/`.text()` of another element, a server response, or a URL/query-string value — not a fixed string literal written by the developer. For each match:

- record whether the source is attacker-influenceable (see the taint sources in this skill's sibling security-review skills: URL params, API responses rendering third-party/user content, `postMessage`, storage written by another origin),
- flag explicitly if the *proposed* replacement is `dangerouslySetInnerHTML` (React) or `v-html` (Vue) — per the Context7-grounded React docs, `dangerouslySetInnerHTML` is a documented "Security Hole" when fed untrusted input directly; a framework migration must not be the moment an existing (bad) pattern becomes a *worse* one by skipping the sanitizer entirely,
- never assume the plugin already sanitizes its input; verify by reading the plugin's actual string-construction code if the finding is high-severity enough to matter (public-facing, user-generated-content-adjacent).

### 4. Global state and timers are migration blockers, not migration details

If a plugin's side effects are genuinely global (a single page-wide autoplay ticker, a shared modal-stack z-index counter), the target architecture needs an explicit decision about where that state lives (a store, a context, a singleton service) — it cannot be silently absorbed into "just render the component and it'll work," because the framework component model assumes state is either component-local or explicitly lifted, not implicitly global via a shared jQuery-plugin-namespace object.

## Minimal safe inventory progression

1. Identify every `$(el).pluginName(...)` call site across the codebase and group by plugin.
2. For each distinct plugin, locate its actual source (not just its call sites) and grep it for the side-effect categories in rule 1.
3. For each plugin, check whether a teardown/destroy path exists in the source and whether it is ever invoked at any call site.
4. Separately, grep the whole codebase (not just plugin source) for `.html(`, `.append(`, `.prepend(`, `.after(`, `.before(`, `.replaceWith(`, and `.innerHTML =` and classify each by literal vs. non-literal input.
5. Cross-reference: does any plugin call site feed plugin options built from non-literal, potentially-attacker-influenceable data (e.g., a plugin's `content` option populated from an API response)? This is a compound finding — flag it distinctly from a plain `.html()` call, since the plugin's internal handling of that option is opaque without reading its source.

## Verification targets

- For any plugin flagged as holding global/singleton state, confirm by checking whether multiple instantiations on the same page actually interfere with each other in the current app (evidence: a bug report, a workaround comment in the code, or a code path that explicitly guards against double-initialization) versus being a theoretical risk only.
- For any `.html()`/`.append()` finding proposed for replacement with `dangerouslySetInnerHTML`/`v-html`, confirm whether a sanitizer (DOMPurify or equivalent) is already present anywhere in the dependency tree before assuming one must be added from scratch.

## When to push back

Push back if the user asks to:

- port a plugin's call site 1:1 into a `useEffect`/lifecycle-hook wrapper without first reading the plugin's own source for global listeners, timers, or singleton state — this reproduces every leak the plugin had, now inside a component that mounts/unmounts far more often than the legacy page ever did,
- replace `.html()` with `dangerouslySetInnerHTML`/`v-html` as a mechanical find-and-replace with no sanitizer discussion — that is a downgrade, not a port, whenever the source data is non-literal,
- mark a plugin "no side effects, safe to wrap" based on reading only its options API or its README, without having actually grepped its source.
