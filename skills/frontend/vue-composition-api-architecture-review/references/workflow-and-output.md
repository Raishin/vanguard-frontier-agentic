# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific concern the composable or component under review actually raises.

## Prerequisites

- Confirm Vue 3.x is in use (`package.json` — `vue: ^3.x`). Composition API, `<script setup>`, and the composable conventions this skill reviews against are Vue 3 features; a Vue 2 codebase with no stated migration plan is out of scope.
- Identify every composable (`use*`-prefixed function) in the diff or review scope, and every component file that consumes one.

## Workflow

1. **Enumerate composables in scope.** For each, read its full body and its return statement.
2. **Classify the return shape.** Does it return a plain object built from `reactive()`, individual `ref()`s, a `toRefs()`-wrapped reactive object, or a mix? This classification drives the reactivity-boundary review — see `references/reactivity-boundaries.md`.
3. **Check consumer call sites.** For each place a composable's return value is consumed, check whether the consumer destructures it. Cross-reference the destructuring pattern against the return-shape classification from step 2.
4. **Check lifecycle-hook and nested-composable call timing.** Verify every `onMounted`/`onUnmounted`/`watch`/`provide`/nested composable call happens synchronously in the initial execution of `setup()` or the `<script setup>` block — not after an `await`, not inside a conditional/loop, not inside an event handler defined during setup and invoked later.
5. **Check `computed()` purity.** Verify every `computed()` getter is a pure read. If a `computed({ get, set })` writable-computed pair is present, verify the `set` function's role (writing back to source state) is not mistaken for an impurity.
6. **Review `<script setup>` organization.** For each component, count distinct concerns handled inline (data-fetching, validation, business-rule logic, presentation-only local state, event handling). Flag extraction candidates per the decision tree below.
7. **Produce ranked findings** using the output contract below.

## Decision tree

- Composable returns a plain `reactive()` object **and** is destructured by a consumer → **HIGH** finding (documented reactivity-loss pitfall). Fix: return `toRefs(state)` or return individual refs from the composable instead of a bare reactive object.
- Composable calls a lifecycle hook (`onMounted`, etc.) inside an async function, after an `await`, inside a conditional, or inside a callback registered for later invocation → **HIGH** finding (violates Vue's synchronous-registration requirement — the hook silently fails to bind to the intended component instance). Fix: call the hook synchronously in the composable's/component's top-level setup execution; move async work inside the hook's own callback body instead.
- `<script setup>` component mixes 3+ distinct concerns (e.g., fetch + validation + layout + event handling) inline with no composable extraction, and the mixed logic has genuine reuse or testability value → **MEDIUM** finding, recommend composable extraction with a concrete split (name the composable, name what it should own).
- `computed()` getter performs a side effect (mutates state, fires a network call, logs with side effects) → **MEDIUM-to-HIGH** finding depending on blast radius (see escalation note below). Fix: move the side effect into a `watch`/`watchEffect` or an explicit method; keep the getter pure.
- Composable returns a plain `reactive()` object and is **not** destructured (consumed only via property access, e.g., `state.count`) → not a finding. `reactive()` is correct and idiomatic when consumers do not destructure.
- `toRef()`/`unref()` used to interop between a prop and a local composable, or to normalize a `ref | value` argument → not a finding; these are the documented escape hatches, not bugs.

## Severity note on computed side effects

Treat an impure `computed()` getter as **HIGH** severity when the side effect is a network write, a mutation of state outside the computed's own dependency graph, or a mutation that can re-trigger the same computed's re-evaluation (infinite-loop risk). Otherwise (a benign console log with no state mutation), **MEDIUM** is appropriate — still a documented anti-pattern, since `computed()` getters are expected to be re-run an unpredictable number of times by Vue's reactivity system, but not a data-integrity risk.

## Output contract

Every response from this skill must return:

1. **Scope** — the composable(s) and/or component(s), files, and specific call sites reviewed.
2. **Ranked findings** — each with file:line, defect category (reactivity-loss / lifecycle-timing / computed-impurity / extraction-candidate), the concrete evidence (the exact destructuring line, the exact async boundary crossed, or the exact mixed-concern list), and a fix sketch matching Vue's documented pattern.
3. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`.
4. **Verdict** — approve / approve-with-notes / block.
5. **Open questions or out-of-scope items** — e.g., "requires Vue Devtools reactivity inspection to confirm the reported UI-not-updating bug traces to this destructuring site," or "auth-token handling in `useSession` is out of scope — recommend vue-ssr-security-review."

## When to push back

Push back if the user asks to:

- treat every `reactive()` usage in the codebase as a bug to convert to `ref()` — `reactive()` is correct and idiomatic for state that is never destructured by consumers; a blanket conversion is unnecessary churn, not a fix,
- adopt Pinia or Vuex as part of this review's recommendation when neither is already a project dependency — that is a state-management architecture decision outside a composable/reactivity review's scope,
- migrate an Options API component to Composition API as a review finding when the PR itself does not already touch Composition API elsewhere — that is a repo-wide migration decision, not a PR-level fix.
