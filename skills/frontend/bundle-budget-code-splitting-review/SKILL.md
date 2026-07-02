---
name: bundle-budget-code-splitting-review
description: Reviews JavaScript/CSS bundle composition against explicit numeric budgets, evaluates route- and component-level code-splitting boundaries, and requires a CI-enforced budget before endorsing any size fix as resolved.
allowed-tools: Read Grep Glob Bash(npm run build:*) Bash(du:*)
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: operational
---

# Bundle Budget & Code-Splitting Review

## Purpose

Teams routinely respond to "the bundle feels big" with an ad hoc `React.lazy()` here and a dynamic `import()` there, ship it, and call the finding closed once the build succeeds. That proves nothing: it does not show whether the total byte weight moved, whether the change reduced or inflated the request count, or whether the win survives the next dependency bump. This skill turns "bundle feels big" into a numeric, device/network-scoped budget tied to INP/TTI, ranks analyzer output by both byte weight and main-thread execution cost, classifies each contributor into a specific remediation path, and refuses to mark a size finding resolved without a CI-enforced budget check — a one-time manual fix without an enforced budget is a regression waiting to happen.

## When to use

Use this skill when the user asks to:

- reduce JavaScript or CSS bundle size,
- review a bundle-analyzer report (webpack-bundle-analyzer, rollup-plugin-visualizer, or equivalent stats output),
- set or enforce a performance budget for a route or shared entry,
- decide where to add route-level or component-level code splitting,
- assess the size impact of adding a new dependency before it merges.

## When NOT to use

- Pure image/media asset optimization — different budget class (bytes-per-image, format/compression), different tooling; not this skill's concern.
- Server-side bundle or cold-start size for a serverless/edge function — different runtime, not the client-bundle budget this skill enforces.
- Deciding whether a chunk's *contents* are dead code once it is correctly split — hand off to `tree-shaking-dead-code-review` for that specific question; a chunk can be correctly split and still be full of unused exports, which is a distinct defect from a splitting-boundary defect.
- Diagnosing which Core Web Vitals sub-phase is regressing before a cause is known — hand off to `core-web-vitals-triage` first if the user only has a vague "it feels slow" complaint with no analyzer report yet; return here once JS weight is confirmed as the dominant contributor.

## Context7 Documentation Protocol

Bundler chunking APIs are version-sensitive and have changed shape recently — do not prescribe a config from memorized training data.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded this session.
2. Call `mcp__Context7__resolve-library-id` for the bundler in scope (Vite, webpack, Rollup, esbuild, Rolldown) before prescribing any chunking configuration.
3. Call `mcp__Context7__query-docs` for the specific mechanism — e.g. "manualChunks vs codeSplitting", "splitChunks cacheGroups", "dynamic import chunk naming" — before ruling on it. Do this per review; do not reuse a prior session's memory of bundler internals.
4. Known version-sensitive trap verified via Context7 as of this skill's `updated` date: Vite 8 (Rolldown-powered) removes the object form of `build.rollupOptions.output.manualChunks` entirely and deprecates the function form; the documented replacement is `build.rolldownOptions.output.codeSplitting` with a `groups` array (`{ name, test }` entries). Do not hand a Vite 8+ project a `manualChunks: { vendor: [...] }` object-form snippet — it will not apply. For Vite <8, function-form `manualChunks(id) { ... }` is still valid but is itself deprecated and should be flagged as a forward-migration item, not endorsed as the long-term pattern.
5. webpack's chunking surface (`optimization.splitChunks.cacheGroups`, dynamic `import()` with `webpackChunkName` magic comments) is comparatively stable across recent majors per Context7-grounded docs, but still confirm the installed webpack major before prescribing a specific `cacheGroups` shape, since defaults (`chunks: 'async'` vs `'all'`) affect which imports are eligible for splitting.
6. If Context7 is unavailable or returns no relevant match for the installed bundler, fall back to `official_docs` and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
7. Never invent a bundler config key, CLI flag, or plugin option that no queried source confirms.

## Lean operating rules

- Establish the numeric budget before ranking anything. A budget with no device/network class and no percentile is not a budget — it is a guess. Refuse to close a finding against an undefined budget.
- Rank analyzer contributors by both parsed/gzipped byte size and main-thread execution cost; these produce different orderings, and execution cost correlates more directly with INP than raw byte size does. Report both, do not collapse them into one number.
- Classify every large module into exactly one path: needed on the critical path (keep, minimize), needed but deferrable (route/component-split candidate), duplicated across chunks (fix chunk-grouping config, not application code), or a heavier-than-necessary dependency (replace, or hand off to `tree-shaking-dead-code-review` if the issue is unused exports rather than the dependency itself).
- Treat over-splitting as a real regression, not just under-splitting. Every new chunk is a new request; if the aggregate request-overhead cost (connection reuse aside, still parse/eval/scheduling cost) offsets the byte savings, the split is net-negative. Require the byte-vs-request-count comparison before endorsing a split.
- Confirm the installed bundler major version via Context7 before prescribing `manualChunks`, `rolldownOptions.codeSplitting`, or `splitChunks` syntax — see Context7 Documentation Protocol. Do not assume a memorized API is still current.
- Never accept "it built without errors" as evidence a split is beneficial. Require a before/after byte comparison from the analyzer output, gzipped or brotli as configured for production.
- Flag any dynamic `import()` whose module specifier is built from unsanitized user input as a code-injection risk, not merely a performance nit — this is a security-relevant finding, not a style note.
- Do not recommend inlining third-party scripts to "save a request" without naming the CSP/subresource-integrity trade-off of inlined vs. externally loaded, SRI-checkable code.
- Require a CI-enforced budget (bundlesize, Lighthouse CI budget, or bundler-plugin size-limit check) as a condition of marking any size finding resolved. A manual one-time fix with no enforcement is not a fix, it is a snapshot.

## References

Load these only when needed:

- [Budget methodology](references/budget-methodology.md) — use when the user has no existing budget, or an existing budget lacks a device/network class or percentile, and one must be established or corrected.
- [Bundler chunking APIs](references/bundler-chunking-apis.md) — use when prescribing or reviewing Vite, webpack, or Rollup chunking configuration; contains the version-sensitive API grounding.
- [Code-splitting boundaries](references/code-splitting-boundaries.md) — use when deciding where to add route- or component-level splitting, or when diagnosing duplicated-dependency-across-chunks and over-splitting.
- [CI enforcement and verification](references/ci-enforcement-and-verification.md) — use for every review to specify the CI-enforced budget mechanism, the verification command, and the adversarial checklist before closing the finding.

## Response minimum

Return, at minimum:

- the numeric budget in scope (bytes, compression form, device/network class, percentile), stated explicitly or flagged as missing,
- the ranked contributor list with both byte size and main-thread execution-cost figures,
- per-item classification (keep / split / replace / dead-code-handoff / chunk-grouping-fix) with the concrete config diff, version-confirmed against the installed bundler major,
- the CI-enforced budget definition to add, not just a one-time manual fix,
- the verification command (`npm run build -- --report` or the project's equivalent analyzer invocation) and the request-count check confirming the split did not net-negative the change.
