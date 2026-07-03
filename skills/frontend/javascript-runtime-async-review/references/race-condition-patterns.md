# Race-Condition and Cancellation Patterns

Use this reference only when reviewing a UI pattern with rapid repeated async calls (search-as-you-type, polling, infinite scroll, tab-switch-triggered refetch, debounced/throttled handlers) for `AbortController`, generation-counter, or equivalent sequencing needs — not for general rejection-handling audit (`rejection-audit.md`) or pure ordering questions with no repeated-call risk (`event-loop-tracing.md`).

## What people get wrong

The common bad assumption is:

> "The requests are sent in order, so the responses will arrive and get applied in order too."

That is false, and it is the single most common source of "stale data flashes onto the screen" bug reports. Nothing about the network or the event loop guarantees that responses resolve in request order — a slower request issued first can resolve *after* a faster request issued second, and if both unconditionally call the same `setState`/DOM-write on resolution, the stale (first-issued, slower) response can overwrite the fresh (second-issued, faster) one. This is not a rare edge case; it is a predictable consequence of variable network latency and is trivially reproducible by throttling one request in DevTools.

## Officially grounded cancellation pattern (MDN)

`AbortController`/`AbortSignal` is the documented mechanism for cancelling `fetch()` and for auto-removing event listeners, and it is the primary tool for closing this class of race condition — it actually cancels the in-flight request/listener rather than merely ignoring its eventual result.

Cancelling `fetch()`:

```js
const controller = new AbortController();

async function search(query) {
  controller.abort(); // cancel any prior in-flight search
  const signal = controller.signal;
  try {
    const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`, { signal });
    const results = await response.json();
    renderResults(results);
  } catch (err) {
    if (err.name === "AbortError") return; // expected: a newer request superseded this one
    renderError(err);
  }
}
```

Note the bug in the naive version of this pattern: reusing a single `controller` across calls means `controller.abort()` must create a **new** controller for the next request (an already-aborted controller's signal cannot be un-aborted). The corrected shape keeps the controller reference in an outer/module/component-instance scope and reassigns it on every new call:

```js
let currentController = null;

async function search(query) {
  currentController?.abort();
  currentController = new AbortController();
  const { signal } = currentController;
  try {
    const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`, { signal });
    renderResults(await response.json());
  } catch (err) {
    if (err.name === "AbortError") return;
    renderError(err);
  }
}
```

Auto-removing an event listener with the same signal, instead of a manual `removeEventListener` call (MDN, `EventTarget/addEventListener`):

```js
const controller = new AbortController();
el.addEventListener("click", handler, { signal: controller.signal });
// Later, one call removes this (and any other listener sharing the signal):
controller.abort();
```

## Non-negotiable design rules

1. **Every rapid-repeated-call site needs one of: `AbortController` cancellation, a request-generation counter, or a documented equivalent (e.g., a library-level cancellation/dedup mechanism like a query library's built-in request deduplication).** Absence of any of these on a search-as-you-type, poll, or tab-switch-refetch pattern is a race-condition finding, not a style note — regardless of how unlikely the reviewer judges the specific timing to be in practice.
2. **A generation-counter guard must increment *before* issuing the new request and must be checked *immediately before* the state-mutating call on resolution**, comparing against the value captured at issue-time — not the current value read again at resolution-time (that comparison is always true and guards nothing):
   ```js
   let latestRequestId = 0;
   async function search(query) {
     const requestId = ++latestRequestId;
     const results = await fetchResults(query);
     if (requestId !== latestRequestId) return; // a newer request superseded this one
     renderResults(results);
   }
   ```
3. **`AbortController.abort()` rejects the pending `fetch()` Promise with an `AbortError`** — the catch/rejection path must explicitly distinguish `AbortError` (expected, safe to silently return) from every other error (must still surface to the user or logging). Treat a catch block that swallows *all* errors identically, including genuine network/server failures, as a separate finding from the missing-cancellation finding — silencing real errors alongside expected aborts hides genuine outages.
4. **Debounce/throttle alone does not fix this class of race.** Debouncing reduces how many requests are *issued*; it does not guarantee the *responses* resolve in issue order. A debounced search-as-you-type handler still needs cancellation or a generation guard on the requests it does issue.
5. **Polling intervals need the same cancellation discipline as one-shot requests**, plus `clearInterval`/`clearTimeout` on unmount/teardown — an in-flight poll response arriving after teardown that still writes to now-stale UI state or a detached DOM node is the same defect class as the search-as-you-type case, compounded by the interval continuing to fire if not cleared.
6. **Session/identity switches (logout, account switch, tab becomes a different user's session) are the highest-severity variant of this pattern.** A stale in-flight request for the previous session resolving after the switch and writing its response into the new session's view is a data-exposure defect (one user's data rendered into another user's session), not merely a UI glitch — treat this specific trigger sequence as HIGH severity by default.

## Verification targets

When repo evidence is available, verify a race-condition finding by:

- confirming the actual trigger sequence with concrete inputs (e.g., "type 'a', then quickly type 'ab' — if the `/search?q=a` response is slower than `/search?q=ab`, it can overwrite the correct results" ) rather than asserting a race "could" happen without describing how,
- checking whether the underlying HTTP client actually supports `signal` (native `fetch` does; some wrapped/legacy clients require an adapter or don't support cancellation at all — verify before recommending `AbortController` as the fix, and recommend a generation-counter guard instead when the client can't cancel),
- checking whether a `try`/`catch` around an aborted request correctly special-cases `AbortError` (rule 3 above) — a fix that adds cancellation but treats the resulting `AbortError` as a real failure introduces a new (spurious error UI) bug.

## When to push back

Push back if the user asks you to:

- "just debounce it more" as the fix for a reported stale-data race — longer debounce reduces frequency, not the underlying unordered-resolution risk, and adds latency without closing the bug,
- skip cancellation because "the backend is fast, this basically never happens" — network latency variance (not backend speed alone) drives this bug, and it is exactly the kind of intermittent, hard-to-reproduce defect that's expensive once it reaches production; treat "basically never happens" as unverified until shown otherwise,
- add a generation counter or `AbortController` but continue writing state before checking the guard — the guard must be the last check *before* the state-mutating call, not merely present somewhere earlier in the function.
