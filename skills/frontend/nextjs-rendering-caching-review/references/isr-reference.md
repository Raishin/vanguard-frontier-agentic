# ISR reference (generateStaticParams + revalidate)

Use this reference only for App Router routes using `generateStaticParams` combined with a time-based `revalidate` export or `next: { revalidate }` fetch option. Do not apply this to Pages Router `getStaticProps`/`getStaticPaths` — that is a different, legacy API surface; if you encounter it, note it as out-of-scope for this skill rather than translating App Router guidance onto it.

## What people get wrong

The naive story is:

> `revalidate: 60` means the page is rebuilt every 60 seconds, so pick a number and move on.

Wrong. `export const revalidate = 60` (or `next: { revalidate: 60 }` on the underlying `fetch()`) means: Next.js will attempt to re-generate the page in the background **at most once every 60 seconds, and only when a request comes in** after that window — it is not a background cron. Until regeneration completes, the **stale** cached version continues to be served (stale-while-revalidate), not a loading state and not a 404. If regeneration throws an uncaught error, Next.js does **not** invalidate the currently-shown page; it keeps serving the last successfully generated version and retries on the next request. This error-handling behavior matters for the review: a `revalidate`-backed route that silently swallows fetch errors can serve indefinitely-stale data without ever surfacing a failure, because the fallback is "keep serving the old page," not "fail loudly."

## Officially grounded shape

- `generateStaticParams` defines which dynamic-segment values (`params`) get prerendered at build time; segments not returned by it are generated on first request (or 404, depending on `dynamicParams` config) and then cached per the route's `revalidate` value.
- `export const revalidate = N` sets the route-level regeneration interval in seconds. A `fetch()` call inside the route can independently set its own `next: { revalidate: N }` — the **effective** revalidation window for the route is the **minimum** of the route-level export and any fetch-level values that feed the rendered output. Do not evaluate only the route-level export and ignore a shorter fetch-level value (or vice versa).
- On-demand revalidation (`revalidatePath`, `revalidateTag`) supersedes the time-based window — it forces the next request to regenerate regardless of how much of the interval has elapsed. A route using only on-demand revalidation with no `revalidate` export is valid (fully static until an explicit invalidation call) — do not flag the absence of a time-based `revalidate` as a defect when on-demand invalidation is present and wired to the actual mutation path.

## Non-negotiable design rules

1. **Trace the effective revalidation window to the shortest contributing value.** If the route export says `revalidate = 3600` but one `fetch()` inside it uses `next: { revalidate: 60 }`, the route effectively refreshes at the 60-second cadence for that data. Flag a review that only cites the route-level export as incomplete.

2. **Confirm error handling does not silently extend staleness beyond intent.** If `getData()` swallows a non-2xx response (e.g. returns a default/cached object instead of throwing), a backend outage will not trigger the "keep serving last good page and retry" fallback correctly — verify the fetch call surfaces failures (throws or returns a non-ok response Next.js can detect) rather than masking them.

3. **Verify `generateStaticParams` coverage matches the actual param space, or that `dynamicParams` is deliberately configured.** Undercovering high-traffic params with `dynamicParams: false` produces 404s for legitimate paths; overcovering a huge param space with build-time generation can blow up build time — this is a build-cost finding, not correctness, but should still be flagged as MEDIUM if extreme.

4. **Do not apply ISR guidance to per-user data.** ISR caches are shared across all requests to that path/param combination. If the page under `generateStaticParams` renders user-specific content (e.g., `/dashboard/[userId]` returning that user's private data) and relies on route-level `revalidate` without a `no-store`/dynamic escape hatch for the personalized parts, this is the cross-user leakage pattern from SKILL.md's hard security gate — escalate to HIGH, not a normal ISR tuning note.

## Minimal safe verification flow

1. Confirm the Next.js major version (ISR semantics referenced here are for App Router, current stable behavior; verify no `use cache`/Cache Components migration has changed the model — see SKILL.md's Context7 protocol).
2. Locate every `revalidate` export and every `next: { revalidate }` fetch option in the route; compute the effective window.
3. Confirm `generateStaticParams` scope is intentional (full enumeration vs. partial + `dynamicParams`).
4. Confirm fetch error handling doesn't mask backend failures.
5. Confirm no per-user data flows through a route relying on shared ISR caching without a scoped escape hatch.

## When to push back

Push back if the user asks for:

- a single global `revalidate` value applied uniformly "to keep it simple" across routes with very different freshness requirements — that produces either unnecessary staleness or unnecessary regeneration load,
- ISR applied to a route serving authenticated, per-user data with no scoped `no-store`/dynamic fetch for the personalized parts,
- removing error handling from the data-fetch function "to simplify the code" — that removes the safety net that keeps a broken backend from ever surfacing as a build/render failure.

Those are not simplifications. They trade away either freshness guarantees or the leak/error-visibility safety net ISR depends on.
