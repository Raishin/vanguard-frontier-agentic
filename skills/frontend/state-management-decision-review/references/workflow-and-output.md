# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure, the classification decision tree, and the required output shape. Load it for every invocation of this skill; the other two references are conditional on what the classification table surfaces.

## What people get wrong

The common bad assumption is:

> "State management" is one decision — pick a library and put things in it.

That is wrong. It is at least three separate decisions:

1. **classification** — is this entity server-owned, client-owned, or derived?
2. **caching policy** — for server-owned entities, what is the cache key, the staleness window, and the invalidation trigger?
3. **topology** — for client-owned entities, how is the store sliced, and what does each consuming component actually subscribe to?

Skipping straight to "which library" without doing step 1 first is how server data ends up duplicated into a client store, or how a single monolithic store becomes the reason every keystroke re-renders unrelated components.

## Step-by-step workflow

1. **Enumerate entities.** List every distinct piece of state touched by the change: each API-fetched resource, each form field, each UI toggle, each computed value. Do not group unrelated entities together — "the todo list" and "the new-todo draft text" are different entities with different lifecycles even though they appear in the same feature.
2. **Classify each entity** using the decision tree below. Do not skip an entity because it "obviously" belongs somewhere — the point of this skill is that the obvious classification is frequently wrong (see "What people get wrong" above).
3. **For every server-state entity**, verify:
   - an explicit query key / cache key exists and is scoped correctly (see `server-state-caching-and-invalidation.md` for scoping detail),
   - an explicit invalidation trigger exists (a mutation's `onSettled`, a `queryClient.invalidateQueries` call, a documented time-based `staleTime`/polling policy — not "it'll refresh on remount sometimes"),
   - if the entity is written optimistically, a rollback path exists (`onMutate` snapshot + `onError` restore).
4. **For every client-state entity**, verify the store slice and selector shape (see `client-store-topology-and-rerenders.md`).
5. **For every derived-state entity**, verify it is computed on read (selector, `useMemo`, or plain derivation) rather than independently stored and manually kept in sync.
6. **For SSR applications**, verify the query client and any global store are instantiated per-request, not as a module-level singleton.
7. **If diagnosing a reported bug or performance complaint**, require concrete evidence before accepting a root cause:
   - a "stale data" report needs the reproduction sequence (what mutated, what stayed stale, what would have refetched it and didn't),
   - a "re-render cascade" or "typing lags" report needs profiler evidence (render count or flame-graph excerpt) before any memoization/selector fix is accepted as verified rather than speculative.
8. **Issue a verdict** with hard stops separated from lower-severity notes, per the output contract below.

## Classification decision tree

```
Is there a remote/server source of truth for this value?
├── YES → is it purely computed from other server/client values with no independent write path?
│   ├── YES → SERVER-DERIVED — must not be independently stored; compute on read
│   └── NO  → SERVER STATE — must use a cache/query mechanism with an explicit key + invalidation trigger
└── NO → is it computed entirely from other state already tracked elsewhere (client or server)?
    ├── YES → CLIENT-DERIVED — must not be independently stored; compute on read (selector/useMemo)
    └── NO  → CLIENT STATE — session-local; scope it to the narrowest store/slice that needs it
```

Entities that fail this tree entirely (state with no source, server or computed — e.g., a value the UI invents and never persists or fetches) are a design smell worth naming explicitly, not silently placing in either bucket.

## Findings contract (response minimum, restated with detail)

Every review response from this skill must contain:

1. **Entity classification table** — columns: entity name, classification (server / client / derived), source of truth, current implementation location (file/hook/store). This table is not optional even for a "quick" review; it is the artifact that makes the rest of the review traceable.
2. **Per server-state entity**: cache key shape, invalidation trigger, rollback path (or "N/A — read-only" / "MISSING — hard stop").
3. **Per client-store slice touched**: selector shape used by each consumer, and whether any consumer subscribes to more of the store than it reads (a re-render risk).
4. **Root-cause statement** (bug-diagnosis reviews only) — one of: stale-cache (invalidation trigger missing/wrong), race-condition (out-of-order async writes), re-render-cascade (over-broad subscription/selector), normalization/duplication bug (same fact stored in two places that can diverge). Do not return "state management issue" as a root cause; it is not falsifiable and gives the requester nothing to fix.
5. **Evidence level** per finding: `repo evidence` (you read the actual code), `documentation-based` (you are asserting a library default/behavior grounded in Context7 or official docs), or `inference` (you are reasoning from incomplete information and it should be verified).
6. **Verdict**: approve / approve-with-notes / block. HARD STOPS (missing optimistic-update rollback, SSR module-level singleton, PII/auth-token persisted without justification) always force block, listed separately from lower-severity notes.
7. **Open questions** — anything the review could not resolve from static evidence alone (e.g., "confirm with a profiler trace before merging the memoization fix").

## Version note

TanStack Query and Zustand APIs and defaults are version-sensitive (e.g., `useShallow` import paths have moved across major versions; SSR guidance differs between the Pages Router, Remix, and Server Component patterns). Verify exact API shape and defaults against the repo's installed version and current official docs before asserting a claim as settled fact — see the Context7 Documentation Protocol in `SKILL.md`.
