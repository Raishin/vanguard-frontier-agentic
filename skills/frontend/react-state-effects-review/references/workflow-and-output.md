# Review Workflow and Findings Contract

Use this reference for the step-by-step procedure, the anti-pattern decision tree, and the required output shape. Load the other two references (`effect-cleanup-and-race-conditions.md`, `stale-closures-and-dependency-arrays.md`) only when the step below tells you to.

## Workflow

1. **Inventory.** List every `useState`, `useEffect`, `useReducer`, and `useLayoutEffect` call site in scope, with file:line. Do not skip `useLayoutEffect` — it is not interchangeable with `useEffect`; it exists specifically for layout measurement that must happen before the browser paints, so a "convert to useEffect" recommendation is wrong unless you have confirmed the effect does not read layout geometry.
2. **Classify each effect** using the decision tree below. This step decides which reference (if any) to load next.
3. **For effects classified as "synchronizing with an external system" that are async** (fetch, subscription callback, timer), load `effect-cleanup-and-race-conditions.md` and verify a cancellation guard exists.
4. **For any effect with an object, function, or derived-value dependency**, or any effect where a value used in the body is missing from the dependency array, load `stale-closures-and-dependency-arrays.md` and verify the omission is provably safe.
5. **Produce ranked findings** using the output contract below. Rank HIGH (race condition on authenticated writes, or a confirmed anti-pattern causing an infinite loop) above MEDIUM (documented anti-pattern with no loop/data-corruption risk) above LOW (style/clarity).

## Anti-pattern decision tree

For each effect, ask in order:

1. **Does this effect synchronize with something outside React** (a DOM API, a browser API like the window title, a subscription to a non-React widget, a network connection, `localStorage`)?
   - **Yes** → this is valid effect usage per `synchronizing-with-effects`. Proceed to step 2 below (async/cleanup check) if it is asynchronous. Do not flag the effect's existence as a defect.
   - **No** → continue to question 2.
2. **Does the effect only call `setState` with a value derived from props or other state, with no external system involved?**
   - **Yes, and the derived value can be computed inline during render** → this is the documented "Adjusting state on prop change in an Effect" or "Adjusting some state when a prop changes" anti-pattern. Fix: compute the value during render instead of storing it in a second `useState` synchronized by an effect. If the current render's derived value depends on the *previous* render's props (e.g., resetting a selection when a list changes), prefer storing an `ID` and comparing it inline during render over an effect.
   - **Yes, and the goal is resetting all state when an identity-like prop changes** (e.g., `userId`, `itemId`) → this is the documented "Resetting state with an Effect" anti-pattern. Fix: pass the identity value as the `key` prop on the component (or a wrapped inner component) so React remounts and resets state automatically, instead of an effect that calls `setState(initialValue)`.
   - **No** → continue to question 3.
3. **Does the effect run logic that should only happen in response to a specific user action** (a click, a form submission) rather than in response to a state/prop change?
   - **Yes** → this is the documented "event-handler logic misplaced in an effect" anti-pattern (e.g., sending an analytics event or a mutation inside an effect that fires whenever some state changes, instead of inside the handler that caused the change). Fix: move the logic into the event handler. If the same logic must run regardless of which handler triggered the state change, that is one of the few legitimate reasons to keep it in an effect — but confirm that is actually true before accepting it as an exception.
   - **No** → continue to question 4.
4. **Does the effect exist solely to run initialization logic once on mount** (e.g., `loadDataFromLocalStorage()`, `checkAuthToken()`) at the top level of `App`?
   - **Yes** → flag it. In development with `StrictMode`, this class of effect runs twice, which is documented as an intentional signal that the logic was not designed to be resilient to remount. Fix depends on the actual intent: module-level code that should run once per page load (not per component mount) usually does not belong in an effect at all; if it must be effect-based and idempotency truly cannot be achieved, a `useRef` guard (`if (ranOnce.current) return; ranOnce.current = true;`) is the documented workaround, not a first choice.

Do not stop at the first "yes" in isolation without checking file:line evidence — cite the exact code that establishes the classification.

## Output contract

Every response must return:

1. **Scope** — the component(s), files, and hook call sites reviewed (file:line for each).
2. **Evidence level** — per finding: `repo evidence` (cited file:line), `documentation-based` (cites the specific react.dev page/section via Context7), or `inference` (a plausible but unconfirmed race/loop that needs live reproduction to prove).
3. **Ranked findings** — for each: anti-pattern category (one of the four decision-tree branches, or "missing cleanup guard", or "stale closure"), file:line, the concrete trigger sequence for any race/loop claim, and a fix sketch that matches the docs' recommended alternative (not a generic "add a check" — name the actual pattern: compute during render, reset via `key`, move to handler, add `ignore` flag/`AbortController`, add missing dependency, or `useEffectEvent` if the React version supports it).
4. **Verdict** — `approve`, `approve-with-notes`, or `block`. Block only for HIGH-severity findings (confirmed infinite loop, or a race condition on an authenticated write/mutation).
5. **Open questions** — anything requiring live reproduction (network timing, StrictMode remount behavior in the target environment) that static review cannot confirm; state this explicitly rather than asserting a bug "will occur" without evidence of the trigger sequence.

## Hard stops

- Do not recommend removing an effect that genuinely synchronizes with an external system. That is valid effect usage per docs, not an anti-pattern — removing it breaks the synchronization.
- Do not flag every `useEffect` call site as suspicious by default. Each finding must map to a specific decision-tree branch with file:line evidence.
- Do not claim a race condition "will occur" without describing the concrete input sequence (which two operations race, in what order, and what the observable symptom is).
