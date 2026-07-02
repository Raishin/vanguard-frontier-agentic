# linkedSignal and DI-boundary patterns

> Load this reference only when `linkedSignal` appears in the diff under review, or when the review scope includes service/DI ownership review.

## What people get wrong

The naive story is:

> `linkedSignal` is just `computed` with a different name; if it appears in a diff, check it the same way you'd check `computed` for purity.

Wrong. `linkedSignal` exists for a distinct problem `computed` cannot solve: **dependent state that must stay valid when its source changes, but can also be manually overridden**. Angular's own guidance frames this as ensuring "the `linkedSignal` always maintains a valid value, even when its underlying dependencies change" — while still allowing `.set()` calls that a plain `computed()` (read-only) would reject. Flagging a `linkedSignal` as "should be a `computed`" without checking whether manual override is a real requirement produces a false-positive finding.

The second naive story, on the DI side:

> A service exists, so as long as it's `providedIn: 'root'`, the DI design is fine.

Wrong. `providedIn: 'root'` is Angular's documented default for singleton services (per Angular's own style guide: "Design services around a single responsibility. Use the `providedIn: 'root'` option for singleton services"), but blast-radius and ownership are separate questions from injection scope. A root-provided service holding cross-cutting mutable state with no documented ownership model (who writes, who reads, what the lifetime and reset story is) is a design gap regardless of whether the DI scope itself is "correct."

## `linkedSignal` review rules

- **Confirm the source relationship.** `linkedSignal(() => source())` (shorthand form) or `linkedSignal({ source: ..., computation: ... })` (object form, which also grants access to the previous value) should have a clear, traceable source signal. Flag a `linkedSignal` with no discernible source dependency as unclear intent.
- **Check for a genuine override requirement.** If nothing in the component ever calls `.set()` on the `linkedSignal`, it is not using the "dependent but overridable" capability and should likely be a plain `computed()` instead — recommend the simpler primitive.
- **Object form for prior-value access.** The object form (`{ source, computation: (sourceValue, previous) => ... }`) exists specifically for cases that need the prior computed/linked value (e.g. preserving a chat history while new content streams in, or converting a domain model to a form model only once a resource loads, per Angular's own AI/forms design-pattern guidance). Do not flag this as unnecessary complexity without checking whether prior-value access is actually used.
- **Do not conflate with `effect`-based derivation.** A `linkedSignal` reset-on-source-change pattern is the documented, correct tool for "resettable derived state" — it is not a workaround for avoiding `effect()`; do not suggest replacing it with an `effect()` that writes to a `signal()`, which reintroduces the state-propagation-via-effect anti-pattern this skill flags elsewhere.

## DI/service-boundary review rules

- **Single responsibility per service.** Per Angular's own style guide, a service should be designed around a single responsibility. A service accumulating multiple unrelated concerns (e.g. auth state + feature-flag cache + analytics buffer in one class) is a boundary finding — recommend splitting along responsibility lines, not along "make it smaller" alone.
- **`inject()` over constructor injection for new/modified code.** Per the official Angular `angular-developer` skill and Angular's current style guidance, new code should prefer the `inject()` function over constructor-parameter injection — note this as a MEDIUM style finding when reviewing new service code that still uses constructor injection, not as a blocking defect for existing code that hasn't been touched.
- **Ownership model for mutable state.** For any injectable service exposing mutable `signal()` state, confirm the review can answer: who is allowed to call `.set()`/`.update()` on it, what triggers a reset, and what the lifetime is (root-singleton for app-lifetime state vs. component-provided for a narrower scope). If the code gives no evidence of an ownership model — e.g. the mutable signal is exposed as a public writable `Signal` rather than behind a method that encapsulates the mutation — flag it as a LOW/MEDIUM boundary gap and request either encapsulation (expose a read-only `Signal` via `.asReadonly()` and a controlled setter method) or explicit documentation, rather than prescribing a specific DI scope.
- **Do not prescribe DI scope changes speculatively.** Do not recommend moving a service from `providedIn: 'root'` to a component-level provider (or vice versa) unless the code in scope shows evidence of the actual lifetime/sharing requirement (e.g. multiple independent component instances each needing isolated state currently share one root singleton). A DI-scope change is a behavioral change, not a style preference.

## When to still flag regardless of `linkedSignal`/DI context

- Any `computed()` purity violation, missing injection-context on an `effect()`, or untracked post-await signal read found incidentally while reviewing `linkedSignal`/DI code — those checks always apply per the main workflow.
- A hardcoded API key, token, or secret found in service state or a `linkedSignal` default/computation — HIGH severity, immediate escalation, regardless of scope.

## When to push back

Push back if the user asks to:

- replace a `linkedSignal` that has a real manual-override requirement with a plain `computed()` "to simplify" — that removes a capability the component actually needs, it doesn't simplify anything,
- move a service to `providedIn: 'root'` or split it into a component-provided instance without evidence of the actual lifetime/sharing requirement — that is a behavioral change being requested as if it were a style fix,
- expose a service's internal mutable signal directly (without `.asReadonly()` or an encapsulating method) "to save a line" — that removes the ownership boundary the review is meant to protect.

That is not simplification. It is removing the guardrail the finding exists to establish.
