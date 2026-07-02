---
name: build-tooling-vite-webpack-review
description: Reviews Vite and Webpack build/chunking configuration and bundle-size composition for duplicate dependencies, unsplit vendor chunks, and tree-shaking failures, always version-labeling config since Vite 8 replaced Rollup-era manualChunks with Rolldown-based codeSplitting.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: compute
---

# Build Tooling (Vite/Webpack) Review

## Purpose

Bundler config advice that doesn't match the installed major version is worse than no advice — it produces a config diff that silently no-ops, throws at build time, or (worst case) applies to the wrong bundler engine entirely. Vite ships two build engines depending on version and config shape (Rollup-backed `rollupOptions`, or Rolldown-backed `rolldownOptions`), and Vite 8 removed the object form of `manualChunks` outright. This skill reviews Vite/Webpack build and chunking *configuration itself* — the config file, not the resulting bundle bytes — and treats the target bundler's exact major version as a required input, not an assumption, before proposing any config diff.

## When to use

Use this skill when the user asks to:

- review or tune Vite `build.rollupOptions`/`build.rolldownOptions` or Webpack `optimization.splitChunks` chunking configuration,
- diagnose a duplicate-dependency-across-chunks defect or an unsplit vendor chunk caused by chunking config (not by application-level import boundaries),
- evaluate a proposed bundler migration (Webpack to Vite, Rollup-backed Vite to Rolldown-backed Vite, or a Vite major-version upgrade) for build-time and config-compatibility impact,
- diagnose a tree-shaking failure that traces back to bundler/build config (module format, `sideEffects` field interaction with the bundler, barrel-file re-exports defeating static analysis) rather than to source-code side effects,
- set up or review a CI bundle-size budget gate at the build-tool level (build script, CI job config).

## When NOT to use

- Setting or auditing the *numeric* size budget itself, ranking analyzer contributors by byte weight vs. execution cost, or deciding route/component-level code-splitting boundaries — hand off to `bundle-budget-code-splitting-review` for that; this skill reviews the bundler config mechanism, not the budget methodology or split-boundary decision.
- Verifying that tree-shaking actually eliminated dead code via a before/after byte diff, or auditing `sideEffects: false` correctness on a specific package — hand off to `tree-shaking-dead-code-review` for that; this skill only flags build-config patterns (barrel imports, module-format mismatches) that are *known to defeat* tree-shaking, it does not verify the outcome.
- Non-bundler build tooling (Babel-only transpilation pipelines with no bundling step, TypeScript `tsc`-only builds, esbuild/Rolldown used as a library outside Vite/Webpack) — different config surface, not in scope here.

## Context7 Documentation Protocol

Vite's build-engine and chunking API surface changed between major versions — do not prescribe a config from memorized training data.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded this session.
2. Call `mcp__Context7__resolve-library-id` for each bundler in scope (`/vitejs/vite`, Webpack, `/rollup/rollup`, and `/rolldown/rolldown` if the project's Vite build is Rolldown-backed) before prescribing any chunking configuration.
3. Call `mcp__Context7__query-docs` for the specific mechanism — e.g. "rollupOptions.output.manualChunks vs rolldownOptions.output.codeSplitting", "splitChunks cacheGroups defaults", "manualChunks object form vs function form" — before ruling on it. Do this per review; do not reuse a prior session's memory of bundler internals.
4. Known version-sensitive facts verified via Context7 as of this skill's `updated` date:
   - Vite's migration guide documents that the **object form** of `build.rollupOptions.output.manualChunks` was **removed**, and the **function form** was **deprecated**; the replacement is `build.rolldownOptions.output.codeSplitting`, which takes a `groups` array of `{ name, test }` entries (Rolldown's declarative chunk-grouping API). Do not hand a project on that config path a `manualChunks: { vendor: [...] }` object-form snippet — it will not apply.
   - Rollup itself (used directly, or via Vite versions/configs that still route through `rollupOptions`) continues to support both the object form and function form of `output.manualChunks` per Rollup's own configuration docs — the removal is specific to Vite's option surface and its migration path toward Rolldown, not a Rollup-wide deprecation. Confirm which bundler engine actually processes the config (`rollupOptions` vs `rolldownOptions` key) before applying either party's rules.
   - Vite ships a built-in `splitVendorChunkPlugin` that composes with function-form `manualChunks` but explicitly logs a warning and no-ops when combined with the object form — treat "vendor chunk isn't splitting and there's a console warning about `splitVendorChunk`" as a symptom of this specific interaction, not a generic bug report.
