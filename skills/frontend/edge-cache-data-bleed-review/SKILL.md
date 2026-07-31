---
name: edge-cache-data-bleed-review
description: Statically review Next.js App Router caching surfaces -- route-level revalidate exports, cache-boundary directives on server functions reading cookies(), generateStaticParams on personalized routes, and Cache-Control/Vary response headers -- for defects that let one user's authenticated response be cached and served back to a different user.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# Edge Cache Data-Bleed Review

## Purpose

Review Next.js App Router pages, server functions, Route Handlers, and response headers for the concrete caching-layer defect this skill is scoped to: a per-user, session-derived response getting written into a cache shared across requests (ISR, `'use cache'`, or a CDN/proxy edge cache) and then replayed to a different user. This skill exists so the review stays anchored to the documented caching primitives Next.js exposes for exactly this problem -- `revalidate`, `'use cache: private'`, `dynamic = 'force-dynamic'`, and the `Cache-Control`/`Vary` response headers -- instead of drifting into a general "caching performance review" of ISR tuning, `fetch` cache options, or CDN cost optimization with no data-exposure angle.

## When to use

Use this skill when the user asks to:

- review a page or layout under `app/` that reads `cookies()` (or another per-request/per-user API) and also carries a route-level `revalidate` export,
- assess whether a server function or Server Component that reads `cookies()` needs `'use cache: private'`,
- review a dynamic route using `generateStaticParams` where the generated params are user or account IDs, to check whether the route is safely dynamic or is silently serving a shared, revalidate-windowed cache to authenticated users,
- audit a Route Handler's or `getServerSideProps`'s response headers (`Cache-Control`, `Vary`) for a page or API response that includes session-, cookie-, or account-derived data,
- perform a pre-launch security review of a Next.js app's caching configuration for cross-user data bleed.

Do not use this skill for:

- ISR/`fetch`-cache performance tuning, `cacheLife`/`stale-while-revalidate` timing choices, or CDN cost/latency optimization with no user-specific-data angle -- those are performance concerns, not this skill's data-exposure scope,
- a purely client-side `localStorage`/`sessionStorage`/in-memory cache with no server-side or CDN-shared cache layer -- browser-local storage is isolated per browser profile and is out of scope for this skill's cross-user cache-bleed concern,
- a bug that requires live traffic reproduction (actually observing User B receive User A's cached response from a deployed CDN) to prove exploitation -- static analysis proves the structural risk, not that it has already been exploited in production.

## Context7 Documentation Protocol

- Resolve the Next.js library ID with `resolve-library-id` (matched result: `/vercel/next.js`) before citing any `revalidate`, `'use cache: private'`, `generateStaticParams`, `dynamic` route-segment, or header-caching behavior claim.
- `/vercel/next.js` and `/websites/nextjs` are high-reputation sources covering the App Router's caching directives (`'use cache'`, `'use cache: private'`), route segment config (`revalidate`, `dynamic`), and header-based caching (`Cache-Control` in Route Handlers and `getServerSideProps`) directly from the framework's own docs and source. Use `query-docs` against them to confirm exact directive syntax and documented per-user-vs-shared cache semantics before writing a finding.
- The `Vary` header's role in CDN/proxy cache-key selection is general HTTP caching semantics, not a Next.js-specific API -- Context7 against `/vercel/next.js` will not reliably surface it. Ground a `Vary` finding against the `official_docs` MDN/HTTP-standard URL in this skill's `metadata.json` instead and label the claim `documentation-based`.
- Read `package.json` first to confirm the Next.js major version and whether the app uses the App Router (where `'use cache: private'`, `revalidate`, and `dynamic` route segment config apply) or the Pages Router (`getServerSideProps`/`getStaticProps`, which use a different, older caching model) -- do not apply App Router directive syntax to a Pages Router codebase or vice versa.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- All findings in this skill's scope default to HIGH severity, except an incomplete `Vary` header alone (no other caching misconfiguration present), which defaults to MEDIUM-to-HIGH depending on reachability. This is a security-scoped skill: do not downgrade a structural cross-user cache-bleed risk to MEDIUM just because it has not been observed exploited yet -- the risk is in the caching structure, not in whether someone has already hit it.
- Trace every finding to a concrete file:line and a concrete data-flow path. A finding that says "this route might leak data between users" without showing the specific `revalidate` export, the specific `cookies()` read it coexists with, or the specific response header value is not a valid finding -- it is a guess.
- Flag any route or layout that combines a route-level `export const revalidate = N` with a `cookies()` read (directly, or through a server function it calls) and has no `'use cache: private'` isolating that per-user lookup. `revalidate` governs a cache entry shared by every request that hits the route within the window; a personalized `cookies()`-derived render sharing that entry is the core defect this skill exists to catch.
- Flag any async function that reads `cookies()` (or another per-request runtime API) to produce a per-user result and does not declare `'use cache: private'` as its own cache boundary. Do not accept "there's no `revalidate` export nearby so it's probably fine" -- a future refactor that wraps the call site in any shared cache scope re-introduces the bleed silently; the safe pattern is the function declaring its own privacy boundary, not the absence of a nearby cache export.
- Flag a dynamic route whose `generateStaticParams` enumerates user- or account-scoped IDs (e.g. `userId`, `accountId`, `orgId`) when the route also carries a `revalidate` export and the page renders authenticated, per-user data. The safe idiom is `export const dynamic = 'force-dynamic'` on that route with no `generateStaticParams`/`revalidate` pairing for the authenticated path.
- Flag any `Response`/`NextResponse` (Route Handler) or `getServerSideProps` `res.setHeader` call that sets `Cache-Control: public` (or omits `Cache-Control` while sitting behind a CDN that defaults to caching) on a response whose body is derived from `cookies()`, a session token, or other per-user request context. The fix is `Cache-Control: private`, or omitting an explicit header and relying on Next.js's own dynamic-rendering default (`private, no-cache, no-store, max-age=0, must-revalidate`).
- Flag any response that sets an explicit `Cache-Control: public` (or otherwise CDN-cacheable) header on user-specific data and also sets a `Vary` header that does not include `Cookie` (or whatever header actually carries the session identifier). A CDN keys its cache only on the headers named in `Vary`; without `Cookie` present, requests from two different users are treated as cache-equivalent.
- Never execute, build, or run application code, and never send live requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) -- use for the step-by-step review procedure, the per-defect decision tree, and the required output shape.
- [Caching directives and route segment config](references/caching-directives-and-route-config.md) -- load when reviewing `revalidate`, `'use cache: private'`, `generateStaticParams`, or `dynamic` route segment config.
- [Cache-Control and Vary response headers](references/cache-control-and-vary-headers.md) -- load when reviewing a Route Handler's or `getServerSideProps`'s response headers. Includes the MDN HTTP-caching grounding reference for `Vary`; load that citation only when a `Vary` finding is actually present.

## Response minimum

Return, at minimum:

- the route(s), server function(s), and/or response-header call sites in scope,
- ranked findings with file:line evidence, defect category (`revalidate-cookie-bleed`, `missing-private-cache-boundary`, `static-params-auth-bleed`, or `header-cache-bleed`), the concrete data-flow trace (the `revalidate`/`generateStaticParams`/`dynamic` config and the `cookies()` read it coexists with, or the header value and the per-user data it exposes), and a fix sketch matching Next.js's documented pattern,
- for every finding involving a `cookies()`-derived value, an explicit statement of whether `'use cache: private'` (or an equivalent per-user isolation boundary) is present on the traced path -- never approve on the assumption one exists elsewhere,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming an actual cross-user response replay requires a live CDN reproduction with two concurrent sessions, not static review").
