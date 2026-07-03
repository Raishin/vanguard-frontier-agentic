# Hallucinated API Verification

Use this reference when a diff — especially one flagged or suspected as AI/LLM-generated — calls a framework/library API (a hook, a component prop, a lifecycle method, a utility export, a CLI flag) and you need to determine whether that API actually exists and behaves as claimed for the project's installed version.

## What people get wrong

The common bad assumption is:

> "It compiles, TypeScript didn't complain, and it reads like idiomatic React/Vue/Angular — so the API is real."

That is not evidence. Three independent failure modes hide behind a clean-looking call:

1. **Version drift** — the API existed in an earlier or later major version than the one installed (e.g., a hook or lifecycle method from a different major, a removed legacy API reintroduced from training data).
2. **Cross-framework bleed** — the model composed a plausible-sounding API by blending conventions from a *different* framework or library (e.g., invoking a Vue-shaped lifecycle name inside a React component, or a Redux Toolkit-shaped selector inside a Zustand store).
3. **Fully invented surface** — a prop, config key, or method that never existed in any version, constructed by pattern-completing the surrounding code's naming conventions.

TypeScript type-checking and successful compilation catch none of these reliably: overly-permissive types (`any`, loose generic constraints, ambient `declare` fallbacks), stale `@types` packages, or type-narrowing gaps let a nonexistent runtime API pass static checks silently.

## Non-negotiable verification rules

1. **Pin the version before checking the API.** Read `package.json` and the lockfile (`package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`) to determine the *actual installed* major/minor version of the framework or library in scope. Do not check an API claim against "React" in the abstract — check it against the resolved version.
2. **Resolve the Context7 library ID before querying.** Call `resolve-library-id` for the framework/library named in the diff, then `query-docs` with the specific API name/signature as the query. Do not skip straight to `query-docs` with a guessed ID.
3. **A miss on Context7 is not automatically a hallucination, but it is not automatically fine either.** If Context7 has no documentation coverage for a library, fall back to the library's official docs site directly (fetch or search), and only after that fails, label the claim `unverified — possible hallucination` rather than silently accepting it.
4. **Do not accept "it's probably a newer/undocumented API."** Generated code asserting the use of a bleeding-edge, unstable, or "experimental" API is a signal to verify harder, not a license to skip verification — check the changelog/release notes for the pinned version, not just the latest docs.
5. **Distinguish "does not exist" from "exists but deprecated/discouraged."** Both are findings, but they carry different severity and different remediation (delete the call vs. migrate to the current-recommended replacement per official docs).
6. **Never paraphrase from training-data memory as if it were verified.** If Context7 and official docs are both unreachable for a given library, say so explicitly and mark every affected claim `inference — not independently verified`, not `documentation-based`.

## Verification workflow

1. Extract every non-trivial framework/library API call touched or added in the diff (hooks, component props, lifecycle/lifecycle-like methods, exported utilities, CLI flags/config keys).
2. Resolve the pinned version for each library from the manifest/lockfile.
3. Resolve the Context7 library ID for that library (`resolve-library-id`), preferring a version-matched result when the tool exposes one.
4. Query Context7 for each API name/signature (`query-docs`) with a specific, non-generic query (e.g., "React 18 useDeferredValue signature" — not "React hooks").
5. Cross-check any result against the pinned version's changelog/release notes if the API's introduction or removal version is ambiguous.
6. Classify each API call as one of:
   - `verified` — found in Context7/official docs for the pinned version, cite the source,
   - `verified but deprecated for this version` — exists, but current docs recommend a replacement,
   - `unverified — possible hallucination` — not found after checking Context7 and official docs,
   - `inference — not independently verified` — verification tooling was unreachable.

## High-risk assumptions to kill

- "The model probably trained on the latest docs, so it's current." Training cutoffs lag releases and generated code frequently mixes API eras.
- "It's a small helper method, not worth checking." Small invented helpers (a nonexistent utility export, a fabricated config key) are exactly the pattern that slips through review silently because no one runs it until production.
- "The linter/type-checker would have caught a nonexistent API." Loose types, ambient declarations, and stale `@types` packages routinely let a nonexistent runtime member pass static analysis.
- "This project always uses the latest version, so version pinning doesn't matter here." Verify the actual lockfile — do not assume.

## When to push back

Push back if the user asks you to:

- approve an API call because "it looks right" without running the verification workflow above,
- skip version-pinning because "we're always on latest" without checking the lockfile,
- treat a Context7 miss as automatic proof the API doesn't exist without a fallback official-docs check.

Those are shortcuts that convert a verification claim into an unverified guess.
