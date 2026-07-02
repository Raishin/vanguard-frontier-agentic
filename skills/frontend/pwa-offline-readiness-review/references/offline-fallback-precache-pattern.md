# Offline Fallback Precache Patterns

Use this reference when reviewing how an app's offline-fallback route is registered and guaranteed-precached, across hand-rolled Workbox, `vite-plugin-pwa`, and `next-pwa` conventions. Query Context7 for the specific plugin/version in scope before asserting a default (see the Context7 Documentation Protocol in `SKILL.md`) — the exact config keys and default file paths below are current as queried, but plugin defaults change across major versions.

## What people get wrong

The naive story is:

> "I have a service worker and it precaches my app shell, so offline works."

Wrong. Precaching an asset and having a route match a failed navigation to a fallback are two independent mechanisms. An app can precache a hundred files and still show the browser's default offline error page on every dropped-network navigation, because nothing registered a catch handler that serves a specific fallback document for failed `document`-destination requests. Conversely, an app can have a beautifully designed `/offline.html` page that is never precached, so the one time it is needed — when the network is down — the browser cannot fetch it either.

Both halves — a precached fallback document, and a route/catch-handler that serves it on navigation failure — must be verified independently.

## Officially grounded shape (per queried sources)

### Hand-rolled Workbox (`workbox-recipes` `offlineFallback` / `workbox-routing` `setCatchHandler`)

The documented pattern (`workbox-recipes/src/offlineFallback.ts`) sets a global catch handler keyed on `request.destination`:

- for `document` (navigation) requests, it looks up the fallback page first via `matchPrecache(pageFallback)`, then falls back to a secondary `workbox-offline-fallbacks` cache,
- optionally does the same for `image` and `font` destinations,
- returns `Response.error()` if neither matches — meaning if the fallback page itself was never precached and never separately cached, the catch handler produces a network error rather than a rendered page.

Review checklist for a hand-rolled implementation:
- Confirm the fallback page path passed to `offlineFallback()` (or an equivalent hand-written `setCatchHandler`) is one of the entries in the `precacheAndRoute()` manifest, not just a same-looking path.
- Confirm `precacheAndRoute()` was actually called (not just `precache()` alone) — `precache()` without `addRoute()`/`precacheAndRoute()` populates the cache but does not register the intercepting route, meaning normal navigations to precached pages would fall through to network first (relevant to caching-strategy review, but also affects whether the fallback lookup via `matchPrecache` finds a served entry).
- Confirm the catch handler is registered globally (`setCatchHandler`), not only on a subset of routes, or navigations outside those routes will surface the raw browser error page.

### `vite-plugin-pwa` (Vite / `generateSW` or `injectManifest`)

`vite-plugin-pwa` does not auto-designate an offline-fallback page by default — precache coverage is controlled by `workbox.globPatterns` (which files get swept into the precache manifest) and `workbox.maximumFileSizeToCacheInBytes` (default documented ceiling around 2 MiB; assets above it are silently excluded from precache even if `globPatterns` would otherwise match them). Review checklist:
- Confirm the offline-fallback HTML file's glob pattern is actually included in `globPatterns` (a common gap: `globPatterns: ['**/*.{js,css,html}']` looks like it covers an `offline.html`, but if the file lives outside the configured `public`/build output directory scanned by the plugin, it will not appear in the manifest).
- Confirm the fallback file is under the size ceiling, or that `maximumFileSizeToCacheInBytes` was explicitly raised for it.
- Because `vite-plugin-pwa` does not wire the catch-handler/fallback-routing logic for you by default under `generateSW`, confirm the project either uses `injectManifest` with a custom service-worker source implementing `offlineFallback`/`setCatchHandler`, or has added equivalent `runtimeCaching` navigation-fallback configuration. Do not assume "precached" implies "served as a navigation fallback" — verify the routing half separately, per the "what people get wrong" framing above.

### `next-pwa` (`@ducanh2912/next-pwa`)

This plugin's documented `fallbacks` option explicitly separates fallback routes by destination (`document`, `data`, `image`, `audio`, `video`, `font`), each pointing to a precached path, e.g.:

```js
fallbacks: {
  document: "/~offline",
  data: "/fallback.json",
  image: "/fallback.webp",
}
```

Its internal fallback handler (`self.fallback`) branches on `request.destination` and matches against those precached paths via `caches.match(fallbackResponse, { ignoreSearch: true })`, returning `Response.error()` if no destination-specific fallback is configured. Review checklist:
- If no explicit `fallbacks.document` is configured, confirm whether the project instead relies on the plugin's documented convention path (`pages/_offline.tsx` or `app/~offline/page.tsx`) — the plugin auto-wires a default at that conventional location if the file exists; absence of explicit config is not automatically a gap if the convention file is present.
- If neither explicit `fallbacks.document` nor the convention file exists, the app has no navigation fallback at all — this is the direct cause of a raw browser offline error page and should be reported as a blocker, not a style nit.
- Confirm the fallback path itself is excluded from any auth-gating middleware — a fallback route that requires authentication to render defeats its own purpose during an offline navigation.

## Minimal safe verification flow

1. Identify which of the three patterns above (hand-rolled Workbox, `vite-plugin-pwa`, `next-pwa`) the project uses; do not assume, check the dependency and config file.
2. Confirm the specific fallback document path is present in the generated/inspectable precache manifest (build output, or DevTools Application > Cache Storage after first load).
3. Confirm a catch-handler/fallback route actually exists and targets that exact path — not merely that the path is precached.
4. Run the live offline-throttle navigation test from `references/live-verification-protocol.md` to confirm the fallback actually renders end to end; steps 1–3 alone are necessary but not sufficient proof.

## High-risk assumptions to kill

- "It's in the precache manifest, so offline works." — precache and fallback-routing are separate mechanisms; both must be verified.
- "The plugin handles this by default." — only true for the specific documented convention path/config key for that plugin and version; verify via Context7, do not assume parity across `vite-plugin-pwa` and `next-pwa` defaults.
- "The fallback page has no external dependencies, so it's safe offline." — verify this; a fallback page pulling a web font from a CDN, or making an inline API call for personalization, fails exactly when offline.
