# Reactivity Boundaries: ref, reactive, computed, and Interop

Use this reference only when reviewing `ref()`/`reactive()`/`computed()` usage, a suspected destructuring-loses-reactivity bug, or `toRef()`/`toRefs()`/`unref()` interop code.

## What people get wrong

The common bad assumption is:

> "`reactive()` and `ref()` are interchangeable; I can destructure either one and the result stays reactive."

That is wrong for `reactive()`. Vue's `reactive()` returns a Proxy that tracks property access *through the proxy itself*. Destructuring a primitive-valued property off that proxy — `const { count } = state` — copies the current primitive value out; the local `count` variable is a plain number/string with no connection back to `state.count`. Further mutations to `state.count` will not update `count`, and Vue's own docs document this as the reason `reactive()`'s usefulness is "limited" for values that need to be passed around or destructured. `ref()` does not have this problem: a `ref` is an object with a `.value` property, so extracting it (or wrapping a `reactive()` object's properties in `toRefs()` first) preserves the reactive connection because you are copying a reference to the ref object, not a snapshot of its value.

## Officially grounded shape

- `ref(initialValue)` returns a ref object with a `.value` accessor; reading/writing `.value` is tracked/triggers updates. In `<script setup>` and templates, refs are auto-unwrapped (no `.value` needed in the template).
- `reactive(obj)` returns a Proxy wrapping `obj`; property access/mutation through the proxy is tracked. Destructuring a primitive property off the proxy loses the reactive connection for that property. Destructuring a nested object/array property does *not* lose reactivity for further mutations on that nested value, because the extracted reference still points at a proxied (or proxy-wrappable) object — the pitfall is specifically about primitive values.
- `toRefs(reactiveObj)` converts every property of a reactive object into an individual ref, returning a plain object of refs. Destructuring the result of `toRefs()` is safe — each destructured value is a ref, not a primitive snapshot.
- `toRef(reactiveObj, key)` creates a single ref synced to one property of a reactive object (or normalizes a plain value/getter into a ref-like object) — the documented escape hatch for passing one reactive property into a function that expects a ref, without wrapping the whole object.
- `unref(refOrValue)` returns `.value` if the argument is a ref, or the argument itself otherwise — the documented escape hatch for composable code that accepts either refs or plain values as arguments.
- `computed(getterOrOptions)` returns a read-only ref by default when given a getter function. Given `{ get, set }`, it returns a writable ref-like computed — the documented pattern for two-way-bindable derived state (e.g., a computed backing a form field that needs to write back to a different source property).

## Non-negotiable design rules

1. **A composable that expects its return value to be destructured by consumers must return refs, not a plain destructurable `reactive()` object.** Return individual refs, or build internal state with `reactive()` and return `toRefs(state)` at the composable's boundary. This is the single most common Composition API defect class and the one most likely to produce a "state doesn't update in the UI" bug report that is expensive to trace back to its source.
2. **`reactive()` is correct, not wrong, when the composable's own internal state is never destructured** — only accessed via dot-notation (`state.count`) inside the composable itself, or passed whole to a template/child without destructuring. Do not flag every `reactive()` call site as a defect; check whether the specific value is actually destructured downstream first.
3. **`computed()` getters must be pure.** No state mutation, no async calls, no network writes inside a getter — Vue may re-run a computed's getter an unpredictable number of times as part of dependency tracking, so any side effect there is nondeterministic and can produce inconsistent state or infinite recomputation loops. A `computed({ get, set })` writable computed's `set` function is not subject to this rule — writing back to source state is its documented purpose.
4. **`toRef()`/`unref()` are documented interop escape hatches, not bugs.** Do not flag their presence as a finding by default — verify instead that they are being used for their documented purpose (normalizing composable arguments, exposing a single reactive property to a child) rather than papering over a reactivity-loss bug found elsewhere.
5. **Shallow variants (`shallowRef`, `shallowReactive`) intentionally do not track nested mutations.** Do not flag a nested-property mutation as "not reactive" without first checking whether the top-level ref/reactive was declared shallow on purpose (e.g., for a large object where deep reactivity is a documented performance cost the team opted out of).

## Verification targets

When repo evidence is available, verify each finding against:

- the composable's actual return statement — is it `return { ...state }` (reactive spread, still loses reactivity on destructure), `return toRefs(state)` (safe), or `return { countRef, doubledRef }` (safe, individual refs)?
- the consumer call site — is it `const { count } = useCounter()` (destructure — check the composable's return shape) or `const counter = useCounter(); counter.count` (property access — reactivity-loss risk does not apply regardless of return shape)?
- whether a flagged `computed()` side effect is actually reachable during normal getter evaluation, or only inside the `set` function of a writable computed (not a finding).

## When to push back

Push back if the user asks to:

- convert a composable's return value from `reactive()`/`toRefs()` to a plain destructured object "for cleaner syntax" — that reintroduces the exact reactivity-loss pitfall this reference exists to catch,
- add a `watch()` inside a `computed()` getter to "fix" an impurity finding — that does not make the getter pure, it adds a second reactive side channel; the fix is to move the side effect out of the computed entirely,
- flag `shallowRef`/`shallowReactive` usage as a bug without first confirming whether the shallow choice was intentional (e.g., checking for an accompanying comment, a large-list-performance context, or explicit `triggerRef()` calls that indicate deliberate manual reactivity control).
