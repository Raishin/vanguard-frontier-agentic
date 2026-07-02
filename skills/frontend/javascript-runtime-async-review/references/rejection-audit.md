# Unhandled-Rejection Audit Checklist

Use this reference only when systematically auditing a file/module's Promise chains and `async` functions for missing `.catch`/try-catch coverage — not for tracing execution order (`event-loop-tracing.md`) or for cancellation/sequencing patterns (`race-condition-patterns.md`).

## What people get wrong

The common bad assumption is:

> "The function is `async`, so any error inside it becomes a normal thrown error the caller will see."

That is incomplete. An `async` function that throws or whose `await`ed Promise rejects returns a **rejected Promise**, not a synchronous throw. If nothing consumes that rejection — no `.catch`, no surrounding `try`/`catch` at an `await` call site, no `await` at all on a fire-and-forget call — the rejection surfaces only as an "Unhandled promise rejection" warning (or, in older/misconfigured environments, silently), and any logic gated on that path (an authorization check, a save confirmation, a UI state update) simply never runs. A rejected permission check that isn't awaited/caught can fail open: the calling code proceeds past the check as if nothing happened, because nothing ever observed the rejection.

## Officially grounded pattern

MDN's Promise/`await` docs converge on one rule: every Promise-producing expression needs exactly one of these along every path that can reject:

- a `.catch(...)` (or `.then(onFulfilled, onRejected)`) attached to the chain, or
- an enclosing `try { await ... } catch (e) { ... }` around the `await`.

```js
// Unhandled: no .catch, and the caller does not await it.
function saveDraft(draft) {
  api.save(draft).then((res) => showSavedToast(res));
}

// Handled: explicit .catch covers the whole chain.
function saveDraft(draft) {
  api
    .save(draft)
    .then((res) => showSavedToast(res))
    .catch((err) => showErrorToast(err));
}

// Handled: try/catch around the await.
async function saveDraft(draft) {
  try {
    const res = await api.save(draft);
    showSavedToast(res);
  } catch (err) {
    showErrorToast(err);
  }
}
```

## Non-negotiable design rules

1. **Every Promise chain must terminate in exactly one rejection-handling construct along every branch.** A `.then(...).then(...)` chain with no trailing `.catch` is a finding even if an earlier link in the chain "usually" succeeds — "usually" is not "always," and the whole point of the audit is the failure path.
2. **A `try` block around a call that does *not* `await` its Promise-returning expression does not catch that Promise's rejection.** `try { somePromiseFn(); } catch (e) {}` only catches a *synchronous* throw during the call setup, not an asynchronous rejection from the returned Promise. Flag this exact pattern by name — it is a common false sense of coverage.
3. **A "fire-and-forget" call (`doSomethingAsync()` with no `await`, no `.then`, no `.catch`) is an unhandled-rejection risk by default.** It is only acceptable when the function is documented/verified to never reject (rare) or when the codebase has a deliberate, reviewed pattern for intentionally-detached tasks (e.g., a wrapper that logs-and-swallows). Do not accept "it's just a fire-and-forget analytics call" as an exemption without checking whether the underlying call can actually reject (network calls almost always can).
4. **`Promise.all`/`Promise.allSettled`/`Promise.race`/`Promise.any` each have different rejection semantics — verify which one is in use before asserting the rejection is handled.** `Promise.all` rejects as soon as any input rejects (and the other results are dropped, not just delayed); `Promise.allSettled` never rejects and instead requires the caller to inspect each `status: "rejected"` result individually — a `.catch` on an `.allSettled` chain never fires for individual-item failures, so per-item error handling must happen inside the mapping/consuming code, not assumed to exist because a `.catch` is present elsewhere in the chain.
5. **Authorization/permission-check code paths get elevated severity, not routine severity, for a missing rejection handler.** If a rejected check silently fails to block the gated action (because nothing awaited/caught it and the surrounding code proceeds unconditionally), that is a fail-open security defect, not a UX polish item.

## Severity escalation

Treat a missing rejection handler as **HIGH severity** when:

- the Promise chain gates an authorization, permission, or entitlement check — a rejection that isn't observed means the gate is bypassed by default (fail-open),
- the Promise chain performs an authenticated write/mutation and the caller has no way to know it failed — silent data loss or an inconsistent state the user believes succeeded,
- the rejection would otherwise crash a Node.js process (unhandled rejections terminate the process by default in modern Node major versions) — verify the target runtime's current behavior via `query-docs` rather than assuming a specific Node version's default.

Otherwise (a rejection that only affects a non-critical UI affordance, like a tooltip's optional prefetch), MEDIUM or LOW is appropriate depending on user-visible impact — but still a finding, not a non-issue.

## Verification targets

When repo evidence is available, verify each finding against:

- whether a global handler exists (`window.addEventListener("unhandledrejection", ...)` in browsers, `process.on("unhandledRejection", ...)` in Node) — a global handler changes the blast radius (it prevents a silent failure) but does **not** fix the underlying logic gap (the gated action still ran unguarded); note both facts, do not treat the global handler as sufficient remediation for a specific chain,
- whether the function is exported/public API — an unhandled rejection inside a shared utility has a larger blast radius than one confined to a single component's internal helper,
- the actual runtime target (browser vs. Node, and which Node major) before asserting process-crash consequences, since default unhandled-rejection behavior has changed across Node versions.

## When to push back

Push back if the user asks you to:

- add a blanket top-level `unhandledrejection`/`unhandledRejection` listener as the fix instead of adding scoped `.catch`/`try-catch` at the actual call sites — a global listener can suppress the crash/warning without fixing the fail-open logic gap underneath it,
- treat `.catch(() => {})` (swallow-and-ignore) as equivalent to proper error handling for an authorization or mutation path — silently swallowing the error is a different (and often worse) defect than the original unhandled rejection, because it removes the warning signal without adding a safe fallback,
- skip the audit on a chain because "it basically never fails in practice" — that is exactly the assumption this audit exists to challenge; require either evidence the operation cannot reject, or a rejection handler.
