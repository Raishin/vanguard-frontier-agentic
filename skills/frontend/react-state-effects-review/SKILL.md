---
name: react-state-effects-review
description: Statically review useState/useEffect/useReducer call sites against React's documented "You Might Not Need an Effect" anti-pattern catalog, plus race-condition and stale-closure detection via dependency-array and cleanup-function analysis, producing ranked file:line findings.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# React State & Effects Review

## Purpose

Review `useState`, `useEffect`, and `useReducer` call sites for the specific defect classes React's own documentation calls out — effects that should be render-time computations or event-handler logic, missing cleanup/cancellation guards on async effects, and stale closures caused by dependency-array mistakes — without re-litigating component decomposition, styling, or live performance profiling in every response. This skill exists so those adjacent concerns stay out of scope and the review stays focused on the documented anti-pattern catalog plus race-condition and stale-closure evidence.

## When to use

Use this skill when the user asks to:

- review a component that uses `useEffect`, `useState`, or `useReducer` before merge,
- diagnose a bug consistent with a race condition (stale data flashes onto the screen, results from a previous request overwrite a later one),
- diagnose an "infinite render loop" or "effect fires too often" report,
- decide whether a given effect is necessary at all.

Do not use this skill for:

- class-component lifecycle methods (`componentDidMount`, `componentDidUpdate`, `componentWillUnmount`) — different API surface; use a general React review instead,
- a bug that requires live reproduction (browser DevTools profiling, network-tab timing) to confirm — hand off to a runtime tool; static analysis can only identify the missing guard, not prove the bug fired in production,
- component decomposition, prop-interface, or context-usage review — that is `react-component-architecture-review`.

## Context7 Documentation Protocol

- Resolve the React library ID with `resolve-library-id` (matched result: `/reactjs/react.dev`) before labeling any specific pattern as an anti-pattern.
- Before citing "You Might Not Need an Effect" guidance, call `query-docs` for the exact pattern in question (e.g., "adjusting state on prop change", "resetting state with a key") — do not assert the anti-pattern classification from memory, and do not assume every effect matches the catalog without checking which entry actually applies.
- Read `package.json` first to confirm the installed React major version. `useEffectEvent` (stable in React 19.2) is the documented fix for a specific class of stale-closure problem; if the repo is on an older major, that specific fix is unavailable and the finding must recommend the pre-19.2 pattern (ref/callback ref, or accepting the value as a dependency) instead.
- If the repo uses the React Compiler, verify current compiler-specific guidance via `query-docs` before assuming manual dependency-array reasoning is unchanged — do not assume compiler-driven memoization rules automatically apply to hand-authored effects.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- First read `package.json` to confirm the installed React major and whether `useEffectEvent` is available before recommending it as a fix.
- For every effect in scope, classify it before evaluating it: (a) synchronizing with an external system (valid effect usage per docs), (b) adjusting/resetting state in response to props or other state with no external system involved (the documented anti-pattern — compute during render or reset via `key` instead), or (c) event-handler logic misplaced in an effect (should run in the handler that caused it, not react to a state change). Do not apply one verdict to all effects uniformly.
- For every async effect that sets state from a promise/callback result, require an explicit cancellation guard (an `ignore` flag checked before the state update, or an `AbortController`). Its absence is a race-condition finding, not a style note — describe the concrete input sequence that triggers stale data (e.g., "slow request for user A resolves after the fast request for user B, overwriting B's data with A's").
- For every dependency array, check each value read inside the effect body against the array. A missing dependency is only a non-issue when the omitted value is provably stable (a `setState` function, a `dispatch` function, a `ref.current` read inside the effect, or a value wrapped in `useEffectEvent` where the repo's React version supports it) — otherwise it is a stale-closure finding.
- Do not fabricate a race condition or stale-closure claim without describing the exact trigger sequence (what user action, what timing, what state transition). A finding that only says "this could be a race condition" without the sequence is not a valid finding.
- Treat effects that intentionally run once for genuine one-time external-system setup (subscribing to a widget, initializing a non-React library) as valid; do not flag the empty dependency array itself as a defect — flag only if the effect body also reads reactive values it omits.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Flag any effect that performs an authenticated write or other state-mutating network call without an idempotency guard or cancellation guard as a HIGH-severity finding — a race condition here risks duplicate financial or state-changing operations, not just a UI glitch.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the anti-pattern decision tree, and the required output shape.
- [Effect cleanup and race conditions](references/effect-cleanup-and-race-conditions.md) — load only when reviewing an async effect (data fetching, subscriptions, timers) for cancellation/cleanup correctness.
- [Stale closures and dependency arrays](references/stale-closures-and-dependency-arrays.md) — load only when a dependency-array omission or stale-closure suspicion is present.

## Response minimum

Return, at minimum:

- the component(s), files, and specific hook call sites in scope,
- ranked findings with file:line evidence, anti-pattern category (per the docs catalog), and a concrete fix sketch matching the docs' recommended alternative,
- for every race-condition finding, the concrete trigger sequence and the missing guard,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "requires live reproduction to confirm timing" for suspected but unconfirmed races).
