---
name: nextjs-rendering-caching-review
description: Statically review Next.js App Router route segments and fetch() calls for rendering-mode (static/ISR/dynamic) and Data-Cache misconfiguration, escalating cross-user data leakage to a security finding rather than a performance nit.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Next.js Rendering & Caching Review

## Purpose

Review Next.js App Router rendering-mode selection (static / ISR / dynamic) and `fetch()` / Data-Cache configuration without re-litigating component architecture, styling, or Pages Router APIs in every response. This skill exists so caching-staleness and cross-user data-leakage risk stay the focus, and so those adjacent concerns stay out of scope.

## When to use

Use this skill when the user asks to:

- review caching/revalidation behavior in an App Router PR,
- investigate a report of stale data being served,
- investigate a report of one user seeing another user's data,
- decide whether a route should be static, ISR, or dynamic.

Do not use this skill for:

- Pages Router codebases (`getStaticProps` / `getServerSideProps`) — different API surface; do not apply App Router `fetch()`-cache guidance to it,
- purely client-side SWR/React Query caching with no server `fetch()` involved,
- component decomposition or state-placement review — that is `react-component-architecture-review`.

## Context7 Documentation Protocol

- Resolve `/vercel/next.js` with `resolve-library-id` before citing any caching-default claim.
- Before asserting a `fetch()` caching default, read the repo's `package.json` to confirm the installed Next.js major version, then call `query-docs` scoped to that version. Next's `fetch()` caching default changed between Next 14 (`cache: 'force-cache'` default) and Next 15 (uncached by default; `GET` Route Handlers also uncached by default). A default claim verified against one major must never be reused for another.
- If the repo has adopted the `use cache` / Cache Components model (Next 15.x canary / Next 16 opt-in), treat that as a distinct caching paradigm from the classic `fetch()`-options model — do not mix `cacheLife`/`cacheTag` guidance with classic `next: { revalidate, tags }` guidance in the same finding without confirming which model the route actually uses.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- First read `package.json` to confirm the installed Next.js major version and whether the deployment target is Vercel or self-hosted. Do not assert a caching default or a platform-specific cache primitive (e.g. Vercel Data Cache persistence across deployments) without confirming both.
- Classify every in-scope route segment as static, ISR, or fully dynamic before evaluating its `fetch()` calls. A route with no `revalidate` export, no `dynamic` export, and no dynamic API (`cookies()`, `headers()`, `searchParams`) usage defaults to static; do not assume dynamic without evidence.
- Treat cross-user Data Cache leakage — a per-user or session-scoped response cached as if it were shared/public — as a HIGH-severity security finding requiring security-review sign-off, not a caching-strategy suggestion. This is the hard security gate for this skill.
- Do not recommend `export const dynamic = 'force-dynamic'` on a whole route to fix a leakage finding without first checking whether a scoped `cache: 'no-store'` on the offending `fetch()` call, or a user-scoped cache tag/key, is sufficient. Route-wide `force-dynamic` is a real TTFB/cost overcorrection.
- Do not conflate Request Memoization (per-render dedup of identical `fetch()` calls, scoped to a single render pass) with the Data Cache (persists across requests/deployments). A finding that treats memoization as if it persisted across users is wrong on its face.
- Do not treat every dynamic route as a defect. Routes that genuinely require per-request data (auth-gated dashboards, personalized content) are correctly dynamic; only flag dynamic classification when the same correctness could be achieved with static or ISR.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Treat any hardcoded API key, token, session secret, or credential found in a `fetch()` call, header, or example data as a HIGH-severity finding requiring immediate escalation, not a caching note.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the rendering-mode classification table, the leakage decision tree, and the required output shape.
- [ISR reference](references/isr-reference.md) — load only for routes using `generateStaticParams` + `revalidate`, or on-demand revalidation via `revalidatePath`/`revalidateTag`.
- [Cache-tag invalidation reference](references/cache-tag-invalidation.md) — load only when tag-based invalidation (`next: { tags }`, `revalidateTag`) is present in the diff.

## Response minimum

Return, at minimum:

- per-route rendering-mode table (route, mode, justification),
- ranked caching findings with file:line, risk class, and fix,
- the Next.js major version the claims were verified against,
- verdict: approve / approve-with-notes / block,
- evidence level and open questions.
