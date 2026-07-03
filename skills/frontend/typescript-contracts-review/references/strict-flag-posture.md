# Strict-Flag Posture

Use this reference when auditing or recommending a tsconfig strict-family flag set, or when a reviewer needs to determine what a repo's `strict: true` actually enables on its installed compiler.

## What people get wrong

The naive story is:

> The repo sets `"strict": true`, so it's using TypeScript's strictest checking.

Incomplete, for two separate reasons:

1. `strict` is a bundle, and the bundle's membership is **open-ended by design**. Per the TypeScript docs: turning `strict` on "is equivalent to enabling all of the strict mode family options... Future versions of TypeScript may introduce additional stricter checking under this flag." A repo pinned to an older `typescript` version gets a smaller bundle than the same `strict: true` on a newer one.
2. Two of the most consequential type-soundness flags — `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes` — are **not** part of the `strict` bundle at all. They are separate opt-in flags. A repo can be maximally "strict" by the `strict: true` definition and still have neither enabled.

## Officially grounded shape (what the docs say)

- `strict: true` is the master flag; it currently bundles at minimum `strictNullChecks`, `noImplicitAny`, `noImplicitThis`, `alwaysStrict`, `strictFunctionTypes`, `strictBindCallApply`, `strictPropertyInitialization`, and (since TypeScript 4.4) `useUnknownInCatchVariables`. Verify the current exact bundle via `query-docs` against the installed version — do not treat this list as closed or version-independent.
- `useUnknownInCatchVariables` (TS 4.4+) changes the default type of a `catch` variable from `any` to `unknown`. It is auto-enabled under `strict`. Its presence means catch-block property access (`err.message`) requires narrowing first (e.g. `instanceof Error`) — code that skips narrowing under an older compiler target may newly fail, or may be silently unsound if the repo is still pre-4.4.
- `noUncheckedIndexedAccess` — **not** in the `strict` bundle — makes indexed access (`arr[i]`, `record[key]`) include `undefined` in the result type. Without it, index signatures and array access lie: `arr[i]` types as `T`, not `T | undefined`, even though out-of-bounds access returns `undefined` at runtime.
- `exactOptionalPropertyTypes` — **not** in the `strict` bundle — makes `{ prop?: string }` distinguish "prop is absent" from "prop is present and set to `undefined`". Without it, `obj.prop = undefined` is accepted as satisfying an optional property even where the author's intent required the key to be entirely absent (relevant for APIs that branch on `'prop' in obj` or `Object.keys`).
- As of TypeScript 5.9, `tsc --init` generates both of these under a separate "Stricter Typechecking Options" block alongside `strict: true` — this is a **default-generation change**, not a retroactive change to what `strict` means. Repos scaffolded before 5.9 will not have picked these up automatically, and repos on TypeScript versions predating their introduction cannot have them regardless of `tsc --init` vintage.

> Version note: exact strict-family bundle membership and default `tsc --init` output are version-sensitive. Confirm both via `query-docs` against the specific `typescript` version in the repo's lockfile before making a definitive claim — do not rely on a memorized flag list.

## Non-negotiable design rules

1. **Read the effective config, not just the top-level file.** `tsconfig.json` can `extends` a base config (monorepo shared config, `@tsconfig/*` package). A flag absent from the leaf file may be inherited, or a permissive leaf-level override may silently defeat a strict base. Resolve the full `extends` chain before concluding a flag is off.
2. **Distinguish "not in `strict`" from "not needed."** `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes` are the two flags most commonly mistaken for redundant with `strict`. Treat their absence as a real gap to call out, not a non-finding, whenever the diff under review does array/record indexing or optional-property assignment that would behave differently under them.
3. **Version-gate every strict-bundle claim.** Do not assert "this repo has `useUnknownInCatchVariables`" from `strict: true` alone — confirm the installed `typescript` version is 4.4+ first.
4. **Treat strictness removal as a migration, not a diff line.** A one-line change disabling `strictNullChecks` or removing `strict` can silently make thousands of previously-checked call sites unchecked. Require an explicit, separately-reviewed migration plan (with a scoped rollout, e.g. per-directory `strict` via TypeScript project references, or documented follow-up) rather than approving it inline in an unrelated feature PR.
5. **Do not conflate ESLint strictness with compiler strictness.** `typescript-eslint`'s `strict-type-checked` config and the compiler's `strict` flag are independent axes — a repo can have one without the other. Check both when the review scope includes lint config.

## Minimal safe audit flow

1. Read `package.json`/lockfile to get the exact `typescript` version.
2. Read `tsconfig.json` and resolve its full `extends` chain to the effective merged config.
3. List which strict-family flags (per the version-confirmed bundle) are on, and separately whether `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes` are on.
4. Compare against current TypeScript-recommended defaults (via `query-docs`) for that version, and note the gap explicitly rather than treating "has `strict: true`" as sufficient.
5. If the diff under review does indexed access or optional-property assignment and the relevant flag is off, name the specific unsound pattern this enables (e.g. "line 42 assumes `config[key]` is never `undefined`; with `noUncheckedIndexedAccess` off, the compiler will not catch this if `key` is absent from `config`").

## High-risk assumptions to kill

- "`strict: true` means fully strict" — it does not include `noUncheckedIndexedAccess` or `exactOptionalPropertyTypes`.
- "The build passes, so the types are sound" — passes only prove soundness relative to whatever flags are actually active, which may be a materially weaker set than the reviewer assumes.
- "Newer TypeScript version means newer defaults apply automatically" — `tsc --init` default generation only affects newly scaffolded configs; upgrading the compiler does not retroactively add opt-in flags to an existing `tsconfig.json`.
- "This monorepo's shared base config is strict, so every package is strict" — a leaf `tsconfig.json` can override or narrow it; always resolve the effective chain.

## Verification targets

- `cat tsconfig.json` and every file in its `extends` chain.
- `npm ls typescript` / lockfile entry for the resolved `typescript` version.
- `tsc --showConfig` (if available in the environment) to print the fully resolved effective compiler options — prefer this over manually merging `extends` chains by hand when the tool is available.

## When to push back

Push back if the user asks to:

- disable `strict` (or any of its family) as a quick fix to unblock a build, without a scoped migration plan,
- treat a passing `tsc --noEmit` as proof of runtime safety for external-data handling — it is not; that is the trust-boundary concern in a separate reference,
- skip checking `noUncheckedIndexedAccess`/`exactOptionalPropertyTypes` because "the repo has `strict: true`" — that claim alone does not cover those flags.
