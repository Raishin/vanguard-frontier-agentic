# Composable and Script-Setup Conventions

Use this reference only when reviewing composable naming/structure, lifecycle-hook registration timing, or `<script setup>` responsibility mixing and extraction candidates.

## What people get wrong

The common bad assumption is:

> "As long as the code works when I test it manually, where I call `onMounted()` or another composable inside a composable doesn't matter."

That is wrong. Vue's Composition API relies on an internal "current active instance" pointer that is only valid during the synchronous execution of `setup()` (or the top-level, non-awaited body of `<script setup>`). Lifecycle-hook registration functions (`onMounted`, `onUnmounted`, `onUpdated`, etc.) and dependency-injection functions (`provide`, `inject`) read that pointer at call time. If the call happens after an `await`, inside a `setTimeout` callback, inside a conditional branch that runs later, or inside an event-handler function that executes after setup has already returned, the active-instance pointer is no longer set (or points at the wrong instance) and the call either silently no-ops or attaches to the wrong component. This frequently "works" in manual testing because the developer doesn't notice the hook never actually fired, and only surfaces as a subtle bug (a cleanup that never runs, a mount hook that fires late or never) later.

## Officially grounded conventions

- **Naming:** composable functions are named with a `use` prefix (`useMouse`, `useFetch`, `useCounter`) by convention — this signals to readers and tooling (including ESLint plugin rules for Composition API) that the function may call other composition APIs (refs, lifecycle hooks, provide/inject) and has rules-of-hooks-like constraints.
- **Input/output convention:** composables typically accept plain values, refs, or getters as input (using `unref()`/`toValue()`-style normalization internally) and return an object of refs (or individually-returned refs) so that consumers can destructure safely — see `references/reactivity-boundaries.md` for the mechanics.
- **Synchronous-registration rule:** lifecycle hooks and `provide`/`inject` must be called synchronously within the composable's/component's initial setup execution. Async work belongs *inside* the hook's callback body (e.g., `onMounted(async () => { await fetchData() })` is correct; `await fetchData(); onMounted(() => {})` is not, because the `onMounted` call itself has been pushed past the synchronous setup window).
- **`<script setup>` is sugar over `setup()`:** every rule that applies to composable calls inside `setup()` applies identically inside the top-level (non-nested, non-async-continuation) body of a `<script setup>` block.

## Non-negotiable design rules

1. **Every lifecycle-hook call and every nested composable call must be reachable synchronously from the top of `setup()`/`<script setup>` execution.** No exceptions for "it's inside a function that always runs during setup anyway" — if that function is itself invoked asynchronously or conditionally, the guarantee breaks. Trace the actual call chain; do not assume synchronicity from proximity in the source file.
2. **A composable's `use*` name must reflect that it may call other Composition APIs.** A plain utility function (no refs, no lifecycle hooks, no other composable calls) does not need the `use` prefix and naming it that way is misleading, not a review-blocking defect — note it, do not escalate it.
3. **Async initialization belongs inside the async operation's own effect (a lifecycle hook body, a `watch`/`watchEffect` callback, or an event handler), never mixed into the composable's own top-level synchronous body in a way that races with hook registration.** If a composable needs to both register a lifecycle hook and kick off an async fetch, register the hook first (synchronously), and start the fetch inside (or after) that synchronous registration completes — not interleaved with it in a way that risks the hook call landing after an early `return`/`await`.

## `<script setup>` organization review

Flag a component for composable extraction when it mixes **three or more** of the following concerns inline, with no existing extraction, **and** the mixed logic has genuine reuse or testability value (not just "this file is long"):

- data-fetching (an inline `fetch`/HTTP client call plus loading/error state),
- validation or business-rule logic (non-trivial conditional/derived logic beyond simple template formatting),
- presentation-only local state (toggle flags, form-field bindings) — this one alone is normal and not a concern by itself,
- non-trivial event-handling logic (more than a one-line dispatch to a store/composable).

When flagging, name the concrete extraction: what the new composable should own (e.g., "extract the fetch + loading/error state into `useOrderHistory(customerId)`"), not a vague "consider extracting some logic." Do not flag a component that has one or two local `ref()`s and a single `fetch` call with no independent business logic — that is normal `<script setup>` usage, not a mixed-concern smell.

## Verification targets

When repo evidence is available, verify each finding against:

- the actual call stack leading to a hook registration — is `onMounted(...)` reachable synchronously from the top of `setup()`, or is it inside a `.then()`, an `async function` body before any `await`... after any `await`, a `v-if`-guarded code path, or a callback passed to `setTimeout`/an event listener?
- whether the same fetch/business logic already appears in another component in the codebase (strengthens an extraction-candidate finding — it demonstrates real duplication, not just theoretical reuse potential).

## When to push back

Push back if the user asks to:

- "just wrap the whole thing in `nextTick().then()`" to make an async-timing bug with a lifecycle hook go away — that changes when the code runs but does not fix the underlying synchronous-registration violation; the hook registration itself must move, not the work around it,
- extract every local `ref()` in a component into a composable regardless of reuse potential — that adds indirection without reducing duplication or improving testability, which contradicts the actual goal of composable extraction,
- rename a plain non-reactive utility function to start with `use` "for consistency" — the `use` prefix is a signal about Composition API usage, not a general naming convention, and misapplying it misleads future readers about the function's constraints.
