# Fetch Waterfall and Auth-Timing Review

Use this reference when diagnosing a sequential-fetch performance issue, or when reviewing whether streaming exposes page structure or content before an authorization check has resolved.

## What people get wrong

Two separate bad assumptions show up together often enough to warrant one reference:

> "These fetches run one after another because that's just how `await` works" — sometimes true, sometimes a waterfall defect.

> "Streaming just sends HTML progressively, auth isn't a streaming concern" — wrong when the streamed content itself is the thing that needed authorization.

Both require actually tracing the await order and the request lifecycle, not pattern-matching on the presence of `await` or `<Suspense>`.

## Waterfall diagnosis: sequential vs. genuinely dependent

Not every sequential `await` chain is a defect. Next.js's own data-fetching guidance shows a legitimately sequential case: fetching an artist's playlists requires the artist's ID, which only exists after the artist fetch resolves. That is a genuine data dependency — the second fetch cannot start before the first resolves, and flagging it as a waterfall defect is a false positive.

The defect pattern is different: two (or more) fetches that do **not** depend on each other's output, but are still written with sequential `await` calls, so the second fetch doesn't start until the first finishes — costing the sum of both latencies instead of the max.

### Diagnostic procedure

1. For each pair of fetches in the flagged code path, determine: does fetch B use any value produced by fetch A (an ID, a token, a computed parameter)? If yes, sequential `await` is correct — do not flag it.
2. If fetch B does not depend on fetch A's output but is still written after `await fetchA()`, this is a real waterfall. Recommend starting both concurrently — begin the fetch (don't await it immediately) and consume both results together, or use per-section Suspense boundaries with each section starting its own fetch independently so they resolve in parallel rather than being serialized by a shared parent awaiting both in sequence.
3. Confirm the recommended restructuring doesn't accidentally introduce a request that now fires on every render when it previously ran once — verify fetch caching/memoization semantics for the confirmed framework version via Context7 before asserting the parallelized version is still correct.

### Fix framing

Do not just say "parallelize this." State which specific fetches are independent (by name/file:line), what the current serialized cost is (sum of both fetches' latency), and what the corrected cost would be (max of both). A vague "these could probably be parallel" is not a completed diagnosis.

## Auth-timing and premature streaming

Streaming sends the response progressively as chunks resolve, which means part of the response can reach the client (and be rendered, or at minimum be visible in dev tools / network trace) before the entire request lifecycle — including any authorization check — has completed, if the authorization check is not structured to gate the stream's start.

### The specific defect pattern

A route or component streams protected content (page structure that reveals a resource exists, partial data, or a shell that implies the current user has access) **before** the code path that verifies the requester is authorized to see that content has resolved and short-circuited on failure. This differs from a plain performance concern: the risk is that an unauthorized request can observe something about a protected resource — its existence, its shape, or fragments of its data — purely from what streamed before the authorization check would have blocked it.

### Review procedure

1. Identify the authorization check for the route/resource in scope (session/role/ownership verification).
2. Trace what streams (renders, sends data, or resolves a Suspense boundary) before that authorization check has run and been evaluated.
3. If anything protected-resource-specific streams before the authorization check resolves and can fail, treat this as a potential information-disclosure finding and escalate — do not file it as a performance observation.
4. Distinguish this from the unprotected-shell case: a generic loading skeleton with no resource-specific information (no IDs, no titles, no counts) streaming before auth resolves is not a disclosure risk by itself. The defect is specifically protected-resource-specific content or metadata reaching the client stream ahead of the authorization gate.
5. Note the bot/crawler exception when relevant: Next.js does not stream progressively to bots/crawlers — it waits for full data resolution first — so a premature-streaming disclosure concern for a browser request does not automatically apply the same way to a bot-facing response, and conflating the two produces an inaccurate threat model.

### Fix framing

The correct fix is almost always structural: perform the authorization check synchronously before starting the response (or before the first byte that carries resource-specific content is flushed), not "add a loading spinner" or "move the check earlier in the component tree" without confirming it actually runs before the first protected content leaves the server.

## Adversarial checklist

- For each flagged sequential-fetch pair: is there an actual data dependency, or is this a real waterfall?
- For the proposed parallelization: does it change caching/memoization semantics for the confirmed framework version?
- Does any protected-resource-specific content (not a generic skeleton) stream before the authorization check has resolved and could fail?
- Is the auth-timing concern being applied correctly to the browser-facing path, given that bot/crawler responses do not stream progressively in the first place?

## When to push back

Push back if the user asks to:

- "just parallelize everything" without tracing which fetches are actually independent,
- move an authorization check later in the render tree "to make the page feel faster," when that change would let protected content start streaming before the check resolves,
- treat a generic loading skeleton with zero resource-specific data as an auth-timing risk — that is a false positive that dilutes real findings.
