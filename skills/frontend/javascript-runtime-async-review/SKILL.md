---
name: javascript-runtime-async-review
description: Review JavaScript/TypeScript for event-loop and microtask/macrotask ordering correctness, unhandled Promise rejection paths, DOM event-listener and timer cleanup, and race-condition risk in rapid-repeated-async UI patterns, tracing actual browser scheduling semantics rather than assumed synchronous-style reasoning about async code.
allowed-tools: Read Grep Glob Bash(git diff:*) WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: delivery
---

# JavaScript Runtime & Async Correctness Review

## Purpose

`async`/`await` reads like synchronous code, which leads developers to reason about it as if it were synchronous — but the underlying microtask/macrotask scheduling model still governs actual execution order, and getting it wrong produces exactly the class of bug that's hardest to catch in normal testing: intermittent, timing-dependent, "works on my machine" race conditions. This skill traces real event-loop ordering, audits every async chain for unhandled-rejection paths, and verifies every listener/timer has a reachable cleanup path, so timing correctness is verified rather than assumed.

## When to use

Use this skill when the user asks to:

- review JavaScript/TypeScript async code (Promises, async/await, timers) for correctness,
- audit event-listener, timer, or observer cleanup to prevent memory leaks,
- diagnose or prevent race conditions in UI patterns with rapid repeated async calls (search-as-you-type, polling, infinite scroll),
- check for unhandled Promise rejections, especially on security-relevant code paths,
- verify actual microtask/macrotask execution order for a specific code sequence.

Do not use this skill for:

- framework-specific effect/hook lifecycle review (React `useEffect` dependency arrays, Vue watchers) — use the matching framework-specific review skill; this skill covers the underlying runtime/scheduling layer those skills build on,
- bundler/build-tool configuration or transpilation-target correctness — out of scope,
- a claim that requires live production traffic or load-test evidence to confirm timing under real network jitter — this is static-review-only; label those findings as needing live verification, do not assert them as proven.

## Context7 Documentation Protocol

- Resolve the docs source with `resolve-library-id` against `/mdn/content` (or the closest current MDN Context7 ID) before asserting any ordering, scheduling, or API-behavior claim.
- Before ruling on a specific ordering question (e.g., "does this `setTimeout(fn, 0)` run before or after this `.then()`"), call `query-docs` for that exact scenario — do not answer from memory or from a synchronous mental model, since ordering intuitions are a frequent source of subtly-wrong review comments.
- Trace microtask-vs-task distinctions explicitly: microtasks (Promise callbacks, `queueMicrotask`, `async`/`await` continuations) drain completely — including microtasks they themselves enqueue — before the next macrotask (`setTimeout`, `setInterval`, I/O, UI rendering) runs. Cite this distinction rather than assuming the reader already applies it correctly.
- For cancellation/lifecycle patterns (`AbortController`, `removeEventListener`), verify current API shape and browser support notes via `query-docs` before recommending a specific signature — do not invent options or assume universal support without checking.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- Do not assume `async`/`await` makes code race-condition-safe by default — it only guarantees ordering within a single linear await chain, not between independently-triggered async operations.
- Trace the actual microtask/macrotask interleaving for any ordering claim rather than asserting it from a synchronous mental model; query current MDN event-loop/microtask docs before ruling, since this is a frequent source of subtly-wrong assumptions.
- Every Promise chain must terminate in a `.catch` or sit inside a try/catch around every `await` — flag any chain that doesn't as an unhandled-rejection risk, with extra weight if it touches an authorization/permission check.
- Every `addEventListener`/`setInterval`/non-one-shot `setTimeout`/`Observer` must be matched to a traced, reachable `removeEventListener`/`clearInterval`/`clearTimeout`/`disconnect` call, including on early-return and error paths.
- Any UI pattern with rapid repeated async calls (search-as-you-type, polling) must use `AbortController`, a request-generation counter, or equivalent sequencing — flag its absence as a race-condition risk, not a style preference.
- Do not accept "passed manual testing" as sufficient evidence for timing-sensitive code — manual testing rarely hits the interleavings that matter; flag as needing live/load-tested verification instead.
- Flag `eval`, `new Function()`, and string-argument timers as code-injection risks requiring explicit justification, not routine patterns.
- Label every ordering/timing claim as `spec-traced`, `documentation-based`, or `needs live-runtime verification` so reviewers know what's actually been verified.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only, plus `git diff` for scoping and `WebFetch` for spec/docs grounding).

## References

Load these only when needed:

- [Event-loop ordering trace patterns](references/event-loop-tracing.md) — use when tracing the actual execution order of a specific microtask/macrotask/await sequence in review.
- [Unhandled-rejection audit checklist](references/rejection-audit.md) — use when systematically auditing a file/module's Promise chains for missing `.catch`/try-catch coverage.
- [Race-condition and cancellation patterns](references/race-condition-patterns.md) — use when reviewing rapid-repeated-async UI code for AbortController, generation-counter, or equivalent sequencing needs.

## Response minimum

Return, at minimum:

- the event-loop/ordering verdict for each async chain reviewed, with the traced resolution order (not assumed),
- the unhandled-rejection audit result for every Promise chain in scope,
- the listener/timer/observer cleanup audit result, matching every registration to its cleanup call,
- race-condition risk flags for any rapid-repeated-call pattern lacking sequencing or cancellation,
- residual risk notes for anything requiring live or load-tested verification beyond this static trace.
