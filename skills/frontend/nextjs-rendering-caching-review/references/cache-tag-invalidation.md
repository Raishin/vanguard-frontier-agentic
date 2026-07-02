# Cache-tag invalidation reference (next: { tags }, revalidateTag)

Use this reference only when the diff includes `next: { tags: [...] }` on a `fetch()` call, or a `revalidateTag`/`revalidatePath` call. Do not load it for routes with no tag-based invalidation.

## What people get wrong

The naive story is:

> Tag it, call `revalidateTag` on mutation, done — invalidation is invalidation.

Wrong. Tag scope is a design decision with the same over-/under-invalidation failure modes as cache-key design anywhere else: too coarse a tag invalidates far more cached data than the mutation actually changed (cache-hit-rate and cost regression across unrelated routes); too fine or simply mismatched a tag misses the mutation entirely (stale-data bug that looks like "the cache is broken" but is actually "the tag was never invalidated"). Official Next.js guidance itself states a preference for tag-based revalidation over path-based (`revalidatePath`) specifically because it is more precise — but precision only helps if the tag design matches the actual data-ownership boundaries.

## Officially grounded shape

- `fetch(url, { next: { tags: ['posts'] } })` associates that cached entry with the tag `'posts'`. Multiple unrelated `fetch()` calls can share a tag; a single `fetch()` call can carry multiple tags.
- `revalidateTag(tag)` (imported from `next/cache`, called from a Server Action or Route Handler) invalidates **every** cached entry carrying that tag, forcing a re-fetch on the next request to any route that reads it — not just the route that triggered the mutation.
- `revalidatePath(path)` invalidates all cached data for a specific route path, without needing to know which tags are attached to it. Official guidance recommends preferring tag-based revalidation over path-based when the tag structure is known, precisely because path-based invalidation is coarser and easier to over-invalidate with.
- Route Handlers are a common on-demand-revalidation entry point (e.g., a webhook receiver calling `revalidateTag` in response to an external CMS publish event) — verify any such handler is itself access-controlled (secret/token check) so an unauthenticated caller cannot trigger arbitrary cache invalidation (a cost/DoS-adjacent concern, not just a staleness one).

## Non-negotiable design rules

1. **Tag granularity should match data-ownership boundaries, not convenience.** A single `'posts'` tag shared by every post's detail page means updating one post's title invalidates every post's cached page. If the mutation is scoped to one entity, prefer a per-entity tag (`post-${id}`) plus a broader list-level tag (`posts`) only where the list view genuinely needs to reflect the change too — and invalidate both explicitly when both are affected.

2. **Every mutation that changes tagged data must call the matching invalidation, and only that data's tag.** Trace each `revalidateTag`/`revalidatePath` call back to the mutation that triggers it; confirm the tag argument matches a tag actually used by a `fetch()` in the affected route, not a stale or copy-pasted tag name from an unrelated feature.

3. **On-demand revalidation entry points (webhook/API routes) must be access-controlled.** An unauthenticated `revalidateTag`/`revalidatePath` endpoint lets any caller force full cache invalidation on demand — flag as a MEDIUM-or-higher availability/cost finding if no secret/token/signature check gates the handler.

4. **Do not use tag-based invalidation as a substitute for per-user cache scoping.** Tags invalidate shared cache entries; they do not make a shared cache entry safe to serve to multiple users. If the underlying leakage concern from SKILL.md's hard security gate applies (per-user data cached without a `no-store`/scoped key), adding a tag does not resolve it — the fetch still needs `cache: 'no-store'` or a genuinely user-scoped cache key, independent of tagging.

## Minimal safe verification flow

1. List every tag used across in-scope `fetch()` calls and every `revalidateTag`/`revalidatePath` call in the diff.
2. For each mutation, confirm it invalidates exactly the tags that cover data it changed — no more, no less.
3. For each on-demand-revalidation Route Handler, confirm it is access-controlled.
4. Confirm no leakage concern is being "fixed" with tagging alone when it actually requires `no-store` or per-user cache-key scoping.

## When to push back

Push back if the user asks for:

- one broad tag applied to "everything on this page" to avoid designing per-entity tags — that guarantees over-invalidation as the app grows,
- an on-demand revalidation Route Handler with no auth check "since it's just a cache refresh" — uncontrolled invalidation is a cost and availability surface, not a harmless refresh,
- using `revalidateTag` as the fix for a reported "user A sees user B's data" bug — that is a caching-scope/leakage bug, not a staleness bug, and tagging does not address it.

Those are not shortcuts. They convert a precise invalidation model into either a blunt one or a papered-over security defect.
