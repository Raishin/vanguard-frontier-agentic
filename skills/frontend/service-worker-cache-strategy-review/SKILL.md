---
name: service-worker-cache-strategy-review
description: Reviews service-worker route-matching and caching-strategy choices (precache vs. cache-first vs. network-first vs. stale-while-revalidate) against request type and security sensitivity, rejecting uniform blanket strategies and flagging authenticated/PII responses cached in the Cache API.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: security
---

# Service Worker Cache Strategy Review

## Purpose

A service worker is a persistent, cross-session, JS-controlled cache layer that sits in front of every network request the browser makes for a scope. Unlike HTTP caching, it is not opt-in per response — once a route is matched, the developer's strategy code decides freshness and disclosure, not `Cache-Control`. The common failure mode is copy-pasting one strategy (usually `CacheFirst`, straight from a tutorial) across navigation, API, and asset routes without differentiating by freshness need or security sensitivity. This skill performs the per-route-class review that catches both the stale-content incidents that uniform cache-first causes on navigation/API routes, and the security incidents that happen when authenticated or PII-bearing responses land in a persistent Cache API store that Workbox strategies do not automatically protect.

## When to use

Use this skill when the user asks to:

- review a service-worker file or Workbox config (`generateSW`/`injectManifest`) for caching-strategy correctness,
- debug reports of stale content after deploy, broken offline pages, or unexpectedly cached dynamic/API data,
- choose or validate a caching strategy per route class (navigation, API, static asset) before shipping,
- audit `scope` / `Service-Worker-Allowed` coverage and cache-versioning/cleanup behavior.

## When not to use

- Manifest/installability review with no caching-behavior question — use `pwa-offline-readiness-review` instead.
- General HTTP `Cache-Control`/`ETag` header review with no service worker involved — that is standard HTTP caching, a narrower and materially different concern than programmatic Cache API control.

## Context7 Documentation Protocol

Workbox strategy internals and Vite-PWA config surface change between major versions — never assert runtime semantics (what bypasses HTTP headers, what revalidates, what the default `expiration` behavior is) from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded in this session.
2. Resolve `/googlechrome/workbox` for the strategy implementation in question (`PrecacheStrategy`, `CacheFirst`, `NetworkFirst`, `StaleWhileRevalidate`, `NetworkOnly`, `ExpirationPlugin`, `cleanupOutdatedCaches`).
3. Resolve `/websites/vite-pwa-org_netlify_app` (or the closest match) when the project uses Vite PWA's `generateSW`/`injectManifest` config surface, `runtimeCaching`, `skipWaiting`/`clientsClaim`, or custom `injectManifest` service-worker source.
4. Query for the exact behavior before ruling — e.g. "does PrecacheStrategy revalidate against Cache-Control", "NetworkFirst networkTimeoutSeconds fallback behavior", "ExpirationPlugin maxAgeSeconds vs maxEntries eviction order" — per review, not once from a prior session.
5. A confirmed, version-specific fact from Context7 (e.g. `precacheAndRoute` serves via `PrecacheStrategy`, which reads from the Cache API and returns immediately on a hit, completely bypassing HTTP `Cache-Control` since no network round-trip occurs) is materially different from the general folk claim "precache is cache-first" — cite the specific mechanism, not the folk version.
6. If Context7 is unavailable or has no relevant match, fall back to `official_docs` / `references/workbox-strategy-semantics.md`, and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
7. Never invent a Workbox option, plugin, or config key that no queried source confirms.

## Lean operating rules

- Classify every matched route into navigation/HTML, API/data (split further: read vs. write, public vs. authenticated), and static asset before judging any single strategy — a strategy is only correct or incorrect relative to its route class.
- Treat `PrecacheStrategy`/precache-and-route as a distinct mechanism from runtime `CacheFirst` — precache serves from the Cache API with no revalidation and no HTTP `Cache-Control` involvement at all; do not describe the two interchangeably.
- Cache API only stores GET responses by spec — if a review encounters a manual `cache.put()` on a non-GET request or response, treat that as a code smell requiring explanation, not a strategy question.
- Any response containing `Set-Cookie`, an echoed `Authorization` value, or clearly PII/payment-bearing JSON is a hard block on caching, regardless of the performance justification offered — recommend `NetworkOnly` or explicit route exclusion instead.
- Never accept "it's cached, so it's fast, so it's fine" as sufficient for navigation/HTML routes — blind cache-first strands users on a stale app shell after every deploy; require `StaleWhileRevalidate` or `NetworkFirst` there instead.
- Flag `cache.put()` on an opaque, cross-origin `no-cors` response — the caller cannot inspect status or headers on an opaque response, so a poisoned or error response can be cached and served indefinitely with no visibility.
- Verify `scope` (registration time) and `Service-Worker-Allowed` (response header, only needed when the script itself sits outside the desired scope) match the intended route coverage exactly — broader-than-needed scope expands blast radius for every finding above.
- Verify a versioned cache-name scheme plus an `activate`-event cleanup step (or Workbox's `cleanupOutdatedCaches`) exists, and that `skipWaiting`/`clients.claim()` or a deliberate user-prompted update flow gets fixes to users in bounded time — otherwise caches grow unbounded and rollbacks/forward-fixes never land.
- Query current Workbox/Vite-PWA docs (see Context7 Documentation Protocol) for the specific strategy/option in question before ruling; runtime semantics are version-sensitive and have changed across Workbox major versions.
- Label every claim as `live evidence`, `spec-cited`, `documentation-based`, or `inference` so the reviewer knows what has actually been verified vs. reasoned about.

## References

Load these only when needed:

- [Workbox strategy semantics](references/workbox-strategy-semantics.md) — use when confirming the exact runtime behavior of a specific strategy (precache vs. `CacheFirst` vs. `NetworkFirst` vs. `StaleWhileRevalidate` vs. `NetworkOnly`), expiration/cleanup mechanics, or Vite-PWA `generateSW`/`injectManifest` wiring.
- [Route classification and strategy matrix](references/route-classification-matrix.md) — use when mapping a concrete route inventory to a strategy per class and justifying the mapping.
- [Cache security and scope audit](references/cache-security-and-scope-audit.md) — use before endorsing any strategy change, when reviewing authenticated/PII route handling, opaque-response caching, or `scope`/`Service-Worker-Allowed`/cache-versioning coverage.

## Response minimum

Return, at minimum:

- the route classification (navigation / API read-public / API read-authenticated / API write / static asset) for every matched route pattern in scope,
- per-class strategy verdict with the Workbox-version-confirmed runtime semantics behind it,
- security flags (blocker severity) for any authenticated/PII response cached, any opaque cross-origin `cache.put()`, or any scope broader than required,
- cache-versioning and `activate`-cleanup audit result, including whether `skipWaiting`/`clients.claim()` or an equivalent update path exists,
- verification steps (DevTools Application > Cache Storage inspection of actual cached entries, offline-throttle test) and a rollback note (cache-name version bump plan).
