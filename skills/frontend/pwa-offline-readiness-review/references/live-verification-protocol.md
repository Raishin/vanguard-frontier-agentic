# Live Installability and Offline Verification Protocol

Use this reference for the step-by-step manual test procedure that turns a static-file read into an evidence-backed verdict, and for interpreting divergence between the Lighthouse PWA category score and live browser behavior.

## What people get wrong

The naive story is:

> "Lighthouse gave the PWA category a 100, so it's installable and offline-ready."

Wrong, or at least insufficient. Lighthouse's PWA audit historically diverges from live installability criteria for several documented reasons: it can run against a cached or stale service-worker state, it evaluates a fixed audit checklist that has changed across Lighthouse versions (some historical PWA-specific badge criteria were deprecated/removed from newer Lighthouse releases entirely), and — critically — it does not reproduce the actual `beforeinstallprompt` event firing in a live user session, which is the real signal a browser uses to decide whether to show an install affordance. A high Lighthouse score is a corroborating signal, not proof.

## Step-by-step protocol

### 1. Confirm HTTPS on the actual deployed origin

Check the origin Lighthouse (or any static review) was run against versus the actual production/target origin. It is a common gap for a review to validate a staging or preview URL that differs in TLS configuration from the real deployment target. Record this as `live evidence` only if checked against the real target origin; otherwise label `documentation-based` / `inference`.

### 2. Confirm service-worker registration reaches `activate`

In a real browser session (DevTools > Application > Service Workers), confirm:
- a service worker is listed for the correct scope,
- its status is `activated and is running` (not merely `installed` or stuck in `waiting`),
- there are no registration errors in the console (common causes of registration failure: scope mismatch between the registering script's location and the intended `scope`, a JavaScript error thrown during script evaluation, or an incorrect MIME type served for the service-worker script causing the browser to refuse it).

A service worker that never reaches `activate` cannot serve precached content or a fallback route, regardless of what the source code says it should do.

### 3. Listen for `beforeinstallprompt` directly

In a real browser session on the target origin, add a listener before any other interaction:

```js
window.addEventListener('beforeinstallprompt', (e) => {
  console.log('beforeinstallprompt fired', e);
});
```

If this event never fires despite all manifest and service-worker criteria appearing correct on paper, treat that as the authoritative negative signal and re-check (in order): `display` value, HTTPS on the exact origin under test, service-worker `activate` state, and whether the browser under test has already recorded a prior install/dismissal for this origin (some browsers suppress repeat prompts for a cooldown period after a user dismisses one — check for a documented per-browser dismissal-cooldown behavior before concluding the app itself is broken).

### 4. Offline-throttle navigation test (the non-negotiable step)

This is the step most reviews skip, and skipping it is the single most common cause of a false "offline-ready" verdict.

1. Load the app normally first (to allow the service worker to install/activate and precache assets).
2. In DevTools > Network, set throttling to **Offline** (not just slow 3G — must be a hard offline state to exercise the fallback path, not a slow-network path).
3. Navigate to a route the user has **not** previously visited in this session (revisiting an already-cached page proves only that specific page's cache entry works, not the fallback mechanism).
4. Observe what renders:
   - **The browser's default offline error page (e.g., the dinosaur game / "No internet" interstitial)** — the fallback route/catch-handler is missing, misconfigured, or the fallback document itself was not precached. This is a hard fail; report per `references/offline-fallback-precache-pattern.md`.
   - **The app's designed offline page** — record as `live evidence` pass. Additionally confirm the page rendered with its full intended styling/images (a fallback page that renders as unstyled HTML because its CSS was not precached is a partial fail, not a full pass).
5. While still offline, attempt navigation to a second previously-unvisited route to confirm the fallback is not a one-off fluke tied to a specific route's cache warm state.

### 5. Confirm the fallback page's own dependencies

With DevTools still offline, inspect the rendered fallback page's network panel for any failed sub-resource requests (fonts, inline API calls, tracking scripts). A fallback page that fails to render its intended appearance because of an uncached font or a blocking synchronous API call is a partial failure — report it distinctly from a fully missing fallback.

## Interpreting Lighthouse-vs-live divergence

| Lighthouse PWA signal | Live signal | Verdict |
|---|---|---|
| High score | `beforeinstallprompt` fires, offline test passes | Corroborated pass — cite both. |
| High score | `beforeinstallprompt` never fires | Live signal wins. Report the divergence explicitly and investigate `display`, origin mismatch, or per-browser dismissal-cooldown before concluding a manifest defect. |
| High score | Offline navigation shows default browser error page | Live signal wins. This is the checklist-theater failure mode this skill exists to catch — report as a blocker regardless of the Lighthouse score. |
| Low/missing score | Live signals pass | Investigate why Lighthouse diverges (stale audit run, outdated Lighthouse version, audit run against wrong origin) before treating the live pass as unreliable — a passing live test is still the stronger evidence. |

## Verdict discipline

Never present a Lighthouse score alone as proof of either installability or offline readiness in the final response. Every "offline-ready" or "installable" verdict in the response must cite the specific live test performed (service-worker activation check, `beforeinstallprompt` listener result, offline-throttle navigation outcome) as the primary evidence, with Lighthouse noted only as a secondary, corroborating data point.
