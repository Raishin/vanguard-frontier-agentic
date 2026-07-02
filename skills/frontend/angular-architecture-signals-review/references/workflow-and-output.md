# Review workflow and findings contract

Use this reference for the full Signals-architecture review procedure and the required output shape.

## What people get wrong

The naive story is:

> `effect()` is the reactive hook, so use it whenever a value should update when something else changes. `computed()` is just a lighter-weight version of the same idea.

Wrong. Angular's own guidance draws a hard line: `computed()` is for **derived state** — it is lazily evaluated, memoized, and must be pure. `effect()` is for **syncing signal state to non-reactive, imperative APIs** (logging, `window.localStorage`, custom DOM behavior, third-party UI library integration) — not for propagating state changes between signals. Angular's docs are explicit that using effects to propagate state changes "can lead to `ExpressionChangedAfterItHasBeenChecked` errors, infinite circular updates, or unnecessary change detection cycles." Treating `effect()` as a generic "run this when that changes" tool, rather than checking whether the callback is actually a side effect on a non-reactive API, is the single most common Signals-architecture defect.

The second most common mistake: assuming every component that hasn't adopted `ChangeDetectionStrategy.OnPush` is behind on best practice. Signals do not require OnPush to function correctly — they improve the precision of change detection either way. Recommending OnPush without checking whether the component relies on implicit mutation-based update paths (rather than signal reads, `async` pipe, or explicit `markForCheck()`) produces broken advice, not an optimization.

## Workflow

1. **Confirm the Angular major version**
   - Read `package.json` for the installed `@angular/core` version before evaluating any Signals-API claim (`linkedSignal`, `afterRenderEffect`, `@Service` decorator, `provideZonelessChangeDetection`) — these landed at different points across v16–v22.

2. **Classify each reactive primitive in scope**
   - `signal(...)` — mutable state.
   - `computed(...)` — derived, read-only, must be pure, lazily evaluated and memoized per Angular's docs.
   - `effect(...)` — side effect on a non-reactive API; requires an injection context.
   - `linkedSignal(...)` — dependent, resettable derived state that can also be manually `.set()` (see `references/linked-signal-and-di-boundaries.md`).

3. **Check `computed()` purity**
   - Read every `computed()` callback. Flag any signal write, DOM mutation, HTTP call, or logging call inside it as a HIGH-severity purity violation — computed callbacks may re-run any number of times (or not at all, if never read) and must not have observable side effects.

4. **Check whether each `effect()` is a genuine side effect**
   - Ask: does this callback interact with a non-reactive, imperative API (DOM, storage, third-party library, logging)? If yes, it's a legitimate use.
   - If the callback only computes a value and writes it into another signal with no other side effect, it is state propagation through an effect — recommend `computed()` (pure derivation) or `linkedSignal()` (derived-but-resettable state) instead, citing the specific Angular guidance against using effects for state propagation.
   - Confirm the `effect()` call site has an injection context (component/directive/service constructor, or an explicit `Injector` in options). Flag a missing injection context as a correctness bug.
   - Check for signal reads after an `await` inside the effect callback — the reactive context does not survive an async boundary, so those reads are untracked and the effect will not re-run when they change. Flag this as a correctness bug, not a style note.

5. **Check change-detection strategy**
   - Note whether each component in scope declares `changeDetection: ChangeDetectionStrategy.OnPush`.
   - If a Signals-adopting component is still on the default strategy, check whether it relies on implicit mutation-based updates (direct property mutation observed via Zone.js, not signal reads or `async` pipe) that OnPush would silently break.
   - If no such reliance is found, flag as a MEDIUM missed-performance-opportunity finding — not a defect.

6. **Check service/DI boundaries**
   - For services holding cross-cutting mutable state, confirm there is a documented (or at least inferable-from-code) ownership model: who writes, who reads, what the lifetime is.
   - Do not prescribe a specific DI scope (`providedIn: 'root'` vs component-provided) without evidence of the actual sharing/lifetime requirement — see `references/linked-signal-and-di-boundaries.md` when this is in scope.

7. **Produce ranked findings**
   - Order by blast radius: correctness bugs (purity violations, missing injection context, untracked post-await reads, leaked secrets) first, then reactive-graph design defects (effect-as-computed misuse), then lower-severity notes (missed OnPush opportunity, DI-ownership documentation gaps).

