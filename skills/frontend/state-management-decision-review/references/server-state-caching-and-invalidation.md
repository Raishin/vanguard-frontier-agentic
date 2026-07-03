# Server-State Caching and Invalidation

Use this reference only when reviewing query-key design, invalidation triggers, optimistic-update rollback, or SSR query-client instantiation. Grounded against TanStack Query (`/tanstack/query`) — verify version-sensitive defaults via Context7 before asserting them as the reviewed repo's behavior.

## What people get wrong

The naive story is:

> I fetched the data with `useEffect` + `fetch`, stored it in `useState`, and I refetch when I need fresh data.

That is not a caching strategy — it is ad-hoc state with none of the properties a server-state library provides: deduping concurrent requests for the same key, background revalidation, automatic staleness tracking, or a declarative invalidation surface. Every "it shows old data after I saved" bug report traces back to some version of this pattern, because there is no single place that knows "this data just became stale, refetch it."

The second common wrong assumption:

> Once I switch to a query library, staleness is handled automatically — I don't need to think about invalidation.

Also wrong. The query library gives you the *mechanism* (a keyed cache with a staleness policy); it does not know *when your mutation invalidates which keys* unless you tell it.

## Officially grounded shape

Per current TanStack Query docs (verify against installed major via Context7):

- **`staleTime` defaults to `0`** — a query is considered stale immediately after any successful fetch, meaning it will refetch on next mount/window-focus/reconnect per the active refetch settings. A reviewed repo that never sets `staleTime` and complains about "too many refetches" is fighting the documented default, not a bug in the library.
- **`refetchOnWindowFocus` defaults to `true`.** If the repo has disabled it, that is a deliberate trade (frequently correct for internal tools with rapidly-changing data; frequently wrong for expensive data that changes rarely) — verify the choice was made intentionally, not accidentally by copying an example.
- **Query keys are the cache identity.** Two `useQuery` calls with the same key (by deep-equality of the key array) share the same cache entry. This is a feature (dedupes concurrent requests for the same data) but also the most common source of unintended cross-user or cross-context data bleed if a key omits a scoping value (e.g., a user ID, a tenant ID, a locale) that should distinguish two logically different results.
- **The documented optimistic-update rollback pattern** is: `onMutate` cancels in-flight queries for the affected key(s), snapshots the current cache value via `getQueryData`, writes the optimistic value via `setQueryData`, and returns the snapshot as mutation context; `onError` restores the snapshot from that context via `setQueryData`; `onSettled` invalidates the key(s) to reconcile with the server's actual result regardless of success or failure. All three phases are part of the documented pattern — a mutation with an `onMutate` optimistic write but no `onError` restore is an incomplete implementation of the documented pattern, not a valid variant.
- **SSR query-client instantiation** must be per-request. Official guidance explicitly warns against creating the `QueryClient` at module/file root level in a server-rendered app ("this also leaks any sensitive data" — because a module-level client is shared across all requests handled by that server process). The documented pattern instead creates the client inside component state (`useState(() => new QueryClient(...))` for the Pages Router / Remix pattern) or a request-scoped factory function that always constructs fresh on the server and memoizes only in the browser (the Server Components pattern).

## Non-negotiable design rules

### 1. Query keys must be fully scoped to what makes the data distinct

If the underlying data differs per user, per tenant, per locale, or per any other dimension, that dimension must appear in the key array. A key like `['orders']` shared across users in a multi-tenant app is a cache-poisoning bug waiting to happen — user B can be shown user A's cached orders if both hit a component using the same key before either fetch resolves distinctly. Prefer `['orders', { userId, ...filters }]`-shaped keys and verify every dimension that changes the response is present.

### 2. Every mutation must name what it invalidates

A mutation with no `onSettled`/`invalidateQueries` call and no direct cache write (`setQueryData`) is a mutation that silently leaves stale data in the cache. Do not accept "the user will probably navigate away and come back, refetching it" as an invalidation strategy — it is not a strategy, it is an accident that happens to often be tolerable.

### 3. Optimistic updates are a three-part contract, not a two-part one

`onMutate` write without `onError` rollback means: user submits an edit, server rejects it (validation failure, conflict, network error), and the UI continues showing the rejected edit indefinitely with no indication anything went wrong. This is a HARD STOP finding per `SKILL.md` — require the `onError` restore before approving.

### 4. SSR query clients are per-request, never a shared singleton

A `const queryClient = new QueryClient()` at module scope in a server-rendered file is a HARD STOP finding — it is not merely a performance smell, it is a mechanism by which one request's fetched data (which may include user-specific or otherwise sensitive data) becomes visible to a subsequent, different user's request handled by the same server process. Verify the client is constructed inside a request-scoped factory or component state, matching the documented pattern for the framework in use (Pages Router, Remix, or Server Components each have a slightly different documented shape — check which applies).

### 5. `staleTime: 0` is a choice, not automatically a bug

Do not flag a query with the default `staleTime` as broken just because it refetches often. First determine whether frequent revalidation is actually correct for that data (rapidly-changing dashboards, real-time-ish lists) before recommending a longer `staleTime`. Recommending `staleTime` increases without confirming the data's actual change frequency is itself a low-confidence finding — label it `inference` and ask, do not assert.

## Minimal safe implementation flow

1. Classify the entity as server state (see `workflow-and-output.md` decision tree).
2. Design the query key with every scoping dimension the response depends on.
3. Decide the staleness policy deliberately (default `0`, or an explicit value) based on how often the underlying data actually changes and how costly a stale read is.
4. Wire every mutation that affects this entity to an explicit invalidation (`onSettled` + `invalidateQueries`) or a direct cache write.
5. If the mutation is optimistic, implement all three phases: `onMutate` snapshot, `onError` restore, `onSettled` invalidate.
6. If SSR, confirm the client is constructed per-request, not at module scope.

## High-risk assumptions to kill

- "It'll refetch eventually" — not an invalidation strategy.
- "The query key doesn't need the user ID, we only have one API base URL" — the key needs to reflect data identity, not URL structure.
- "Optimistic update without rollback is fine, errors are rare" — the failure mode when it does happen is a UI silently lying to the user about what the server accepted.
- "One shared QueryClient instance is simpler" — in SSR, simpler and correct are not the same thing here; module-level instantiation is a documented anti-pattern, not a style preference.
- "Increasing staleTime will fix the re-render complaint" — staleTime affects refetch/staleness, not re-render count; a re-render complaint is a selector/subscription problem (see `client-store-topology-and-rerenders.md`), not a caching-policy problem. Do not conflate the two.

## Safe verification targets

- Grep for `new QueryClient(` and check whether each call site is inside a component/request-scoped function or at module scope.
- Grep for `useMutation(` and check whether `onMutate` call sites have a matching `onError` that calls `setQueryData` with the snapshot, and an `onSettled` that invalidates.
- Inspect each `queryKey` array literal for missing scoping dimensions (compare against the fields the associated `queryFn` actually uses to build its request).

## When to push back

Push back if the user asks for:

- a single shared `QueryClient` across all SSR requests "for performance,"
- an optimistic update shipped without a rollback path "we'll add it later,"
- a query key without user/tenant scoping "because we'll add multi-tenancy later" — retrofitting scoped keys after data has already leaked across contexts is materially harder than designing them in from the start.

That is not "faster." It is a data-integrity and, in the SSR-singleton case, a data-leak risk.
