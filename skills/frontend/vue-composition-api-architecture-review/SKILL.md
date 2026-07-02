---
name: vue-composition-api-architecture-review
description: Statically review Vue 3 Composition API code — composable extraction quality, reactivity-boundary correctness (ref/reactive/computed usage, destructuring-loses-reactivity pitfalls), and script-setup component organization — against Vue's own documented composable conventions and reactivity fundamentals.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Vue Composition API Architecture Review

## Purpose

Review Vue 3 composables and `<script setup>` components for the specific defect classes Vue's own documentation calls out — reactivity lost through destructuring, lifecycle hooks registered outside the synchronous setup window, impure `computed()` getters, and script-setup components that mix data-fetching, business logic, and presentation with no composable extraction — without re-litigating component prop design, template markup, styling, or SSR/hydration concerns in every response. This skill exists so those adjacent concerns stay out of scope and the review stays focused on the documented reactivity-boundary and composable-extraction catalog.

## When to use

Use this skill when the user asks to:

- review a new or refactored composable (a `use*`-prefixed function) before merge,
- diagnose a report that "reactive state doesn't update in the UI" after destructuring a composable's return value,
- review a `<script setup>` component for organization and whether logic should be extracted into a composable,
- decide whether a given `computed()` or lifecycle-hook usage is correct.

Do not use this skill for:

- Options API-only codebases with no Composition API usage and no stated migration plan — there is no reactivity-boundary or composable-extraction surface to review,
- SSR-specific security concerns (session/auth-token handling inside composables, hydration mismatches) — hand off to `vue-ssr-security-review`,
- a bug that requires live reproduction (Vue Devtools reactivity inspection, browser profiling) to confirm — static analysis can identify the missing `toRefs()`/guard, not prove which specific runtime instance broke.

## Context7 Documentation Protocol

- Resolve the Vue library ID with `resolve-library-id` (matched result: `/vuejs/vue`) before labeling any specific pattern as a reactivity-loss bug or a composable-convention violation.
- `/vuejs/vue` is Vue's core source-and-test repository, not the prose docs site. Use `query-docs` against it to corroborate exact runtime behavior (e.g., the `computed({ get, set })` writable-computed shape, `reactive()`/`ref()` tracking mechanics) from source and unit tests. It does not reliably surface the narrative guidance in the Composables and Reactivity Fundamentals guides — for that guidance, use the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based`.
- Before asserting a specific destructuring pattern loses reactivity, confirm which construct is being destructured: destructuring a `reactive()`-returned proxy loses reactivity on primitive-valued properties; destructuring a `ref()` or a `toRefs()`-wrapped object does not, because each extracted value is itself a ref. Do not apply a blanket "destructuring is unsafe" rule — check the return type first.
- Read `package.json` first to confirm Vue 3.x is in use; Composition API and its documented composable conventions (including `<script setup>`) are Vue 3 features, not Vue 2.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- For every composable in scope, first determine whether its return value is intended to be destructured by consumers. If yes, verify it returns individual refs or a `toRefs()`-wrapped reactive object — a plain `reactive()` object returned for destructuring is the documented reactivity-loss pitfall, not a style preference.
- For every composable, verify the `use*` naming convention and verify that lifecycle hooks (`onMounted`, `onUnmounted`, `watch`, etc.) and other composables are called synchronously during the composable's/component's initial `setup()`/`<script setup>` execution — not inside an async callback, `await`-continuation, conditional, loop, or event handler registered after setup has returned. Vue's synchronous-registration requirement is a hard rule, not a lint nicety: a hook registered after an `await` silently fails to attach to the correct component instance.
- For every `computed()` in scope, verify the getter is a pure read with no side effects (no state mutation, no async calls, no logging with side effects). A writable computed (`computed({ get, set })`) is the documented pattern for two-way-bindable derived state — do not flag its `set` function as an impurity; that is its intended role.
- Review `<script setup>` components for responsibility mixing: data-fetching, validation/business logic, and presentation-only state all inline with no composable extraction. Flag extraction candidates only when the mixed logic is non-trivial and would reduce duplication or improve testability if extracted — do not flag a component for using one or two local `ref()`s that have no reuse potential.
- Do not fabricate a reactivity-loss finding without showing the exact destructuring line and the specific property that would stop updating. A finding that only says "this might lose reactivity" without the concrete assignment is not a valid finding.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the reactivity/composable decision tree, and the required output shape.
- [Reactivity boundaries](references/reactivity-boundaries.md) — load only when reviewing `ref()`/`reactive()`/`computed()` usage, a suspected destructuring-loses-reactivity bug, or `toRef()`/`toRefs()`/`unref()` interop code.
- [Composable and script-setup conventions](references/composable-and-script-setup-conventions.md) — load only when reviewing composable naming/structure, lifecycle-hook registration timing, or `<script setup>` responsibility mixing and extraction candidates.

## Response minimum

Return, at minimum:

- the composable(s) and/or component(s), files, and specific call sites in scope,
- ranked findings with file:line evidence, defect category (reactivity-loss, lifecycle-timing, computed-impurity, or extraction candidate), and a concrete fix sketch matching the docs' recommended pattern,
- for every reactivity-loss finding, the exact destructuring line and the property that would stop updating,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "requires Vue Devtools reactivity inspection to confirm at runtime").
