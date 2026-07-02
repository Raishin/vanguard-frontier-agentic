# Migration and Config-Diff Safety

Use this reference before endorsing any bundler migration (Webpack to Vite, Rollup-backed Vite to Rolldown-backed Vite, or a Vite/Webpack major-version bump) or before calling any chunking-config change complete.

## What people get wrong

The naive assumption is:

> The build succeeded with the new config, so the migration/change is done.

Wrong on two counts. First, a removed-but-not-erroring option (see `references/vite-chunking-config.md` on the Vite `manualChunks` object-form removal) means a "successful" build can silently be running a different chunking strategy than the team believes — success is not evidence of equivalence. Second, "the build succeeded" says nothing about whether the resulting bundle composition, chunk count, or build time actually moved in the intended direction, or whether a rollback path exists if it didn't.

## Non-negotiable rules

1. **Capture a measured baseline before any config or bundler change is applied.** At minimum: current chunk list with sizes (from the existing build's stats/manifest output), current total build time, and current bundler/major version. Without this, "did the migration help" is unanswerable after the fact.
2. **State the rollback path explicitly before endorsing the change.** For a config-only change, this is typically "revert this diff, config keys are additive/isolated." For a bundler-major or engine-swap migration (e.g., adopting Rolldown-backed Vite), confirm whether the change is a single reversible config-key swap or whether it also touches lockfile-pinned plugin versions that may not be compatible with the prior engine — a plugin written against Rollup's plugin API is not guaranteed compatible with Rolldown's plugin API even where the two claim general compatibility; confirm via Context7 rather than assuming.
3. **Never treat "build succeeded, no errors" as proof a chunking-config change took effect.** Require a chunk-list diff (names, count, sizes) between before and after. An unchanged chunk list after an intentional config edit means the edit likely didn't apply — check for the specific silent-no-op patterns named in the Vite and Webpack references before assuming the config is simply already optimal.
4. **Do not endorse a bundler migration on build-time or DX grounds alone without also stating the bundle-composition impact**, or explicitly noting that composition impact is unverified. A faster build with an unreviewed change in chunk boundaries can trade build-time wins for runtime request-count or duplicate-dependency regressions — that trade-off must be named, not silently accepted.
5. **Flag any migration proposal that lacks a stated baseline or rollback path as incomplete**, regardless of how confident the proposal otherwise sounds — this is the same discipline this skill applies to config-key claims: unverified confidence is not evidence.
6. **When the user's actual goal is a numeric bundle-size budget or code-splitting boundary decision** (not a config-mechanism review), hand off to `bundle-budget-code-splitting-review` rather than expanding this skill's scope to cover budget methodology.
7. **When the user's actual goal is verifying that a config change eliminated dead code** (byte-level tree-shaking outcome, `sideEffects` correctness), hand off to `tree-shaking-dead-code-review` rather than asserting a tree-shaking outcome from config inspection alone — this skill can flag config patterns *known to defeat* tree-shaking (barrel imports, module-format mismatches) but does not verify the resulting byte-level outcome itself.

## Minimal safe review flow

1. Confirm the current bundler, major version, and (for Vite) the active output-options key (`rollupOptions` vs `rolldownOptions`).
2. Capture the baseline: current chunk list/sizes and current build time, from user-provided build output or stats file — do not fabricate baseline numbers.
3. Ground every config-key claim via Context7 for the confirmed version before proposing a diff (see the Context7 Documentation Protocol in `SKILL.md`).
4. Propose the config diff, not an applied change — this is a static-review skill; it does not run builds.
5. State the verification command the user (or their CI) must run to confirm the diff took effect (e.g., `npm run build -- --report` or the project's equivalent stats-output invocation) and what a successful before/after diff looks like.
6. State the rollback path explicitly, even if it is as simple as "revert this config diff."

## When to push back

Push back if the user asks for:

- a bundler-major or engine (Rollup-to-Rolldown) migration with no stated baseline and "we'll measure it after,"
- a chunking-config change accepted as complete purely because the build didn't error,
- merging all of `node_modules` into a single vendor chunk (Webpack) or an equivalent broad grouping (Vite) presented as a universal best practice rather than a named caching/parallelism trade-off,
- a `manualChunks` config change on a Vite project without first confirming which output-options key and Vite major are actually in play.

Those are not shortcuts. They are a config diff without evidence it does what it claims.
