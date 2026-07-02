# Review workflow and findings contract

Use this reference for the full rendering/caching review procedure and the required output shape.

## What people get wrong

The naive story is:

> Static is faster, dynamic is safer for personalized data, so just eyeball the route and guess.

Wrong. Rendering mode and cache configuration are decided per `fetch()` call, not just per route — a nominally "static" route can still leak per-user data if one of its `fetch()` calls defaults to `force-cache` on a response that embeds session-scoped content. Conversely, a route marked `force-dynamic` "to be safe" pays full per-request render cost even when only one of its five `fetch()` calls actually needs freshness. The review has to operate at both the route level and the individual `fetch()`-call level, and it has to separate two independently-varying axes: **staleness** (is this data fresh enough) and **audience** (is this response safe to share across users).

## Workflow

1. **Confirm the Next.js major version and deployment target**
   - Read `package.json` for the exact Next.js version. Read hosting config (`next.config.js`, deployment docs, CI) for Vercel vs self-hosted.
   - Do not proceed to assert a caching default before this step; Next 14 and Next 15 have different `fetch()` defaults (see Context7 Documentation Protocol in SKILL.md).

2. **Classify each route segment's rendering mode**
   - **Static**: no `dynamic` export or `dynamic = 'auto'`/`'force-static'`, no dynamic APIs (`cookies()`, `headers()`, `searchParams` read at render), and no `fetch()` call configured `no-store` or tagged with per-request freshness.
   - **ISR**: has `export const revalidate = <seconds>`, or at least one `fetch()` using `next: { revalidate: N }`, combined with `generateStaticParams` for dynamic segments.
   - **Dynamic**: has `export const dynamic = 'force-dynamic'`, uses `cookies()`/`headers()`/uncached `searchParams`, or has a `fetch()` using `cache: 'no-store'` that determines the response.
   - Record the exact evidence (file:line) backing each classification. Do not infer mode from route naming or folder structure.

3. **Extract every `fetch()` call's cache configuration**
   - Note `cache: 'force-cache' | 'no-store'`, `next: { revalidate, tags }`, and whether the option is explicit or relying on the version's default.
   - Cross-reference against the confirmed Next.js major (step 1) to know what "no option specified" actually means for that call.

4. **Cross-reference data sensitivity against cache scope**
   - For each `fetch()` call, determine whether the fetched or returned data is per-user/session-scoped: does it forward an `Authorization` header, a session cookie, or a user ID from `cookies()`/`headers()`/route params into the request or the response shape?
   - If yes, and the call uses the default/`force-cache` behavior with no user-scoped cache key or tag differentiating it per user, this is a **leakage candidate** — proceed to the decision tree below.

5. **Check `revalidateTag`/`revalidatePath` scope**
   - Confirm each invalidation call targets exactly the tag/path that changed. A `revalidateTag('posts')` call fired from an endpoint that only mutates one post's title is over-broad if other tags exist per-entity; a call that never fires after the relevant mutation is under-invalidation (stale-data risk).

6. **Produce ranked findings**
   - Order by blast radius: cross-user data leakage first (always HIGH, security sign-off required), then correctness/staleness defects, then avoidable performance cost (unnecessary `force-dynamic`), then lower-severity notes.

## Decision tree

- `fetch()` call includes `Authorization`, forwards a session cookie, or otherwise returns per-user data **AND** uses the version's default/`force-cache` behavior with no `no-store` and no user-scoped tag/key → **HIGH: cross-user leakage risk.** Escalate per SKILL.md's hard security gate — this is a security finding, not a caching-strategy note.
- Route needs freshness within N seconds and currently has no `revalidate`/tag strategy → recommend ISR with the appropriate `revalidate` value or `next: { revalidate: N }` on the relevant `fetch()` calls. Do not default to blanket `force-dynamic`.
- Route is fully static content but marked `force-dynamic` (or has an unnecessary `no-store` fetch) → flag as avoidable TTFB/hosting cost; recommend the minimal-scope fix (fix the one `fetch()` call, not the whole route, unless every call genuinely needs it).
- `revalidateTag`/`revalidatePath` scope is broader than the mutation that triggered it → flag as over-invalidation (cache-hit-rate/cost regression). Scope narrower than the mutation → flag as stale-data risk.

## Output contract

Return:

1. Next.js major version and deployment target confirmed (or explicitly noted as unconfirmed)
2. Per-route rendering-mode table: route | mode | evidence (file:line) | justification
3. Ranked findings, each with:
   - file:line evidence
   - risk class (cross-user leakage / staleness / avoidable dynamic cost / over- or under-invalidation)
   - concrete fix, scoped to the narrowest sufficient change
   - severity (HIGH / MEDIUM / LOW)
   - evidence level (`repo evidence`, `documentation-based`, `inference`)
4. Verdict: approve / approve-with-notes / block
5. Open questions or explicitly out-of-scope items (e.g. Pages Router files encountered, or unconfirmed deployment target)

## Validation gates

- Every caching-default claim states the Next.js major version it was verified against.
- Every cross-user-leakage finding identifies exactly which data field is at risk and why the current cache configuration would serve it to another user — not just "this looks user-specific."
- No finding assumes Vercel-specific cache behavior (e.g. Data Cache persistence across deployments, on-demand ISR via the Vercel API) for a self-hosted deployment without confirming the deployment target first.
- No leakage finding is downgraded from HIGH severity for "code cleanliness" reasons; the security-notes hard gate applies regardless of how minor the fix looks.

## Common failure modes

- Assuming Next 13/14 caching defaults (`fetch()` cached by default) on a Next 15+ codebase, or vice versa — always version-check first.
- Treating all dynamic rendering as a defect, ignoring that some routes genuinely require per-request data.
- Conflating Request Memoization (dedup within one render pass) with the Data Cache (cross-request persistence) — a memoized call is not a caching-leakage risk across users because it does not persist past the single render.
- Recommending route-wide `force-dynamic` as the default fix for a single-`fetch()` leakage finding, incurring unnecessary TTFB cost on the rest of the route.

## Adversarial checklist

Before finalizing a finding, answer these:

- Does any `fetch()` call returning per-user data lack an explicit `no-store` or a user-scoped cache tag/key?
- Is the caching-default claim matched to the actual Next.js major version in `package.json`, not assumed from general familiarity with "Next.js"?
- Does a `revalidateTag`/`revalidatePath` call risk invalidating (or under-invalidating) data unrelated to, or needed by, the change that triggered it?
- Is a route's dynamic classification actually required by real per-request data, or could it be static/ISR with equivalent correctness and lower cost?
- Is the deployment target (Vercel vs self-hosted) confirmed before citing a platform-specific cache behavior?

If any answer is "not sure," lower the finding's confidence and label the evidence level accordingly — do not present it as a confirmed defect, except for leakage findings with clear file:line evidence of per-user data plus default/cached fetch behavior, which stay HIGH regardless.
