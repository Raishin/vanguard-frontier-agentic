# Review workflow and findings contract

Use this reference for the full component-architecture review procedure and the required output shape.

## What people get wrong

The naive story is:

> Split anything that "feels big," lift state to the top, and reach for a context or a store whenever a prop is passed more than once.

Wrong. Splitting a component without reducing coupling just moves the same coupling into a new file. Lifting state past the actual point of shared ownership creates unnecessary re-renders and unrelated components that now depend on state they don't own. Reaching for context or a store before checking whether the "problem" is actually decomposition treats a structural defect as a tooling gap.

React's own guidance (`Thinking in React`, `Sharing State Between Components`, `Passing Data Deeply with Context`) is explicit that component boundaries should follow **separation of concerns** — a component should ideally be concerned with one thing — and that context is a last resort after prop passing becomes genuinely inconvenient across a large distance, not a default.

## Workflow

1. **Classify each component in scope**
   - Presentational (renders UI from props, no data-fetch/business-logic)
   - Container (owns data-fetch, business-logic, or both, delegates rendering)
   - Compound (exposes a composite API — e.g. `<Tabs><Tabs.Panel/></Tabs>` — where prop/context threading is intentional)

2. **Count responsibilities per component**
   - Data-fetching, business-logic, layout/presentation, event-handling are the four responsibility buckets.
   - A component mixing 3 or more buckets is a decomposition candidate — but only escalate to a "split" recommendation per the decision tree below, not automatically.

3. **Trace prop chains**
   - Follow each prop 3+ levels deep. Note every intermediate component that receives the prop only to pass it through without consuming it.
   - Cross-check against the compound-component reference before flagging: an intentional compound API often has legitimate multi-level prop/context threading.

4. **Check context usage**
   - For each `useContext` / `Context.Provider` (or `<Context value={...}>` in newer React majors — verify against the repo's confirmed version), determine how often the provided value changes.
   - Flag only when a frequently-changing value is provided broadly and re-renders an oversized subtree, or when context is used in place of a 1–2 level prop pass that would be simpler and more traceable.

5. **Check hook-rule compliance**
   - Hooks must be called unconditionally at the top level of a function component or custom hook — never inside conditions, loops, nested functions, early returns, or try/catch blocks (per React's Rules of Hooks). Flag any violation as HIGH severity; it is a correctness bug, not a style preference.

6. **Produce ranked findings**
   - Order by blast radius: correctness bugs (hook-rule violations, leaked secrets) first, then testability/maintainability defects (mixed-responsibility components, real prop drilling), then lower-severity notes (naming, minor duplication).

## Decision tree

- Component has **more than 1 unrelated responsibility** AND is **reused in more than 1 place** → recommend split, with a sketch of the extracted boundary.
- Prop drilling exceeds **3 levels** AND the prop is passed through **more than 2 non-consuming intermediates** → recommend context or composition (children/slots) per React's own composition guidance — do not default to recommending a global store.
- A single context value **changes frequently** and **re-renders a large subtree** → recommend splitting the context (e.g., separate state and dispatch contexts) or memoizing consumers, citing the specific re-render evidence available (repo evidence of consumer count, or `inference, not measured` if no profiler data exists).
- Deep prop/context threading matches a **compound-component or render-prop API** already in use elsewhere in the codebase → do not flag; note it as intentional.

## Output contract

Return:

1. Component(s)/files in scope
2. Ranked findings, each with:
   - file:line evidence
   - responsibility-mix or drilling-depth evidence backing the finding
   - concrete refactor sketch
   - severity (HIGH / MEDIUM / LOW)
   - evidence level (`repo evidence`, `documentation-based`, `inference`)
3. Verdict: approve / approve-with-notes / block
4. Open questions or explicitly out-of-scope items (e.g. "requires a dedicated refactor plan" cap when more than 5 components would need rewriting, or missing live re-render data)

## Validation gates

- Every architectural claim traces to a specific file:line.
- Every "React recommends X" statement is backed by a Context7-queried doc citation matched to the repo's confirmed React version — never asserted from memory.
- No finding recommends a specific state-management library unless one is already present in the repo's dependencies.
- No finding proceeds to recommend a rewrite of more than 5 components in one review; flag as "requires a dedicated refactor plan" instead and stop there.

## Common failure modes

- Treating every context usage as bad. Context is appropriate for rarely-changing, broadly-needed values like theme, auth, or locale — that is the documented use case, not an anti-pattern.
- Recommending Redux/Zustand/Jotai when the actual problem is component decomposition, not state-management tooling.
- Flagging deep prop chains that are actually intentional render-prop or compound-component APIs as if they were accidental drilling.
- Estimating re-render frequency or count without a live profiler and presenting it as measured fact instead of `inference, not measured`.

## Adversarial checklist

Before finalizing a finding, answer these:

- Would splitting this component actually reduce coupling, or just move the same coupling into a new file?
- Is the prop-drilling flagged actually a compound-component API (intentional) rather than an architectural smell?
- Does the reviewer have repo evidence this component re-renders excessively, or is that assumed?
- Is the Context7 citation matched to the repo's actual React version, not the latest docs by default?

If any answer is "not sure," lower the finding's confidence and label the evidence level accordingly — do not present it as a confirmed defect.
