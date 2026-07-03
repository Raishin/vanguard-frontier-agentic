# INP Phase Decomposition

Use this reference when the regression or complaint is about interaction responsiveness — a page "feels laggy," a PSI/CrUX INP score drop, or a Search Console page-experience INP warning.

## What people get wrong

The naive story is:

> "INP is high, so the JavaScript bundle is too big — code-split it."

Sometimes true, usually incomplete. INP measures the full latency of the interaction that produced the worst (or a high-percentile) responsiveness value in a session, per the W3C Event Timing spec. Bundle size affects one of three sub-phases; the other two are frequently the actual bottleneck and a bundle-size fix does nothing for them.

## The three documented sub-phases

Per web.dev's INP guidance, decompose the flagged interaction into:

1. **Input delay** — from the user's physical input (click/tap/keypress) to the start of the event handler(s) processing it. Grown by: the main thread being busy with an unrelated task (a long task from a timer, a fetch-then-render cycle, a third-party script) when the input arrives.
2. **Processing time** — the actual execution time of the event handler(s) triggered by the input, including any synchronous work chained inside it (state updates, synchronous re-renders, layout-triggering DOM reads/writes).
3. **Presentation delay** — from processing completion to the next frame actually being presented, including style/layout/paint work and any work the browser defers to the next task (e.g. `requestAnimationFrame` callbacks).

Every INP finding in this skill's output must name which of these three phases is dominant, tied to a specific long-task or event-handler entry — not just restate the total INP number.

## Framework-specific attribution — verify via Context7, do not assume

Resolve the framework via Context7 (see SKILL.md's Context7 Documentation Protocol) before attributing processing-time cost to a specific rendering mechanism. Confirmed via Context7 against `/reactjs/react.dev`:

- React's `useTransition`/`startTransition` exists specifically to move non-urgent state updates off the synchronous, blocking render path so urgent updates (the actual click/keypress response) aren't queued behind them — per React's own framing, transitions "will be interrupted if more urgent updates like clicks or key presses come in." If processing-time is dominated by a large synchronous re-render triggered by an interaction, and part of that re-render is not required to satisfy the immediate visual feedback (e.g. re-rendering a large list after a filter change), that is the documented mechanism to reach for — verify the installed React major supports it (React 18+) before recommending it.
- Do not claim a synchronous `setState` inside an event handler is inherently the INP cause without a long-task/processing-time artifact — React batches many such updates, and the actual cost may be in downstream layout/paint work (presentation delay), not the state update itself.
- For any other framework in scope (Vue, Svelte, Angular), do not assume an analogous "transition" primitive exists or behaves the same way — query Context7 for that framework's current concurrency/scheduling primitives before recommending one.

## Root-cause attribution — require an artifact, not an assertion

For each phase you name as dominant, require one of:

- A `PerformanceEventTiming` entry (per the W3C Event Timing spec: `processingStart`, `processingEnd`, `startTime`, `duration`) from field RUM instrumentation (the `web-vitals` library's INP attribution build, or equivalent).
- A long-task entry (`PerformanceLongTaskTiming`) overlapping the input-delay or presentation-delay window, showing what else the main thread was doing.
- A DevTools/Lighthouse trace showing the specific event handler's call stack and duration.

If none of these are available, the finding is capped at `inference` and must say so.

## Minimal targeted fix per phase (and who owns it)

- **Input-delay-dominant** → identify and break up or defer the unrelated main-thread work that was running when input arrived (a competing long task, an eagerly-executing third-party script, an unbatched synchronous fetch handler). If the cause is JS parse/execution weight from the initial bundle, hand off to `bundle-budget-code-splitting-review`.
- **Processing-time-dominant** → reduce the synchronous work inside the handler itself: defer non-urgent state updates (verify current framework primitive via Context7), memoize expensive derived computation, move genuinely heavy computation off the main thread (a Web Worker) rather than accepting it as unavoidable.
- **Presentation-delay-dominant** → reduce layout/paint cost triggered by the update (avoid forced synchronous layout / layout thrashing, reduce the DOM subtree size affected by the update, check for expensive CSS selectors or filters/shadows recalculated on every frame).

## Verification target

Re-run field-representative interaction traces (Lighthouse's user-flow / timespan mode against the specific interaction, or a `web-vitals` INP attribution build in a staging RUM collector) and diff the three-phase breakdown against the pre-regression trace, not just the top-line INP number. See `references/evidence-tiers-and-handoff.md` for the field-data confirmation window before declaring the metric fixed.
