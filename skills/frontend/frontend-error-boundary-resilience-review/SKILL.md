---
name: frontend-error-boundary-resilience-review
description: Reviews error-boundary placement, fallback UX, and failure-isolation strategy to prevent a single component's runtime error from crashing the whole page, ensuring granular, accessible degradation instead of an app-root-only safety net or a silent, unlogged failure.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: resilience
---

# Frontend Error Boundary & Resilience Review

## Purpose

Review the placement, granularity, and fallback UX of error boundaries across a frontend application so that a single component's runtime error is isolated to its own section instead of blanking the entire page. This skill exists because "we have an error boundary" is frequently true and still insufficient: a single app-root boundary, a fallback that leaks `error.stack` to end users, or a caught error that is never logged are each individually shippable-looking and each individually a defect. This skill treats failure isolation as a design surface with its own placement rules, accessibility requirements, and observability contract — not an afterthought bolted on after an incident.

## When to use

Use this skill when the user asks to:

- review a new feature's error-handling design before it ships,
- investigate a production incident where a non-critical widget's error blanked an entire page or unrelated flow,
- audit an existing application for missing error boundaries around suspending, async, or third-party-embedded components,
- review fallback UI copy, accessibility, or recovery affordances for an error state.

Do not use this skill for:

- diagnosing the specific root cause of a hydration-mismatch error — that is `ssr-hydration-streaming-diagnosis`, though the two are frequently reviewed together since every suspending read needs a paired error boundary,
- reviewing the shape or contract of API error responses from the server — that is `api-integration-contract-review`.

## Context7 Documentation Protocol

- Resolve `/reactjs/react.dev` before approving any specific error-boundary implementation. React does not currently offer a function-component error-boundary primitive — `static getDerivedStateFromError` and `componentDidCatch` are class-component-only lifecycle methods, and React's own docs point to the community-maintained `react-error-boundary` package for teams that want a component API without hand-writing the class boilerplate. Recommending a hook-based error boundary (`useErrorBoundary` as a from-scratch primitive, for example) as if React ships one natively is a version-sensitive mistake; verify before approving.
- Call `query-docs` scoped to the confirmed React major version for "error boundary" and, if the tree also suspends via `use()` or `Suspense`, for "use hook error boundary" — a rejected Promise from `use()` propagates to the nearest Error Boundary, not the nearest Suspense fallback, so a suspending read wrapped only in `<Suspense>` with no ancestor Error Boundary crashes to unhandled on rejection.
- If the codebase uses `react-error-boundary`, confirm the installed version's API surface (`fallback` vs `fallbackRender` vs `FallbackComponent`, `resetKeys`, `onReset`) via `query-docs` before approving a specific prop pattern — this library's API has shifted across major versions.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label every claim `documentation-based, unverified against current release`.

## Lean operating rules

- One error boundary at the app root is not "handled" — it is a single point of total failure with narrower blast radius than no boundary at all, but it still takes down the whole page for any descendant error. Require granular, per-independently-failable-section boundaries around anything non-critical (a recommendations panel, an ad slot, a third-party embed) so its failure cannot take the checkout form or the primary content with it.
- Every suspending or async-rejectable read (`use()` on a Promise, a lazy-loaded component via `React.lazy`, a third-party SDK embed that can throw) needs a paired ancestor Error Boundary. A Suspense boundary alone only handles the pending state, not rejection — treat a suspending read with no ancestor Error Boundary as a crash-to-blank defect, not a style preference.
- Fallback UI must never render raw `error.message` or `error.stack` to end users in production. That is an information-disclosure defect, not a copy nitpick — internal implementation detail (stack traces, library names, internal endpoint paths) belongs in server-side/observability logs, never in the DOM a user or attacker can read.
- A caught error that is never logged anywhere is a worse outcome than an uncaught one: the uncaught error is at least visible in a crash report or user complaint, while a silently-swallowed caught error erodes observability into real failure rates with no signal. `componentDidCatch` (or the `onError`/`onReset` hook equivalent in `react-error-boundary`) must always forward to logging/observability, even when the UI gracefully degrades.
- Fallback UI is not just a decision-tree checkbox — it must be perceivable by assistive technology. Apply the ARIA APG alert pattern (`role="alert"` or an `aria-live` region) so screen-reader users are notified of the failure, not left staring at a silently-changed DOM region.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only) — review from the component tree, existing boundary placement, and any incident report or fallback UI source the user provides.

## References

Load these only when needed:

- [Boundary placement and granularity](references/boundary-placement-and-granularity.md) — use when mapping the component tree to confirm every suspending/async/third-party component has an ancestor error boundary, and when granularity is too coarse (app-root-only).
- [Fallback UX and accessibility](references/fallback-ux-and-accessibility.md) — use when reviewing fallback UI copy, information-disclosure risk, accessible alert semantics, recovery affordances, or layout-shift impact of a fallback swap.
- [Observability and recovery](references/observability-and-recovery.md) — use when confirming caught errors are still logged, and when reviewing reset/retry design (`resetKeys`, retry affordances) so a boundary doesn't stay permanently tripped.

## Response minimum

Return, at minimum:

- the error-boundary placement map (component → boundary → fallback UX → logging hook) or the specific gap found,
- evidence level (`documentation-based`, `repo evidence`, `user-provided evidence`, or `inference`) and what Context7 query grounded the claim,
- whether fallback UI leaks raw error detail and whether it uses accessible alert semantics,
- confirmation that caught errors are still logged to observability, or explicit flag if not,
- verdict: approve / approve-with-notes / block.
