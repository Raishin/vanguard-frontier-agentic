# Workbox Strategy Semantics

Use this reference when a review needs to confirm the exact runtime behavior of a caching strategy — precache vs. `CacheFirst` vs. `NetworkFirst` vs. `StaleWhileRevalidate` vs. `NetworkOnly` — or the Vite-PWA `generateSW`/`injectManifest` config surface that produces it, before ruling on whether it matches a route's freshness/security needs.

## What people get wrong

The common bad assumption is:

> "Precaching is just cache-first for build assets."

Incomplete, and dangerous when carried into a security review. Per Workbox v7 source (`workbox-precaching/src/PrecacheStrategy.ts`, confirmed via Context7 against `/googlechrome/workbox`), `precacheAndRoute()` registers a `PrecacheRoute` whose `_handle` calls `handler.cacheMatch(request)` and **returns immediately on a hit — no network request, no revalidation, no HTTP involvement at all**. That is not "cache-first that also checks freshness sometimes" — it is a pure Cache API read with zero interaction with `Cache-Control`, `ETag`, or `Vary`. A runtime `CacheFirst` strategy is a different, separate mechanism: it also serves from cache on a hit, but falls through to a real network fetch (and populates the cache) on a miss. Treating the two as interchangeable in a finding misattributes which HTTP headers matter and which don't.

> Version note: Workbox internals are version-sensitive. Confirm exact behavior against the installed Workbox major version via Context7 (`/googlechrome/workbox`) or the live `https://developer.chrome.com/docs/workbox/` docs before ruling — do not cite this file's summaries as the final word for an unverified version.

## Officially grounded strategy shape

Per Workbox docs/source (Context7-confirmed):

- **Precache (`precacheAndRoute`)** — build-time-known, content-hashed assets. Served from Cache API with no revalidation, no HTTP cache-control involvement. Correct only for immutable, versioned assets where the precache manifest itself is the freshness mechanism (a new manifest = new cache entries).
- **`CacheFirst` (recipe/strategy)** — checks cache; on hit, returns immediately (no revalidation on hit, same as precache once cached); on miss, fetches network and populates cache. Per the Workbox Recipes README, this is "ideal for assets that rarely change, such as fonts or images" — not for anything that can change without a URL change.
- **`StaleWhileRevalidate`** — serves the cached response immediately if present, then makes a network request in the background to update the cache for the *next* request. Balances performance and freshness; the response returned to this request can still be stale by one cycle.
- **`NetworkFirst`** — attempts network first (optionally bounded by `networkTimeoutSeconds`); falls back to cache only on network failure/timeout. Appropriate for content that must be fresh when possible but should still work offline/flaky-network.
- **`NetworkOnly`** — never touches the cache for this route. Use for anything that must never be served stale or never be persisted client-side (mutations, authenticated reads that are hard-blocked from caching).
- **`ExpirationPlugin`** (`maxEntries`, `maxAgeSeconds`) — bounds a runtime cache's size/age; without it, a runtime cache used with `CacheFirst`/`StaleWhileRevalidate`/`NetworkFirst` grows unbounded. This does not apply to precache, which is version-bounded by the manifest itself.
- **`cleanupOutdatedCaches`** (`workbox-precaching`) — removes precache caches from prior manifest versions on activate. Confirm this (or equivalent manual `activate`-event cache deletion, as shown in Vite-PWA's `injectManifest` example querying `manifestURLs` and deleting non-listed keys) is present before endorsing any precache-based strategy.

## Vite-PWA config surface (when in use)

- `generateSW` strategy: `runtimeCaching` array maps `urlPattern` → `handler` (`'CacheFirst'`, `'NetworkFirst'`, `'StaleWhileRevalidate'`, `'NetworkOnly'`, etc.) + `options` (`cacheName`, `expiration`, `cacheableResponse`). Confirm every `runtimeCaching` entry's `urlPattern` is scoped tightly enough that it does not accidentally net authenticated API routes into a `CacheFirst` bucket meant for fonts/static assets.
- `injectManifest` strategy: a custom `sw.ts`/`sw.js` source, precache manifest injected via `self.__WB_MANIFEST`. Review the custom source directly — Vite-PWA does not add any safety net here; every strategy choice, `activate` cleanup, and `skipWaiting`/`clientsClaim()` call is entirely the author's responsibility.
- `clientsClaim()` + `self.skipWaiting()` — needed so a newly activated service worker takes control of open clients immediately rather than waiting for a full reload; absence of these (or an equivalent deliberate "update available, reload?" UX) means users can be stuck on stale app-shell logic indefinitely even after a successful deploy.

## Verification targets

- Confirm the strategy name in code/config against this file's semantics table, then confirm current behavior via Context7 before finalizing — do not rely on the strategy's *name* alone (e.g., "NetworkFirst" implementations have varied `networkTimeoutSeconds` defaults across versions).
- For any runtime-cached route, confirm an `ExpirationPlugin` (or Vite-PWA `expiration` option) is present with a `maxAgeSeconds`/`maxEntries` bound appropriate to the route's sensitivity and volatility.
- For precache, confirm `cleanupOutdatedCaches` or equivalent manual `activate` cleanup runs so superseded manifest versions are purged.

## High-risk assumptions to kill

- "Precache is basically cache-first" — precache bypasses HTTP entirely; runtime `CacheFirst` still makes a real network request on a cache miss.
- "`StaleWhileRevalidate` means the user always gets fresh data" — the response served to the *current* request can be a stale copy; freshness only improves for the *next* request.
- "No `ExpirationPlugin` is fine, the cache will sort itself out" — Cache API storage does not auto-evict; unbounded runtime caches grow until storage quota pressure causes browser-driven eviction, which is unpredictable and not a substitute for a deliberate policy.
- "We use `generateSW`, so Vite-PWA handles safety for us" — `generateSW` only wires the strategy the author picked in `runtimeCaching`; it does not choose a safe strategy per route automatically or exclude authenticated routes on its own.
