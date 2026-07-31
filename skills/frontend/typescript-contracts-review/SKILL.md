---
name: typescript-contracts-review
description: Review TypeScript diffs and tsconfig strictness posture for sound type contracts — auditing any/assertion usage at trust boundaries, unsound narrowing, and exported public-API type-surface breakage — so that a passing compile is meaningful evidence rather than a decorative pass, and requiring paired runtime validation wherever external data enters the type system.
allowed-tools: Read Grep Glob Bash(git diff:*) Bash(tsc --noEmit:*) WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: compliance
---

# TypeScript Contracts Review

## Purpose

"The build passes" is only meaningful evidence of type safety if the active tsconfig is actually strict and the code doesn't defeat it with `any`, unchecked assertions, or broad suppression comments — and TypeScript types are fully erased at compile time, so a type annotation on external data (a parsed JSON response, a third-party SDK payload, a URL parameter) provides zero runtime protection unless paired with an actual runtime validator. This skill exists so those two failure modes — a loose or silently-weakened tsconfig, and type annotations that assert safety no runtime check backs up — get audited explicitly instead of being assumed away by a green checkmark. It also checks exported public-API type surfaces so a "just an internal refactor" diff doesn't silently break every downstream consumer of a published package.

## When to use

Use this skill when the user asks to:

- review a TypeScript/TSX diff for type-safety soundness before merge,
- audit a codebase's tsconfig strictness posture against current TypeScript-recommended defaults,
- check `any`/type-assertion/non-null-assertion usage, especially at external-data boundaries,
- verify a discriminated union or type-guard function actually narrows correctly,
- assess whether a change to an exported/published package breaks its public API type surface.

Do not use this skill for:

- pure runtime/logic bug hunting that has nothing to do with type contracts — that is a general code-review task,
- JavaScript files with no type annotations or JSDoc types — there is no type contract to audit,
- live `tsc --noEmit`/type-coverage execution results interpretation beyond what this static review can determine from the diff — report that a live run is needed rather than fabricating its result.

## Context7 Documentation Protocol

- Resolve the TypeScript library ID with `resolve-library-id` before ruling on any compiler-flag or strict-family question; do not answer from memory, because recommended defaults change across releases (confirmed via Context7: TypeScript 5.9's `tsc --init` now emits `noUncheckedIndexedAccess: true` and `exactOptionalPropertyTypes: true` as a separate "Stricter Typechecking Options" block alongside `strict: true`, where earlier `tsc --init` output only set `strict: true`).
- Before asserting which individual flags `strict: true` bundles (`strictNullChecks`, `noImplicitAny`, `noImplicitThis`, `alwaysStrict`, `strictFunctionTypes`, `strictBindCallApply`, `strictPropertyInitialization`, `useUnknownInCatchVariables`), call `query-docs` against the current TypeScript docs rather than reciting a memorized list — the bundle is described as open-ended ("future versions of TypeScript may introduce additional stricter checking under this flag"), so a stale list under-reports what a repo's `strict: true` actually enables on its installed compiler version.
- Before flagging or endorsing a `typescript-eslint` rule (e.g. `no-explicit-any`, `no-unsafe-assignment`, `no-non-null-assertion`), verify via `query-docs` whether that rule is in the repo's active shared config (`recommended`, `recommended-type-checked`, `strict-type-checked`) — rule membership across those tiers has changed between major versions (confirmed via Context7: the v7→v8 `recommended-type-checked` diff added/removed multiple rules), so do not assume a rule is enabled just because the repo extends a config by name without checking the installed version.
- Read `package.json`/lockfile first to confirm the installed `typescript` and `typescript-eslint`/`@typescript-eslint/*` major versions before citing version-gated behavior (e.g. `exactOptionalPropertyTypes` semantics, `satisfies` operator availability, `const` type parameters) — do not assume a feature is available because it appears in current docs if the installed major predates it.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label every version-sensitive claim `documentation-based, unverified against installed compiler version`.

## Lean operating rules

- Never accept "the build passes" as sufficient evidence of type safety without checking the actual tsconfig strict-family flags in effect — a loose config proves far less than developers assume, and `strict: true` alone (pre-5.9 `tsc --init` default) does not imply `noUncheckedIndexedAccess` or `exactOptionalPropertyTypes` are on.
- Every new `any` must carry an adjacent justification comment; flag unjustified `any` as blocking, especially inside application logic that later consumers trust as validated.
- Every trust-boundary type (parsed JSON, third-party SDK response, `postMessage` payload, URL/query-param, form input, environment variable) must be paired with an actual runtime validator — a type annotation alone is erased at compile time and enforces nothing at runtime.
- Treat any proposal to loosen existing tsconfig strictness (removing `strict`, disabling `strictNullChecks`) as requiring an explicit, separately-reviewed migration plan, not a routine PR change.
- Flag broad `@ts-nocheck`/file-level suppression as blocking by default; require `@ts-ignore`/`@ts-expect-error` to carry an adjacent comment explaining why, weighted higher severity if the suppressed code is security- or trust-boundary-relevant.
- Do not let generic-type complexity become unreadable; a type requiring a comment to explain what it constrains is a design smell worth simplifying, not a badge of sophistication.
- Do not run or assert `tsc --noEmit` success from memory — invoke it or explicitly flag that CI must, rather than fabricating a compile-success claim.
- When a diff touches an exported/published package's public surface, check for a breaking type change (removed export, narrowed parameter type, widened return type becoming a narrower consumer-facing type, added required property) before approving; a type-only change can still be a semver-breaking change even with zero runtime behavior difference.

## References

Load these only when needed:

- [Strict-flag posture reference](references/strict-flag-posture.md) — use when auditing or recommending a tsconfig strict-family flag set, including which flags are bundled under `strict` vs. separately opt-in (`noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`), and how to read a repo's actual effective config (including `extends` chains).
- [Trust-boundary validation patterns](references/trust-boundary-validation.md) — use when auditing external-data ingestion points (API responses, `JSON.parse`, `postMessage`, URL parsing, form/env input) for paired runtime validation against their declared types.
- [Public API surface diffing](references/public-api-surface-diff.md) — use when a diff touches an exported/published package and a breaking-change check against the previous public `.d.ts`/export surface is needed.

## Response minimum

Return, at minimum:

- the tsconfig strictness posture summary (which strict-family flags are on/off vs. current TypeScript-recommended defaults, and the compiler version that posture was checked against),
- the `any`/assertion audit for every new `any`, `as`, and `!` in the diff, each flagged with its justification or lack thereof and file:line evidence,
- the trust-boundary validation audit for every external-data ingestion point touched, naming the missing or present runtime validator,
- the public-API surface diff and any breaking-change flags, when the diff touches an exported package,
- verdict (approve / approve-with-notes / block),
- residual risk notes for anything requiring a live `tsc --noEmit`/type-coverage run beyond this static diff review.
