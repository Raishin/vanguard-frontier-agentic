# Route Classification and Strategy Matrix

Use this reference when mapping a concrete route inventory (the actual `registerRoute`/`urlPattern` matchers in the service worker or Workbox config) to a caching strategy per class, and when a review needs to justify why one class gets a different strategy than another.

## What people get wrong

The common bad assumption is:

> "We picked a caching strategy for the service worker" — as if one strategy applies to the whole app.

Wrong. A service worker's `fetch` handler intercepts every request type in its scope — HTML navigations, API calls, images, fonts, scripts — and each has a different freshness contract and a different security profile. A single copy-pasted strategy (almost always `CacheFirst`, lifted from a tutorial about caching fonts) applied uniformly is the single most common defect this skill exists to catch. It is not "faster" across the board; it is wrong for at least two of the four classes below in nearly every real app.

## Non-negotiable design rule: classify before you judge

Before ruling on any single `registerRoute`/`runtimeCaching` entry, classify every matched route into one of these buckets. A strategy is only correct or incorrect *relative to its class* — there is no universally "best" strategy.

## Route class → strategy matrix

| Route class | Example | Correct default strategy | Why |
|---|---|---|---|
| Static asset, content-hashed filename | `/assets/app.a1b2c3.js`, webfonts | Precache (`precacheAndRoute`) or `CacheFirst` with long `maxAgeSeconds` | Immutable — a content change produces a new URL, so serving a stale cached copy under the old URL is impossible by construction. |
| Static asset, NOT content-hashed | `/logo.png`, `/favicon.ico` | `StaleWhileRevalidate` with `ExpirationPlugin` | Can change without a URL change; needs periodic revalidation, but instant-from-cache is still safe since content is non-sensitive. |
| Navigation / HTML (app shell, document requests) | `/`, `/dashboard` (document destination) | `StaleWhileRevalidate` or `NetworkFirst` (short `networkTimeoutSeconds`) — never blind `CacheFirst` | Blind cache-first strands users on a stale app shell after every deploy, since the HTML entry point is how new asset references reach the client at all. |
| API GET, public/non-sensitive | `/api/public-catalog` | `StaleWhileRevalidate` or `NetworkFirst` with short TTL (`ExpirationPlugin.maxAgeSeconds`) | Needs freshness bounded by business tolerance for staleness; non-sensitive, so caching itself is not a security concern — only staleness is. |
| API GET, authenticated/PII-bearing | `/api/user`, `/api/account`, `/api/orders/:id` | `NetworkOnly`, or explicit exclusion from any `registerRoute` match entirely | Hard security block — see `cache-security-and-scope-audit.md`. No performance justification overrides this. |
| API mutation (POST/PUT/PATCH/DELETE) | any write endpoint | Not cacheable by Cache API spec (GET-only); verify no workaround exists | If a manual `cache.put()` appears on a mutation response, that is a code smell requiring explanation on its own, independent of strategy choice. |
| Cross-origin, third-party, `no-cors` | third-party widgets, uncontrolled CDNs | Avoid `cache.put()` on the resulting opaque response, or scope tightly with clear justification | Opaque responses cannot be inspected for status/headers — see `cache-security-and-scope-audit.md`. |

## Verification targets

- Enumerate every `registerRoute`/`urlPattern` matcher in the file/config under review and assign it exactly one row from the matrix above — an unassignable route (matches nothing cleanly) is itself a finding: the matcher is probably too broad.
- For each matcher, confirm the *matcher itself* is scoped correctly — a regex like `/^\/api\//` bundling public and authenticated endpoints under one strategy is a classification bug even before the strategy choice is evaluated.
- Confirm no single strategy declaration (e.g., one `CacheFirst` block) is reused across more than one route class without an explicit justification for why that class's freshness/security needs happen to coincide.

## High-risk assumptions to kill

- "One strategy for the whole app is simpler to maintain" — simplicity here trades directly against both stale-deploy incidents and cross-user data leakage; the per-class matrix is the actual minimum safe design, not an optional refinement.
- "It's a GET request, so it's safe to cache" — GET-only is a Cache API constraint, not a security guarantee; `/api/user` is a GET and is exactly the kind of route that must never be cache-first.
- "The tutorial cached fonts with CacheFirst, so CacheFirst is the safe default" — `CacheFirst` is only safe for the specific class (immutable/rarely-changing, non-sensitive) the tutorial was written for.

## When to push back

Push back if the user asks to:

- apply one strategy across the whole `fetch` handler "to keep the config simple,"
- cache an authenticated or account-scoped API route with any strategy other than `NetworkOnly`/exclusion, regardless of the performance rationale offered,
- skip per-route classification because "it's basically all API calls" — read/write and public/authenticated are not the same bucket even when the URL prefix is shared.
