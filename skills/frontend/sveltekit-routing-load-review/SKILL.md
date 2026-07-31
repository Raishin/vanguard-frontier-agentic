---
name: sveltekit-routing-load-review
description: Statically review SvelteKit route files (+page.js, +page.server.js, +layout.js, +layout.server.js, +server.ts) to verify universal-vs-server load placement, catching server-only secrets, database clients, or privileged API access that would leak into or execute inside the browser.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# SvelteKit Routing & Load Function Review

## Purpose

Review SvelteKit route files for correct universal-vs-server `load` function placement without re-litigating progressive-enhancement UX, form-action design, or component styling in every response. This skill exists because SvelteKit's universal `load` functions (`+page.js` / `+layout.js`) run on the server for the initial SSR render **and again in the browser** on every subsequent client-side navigation — a fact that is routinely missed and turns a misplaced database call or secret reference into a client-side credential exposure, not just a style defect.

## When to use

Use this skill when the user asks to:

- review a new or changed SvelteKit route's `load` functions before merge,
- investigate a secret, database error, or unexpected network call surfacing in the browser console or network tab for a SvelteKit route,
- determine whether data-fetching for a given route is placed in the correct file (`+page.js` vs `+page.server.js`, `+layout.js` vs `+layout.server.js`),
- audit `+layout`/`+page`/`+server` file precedence for a route tree.

Do not use this skill for:

- progressive-enhancement or `<form>` action / `use:enhance` UX review — use `sveltekit-progressive-enhancement-review` instead,
- pure component-internal state with no `load` function involved,
- live performance profiling of load waterfalls — that needs runtime tracing, not static review.

## Context7 Documentation Protocol

- Resolve the library ID with `resolve-library-id` (matched result: `/sveltejs/kit`) before citing any SvelteKit-specific claim.
- Before asserting the universal-vs-server execution model, call `query-docs` against `/sveltejs/kit` for "load functions" and quote the precise rule: a universal `load` (`+page.js`/`+layout.js`) runs on the server during SSR and hydration, then runs **again in the browser** on every subsequent client-side navigation (or always in the browser if SSR is disabled / the route is a SPA); a server `load` (`+page.server.js`/`+layout.server.js`) always runs only on the server. If both exist for a route, the server `load` runs first and its return value becomes the `data` property passed into the universal `load`.
- Verify the SvelteKit major version installed in the repo (`package.json`) before asserting version-specific behavior of `$env/static/private` / `$env/dynamic/private` enforcement or server-only module protection — this has evolved across releases (private `$env/*` client-side imports were blocked starting in a 1.0-pre release).
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Never assume a repo's SvelteKit config (`kit.env.privatePrefix`, adapter, `ssr`/`csr` page options) matches defaults without reading `svelte.config.js` and any per-route page-option exports — page options change when/whether a universal `load` ever runs on the server at all.

## Lean operating rules

- First classify every load-bearing route file as universal (`+page.js`, `+layout.js`) or server (`+page.server.js`, `+layout.server.js`) by filename suffix alone — never by guessing from content or route name.
- Treat every universal `load` function as browser-reachable code, full stop. It is not "mostly server-side" or "server-side on first load" — SvelteKit re-runs it client-side on every subsequent navigation by default.
- Do not assume SvelteKit's build-time `$env/static/private` / `$env/dynamic/private` import guard catches every leak. It blocks direct imports of those modules from client-reachable files, but it does not catch manual `process.env` reads, a `$lib/server`-protected module re-exporting a secret through a non-`.server` module, or a secret being copied into the plain object a server `load` returns.
- Trace secrets and DB clients through the full import graph, including transitive `$lib` imports — a universal `load` that imports an innocuous-looking `$lib/utils.js` which itself imports `$lib/server/db.js` is still a leak path (and SvelteKit's server-only module protection should catch that specific case at build time; verify it does, do not assume).
- Treat any private value present in the plain object a server `load` (or `+layout.server.js`) returns as a candidate leak the moment a universal `load`, a client component, or `page.data`/`$page.data` can read it — server `load` output crosses the server/client boundary via devalue serialization, it is not automatically safe just because it originated server-side.
- Never mark a `+page.server.js` or `+layout.server.js` file itself as a browser-exposure risk for containing secrets or DB calls — those files are always server-only by SvelteKit's routing contract; the risk is in what they choose to *return*, not that they execute.
- Do not flag deep or repeated `+layout.server.js` logic as a leak; flag it as a MEDIUM duplication/maintainability finding only, distinct from HIGH-severity leak findings.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the leak-tracing decision tree, and the required output shape.
- [Routing conventions](references/routing-conventions.md) — load only when `+layout`/`+page`/`+server` file precedence or route-tree structure (not just load placement) is in question.

## Response minimum

Return, at minimum:

- the route(s) and file(s) in scope, each labeled universal or server load,
- ranked findings with file:line evidence and the full import/data-flow trace for any leak finding,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., unverified `svelte.config.js` env prefix, unread transitive `$lib` module).
