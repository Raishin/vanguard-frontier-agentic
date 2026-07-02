# Client-Store Topology and Re-Renders

Use this reference only when reviewing store-slice design, selector shape, `useShallow` usage, or a reported re-render-cascade / "typing lags" complaint tied to a client store. Grounded against Zustand (`/pmndrs/zustand`) — verify version-sensitive import paths and defaults via Context7 before asserting them as the reviewed repo's behavior.

## What people get wrong

The naive story is:

> I put everything in one global store because it's easier than deciding what goes where.

That is not a topology decision — it is the absence of one, and it has a specific, predictable cost: every component that reads *any* field from that store re-renders on *every* write to that store unless each consumer's selector is narrow enough to opt out. A single monolithic store with wide, whole-store selectors (`const state = useStore()`) is the most common root cause of "every keystroke re-renders half the tree" complaints — not React itself, not the framework, the subscription shape.

The second common wrong assumption:

> Memoizing the component (`React.memo`) will fix the re-render cascade.

Often wrong, or incomplete. `React.memo` prevents a re-render caused by a *parent* re-rendering with the same props; it does nothing for a re-render caused by the component's *own* subscription to a store firing because some unrelated field in that store changed. If the component calls `useStore((s) => s)` or otherwise subscribes to the whole store, `React.memo` will not help — the selector itself is the leak.

## Officially grounded shape

Per current Zustand docs (verify against installed major via Context7):

- **Selectors control subscription granularity.** `useStore((state) => state.field)` subscribes only to `field`; the component re-renders only when the *selector's return value* changes (by default, via `Object.is` reference equality), not on every store write.
- **Object/array selectors need shallow comparison.** A selector that returns a freshly-constructed object or array on every call — `useStore((state) => ({ nuts: state.nuts, honey: state.honey }))` — produces a new reference every render even when `nuts` and `honey` are unchanged, defeating the point of selecting narrowly. The documented fix is `useShallow` from `zustand/react/shallow` (or `zustand/react` depending on installed version — confirm the current import path via Context7 before citing it), which memoizes the selector so the component only re-renders when the shallow-compared output actually changes.
- **Selecting the whole store defeats granularity entirely.** `const state = useStore()` (no selector function) subscribes the component to every field; this is the store-topology equivalent of the server-state "no query key scoping" mistake — it works, until it is the reason everything re-renders.
- **`persist` middleware controls what is written to storage, not what triggers re-renders.** Do not conflate persistence configuration with subscription/selector configuration; they solve different problems and a fix for one does not address the other.

## Non-negotiable design rules

### 1. Slice the store by consumer lifecycle and update frequency, not by feature name alone

Fields that change on every keystroke (a search-box draft value) and fields that change rarely (a user's theme preference) do not belong in the same store if components reading the rare field would otherwise re-render on every keystroke via a wide selector. Prefer multiple narrow stores or well-separated slices over one store with defensive selectors bolted on afterward.

### 2. Every consumer selects only the fields it reads

A component that reads `user.name` should select `(state) => state.user.name`, not `(state) => state.user` (unless it genuinely reads the whole `user` object) and never `(state) => state` (unless it is a debug/devtools consumer). Grep for `useStore(state => state)` or a bare `useStore()` call with no selector as a first-pass smell.

### 3. Object/array-returning selectors require `useShallow` or an equivalent stable-reference strategy

A selector returning an inline object or array literal without `useShallow` is a re-render-cascade finding, not a style nit — it produces a new reference on every store update regardless of whether the selected values changed, defeating the selector's purpose. Confirm the current `useShallow` import path via Context7 before recommending the fix, since it has moved across major versions.

### 4. Do not accept a re-render fix without profiler evidence

Per `SKILL.md`: a memoization, selector-narrowing, or store-splitting change proposed as the fix for a reported "typing lags" or "re-renders too much" complaint must be accompanied by a before/after render count or flame-graph excerpt. "This should reduce re-renders" is a hypothesis; treat it as `inference`-level evidence until measured, and say so explicitly rather than approving it as verified.

### 5. Server-owned data does not belong in a client store's slice design at all

If the classification step (see `workflow-and-output.md`) placed an entity in a client store but it is actually server state, the fix is reclassification and migration to a query/cache library — not a more clever selector. Do not let a well-designed selector distract from the fact that the entity is in the wrong system entirely.

## Minimal safe implementation flow

1. Classify the entity as client state (see `workflow-and-output.md` decision tree) — confirm it is not actually server state wearing a client-store costume.
2. Decide which store/slice it belongs in based on update frequency and consumer lifecycle, not just feature grouping.
3. For each consumer, write the narrowest selector that returns exactly the fields it reads.
4. If the selector returns an object or array literal, wrap it in `useShallow` (confirm current import path via Context7) or restructure to select primitives individually.
5. If a re-render complaint motivated the review, capture profiler evidence before and after the change to confirm the fix actually reduced render count.

## High-risk assumptions to kill

- "One global store is simpler" — simpler to set up, not simpler to reason about once selectors are wide and re-renders cascade.
- "`React.memo` will fix it" — does not help when the component's own store subscription is the source of the re-render.
- "Selecting an object is fine, it's just one extra render" — one extra render per keystroke, multiplied across every consumer with the same pattern, is the actual complaint being diagnosed.
- "We'll add `useShallow` later if it becomes a problem" — retrofitting selector discipline after a store has many wide-selector consumers is materially harder than establishing the pattern from the first consumer.
- "Increasing `staleTime` will fix the re-render complaint" — `staleTime` is a TanStack Query caching concept and has no effect on Zustand (or any client store) subscription/re-render behavior; do not let a caching-policy fix stand in for a selector fix. See `server-state-caching-and-invalidation.md` for the caching side of this distinction.

## Safe verification targets

- Grep for `useStore(` (or the project's actual store hook name) call sites with no selector argument, or with `state => state` — whole-store subscriptions.
- Grep for selector bodies returning an object or array literal (`=> ({`, `=> [`) and check whether `useShallow` wraps the call.
- Confirm the `useShallow` import path (`zustand/react/shallow` vs `zustand/react`) matches the installed Zustand major version.
- For a reported re-render complaint, ask for or request a profiler render count/flame-graph excerpt scoped to the affected component before accepting a selector or memoization fix as verified.

## When to push back

Push back if the user asks for:

- one monolithic store for the whole app "to keep things simple,"
- a memoization or selector change presented as a confirmed fix with no profiler evidence,
- a bare whole-store selector "because we need most of the fields anyway" without checking whether that is actually true per consumer.

A store topology that is easy to set up and expensive to render is not a good trade — it is deferred cost, and it lands on every future consumer of that store, not just the one being reviewed.
