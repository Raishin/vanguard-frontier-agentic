# Official sources

Use this reference only when a routing decision needs source grounding for React/Next.js domain vocabulary, or when confirming a catalog agent name.

## Sources

Use these as starting points, not as proof of the user's live frontend deployment or repository state:
- https://react.dev/learn — React fundamentals, hooks, Server Components, error boundaries
- https://nextjs.org/docs — Next.js App Router rendering, caching, streaming
- https://www.w3.org/WAI/WCAG22/quickref/ — WCAG 2.2 success criteria reference (for confirming the `accessibility` domain boundary against `accessibility-wcag-agent` vs `html-semantics-agent`)

## Grounding rule

Official documentation explains framework and specification behavior. It does not prove the user's current repository state, installed dependency versions, build configuration, deployed environment, or production incident cause. Prefer repo evidence (actual source, `package.json`, lockfiles, config files) or sanitized user-provided evidence for current-state claims. Maestro's own Context7 use is limited to routing-vocabulary grounding (see `SKILL.md`'s Context7 Documentation Protocol) — never to producing the specialist's answer itself.

## Current MCP/documentation refresh (2026-07-02)

Framework facts sampled via Context7 that inform routing-domain boundaries:

- React: Server Components cannot use most Hooks — they do not persist in memory after render and cannot hold their own state (`useState`, `useMemo`, etc. are Client Component-only). This is why a "Server Component uses `useState`" signal routes to `react-specialist-agent` or `nextjs-specialist-agent` as a defect report, not as a valid pattern to document.
- React: rendering errors are caught via an Error Boundary (a class component implementing the error-boundary lifecycle, or `<ErrorBoundary>` wrapping a component that calls `use`) — relevant when disambiguating an `ssr-hydration` "error boundary placement" signal from a general `react` component-architecture signal.
- Next.js App Router: `fetch()` caching is explicit per call (`cache: 'force-cache'` default/static, `cache: 'no-store'` dynamic per request, `next: { revalidate: N }` time-based); a `'use cache'` directive also exists for cache-scoped functions/components with `cacheLife`/`cacheTag`. A task naming any of these cache mechanics routes to `nextjs-specialist-agent`, not to `build-tooling-bundling-agent` (which owns bundler-level caching, a distinct concern).
- Next.js App Router: the initial HTML stream renders static content and Suspense fallbacks first, then streams completed Server Component output with inline scripts for hydration as data resolves — this is the mechanic behind `ssr-hydration-streaming-agent`'s domain (hydration mismatch, Suspense boundary placement, TTFB/LCP impact), distinct from `web-performance-core-vitals-agent`'s domain (Core Web Vitals budget/field-data triage once the page is already interactive).

Review implications:

- When a task's signal is ambiguous between a framework specialist and `ssr-hydration-streaming-agent`, prefer the framework specialist for component-authoring concerns (hooks, cache config, template syntax) and `ssr-hydration-streaming-agent` for hydration-mismatch/streaming-boundary/timing concerns specifically.
- Do not let a routing decision assert framework behavior from memory when Context7 is available and the task's classification depends on it; mark the routing basis `documentation-based` when Context7 confirms current docs, or `inference` when neither is available.
