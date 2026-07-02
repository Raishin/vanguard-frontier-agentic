# Resource-Loading Order and Render-Blocking Audit

> Verify exact `<link>`/`<script>` attribute behavior against current MDN docs via Context7 before ruling — browser render-blocking rules and resource-hint attributes are implementation-defined and have changed (e.g. `blocking="render"`, `fetchpriority`). Do not assert them from memory.

## What people get wrong

The naive story is:

> "Put scripts at the bottom of `<body>` and it's fixed."

Incomplete. That mitigates one failure mode (parser-blocking synchronous `<script>` in `<head>`) but says nothing about CSS blocking, resource priority, third-party origin cost, or whether the thing you deferred was actually needed for first paint.

## Officially grounded pipeline

Per MDN's critical rendering path reference: the browser builds the DOM as HTML is parsed, builds the CSSOM from requested/inline styles, combines DOM + CSSOM into the render tree, computes layout, then paints and composites pixels. Two consequences follow directly from that pipeline:

- **CSS is render-blocking by default.** The browser withholds first paint until it has the CSSOM, specifically to avoid a flash of unstyled content. A `<link rel="stylesheet">` in `<head>` blocks rendering unless explicitly marked otherwise (e.g. via a non-render-blocking media query that doesn't match the current viewport, or `blocking` control on supporting elements).
- **A synchronous `<script>` (no `async`/`defer`/`type="module"`) blocks HTML parsing at the point it appears,** because the parser must assume the script could call `document.write` or otherwise mutate the DOM before parsing continues. It does not block CSSOM construction, but it can be blocked *by* an earlier stylesheet (browsers commonly delay script execution until preceding CSS has loaded, since the script might query computed style).

So "render-blocking" is not one failure mode — audit each resource against which stage it blocks:

| Resource | Blocks HTML parse? | Blocks first paint? |
|---|---|---|
| `<link rel="stylesheet">` (no disabling media) | No | Yes |
| `<script src="...">` (no async/defer/module) | Yes, at that point | Indirectly (delays DOM/paint readiness) |
| `<script defer>` | No | No (executes after parse, before `DOMContentLoaded`) |
| `<script async>` | No | No, but executes whenever it finishes downloading — order not guaranteed relative to other scripts |
| `<script type="module">` | No (deferred by default) | No |
| `@import` inside CSS | N/A | Yes, and serializes — it blocks until fetched, adding a network round trip the browser couldn't discover from the HTML in parallel |

Live evidence for a given page: `PerformanceObserver({type: "resource", buffered: true})` and check `entry.renderBlockingStatus === "blocking"` (per `PerformanceResourceTiming.renderBlockingStatus`), or `performance.getEntriesByType("resource")` filtered the same way. This is the only way to get a ground-truth render-blocking verdict for a *live* page rather than an inferred one from reading markup.

## Non-negotiable review rules

### 1. Classify every new/modified `<link>` and `<script>` by blocking behavior, not by intent

Do not accept "it's deferred" as true because the author says so — check the actual attribute (`defer`, `async`, `type="module"`, `blocking`) and its placement.

### 2. `@import` in CSS is a hidden render-blocking network round trip

Flag any new `@import` in a stylesheet that's itself render-blocking. The browser cannot discover and start fetching an `@import`-ed stylesheet until it has already parsed the CSS that contains it — it cannot be discovered by the HTML preload scanner the way a `<link>` can. Prefer a second `<link rel="stylesheet">` in the HTML.

### 3. Resource hints are not interchangeable

Per MDN's `<link>` reference and the `dns-prefetch`/`preconnect`/`preload` guides:

- `dns-prefetch` — resolve DNS only. Cheapest hint; useful for many possible-but-uncertain origins.
- `preconnect` — DNS + TCP + TLS handshake. More expensive per origin; browsers cap how many they'll act on, so do not `preconnect` to more than a handful of origins.
- `preload` — fetch a *specific, known-needed* resource at high priority, before the parser would otherwise discover it. Requires the correct `as` value (`script`, `style`, `font`, `image`, etc.) or the browser may apply the wrong priority/type-check and silently double-fetch. Cross-origin font preloads require `crossorigin` even for same-site font hosting, because fonts are fetched in "anonymous" CORS mode regardless of same-origin-ness.
- `modulepreload` — the module-script-specific equivalent of `preload`, which also allows the browser to parse and cache the module graph ahead of execution.
- `prefetch` — low-priority fetch for a resource likely needed for a *future* navigation, not the current render. Do not use it for anything on the current page's critical path — it competes for bandwidth at low priority and is the wrong tool if the goal is to affect *this* page's LCP.

A `preload` misused for a resource that turns out unused within a few seconds of load produces a Chrome DevTools/Lighthouse warning and wastes bandwidth — flag speculative preloads with no confirmed use.

### 4. Third-party resource hints have a cost, not just a benefit

`preconnect`/`dns-prefetch` to a third-party origin (analytics, font CDN, ad tech, social widget) opens a connection or resolves DNS to that origin earlier than the page would otherwise need to — which also means that origin observes the visit earlier and independent of whether the resource is ever actually used. Weigh this against the latency win; do not recommend blanket `preconnect` to every third-party origin referenced anywhere on the page.

### 5. LCP-candidate resource should be the highest-priority fetch on the page

If the LCP element is a background `<img>`, hero image, or web font, confirm it is discoverable by the HTML preload scanner (i.e. present as a real `<img src>`/`<link>` in markup, not injected by a blocking JS bundle) and, where warranted, has an explicit `fetchpriority="high"` or `<link rel="preload">`. An LCP image that only becomes discoverable after a render-blocking JS bundle executes is a common, easily fixed regression.

## Minimal safe review flow

1. List every `<link>`, `<script>`, and `@import` added or modified in the diff.
2. Classify each by blocking behavior per the table above (parse-blocking / paint-blocking / neither).
3. For each resource hint (`preload`/`prefetch`/`preconnect`/`dns-prefetch`/`modulepreload`), confirm it targets a resource actually needed for *this* page's first render, has correct `as`/`type`/`crossorigin`, and isn't redundant with browser-default discovery.
4. Identify the LCP candidate and confirm it is discoverable without executing render-blocking JS first.
5. Flag any `@import`, unbounded `preconnect` list, or speculative preload as findings, not silent passes.
6. State whether the verdict is markup-inferred (`inference`) or confirmed via a live `PerformanceObserver`/DevTools trace (`live evidence`).

## When to push back

Push back if the user asks for:

- `preconnect` to every third-party domain referenced anywhere in the codebase "just in case,"
- moving all `<script>` tags to `async` without checking execution-order dependencies between them,
- a blanket `preload` for every image on the page instead of just the LCP candidate,
- removing a render-blocking stylesheet without confirming what visual state exists before it loads (flash of unstyled content is a real regression, not just a metric).
