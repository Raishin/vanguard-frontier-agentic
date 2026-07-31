---
name: tree-shaking-dead-code-review
description: Verifies that a bundler's tree-shaking actually eliminated dead code by inspecting output bytes and sideEffects/module-format configuration, rather than trusting a clean build as proof of elimination.
allowed-tools: Read Grep Glob Bash(npm run build:*)
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: operational
---

# Tree-Shaking & Dead-Code Review

## Purpose

"The build succeeded" and "the code was tree-shaken" are unrelated claims. A production build with zero errors can still ship a fully dead, unreferenced module byte-for-byte, because tree-shaking is not a guaranteed pass — it is a conditional static-analysis optimization that silently no-ops the moment it hits a CJS `require`, a missing or wrong `sideEffects` field, a barrel-file re-export, or a development-mode build. This skill treats "eliminated" as a claim that must be proven from bundle-analyzer output bytes and module lists, before and after, in a production build — never inferred from build success, and never inferred from a `sideEffects: false` change applied without reading the target module for real side effects first, since that specific mistake causes silent runtime breakage (dropped polyfills, dropped CSS injection, dropped security-relevant initialization), not just a size regression.

## When to use

Use this skill when the user asks to:

- verify why a supposedly-unused import is still present in bundle-analyzer output,
- evaluate whether a new dependency is tree-shakeable before it is added,
- configure or review a `sideEffects` field in package.json for an app or a library the user is authoring,
- explain why a `sideEffects: false` change either had no effect or broke something at runtime.

## When NOT to use

- General bundle-size triage with no specific dead-code suspicion yet — use `bundle-budget-code-splitting-review` first to establish the numeric budget and rank contributors; hand off here once a specific module is suspected of being fully unused rather than merely heavy.
- Deciding route- or component-level code-splitting boundaries — that is a splitting-boundary decision, not an elimination-verification decision; `bundle-budget-code-splitting-review` owns it.
- Diagnosing which Core Web Vitals sub-phase is regressing with no analyzer report yet — hand off to `core-web-vitals-triage` first, return here once dead code is the named suspect.

## Context7 Documentation Protocol

Tree-shaking configuration surface differs by bundler and has shifted across majors (Rolldown replacing Rollup inside Vite 7+, webpack's `sideEffects`/`usedExports` interaction, Rollup's `treeshake` preset system). Do not prescribe a fix from memorized training data.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded this session.
2. Call `mcp__Context7__resolve-library-id` for the bundler actually installed in the project (webpack, Rollup, or Vite/Rolldown) before prescribing any `sideEffects` or `treeshake` configuration — do not assume the bundler from the framework name alone.
3. Call `mcp__Context7__query-docs` for the specific mechanism in question — e.g. "sideEffects array vs boolean", "treeshake.moduleSideEffects options", "production mode requirement for usedExports" — before ruling on it. Do this per review; do not reuse a prior session's memory of bundler internals.
4. Context7-grounded facts to confirm, not assume:
   - webpack: tree-shaking requires `mode: 'production'` (or `optimization.usedExports` explicitly enabled) to take visible effect; `sideEffects: false` in package.json additionally requires `optimization.providedExports` (on by default in production mode) to let webpack drop unused-export modules; a module with real side effects (e.g. CSS imports) must be listed explicitly, e.g. `"sideEffects": ["**/*.css"]`, or that side effect is silently dropped.
   - Rollup: `treeshake` accepts `false`, a preset (`'smallest' | 'safest' | 'recommended'`), or a fine-grained object — `moduleSideEffects` (boolean, `'no-external'`, a string array, or a predicate function) and `propertyReadSideEffects` are the two options most often responsible for either under- or over-aggressive elimination.
   - Vite 7+ ships Rolldown as its production bundler; `build.rollupOptions` is now an alias for `build.rolldownOptions` and is deprecated in favor of it — verify which option surface the installed Vite major actually documents before prescribing config keys.
5. If Context7 is unavailable or returns no relevant match for the installed bundler, fall back to `official_docs` and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
6. Never invent a bundler config key, CLI flag, or `package.json` field that no queried source confirms.

## Lean operating rules

- Require confirmation the build ran in production mode before evaluating any tree-shaking claim. A development-mode build routinely skips or partially applies elimination by design; "it's still there in dev" proves nothing.
- Determine the suspect dependency's actual module format from its package.json (`exports`/`module` with an `import` condition vs. a `main`/`require`-only CJS entry) — do not infer format from the package's popularity or age. CJS defeats static tree-shaking analysis regardless of bundler, and is the most common silent cause of "unused code that won't go away."
- Check `sideEffects` in both the app's own package.json and the suspect dependency's package.json. A missing field or `sideEffects: true` tells the bundler to assume every import has a side effect and keep it — this is often correct behavior being mistaken for a bug.
- Before proposing or accepting `sideEffects: false` on any module, read that module for top-level side-effecting code — global polyfills, CSS-in-JS injection, prototype patching, analytics auto-init, CSP nonce injection, sanitizer initialization. A false `sideEffects: false` claim is a correctness bug that ships silently; it will not show up as a build error.
- Rule out a barrel-file re-export pattern (`import * as utils from './utils'`, or a package `index.js` that re-exports everything) in the app's own code before blaming the dependency — this is a common tree-shaking blocker that has nothing to do with the dependency's configuration.
- Never accept "no build errors" or "the build completed" as proof of elimination. Require a bundle-analyzer or output-file diff showing the specific module or byte range is actually absent, taken from a production build, before and after the change.
- After any `sideEffects: false` change, require a runtime smoke test of the affected surface, not just a rebuild — dropped initialization code fails at runtime, not at build time.
- Confirm the installed bundler and its major version via Context7 before prescribing exact config syntax — see Context7 Documentation Protocol. Do not assume a memorized API is still current.

## References

Load these only when needed:

- [Module format and sideEffects field](references/module-format-and-sideeffects.md) — use when determining whether a dependency is ESM or CJS, and when reading or writing the `sideEffects` field in package.json (webpack's flag semantics and the production-mode requirement).
- [Rollup and Vite treeshake options](references/rollup-vite-treeshake-options.md) — use when the project is Rollup- or Vite/Rolldown-based and the review needs `treeshake.moduleSideEffects`, `propertyReadSideEffects`, or preset-level configuration.
- [ESM/CJS interop and verification](references/esm-cjs-interop-and-verification.md) — use when the suspect module's format is ambiguous, when CJS interop is suspected as the blocker, and for the before/after diff and runtime-smoke-test verification workflow that closes out every review.

## Response minimum

Return, at minimum:

- the module format (ESM/CJS) and `sideEffects` field state for both the app and the suspect dependency, cited from the actual package.json content read,
- confirmation the evidence came from a production-mode build, not development mode,
- the exact package.json or bundler-config change proposed, with syntax version-confirmed via Context7 against the installed bundler major,
- the before/after bundle-analyzer byte and module-count diff proving elimination, not just build success,
- a correctness caveat and runtime-smoke-test requirement whenever `sideEffects: false` is newly applied.