## Decision tree

- `computed()` callback has **any side effect** (signal write, DOM mutation, HTTP call) → HIGH finding: purity-contract violation. Fix: move the side effect into an `effect()`, keep `computed()` pure.
- `effect()` callback **only derives and stores** a value with no genuine side effect (no DOM/logging/storage/third-party sync) → MEDIUM finding: recommend `computed()` (if the value doesn't need manual override) or `linkedSignal()` (if it does).
- `effect()` **reads a signal after an `await`** → HIGH finding: untracked dependency, effect will not re-run correctly.
- `effect()` call site has **no resolvable injection context** → HIGH finding: correctness bug (Angular requires an injection context or explicit `Injector`).
- Component uses **signal-based state but Default change-detection strategy**, and does **not** rely on implicit mutation-based updates → MEDIUM finding: missed OnPush opportunity, not a block.
- Component uses **signal-based state but Default strategy**, and **does** rely on implicit mutation-based updates that OnPush would break → do not recommend OnPush without also flagging the mutation-based update paths that must be migrated first; treat as a scoped follow-up, not an inline fix.
- Service holds cross-cutting mutable state with **no ownership model evident from the code** → LOW/MEDIUM finding: request explicit ownership documentation; do not prescribe a DI scope change without more evidence.

## Output contract

Return:

1. Component(s)/service(s)/files in scope
2. Ranked findings, each with:
   - file:line evidence
   - primitive-misuse category (purity violation / derivation-via-effect / injection-context error / untracked-post-await read / missed-OnPush-opportunity / DI-ownership gap)
   - concrete fix sketch (e.g. "move to `computed()`", "add `Injector` option", "confirm no implicit-mutation reliance before adding OnPush")
   - severity (HIGH / MEDIUM / LOW)
   - evidence level (`repo evidence`, `documentation-based`, `inference`)
3. Verdict: approve / approve-with-notes / block
4. Open questions or explicitly out-of-scope items (e.g. zoneless migration recommendation excluded by design, unconfirmed Angular major version, SSR/hydration concerns deferred to `angular-ssr-hydration-review`)

## Validation gates

- Every Signals-semantics claim cites the version-matched Angular docs (via Context7 `query-docs` against the confirmed major, or `metadata.json` `official_docs` with an "unverified against current release" label if Context7 is unavailable).
- Every "`computed()` must be pure" finding shows the specific side effect present in the callback — no bare assertion.
- No finding recommends a blanket OnPush migration across the whole app in one review — scope findings to the files actually under review.
- No finding recommends removing Zone.js or adopting `provideZonelessChangeDetection` — that is out of scope by design.

## Common failure modes

- Treating every `effect()` as wrong. `effect()` is correct for genuine side effects (logging, DOM sync, `localStorage` writes, third-party UI library integration) — Angular's docs list these as the intended use cases.
- Missing that `linkedSignal` is intentionally different from `computed` — it exists specifically for resettable derived state that can also be manually overridden (e.g. a selected shipping option that resets when the options list changes but can also be user-set); do not flag `linkedSignal` usage as "should be `computed`" without checking whether manual override is a real requirement.
- Assuming OnPush is always strictly better without checking for other change-detection triggers the component relies on — `async` pipe subscriptions and signal reads still work under OnPush, but manual property mutation observed only via Zone.js does not trigger a check under OnPush.
- Recommending a DI-scope change (root vs component-provided) as a drive-by note without evidence of the actual lifetime/sharing requirement.

## Adversarial checklist

Before finalizing a finding, answer these:

- Does any `computed()` callback perform a side effect (write, log, HTTP call, DOM mutation)?
- Is an `effect()` actually necessary, or does it just compute-and-store a value that `computed()` (or `linkedSignal()`) should own?
- Is the Signals-API claim (e.g. `linkedSignal` availability, `@Service` decorator) checked against this repo's actual Angular version, not assumed from the latest docs?
- Would switching this component to OnPush break any implicit mutation-based update path it currently relies on?
- Does the `effect()` read any signal after an `await`, silently breaking its reactive tracking?

If any answer is "not sure," lower the finding's confidence and label the evidence level accordingly — do not present it as a confirmed defect.
