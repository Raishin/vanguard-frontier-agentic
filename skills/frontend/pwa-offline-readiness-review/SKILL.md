---
name: pwa-offline-readiness-review
description: Validates installability against W3C manifest criteria and tests real offline navigation behavior end to end, rejecting a manifest-schema-valid but practically non-installable or non-functional-offline PWA.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: operational
---

# PWA Offline Readiness Review

## Purpose

A `manifest.json` that passes JSON-schema validation, and a Lighthouse PWA badge that reads 100, both prove far less than they appear to. The W3C manifest spec, `beforeinstallprompt` behavior, and offline-fallback rendering are three separate systems that must each work — a pass on one says nothing about the other two. The common failure mode is "checklist theater": a team ships a schema-valid manifest and a registered service worker, calls the app a PWA, and only discovers in production that the install banner never fires (`display: browser` left at a framework default) or that a network drop shows the browser's stock offline error page instead of a designed fallback. This skill performs the end-to-end verification — install criteria against the live origin, service-worker activation state, and an actual offline-throttle navigation test — that a static manifest read cannot substitute for.

## When to use

Use this skill when the user asks to:

- audit whether an app is a "real" installable PWA, not just manifest-schema-valid,
- debug why the install prompt / `beforeinstallprompt` never fires despite a Lighthouse PWA pass,
- review offline-fallback behavior, or debug a network drop showing the browser's default error page instead of a designed offline page,
- validate manifest fields (`name`, icons, `start_url`, `scope`, `display`) against W3C installability criteria before a release.

## When not to use

- Caching-strategy correctness for an already-installable app (route classification, `CacheFirst` vs. `NetworkFirst`, authenticated-response caching risk) — hand off to `service-worker-cache-strategy-review`; that skill owns caching-strategy depth and this one does not duplicate it.
- General HTTP `Cache-Control`/CDN caching review with no manifest or service-worker installability question involved.

## Context7 Documentation Protocol

Manifest-injection defaults, precache-manifest generation, and offline-fallback wiring differ by framework PWA plugin and by version — never assert a gap is "the app's fault" without checking the plugin's current documented default first.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded in this session.
2. Always ground manifest-field and installability-criteria claims against the W3C manifest spec directly (`official_docs`), not a vendor summary of it — vendor blog posts routinely lag or simplify the spec's `display`/`purpose`/icon-size language.
3. If the project uses `vite-plugin-pwa`, resolve `/websites/vite-pwa-org_netlify_app` and query its manifest-generation, `generateSW`/`injectManifest`, and precache `globPatterns`/`maximumFileSizeToCacheInBytes` behavior before asserting an asset or route is missing from precache "because the app didn't configure it" — the plugin has documented defaults and size ceilings that silently exclude assets.
4. If the project uses `next-pwa` (`@ducanh2912/next-pwa`), resolve `/ducanhgh/next-pwa` and query its `fallbacks` config (`document`/`data`/`image`/`audio`/`video`/`font`) and default-offline-page conventions (`pages/_offline.tsx` / `app/~offline/page.tsx`) before concluding a fallback route is absent — the plugin auto-wires a default path if the file exists at the conventional location.
5. If the underlying precaching/fallback mechanism is hand-rolled Workbox (not a framework plugin), resolve `/googlechrome/workbox` and query `offlineFallback`/`setCatchHandler`/`precacheAndRoute` semantics — confirm whether the fallback handler checks the precache before an `offline-fallbacks` cache, since a bug in that check order is a common cause of the fallback silently failing.
6. Query for the exact behavior in question per review, not from memory of a prior session — plugin defaults and Workbox recipe internals change across major versions.
7. If Context7 is unavailable or has no relevant match, fall back to `official_docs` / the bundled references, and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
8. Never invent a manifest field, plugin config key, or Workbox API that no queried source confirms.

## Lean operating rules

- Verify HTTPS on the actual target origin first — the W3C-implementing browsers refuse installability outright without it (localhost is the only common documented exception); if HTTPS is not confirmed, stop there, because every downstream installability check is moot until it is fixed.
- Treat `display: browser` as a deliberate product choice to surface and confirm with the user, not a silent bug to "fix" — it is valid per spec, but it disqualifies the install-prompt flow by design, and teams frequently leave it at a framework default without realizing the consequence.
- Do not declare an app "installable" from manifest JSON validity alone — confirm HTTPS, a service worker that reaches the `activate` state, and a qualifying `display` value together; each is independently necessary and none is sufficient alone.
- Do not declare an app "offline-ready" from precache-manifest file lists or service-worker registration code alone — perform an actual DevTools offline-throttle navigation to a previously unvisited route and observe what renders; precaching and route-matching are separate concerns, and a precached fallback page with an unmatched route still fails offline.
- Confirm the offline-fallback page is itself guaranteed precached, including its own dependencies (fonts, inline API calls, images) — a fallback page that depends on an uncached asset fails to render exactly when it is needed.
- Treat the Lighthouse PWA category score as a secondary, corroborating signal only — it can pass on stale audit state and does not reproduce the live `beforeinstallprompt` browser signal; require the manual browser-session listener test as the primary proof of installability.
- Flag any `start_url`, icon, or fallback-asset reference pointing to an HTTP origin or an unpinned third-party CDN — a mixed-content or unverifiable icon source undermines both installability and the fallback page's guarantee of availability.
- Never recommend caching user-specific or authenticated content inside the offline-fallback route — it must render identically and safely for any unauthenticated network-offline visitor; if the app's fallback design leaks account state, that is a blocker, not a caching-strategy nuance for the sibling skill to solve.
- Label every claim as `live evidence`, `spec-cited`, `documentation-based`, or `inference` so the reviewer knows what was actually tested versus reasoned about from file contents.

## References

Load these only when needed:

- [W3C manifest installability checklist](references/manifest-installability-checklist.md) — use when validating manifest field-by-field against W3C installability criteria (`name`/`short_name`, icons, `start_url`/`scope`, `display`) and when diagnosing a missing install prompt.
- [Offline fallback precache patterns](references/offline-fallback-precache-pattern.md) — use when reviewing how the offline-fallback route is registered and precached, across hand-rolled Workbox, `vite-plugin-pwa`, and `next-pwa` conventions.
- [Live installability and offline verification protocol](references/live-verification-protocol.md) — use for the step-by-step manual test procedure (HTTPS check, service-worker activation, `beforeinstallprompt` listener, DevTools offline-throttle navigation) and for interpreting Lighthouse-vs-live divergence.

## Response minimum

Return, at minimum:

- pass/fail verdict per W3C installability field (HTTPS, `name`/`short_name`, icons, `start_url`/`scope`, `display`), each labeled with its evidence level,
- service-worker registration and activation status (not just "registered" — confirm `activate` was reached),
- the actual (not inferred) offline-navigation test result for at least one previously unvisited route, and the offline-fallback page's precache/dependency-precache status,
- explicit note on whether `display` disqualifies install prompts by design and whether that was confirmed as intentional with the user,
- a handoff note to `service-worker-cache-strategy-review` if the gap identified is a caching-strategy implementation detail rather than an installability/offline-navigation gap.