5. Webpack's `optimization.splitChunks` surface (`cacheGroups`, `chunks: 'all' | 'async' | 'initial'`, `minSize`, `maxInitialRequests`) is comparatively stable across recent majors per Context7-grounded docs, including its documented defaults (`minSize: 20000`, `defaultVendors`/`default` cache groups with `priority: -10`/`-20`) — still confirm the installed Webpack major before asserting a specific default, since defaults have shifted across major versions historically.
6. If Context7 is unavailable or returns no relevant match for the installed bundler, fall back to `official_docs` and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
7. Never invent a bundler config key, CLI flag, or plugin option that no queried source confirms.

## Lean operating rules

- Confirm the installed Vite major version and which output-options key (`rollupOptions` vs `rolldownOptions`) the project's own config already uses before proposing a diff — these are two different bundler engines with two different chunking APIs, and giving the wrong era's config is a concrete, verified failure, not a style choice.
- Distinguish Vite's dev-server behavior (native ESM, no bundling, chunking config irrelevant) from its production build behavior (bundled via Rollup or Rolldown depending on version/config); every recommendation in this skill applies only to the production build path.
- Flag barrel-file (`index.ts` re-export) imports and wildcard imports of large libraries (icon sets, date libraries, utility libraries) as build-config-relevant tree-shaking risks when the project's module format or bundler settings can't statically resolve them — name the specific named/subpath-import fix, and hand off to `tree-shaking-dead-code-review` to verify the byte-level outcome.
- Treat a duplicate-dependency-across-chunks finding as a chunk-grouping *config* defect (fix `manualChunks`/`codeSplitting`/`cacheGroups`), not an application-code defect — do not propose restructuring import statements to solve what a config change solves more directly.
- Require a stated measured baseline (current bundle output, build time) before endorsing any bundler or bundler-major migration, plus a rollback path (pinned lockfile entry, ability to revert the config key).
- Recommend a CI bundle-size budget gate at the build-tool/CI-job level for any budget-critical route that lacks one, rather than treating build-config review as a one-time audit — but define the numeric budget itself only via `bundle-budget-code-splitting-review`.
- Do not evaluate bundle-size numbers as if they were field/RUM Core Web Vitals data; a build's stats output is lab data from a CI build unless explicitly stated otherwise.
- Never accept "it built without errors" as evidence a chunking config change is correct — a removed or renamed option key can silently no-op (see the Vite 8 `manualChunks` object-form removal) rather than error.

## References

Load these only when needed:

- [Vite chunking config reference](references/vite-chunking-config.md) — use when reviewing or writing `rollupOptions.output.manualChunks` or `rolldownOptions.output.codeSplitting`, or diagnosing which bundler engine a Vite config actually invokes.
- [Webpack splitChunks reference](references/webpack-splitchunks-config.md) — use when reviewing or writing `optimization.splitChunks`, `cacheGroups`, or `runtimeChunk` configuration.
- [Migration and config-diff safety](references/migration-and-config-diff-safety.md) — use before endorsing a bundler migration or a chunking-config change, to confirm baseline capture, rollback path, and the verification steps that must precede calling a config change complete.

## Response minimum

Return, at minimum:

- the bundler and confirmed major version (and, for Vite, the confirmed output-options key: `rollupOptions` or `rolldownOptions`) the guidance targets,
- build-config findings (duplicate-dependency chunk-grouping defects, config keys that no longer apply for the confirmed version, barrel-file/module-format patterns defeating tree-shaking) with file/line evidence,
- proposed chunking/config diff (not applied), labeled lab vs field for any performance number cited,
- evidence level (`live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`),
- migration caveat (baseline + rollback) if a bundler or bundler-major change is recommended, and a handoff note to `bundle-budget-code-splitting-review` or `tree-shaking-dead-code-review` if the user's actual question is budget-setting or tree-shaking verification rather than config review.
