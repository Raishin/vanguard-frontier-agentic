---
name: state-management-decision-review
description: Reviews whether data is correctly classified as server state, client state, or derived state, and whether the resulting store/cache design (query keys, invalidation, optimistic-update rollback, SSR instantiation, selector shape) avoids duplication, stale-data bugs, and unnecessary re-render cascades.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# State Management Decision Review

## Purpose

Review a proposed or existing state-management design without re-litigating routing/URL-state ownership, API contract shape, or SSR hydration-mismatch diagnosis in every response. Most state-management bugs trace back to a single category error: treating server-owned data (fetched from an API, subject to staleness, shared across users or tabs) as if it were client-owned data (form input, UI toggles, ephemeral interaction state). Once that error is made, every downstream decision — where to put the data, when to refetch it, how to invalidate it, whether an update needs a rollback path — compounds it. This skill exists to catch that root-cause error early and to evaluate the caching/invalidation strategy and store topology against two measurable failure modes: stale/duplicated data and unnecessary re-render cascades.

## When to use

Use this skill when the user asks to:

- review a PR that introduces new fetching, caching, or store logic,
- diagnose a "stale data after save" or "the list didn't update after I created/deleted an item" bug report,
- diagnose a "page freezes while typing" or "every keystroke re-renders half the tree" performance complaint tied to a shared store,
- evaluate a proposal to introduce a new state-management library or a new global store,
- audit an existing store for entities that duplicate or conflate server-fetched data with client-only data.

Do not use this skill for:

- routing/URL-state ownership review (filters, pagination, tabs that should be reconstructable from the URL) — that is `routing-navigation-review`,
- BFF/API contract shape, request/response schema, or endpoint design review — that is `api-integration-contract-review`,
- SSR hydration mismatch diagnosis unrelated to store/query-client serialization (server/client markup divergence) — that is `ssr-hydration-streaming-diagnosis`. This skill only covers SSR store/queryClient *instantiation* (per-request vs. shared singleton), not hydration mismatch mechanics.

## Context7 Documentation Protocol

- Resolve `/tanstack/query` with `resolve-library-id` before asserting any caching default (`staleTime`, `gcTime`, `refetchOnWindowFocus`) — these are documented defaults, not universal truths, and the repo's installed major version governs behavior. `staleTime` defaults to `0` (query is considered stale immediately after any successful fetch) and `refetchOnWindowFocus` defaults to `true`; do not assume the reviewed repo has left these at default without checking the `QueryClient` construction site.
- Before flagging an optimistic-update pattern as missing rollback, call `query-docs` on `/tanstack/query` for "optimistic updates" to confirm the current documented shape (`onMutate` cancels in-flight queries, snapshots prior data, returns it as mutation context; `onError` restores the snapshot from that context; `onSettled` invalidates). Do not invent an alternate rollback API.
- Resolve `/pmndrs/zustand` with `resolve-library-id` before recommending a selector-based re-render fix. Confirm current `useShallow` import path and behavior via `query-docs` before telling a user to add it — the import path (`zustand/react/shallow` vs `zustand/react`) and default `Object.is` comparator behavior are version-sensitive.
- Before approving a `persist` middleware usage that touches auth/session data, call `query-docs` on `/pmndrs/zustand` for "persist middleware" to confirm current `partialize` and `storage` options exist and are the documented way to exclude sensitive fields — do not assume a field-exclusion API without checking it.
- If Context7 is unavailable for either library, fall back to the `official_docs` URLs in this skill's `metadata.json` and label every caching-default or API-shape claim `documentation-based, verify against installed version` rather than stating it as settled fact.
- Read `package.json` first to confirm which server-state library (if any) is actually installed and its major version. Do not recommend a fix keyed to an API that the installed major version does not have.

## Lean operating rules

- Build the entity classification table before evaluating anything else. Every piece of state in scope must land in exactly one bucket: **server state** (fetched from an API/DB, has a remote source of truth, can go stale, is potentially shared across users or tabs), **client state** (exists only in this session — form inputs before submit, modal open/closed, hover/focus, drag position), or **derived state** (computed from server and/or client state — never independently stored). Do not evaluate caching strategy before this table exists; the table is the review's foundation, not an afterthought.
- Any entity classified as server state that is held in `useState`/`useReducer`/a plain client store instead of a query/cache library is a category-error finding, not a style note — it is the root cause of most manual-refetch and stale-cache bugs the skill exists to catch.
- Any entity classified as derived state that is independently stored (rather than computed on read, in a selector, or in a memoized derivation) is a duplication finding — it can drift from its inputs and is a second, harder-to-find source of staleness.
- For every server-state entity, require an explicit, inspectable cache key and an explicit invalidation trigger (what mutation, what event, or what time-based policy causes a refetch). "It'll refetch eventually" without a named trigger is not an answer.
- For every optimistic update (a mutation that updates the UI before the server confirms), require a paired rollback path (`onError` restoring a snapshot taken in `onMutate`) per the current documented pattern. An optimistic update with no rollback path is a HARD STOP, not a note — it means a failed mutation leaves the UI showing state the server never accepted, with no correction mechanism.
- For SSR applications, require that the query client and any global store be instantiated per-request (inside component state / a request-scoped factory), never as a module-level singleton created once at import time. A module-level singleton in SSR is a HARD STOP — it is a cross-request/cross-user data-leak vector, not merely a performance concern.
- Do not accept a re-render-cascade "fix" (memoization, selector narrowing, splitting a store) without profiler evidence (a before/after render count or a flame-graph excerpt). A fix justified only by "this should reduce re-renders" is speculation, not a verified finding — require the evidence or explicitly flag it as unverified.
- Do not recommend introducing a new global store as the default fix for a data-caching problem that a server-state library already solves (deduping, background refetch, invalidation). Naming a new store as the fix for a caching bug is itself a finding to push back on.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only). Profiler evidence must come from the user/PR description, not from live reproduction performed by this skill.
- Treat any store or persisted-storage design that writes auth tokens, session identifiers, or PII to `localStorage`/`sessionStorage` (directly or via a `persist` middleware without a `partialize` exclusion) as a security-relevant finding requiring explicit encryption/expiry/justification — do not wave it through as a caching-pattern detail.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the classification/decision tree, and the required output shape.
- [Server-state caching and invalidation](references/server-state-caching-and-invalidation.md) — load only when reviewing query-key design, invalidation triggers, optimistic-update rollback, or SSR query-client instantiation.
- [Client-store topology and re-renders](references/client-store-topology-and-rerenders.md) — load only when reviewing store-slice design, selector shape, `useShallow` usage, or a reported re-render-cascade / typing-jank complaint.

## Response minimum

Return, at minimum:

- the entity classification table (server / client / derived) for every entity in scope,
- for each server-state entity: its cache key, its invalidation trigger, and (if applicable) its optimistic-update rollback path,
- for each client-store slice reviewed: its selector shape and whether it is exposed to unnecessary re-render risk,
- for bug diagnosis: a root-cause statement distinguishing stale-cache vs. race-condition vs. re-render-cascade vs. normalization/duplication bug — not a vague "state management issue",
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block), with HARD STOPS (missing rollback, SSR singleton) called out separately from lower-severity notes,
- open questions or scope the review could not cover (e.g., "re-render claim requires profiler evidence to confirm").
