# SSR/ISR Invocation Cost Modeling

Use this reference when the primary cost driver being modeled is server-side rendering, incremental static regeneration (ISR), or edge-function invocation — i.e., compute that runs per-request or per-revalidation rather than being served as a static, fully-cached asset.

## What people get wrong

The common bad assumption is:

> "We use SSR/ISR, so it's basically static — the cache handles it."

That is incomplete, and it is the single most common cause of frontend cloud-cost surprises. Caching mode is not binary; it determines *how often* compute runs, and that invocation rate is the entire cost driver. Two surfaces that both say "we use ISR" can have 100x different compute bills depending on revalidation cadence, tag-invalidation frequency, and whether requests bypass cache due to query-string or header variance.

## Officially grounded shape (Context7-verified, Next.js docs)

Per current Next.js documentation (`/vercel/next.js`):

- **Time-based revalidation** (`fetch(url, { next: { revalidate: N } })`) uses a stale-while-revalidate pattern: cached content is served immediately, and regeneration happens in the background once the content's age exceeds `N` seconds. This bounds worst-case regeneration frequency to roughly `traffic-during-window / N`, not per-request.
- **On-demand revalidation** (`revalidateTag()`, `revalidatePath()`) explicitly invalidates cached content from a server action or route handler, causing the *next* request after invalidation to trigger a fresh render. This decouples regeneration from a timer entirely — invocation count is driven by how often the invalidation trigger fires (e.g., a CMS webhook), not by a fixed interval.
- **Image optimization** (legacy `next/image` pipeline): images are optimized dynamically on first request and cached in `<distDir>/cache/images`; on expiration, a stale image is served immediately while regeneration happens in the background and the new result is cached. This means image-transform compute cost is driven by the *cardinality of distinct requested variants* (width/quality/format combinations), not by pageview count — a large `deviceSizes`/`imageSizes` matrix with the same source image multiplies cache-miss compute even for one logical image.

> Version note: caching defaults have changed across Next.js major versions (`fetch` caching behavior, `dynamicIO`/cache-components experiments). Verify default caching behavior against the installed version via Context7 or official docs before assuming a specific revalidation model — do not assume App Router defaults carry over from Pages Router or from an older major version.

## Non-negotiable design rules

### 1. Classify the invocation shape before pricing anything

Do not price "SSR" as a single line item. Classify each route/surface into one of:

- **Static / fully cached** — served from CDN edge cache, effectively zero marginal compute per request.
- **Time-based ISR** — bounded regeneration rate; cost scales with `unique-pages / revalidate-interval`, not with traffic.
- **On-demand revalidated** — regeneration rate scales with invalidation-trigger frequency (e.g., CMS publish events), which can spike independently of traffic.
- **Per-request SSR (no cache)** — regeneration rate equals request rate; this is the only shape where cost is strictly traffic-linear in compute, and the most expensive per-pageview.

Mixing these into one blended "SSR cost" figure hides which routes are the actual problem.

### 2. Treat cache-key fragmentation as a cost multiplier

Query parameters, cookies, or headers included in the cache key (e.g., per-locale, per-experiment-variant, per-logged-in-state rendering) multiply the effective number of "unique pages" that each need independent regeneration. A page with 3 locales x 4 A/B variants x authenticated/anonymous is 24 cache entries, not 1 — model compute cost against that multiplied cardinality, not the logical route count.

### 3. Distinguish linear from non-linear cost growth explicitly

- **Linear**: cost per pageview is roughly constant as traffic grows (static assets, well-bounded time-based ISR, edge-cached SSR responses).
- **Non-linear**: cost grows faster than traffic (per-request SSR with no cache, unbounded image-variant cardinality, cache-key fragmentation that scales with user/session count, retry storms from a slow origin).

Flag any non-linear pattern as an architectural risk even if the current bill is small — non-linear cost curves are the ones that blow budgets during a traffic spike or a viral moment, precisely when the business can least afford a surprise, and precisely when engineering has the least slack to fix it under pressure.

### 4. Do not price edge/serverless invocations from memory

Provider pricing for edge-function invocations, SSR compute duration, and image-transform requests changes across pricing tiers and providers, and differs materially between "included in platform plan" vs. "metered pay-per-use" billing. Ground any specific per-invocation or per-GB-second rate in the provider's current published pricing page or a user-supplied billing export — do not assert a specific dollar rate from training-data memory, since these rates are revised frequently and vary by committed-use tier.

## Minimal safe modeling flow

1. Classify each route/surface by invocation shape (static / time-based ISR / on-demand / per-request SSR).
2. For each non-static shape, identify the cache-key dimensions (locale, auth state, experiment variant, query params) and compute effective cardinality.
3. Get actual or estimated traffic volume per surface (analytics export, CDN log sample, or user-stated estimate — label whichever it is).
4. Compute expected invocation count: time-based ISR ≈ `cardinality / revalidate_interval_seconds × window_seconds`; on-demand ≈ trigger frequency; per-request SSR ≈ request volume.
5. Apply current provider pricing (billing export preferred; public rate card as fallback, labeled `modeled-from-public-pricing`) to invocation count and compute duration/memory profile.
6. Flag any surface where invocation count scales faster than traffic as non-linear risk, independent of the resulting dollar figure.

## Adversarial checklist

Before presenting an SSR/ISR/edge cost figure as reliable, answer these:

- Did I classify by actual invocation shape, or did I average everything into one "SSR" bucket?
- Does the cache key include anything that fragments cardinality (locale, auth, experiment, query string)?
- Is the revalidation interval time-based, on-demand, or both layered together?
- Did I verify current caching-default behavior against the installed framework version via Context7/official docs, or am I assuming defaults from memory?
- Is the pricing rate billing-data-verified, or modeled from a public rate card that may not reflect the account's actual committed-use discount?
- Have I flagged any non-linear growth pattern regardless of whether the current dollar total looks small?

If any answer is "I don't know," say so explicitly rather than presenting a confident number.

## When to push back

Push back if the user asks to:

- disable a cache layer, revalidation window, or CDN tier purely to "make the number smaller" without checking Core Web Vitals impact,
- assume a flat per-request cost figure without classifying invocation shape first,
- accept a public price-sheet estimate as final without flagging that real committed-use pricing may differ substantially,
- treat a non-linear cost-growth pattern as acceptable because "the bill is still small today."

Those shortcuts produce a number that looks precise and is not, and they defer an architectural risk instead of surfacing it.

