# Effect Cleanup and Race Conditions

Use this reference only when the effect under review is asynchronous — it fetches data, opens a connection, subscribes to a stream, or sets a timer — and you need to verify the cleanup/cancellation guard is correct.

## What people get wrong

The common bad assumption is:

> "The effect updates state when the promise resolves, so it's correct."

That is incomplete. React does not cancel in-flight promises when an effect re-runs or the component unmounts. If a slow request from an earlier render resolves *after* a faster request from a later render, the stale result can overwrite the fresh one unless the effect explicitly guards against it. This is the documented root cause of "data flashes to a previous value when switching quickly."

## Officially grounded pattern

React's own docs (`you-might-not-need-an-effect.md`, `synchronizing-with-effects.md`, and the `useEffect` reference) all converge on the same shape for fetch effects: a boolean `ignore` flag set in the cleanup function, checked before every state update derived from the async result.

```js
function SearchResults({ query }) {
  const [results, setResults] = useState([]);
  const [page, setPage] = useState(1);
  useEffect(() => {
    let ignore = false;
    fetchResults(query, page).then(json => {
      if (!ignore) {
        setResults(json);
      }
    });
    return () => {
      ignore = true;
    };
  }, [query, page]);
  // ...
}
```

The same pattern applies with `async`/`await` syntax — declare `ignore` outside the async function, check it after the `await`, and return the cleanup function that flips it:

```js
useEffect(() => {
  let ignore = false;

  async function startFetching() {
    const json = await fetchTodos(userId);
    if (!ignore) {
      setTodos(json);
    }
  }

  startFetching();

  return () => {
    ignore = true;
  };
}, [userId]);
```

`AbortController` is an equally valid, and more resource-efficient, variant when the underlying request API supports cancellation (most `fetch`-based clients do) — it actually cancels the network request instead of only ignoring its result. Prefer it when the codebase already has an established `AbortController` convention; do not introduce it as a novel pattern in a codebase that consistently uses the `ignore`-flag convention without a reason tied to the specific finding (e.g., an expensive request that should genuinely be aborted, not just ignored).

## Non-negotiable design rules

1. **Every async effect that calls `setState` from a resolved promise or callback must have a guard.** No exceptions for "it's unlikely to race in practice" — the trigger sequence (fast navigation, fast re-render, flaky network) is exactly the kind of intermittent, hard-to-reproduce condition that is expensive to debug once it reaches production. Treat the absence of a guard as a finding regardless of how unlikely the reviewer judges the race to be.
2. **The guard must be checked immediately before every state update inside the async chain**, not just the first one. An effect that fetches and then makes a second dependent call must check `ignore` (or `signal.aborted`) before each `setState`, not only the first.
3. **`StrictMode` double-invocation in development is not the same bug as a production race condition.** `StrictMode` runs setup → cleanup → setup once, synchronously, specifically to verify that the cleanup function correctly undoes the setup. If an effect breaks under `StrictMode` double-invocation, that is evidence the cleanup is incomplete or missing — it is not a `StrictMode`-only artifact to work around by disabling `StrictMode`. Do not recommend removing `StrictMode` as a fix for a broken cleanup function.
4. **A `setTimeout`/`setInterval`-based effect needs `clearTimeout`/`clearInterval` in its cleanup**, and a subscription-based effect needs the corresponding unsubscribe/disconnect call — the same guard-in-cleanup principle applies beyond fetch. Treat a missing `clearInterval`/unsubscribe as the same severity class as a missing `ignore` flag: it is a resource leak and, if the interval body calls `setState`, a potential update-on-unmounted-component or stale-closure bug.

## Severity escalation

Treat a missing cancellation guard as **HIGH severity, not MEDIUM**, when the async effect's resolution triggers:

- an authenticated write, mutation, or other state-changing network call (duplicate submission risk, not just a stale read),
- a redirect, navigation, or auth-state change (wrong-user data exposure risk),
- a write to a shared/global store rather than local component state (blast radius extends beyond the component).

Otherwise (a stale read into local UI state with no side effect beyond a flicker), MEDIUM is appropriate — still a real bug, but not a data-integrity or security risk.

## Verification targets

When repo evidence is available, verify the finding against:

- the actual async function signature — does it accept or already use an `AbortSignal`? If yes and the effect ignores it, that is a stronger finding (cancellation capability exists and is unused) than the case where no cancellation mechanism exists at all.
- whether the dependency array includes every reactive value used to construct the request (query, page, userId in the examples above) — a missing request-parameter dependency is a *different* finding (stale closure, see the dependency-array reference) that often co-occurs with a missing cleanup guard in the same effect.

## When to push back

Push back if the user asks to:

- "just add a loading spinner" as the fix for a reported race condition — a spinner does not prevent stale data from overwriting fresh data; it only hides the timing, it does not fix it,
- suppress the exhaustive-deps lint warning on an async effect instead of adding the guard — the warning is frequently the signal that led to finding the missing guard in the first place,
- disable `StrictMode` to make a race condition symptom disappear in development instead of fixing the missing cleanup — this hides the bug in dev while leaving it live in production.
