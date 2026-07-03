---
name: angular-architecture-signals-review
description: Statically review Angular component and service architecture for correct Signals usage (signal/computed/effect boundaries and purity), appropriate change-detection strategy (OnPush vs default), and service/DI boundary design, grounded in Angular's own Signals, change-detection, and dependency-injection guidance.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Angular Architecture & Signals Review

## Purpose

Review Angular reactive-primitive boundaries (`signal`/`computed`/`effect`/`linkedSignal`), change-detection strategy, and service/DI ownership without re-litigating SSR/hydration concerns, RxJS-only legacy code with no Signals involved, or full zoneless-migration planning in every response. This skill exists so those adjacent concerns stay out of scope and the review stays focused on reactive-graph correctness and change-detection performance opportunities that are actually verifiable from the diff.

## When to use

Use this skill when the user asks to:

- review a component migrated to or newly written with Signals for correctness,
- assess whether an `effect()` call has an inappropriate or missing side effect,
- review whether `computed()` usage stays pure,
- perform a change-detection performance review (OnPush adoption, strategy mismatches),
- review service/DI boundaries for ownership of cross-cutting mutable state.

Do not use this skill for:

- pure RxJS-only legacy code with no Signals involved and no stated migration plan — that is a different, RxJS-specific review,
- SSR/hydration-specific concerns — use `angular-ssr-hydration-review` instead,
- recommending an app-wide zoneless migration — that is a dedicated architectural decision, not a PR-review fix.

## Source priority (official Angular skills first)

Consult sources in this fixed order, and label findings by which layer they draw from:

1. **The official Angular team's `angular-developer` skill** ([github.com/angular/skills](https://github.com/angular/skills/tree/main/angular-developer)) is the AUTHORITATIVE primary source for what is idiomatic Angular — Signals-first reactivity (`signal`/`computed`/`linkedSignal`/`resource`), modern control flow (`@if`/`@for`/`@switch`), standalone-by-default, `inject()` + `providedIn: 'root'` DI, and SSR/hydration strategy. Prefer its idioms over memory.
2. **Context7 `angular_dev` docs** (`/websites/angular_dev`, or the version-pinned `/websites/v20_angular_dev`) confirm those idioms against the repo's pinned `@angular/core` major (read `package.json` first). API/primitive availability is versioned — the official skill states the modern shape, the versioned docs confirm it applies to THIS repo's major.
3. **This skill's static-review judgment** (severity classification, `computed()` purity rules, effect-vs-derivation rules, injection-context and post-await-tracking checks, missed-OnPush and DI-ownership findings) is the review layer applied ON TOP of 1 and 2.

Where guidance conflicts: the official Angular `angular-developer` skill + version-matched `angular_dev` docs WIN on "what is idiomatic Angular" (which primitive/API to use, current recommended shape). THIS skill governs "what to flag in review" (which deviations rise to a HIGH/MEDIUM/LOW finding and why). Never let this skill's flagging override an idiom the official skill and pinned docs endorse; conversely, an idiom being official does not exempt a concrete purity/effect/injection defect from being flagged.

## Context7 Documentation Protocol

- Resolve the Angular library ID with `resolve-library-id` (matched result: `/websites/angular_dev` or `/websites/v20_angular_dev` when the repo's confirmed major is pinned) before citing any Signals-, change-detection-, or DI-specific claim.
- Before asserting a `computed`/`effect`/`linkedSignal` usage is correct or incorrect, call `query-docs` against the repo's actual Angular major version (read `package.json` first — `@angular/core`) and cite the doc section. Signals stabilized incrementally across Angular v16-v20 and `linkedSignal` is a newer primitive; do not assume availability without confirming the version.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Never assume the latest Angular docs (e.g. `@Service` decorator preference noted for v22+, `provideZonelessChangeDetection`) apply to an older major present in the repo.

## Lean operating rules

- First read `package.json` to confirm the installed `@angular/core` major version. Do not make a Signals-API-availability claim (e.g. `linkedSignal`) without confirming the version supports it.
- Classify each reactive primitive in scope as `signal` (state), `computed` (derived, pure, memoized) or `effect` (side effect on non-reactive APIs) before evaluating it. Do not apply one correctness rule to all three uniformly.
- `computed()` must be pure per Angular's own guidance (lazily evaluated and memoized). Any side effect inside a `computed()` callback (signal writes, DOM mutation, HTTP calls, logging) is a HIGH-severity purity violation, not a style note.
- `effect()` exists for syncing signal state to imperative, non-reactive APIs (logging, `window.localStorage`, custom DOM behavior, third-party UI library sync) per Angular's documented use cases. An `effect()` that only derives and stores a value other state depends on is propagating state through a side-effect channel — Angular's own docs warn this risks `ExpressionChangedAfterItHasBeenChecked` errors, circular updates, and unnecessary change-detection cycles; recommend `computed()` or `linkedSignal()` instead.
- `effect()` requires an injection context (component/directive/service constructor, or an explicit `Injector` passed via options). Flag any `effect()` call site that cannot resolve an injection context as a correctness bug, not a style preference.
- Signals read after an `await` inside an `effect()` lose reactive tracking (the reactive context does not survive the async boundary). Flag any effect that reads a signal post-await and expects it to be tracked as a dependency.
- Do not flag a component that stays at the default (non-OnPush) change-detection strategy as broken. Signals adoption without OnPush is a missed performance opportunity (MEDIUM), not a defect — verify the component doesn't rely on an implicit mutation-based update path that OnPush would break before recommending the switch.
- Do not recommend removing Zone.js or switching to zoneless (`provideZonelessChangeDetection`) as a review finding. That is an app-wide architectural decision requiring a dedicated migration plan.
- Treat services holding cross-cutting mutable state without a documented ownership model as a DI-boundary finding, but do not recommend a specific DI pattern (`providedIn: 'root'` singleton vs component-scoped provider) without confirming the actual sharing/lifetime requirement from the code in scope.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Treat any hardcoded API key, token, or secret found in component/service state, default values, or example data as a HIGH-severity finding requiring immediate escalation, not a style note.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the effect-vs-computed decision tree, the OnPush escalation rule, and the required output shape.
- [linkedSignal and DI-boundary patterns](references/linked-signal-and-di-boundaries.md) — load only when `linkedSignal` appears in the diff, or when the review scope includes service/DI ownership review.

## Response minimum

Return, at minimum:

- the component(s)/service(s) and files in scope,
- ranked findings with file:line evidence, primitive-misuse category (purity violation / derivation-via-effect / injection-context error / untracked-post-await read / missed-OnPush-opportunity / DI-ownership gap), and a concrete fix sketch per finding,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block),
- open questions or explicitly out-of-scope items (e.g. zoneless migration flagged as out of scope, missing confirmed Angular major version).
