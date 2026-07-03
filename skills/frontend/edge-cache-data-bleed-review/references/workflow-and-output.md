# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific caching surface the code under review actually raises.

## Prerequisites

- Confirm the Next.js major version and router in `package.json`. `'use cache: private'`, `revalidate` route segment config, and `dynamic = 'force-dynamic'` are App Router (Next.js 13+ `app/`) concepts. A Pages Router app (`pages/`) uses `getServerSideProps`/`getStaticProps` and `res.setHeader('Cache-Control', ...)` instead -- the header-based findings still apply there, but the directive-based findings (`'use cache: private'`, route segment `revalidate`) do not.
- Identify every route, layout, server function, and Route Handler that reads `cookies()`, `headers()`, or another per-request/per-user runtime API. These are the candidate personalization points this skill traces forward from.

## Workflow

1. **Locate every route segment config export.** Grep for `export const revalidate`, `export const dynamic`, and `generateStaticParams` across `app/**/page.tsx`, `app/**/layout.tsx`, and `app/**/route.ts`.
2. **For each route carrying `revalidate`, trace whether it (or any server function/component it renders) reads `cookies()`, `headers()`, or another per-request API.** If it does, check whether that specific lookup is isolated behind `'use cache: private'`. See `references/caching-directives-and-route-config.md` for the decision tree.
3. **For each route with `generateStaticParams`, inspect the params being enumerated.** If they are user-, account-, or org-scoped IDs, check whether the route is also authenticated (renders session-derived data) and whether it carries `revalidate` or defaults to static generation without `dynamic = 'force-dynamic'`.
4. **Enumerate every server function (not just page-level components) that reads `cookies()`/`headers()` to produce a per-user result.** For each, confirm `'use cache: private'` is declared as the function's own first statement -- do not accept "no cache export nearby" as sufficient; the isolation must be intrinsic to the function.
5. **Enumerate every Route Handler (`route.ts`) and `getServerSideProps` call that sets response headers.** For each, trace whether the response body is derived from `cookies()`/session/account data, and check the `Cache-Control` and `Vary` header values. See `references/cache-control-and-vary-headers.md`.
6. **Produce ranked findings** using the output contract below.

## Decision tree

- Route carries `export const revalidate = N` and reads `cookies()` (directly or via a called function) with no `'use cache: private'` isolating that read → **HIGH** finding, `revalidate-cookie-bleed`. The shared ISR cache entry serves the first requester's personalized render to everyone else within the window.
- A server function reads `cookies()`/`headers()` to produce a per-user result and declares no `'use cache: private'` (or equivalent) boundary of its own → **HIGH** finding, `missing-private-cache-boundary`, regardless of whether a `revalidate` export currently sits nearby -- the risk is that any future caching wrapper reintroduces the bleed with no defense at the function itself.
- `generateStaticParams` enumerates user/account-scoped IDs, the route also carries `revalidate`, and the page renders authenticated per-user data → **HIGH** finding, `static-params-auth-bleed`. The safe fix is `export const dynamic = 'force-dynamic'` with no `revalidate`/static-params pairing for that authenticated path.
- `generateStaticParams` enumerates IDs for genuinely public, non-personalized content (e.g. public blog post IDs with no auth-gated data in the render) → not a finding; state this explicitly rather than silently omitting it.
- A Route Handler or `getServerSideProps` response derived from `cookies()`/session data sets `Cache-Control: public` (or omits `Cache-Control` while behind a CDN that caches by default) → **HIGH** finding, `header-cache-bleed`.
- The same response also sets `Vary` but the value does not include `Cookie` (or the actual header carrying the session identifier) → **MEDIUM-to-HIGH** finding depending on whether the surface is public-facing or requires an existing session to reach, `header-cache-bleed`. A `Cache-Control: private` response makes the `Vary` gap moot -- only flag it when the response is otherwise CDN-cacheable.
- Response sets `Cache-Control: private`, or relies on Next.js's documented dynamic-rendering default (`private, no-cache, no-store, max-age=0, must-revalidate`), for a `cookies()`-derived response → not a finding.

## Output contract

Every response from this skill must return:

1. **Scope** -- the route(s), server function(s), and/or response-header call sites reviewed.
2. **Ranked findings** -- each with file:line, defect category (`revalidate-cookie-bleed` / `missing-private-cache-boundary` / `static-params-auth-bleed` / `header-cache-bleed`), the concrete data-flow trace (the route segment config and the per-request read it coexists with, or the header value and the per-user data source), and a fix sketch matching Next.js's documented pattern.
3. **Cache-boundary status per `cookies()`-derived finding** -- an explicit statement of whether `'use cache: private'` (or `Cache-Control: private`) is present on the traced path; never infer one exists elsewhere in the codebase.
4. **Evidence level per finding** -- `repo evidence`, `documentation-based`, or `inference`. Label structural risk findings as structural risk explicitly -- do not imply confirmed exploitation without live evidence (e.g., a captured cross-user response from a deployed CDN).
5. **Verdict** -- approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** -- e.g., "confirming an actual cross-user response replay requires a live CDN reproduction with two concurrent sessions, not static review," or "ISR revalidation-timing tuning is out of scope -- this review covers cache-boundary correctness, not cache-hit-ratio performance."

## When to push back

Push back if the user asks to:

- approve a `revalidate`-plus-`cookies()` route because "the window is only 10 seconds, so the exposure is small" -- any nonzero shared window during which a personalized response is replayed to a different user is a data-exposure defect, not a tunable performance tradeoff,
- skip the `'use cache: private'` check on a server function because "nothing currently wraps it in a shared cache" -- the isolation needs to be intrinsic to the function so a future refactor cannot silently reintroduce the bleed,
- treat a `Vary: Cookie` header as sufficient on its own without checking `Cache-Control` -- if `Cache-Control` is `private` (or absent, defaulting to Next.js's dynamic-rendering default), the CDN never caches the response at all and `Vary` is moot; if `Cache-Control` is `public`, `Vary: Cookie` alone does not make blanket caching safe unless every downstream cache actually honors `Vary` correctly,
- downgrade an untraced `generateStaticParams`-plus-`revalidate` finding on an authenticated route to informational because "it's probably fine" -- this skill's default is HIGH for exactly this class of unproven claim.
