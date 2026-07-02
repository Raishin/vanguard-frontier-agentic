# Cache Security and Scope Audit

Use this reference before endorsing any caching-strategy addition or change, when reviewing authenticated/PII route handling, opaque cross-origin response caching, or `scope`/`Service-Worker-Allowed`/cache-versioning coverage. This is the hard-gate section of the skill — findings here are blockers, not style notes.

## What people get wrong

The common bad assumption is:

> "It worked fine when I tested it, so the caching is safe."

Wrong, and dangerously so for shared/multi-account devices. A service worker's Cache API store is per-origin and persists across navigations and sessions — it has no built-in concept of "which user" a cached response belongs to. If an authenticated route (`/api/user`, `/api/account`, `/api/orders/:id`) is cached with any strategy that reads before checking identity, a single-session manual test will look correct while silently setting up cross-user data disclosure the moment a second user logs in on the same device without a full cache purge. This is not a hypothetical: it is the single most severe class of finding this skill exists to catch, and it is invisible to functional QA that only ever tests with one account.

## Non-negotiables

- Do not endorse caching a response containing `Set-Cookie`, an echoed `Authorization` header/token value, or clearly PII/payment-bearing JSON, under any performance justification. The correct strategy for such a route is `NetworkOnly` or explicit exclusion from every `registerRoute`/`urlPattern` matcher — not a "short TTL" compromise.
- Do not endorse a `scope` broader than the routes the service worker is meant to control. `scope` is set at `register()` time (`navigator.serviceWorker.register(url, { scope })`) and defines the maximum set of URLs the worker can intercept; a worker registered at `/` when it only needs to control `/app/` unnecessarily expands the blast radius of every other finding in this file.
- `Service-Worker-Allowed` is a response header needed only when the service-worker *script* itself is served from a path outside the desired scope (e.g., script at `/sw.js` needs to control `/app/`, requiring the server to send `Service-Worker-Allowed: /app/` on the script response). Confirm it is present and no broader than necessary whenever the script path and intended scope diverge.
- Flag `cache.put()` on an opaque (`no-cors`, cross-origin) response as an unreviewable-content risk per OWASP cache-poisoning guidance applied to Cache API misuse: an opaque response's status and headers cannot be inspected by the calling JS, so a `404`, an error page, or a compromised/poisoned upstream response can be cached and served as if it were a valid `200` indefinitely, with zero visibility from the caching code.
- Do not accept "we'll purge the cache manually if this becomes a problem" as a mitigation for authenticated-data caching. There is no reliable trigger for "manually" here — the fix is not caching it in the first place.

## Minimal safe audit flow

1. Enumerate every route matcher and cross-reference against `route-classification-matrix.md` to identify which, if any, are classified authenticated/PII.
2. For every authenticated/PII route, confirm the applied strategy is `NetworkOnly` or that the route is excluded from all matchers (i.e., falls through to the browser's normal network path with no service-worker interception at all).
3. Open DevTools → Application → Cache Storage (or run the equivalent inspection command in an automated check) and manually inspect actual cached entries — do not trust config intent alone. Look for any entry whose URL matches an authenticated/PII route class, and for any response body containing session tokens, emails, names, or account identifiers that shouldn't be persisted client-side.
4. Confirm `scope` at registration matches the intended coverage, and `Service-Worker-Allowed` (if applicable) is no broader.
5. Confirm a versioned cache-name scheme exists (e.g., `app-cache-v3`) and that `activate` either calls `cleanupOutdatedCaches()` (Workbox precaching) or manually deletes cache keys not in the current manifest/version — an unbounded, unversioned cache cannot be reliably rolled back or purged.
6. Confirm any `cache.put()` call on a cross-origin request is not operating on an opaque response, or is explicitly justified and reviewed if it must be (e.g., a trusted, pinned third-party origin with documented risk acceptance).

## Adversarial checklist

Before signing off on any caching-strategy change, answer these:

- Is every authenticated/PII route verified excluded by inspecting actual cached entries in DevTools, not just by reading the strategy config?
- If this device is shared or a second user logs in without a full logout/cache-clear, can any previously cached response leak across accounts?
- Does `scope` match only what this worker needs to control, verified against the actual `register()` call, not assumed from the script's file location?
- Is there a versioned cache-name and `activate`-time cleanup path, verified by checking that an old-version cache key actually gets deleted after a version bump — not just present in code but unexercised?
- Is every `cache.put()` on a cross-origin request either non-opaque (`cors` mode with an inspectable response) or explicitly justified?

If any of these cannot be answered from direct evidence (config read, DevTools inspection, or explicit user-provided confirmation), the finding is a residual risk, not a pass.

## When to push back

Push back if the user asks for:

- caching an authenticated API response "just for this one screen" to make it feel faster — the risk is identical regardless of scope of use,
- a broader `scope` than the app actually needs "in case we add more routes later" — expand scope when the need is real, not speculatively,
- skipping the DevTools cache-entry inspection step because "the config looks right" — config intent and actual cached content diverge often enough (matcher bugs, stale test data, prior strategy still in a live cache) that this step is not optional.
