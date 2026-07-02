# Public API Surface Diffing

Use this reference when a diff touches an exported/published package (a library consumed by other packages or external users via its declared entry points and generated `.d.ts` files) and a breaking-change check against the previous public type surface is needed.

## What people get wrong

The naive story is:

> No runtime logic changed, only type signatures — so this can't be a breaking change.

Wrong. TypeScript's declaration-file contract (`.d.ts`) *is* the public API surface for a published package's consumers. A change with zero runtime behavior difference can still be a semver-breaking change for anyone whose code type-checks against this package, because their build breaks even though the JavaScript that ships would have behaved identically.

## Officially grounded shape

Per the TypeScript declaration-files handbook, `.d.ts` files describe the shape of a library to consumers — they are the contract a downstream `tsc` run checks against, independent of the actual runtime implementation. Consequences that matter for a breaking-change review:

- **Removing an export** (a function, type, interface, class, or re-export) breaks every consumer importing it, even if nothing else changed.
- **Narrowing a parameter type** (accepting fewer input shapes than before, e.g. `string` → `'a' | 'b'`) breaks consumers who were legitimately passing a now-disallowed value, even if that value never caused a runtime error.
- **Widening a publicly-returned type in a way that removes previously-guaranteed members** (e.g. a return type that used to always include `id: string` now types it as `id?: string`, or a return type widens from a specific union to a broader one) breaks consumers whose downstream code relied on the narrower guarantee, even though the runtime object may not have changed at all.
- **Adding a required property to an exported interface/type** that consumers are expected to construct (not just consume) breaks every call site constructing that type without the new property.
- **Changing a type from an interface to a type alias (or vice versa)** is usually safe for consumption but can break consumers who were declaration-merging against the interface — a real but narrower risk to check when the package is designed for extension.
- **Generic parameter changes** (adding a required type parameter, changing a default, changing variance-relevant usage) can break consumers who don't explicitly specify type arguments and were relying on inference.

## Non-negotiable design rules

1. **Diff the emitted public surface, not just the source diff.** The source diff shows what changed in the implementation file; the question that matters is what changed in what the package actually exports from its declared entry point(s) (per `package.json` `exports`/`types`/`main` fields). An internal type change that never reaches an exported symbol is not a public API break.
2. **Type-only changes are real changes.** Do not wave through a diff as "just types, no behavior change" — for a published package, a type-only change to an exported symbol is exactly the kind of change semver-breaking-change review exists for.
3. **Distinguish parameter-position from return-position changes** — narrowing accepted input is breaking; narrowing *offered* output guarantees is also breaking, but for a different reason (consumers relying on the wider prior contract). Both directions matter; do not only check one.
4. **Check re-exports, not just direct declarations.** A type re-exported from a barrel file (`export * from './internal'`) is part of the public surface exactly as much as a type declared at the entry point — a change to the underlying `./internal` type is a public break even if the entry-point file itself has no diff.
5. **A change behind an internal-only export or a `@internal`/unexported symbol is not a public break** — confirm the changed symbol is actually reachable from the package's declared public entry point(s) before flagging it as breaking; do not over-flag purely-internal refactors.

## Minimal safe audit flow

1. Identify the package's declared public entry point(s) from `package.json` (`main`, `module`, `types`/`typings`, or the `exports` map's `types` conditions for each subpath).
2. From the diff, list every exported symbol (function, class, interface, type alias, enum, const) whose declaration changed, was added, or was removed — including via re-export chains reachable from the entry point(s).
3. For each changed exported symbol, classify the change: added (usually safe), removed (breaking), parameter narrowed (breaking for callers), parameter widened (usually safe), return narrowed/property-removed-or-optionalized (breaking for consumers), return widened purely additively (usually safe).
4. For anything classified breaking, state the concrete failure mode for a consumer: what specific call pattern that currently type-checks against the published types would fail to type-check against the new ones.
5. If the repo has no automated API-surface-diff tooling in place (e.g. no `api-extractor`, no `.d.ts`-snapshot test), note that as a process gap rather than silently accepting the manual audit as a full substitute for automated coverage on every future change.

## High-risk assumptions to kill

- "No runtime behavior changed, so it's not breaking" — irrelevant for a published package's type contract.
- "This is just a refactor, the public API is the same" — verify against the actual entry-point re-export chain; an internal rename can accidentally change what a barrel file re-exports.
- "Adding a property to an interface is always additive/safe" — true for consumption (reading), false for construction (a consumer building an object literal to satisfy that interface now needs the new property too, unless it was added as optional).
- "TypeScript would catch this for us automatically" — it catches it for *this repo's own* compile, not for downstream consumers' separate compiles against the published `.d.ts`; the break only surfaces in their build, after the package is published.

## Verification targets

- `package.json` `exports`/`types`/`main` fields, to establish the actual public entry point(s).
- `git diff` scoped to files reachable from those entry points (direct declarations and re-export chains).
- If available, a generated `.d.ts` diff (build output before/after) or an `api-extractor`-style report — prefer this over manual source-diff reasoning when the tooling exists, since re-export chains and inferred types are easy to miscount by hand.

## When to push back

Push back if the user asks to:

- ship a narrowed parameter type or removed export as a patch/minor version bump without flagging it as a breaking (major) change,
- describe a type-only breaking change as "not a real change" because runtime behavior is unaffected,
- skip the public-surface check because "it's just an internal refactor" without first confirming the changed symbol isn't reachable from a declared entry point.
