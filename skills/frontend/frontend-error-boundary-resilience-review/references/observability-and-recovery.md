# Observability and Recovery

Use this reference when confirming caught errors are still logged to observability, and when reviewing reset/retry design (`resetKeys`, retry affordances) so a boundary does not stay permanently tripped.

## What people get wrong

The naive story is:

> Once the fallback renders, the failure is handled — we're done.

Incomplete on two fronts. First, "handled" for the user is not "handled" for the team that owns the code: if `componentDidCatch` (or the `onError` equivalent) never forwards the error anywhere, the failure rate for that section is invisible to everyone except the users who hit it and don't complain. Second, a boundary with no reset path is a one-way door — once tripped, that section of the UI stays broken for the rest of the session (or until a full page reload), even if the underlying transient failure (a network blip, a flaky chunk load) would have succeeded on retry a second later.

## Officially grounded shape

- `componentDidCatch(error, info)` is called when a child component (including distant descendants) throws during rendering, and is the documented place to log the error to a reporting service. `info.componentStack` carries the component stack trace of the failure.
- React 19 added `onCaughtError`, `onUncaughtError`, and `onRecoverableError` root options that complement `componentDidCatch`: `onCaughtError` fires when React catches an error inside an Error Boundary, `onUncaughtError` fires when an error is thrown with no Error Boundary to catch it, and `onRecoverableError` fires when React automatically recovers from an error (for example, a client-side retry after a server render error outside the shell). These are root-level hooks, not a replacement for `componentDidCatch` — use them for cross-cutting/root-level observability wiring, and `componentDidCatch` for the per-boundary logging call.
- In development builds, errors caught by `componentDidCatch` still bubble to `window` (so `window.onerror` / `window.addEventListener('error', ...)` also intercepts them). In production builds they do not bubble to ancestor handlers once caught — meaning production logging depends entirely on the boundary's own `componentDidCatch`/`onError` actually shipping the error somewhere; there is no free window-level safety net in production for an error a boundary already caught.
- `react-error-boundary`'s documented retry pattern combines `resetKeys` (an array compared by reference/value across renders — when any entry changes, the boundary automatically resets) with an explicit retry action in the fallback that triggers the state change feeding `resetKeys` (commonly wrapped in `startTransition`). The official React docs' own `use()` example pairs a `resetKeys={[albumsPromise]}` boundary with a `handleRetry` button that calls `refetchData` and updates the promise in state, which changes the `resetKeys` value and re-triggers the Suspense/Error Boundary pair.
- For a server-rendered component that errors outside the shell (wrapped in `<Suspense>`), React's documented recovery path is: emit the closest Suspense fallback into the HTML, then retry rendering on the client once JS loads; if the client retry also fails, the closest parent Error Boundary determines the final presentation, and the server `onError` / client `onRecoverableError` callbacks fire regardless of the outcome so the failure is still observable even when the retry silently succeeds from the user's point of view.

## Non-negotiable design rules

### 1. Every `componentDidCatch`/`onError` must forward to logging, with no silent-catch exceptions

A boundary that implements `getDerivedStateFromError` (or `fallback`/`fallbackRender`) to show a nice fallback but leaves `componentDidCatch`/`onError` empty or absent is a silent failure factory. Treat "renders a fallback with no logging call" as equivalent in severity to "no boundary at all" from an observability standpoint — the user experience differs, but the team's blindness to the failure rate is identical.

### 2. Do not rely on production window-level bubbling as a logging strategy

Because production builds do not bubble a caught error to `window` once a boundary has caught it, a review must confirm the logging call inside `componentDidCatch`/`onError` itself — not assume some ambient error-tracking SDK's global handler will pick it up. If the team's error-tracking setup depends on global `window.onerror`/`unhandledrejection` listeners, an error boundary silently intercepts exactly the errors those listeners are meant to catch, in production, unless the boundary explicitly re-reports it.

### 3. A boundary with a plausibly-transient failure needs an explicit reset path, not just a fallback

If the underlying cause can plausibly succeed on retry (a `use()` rejection from a flaky fetch, a `React.lazy` chunk load failure from a transient network blip), the boundary should support `resetKeys` (or an equivalent reset mechanism) wired to a user-triggered retry action, not just a static fallback that requires a full page reload to recover. Confirm the retry action actually changes the value(s) `resetKeys` compares — a retry button that re-renders the same fallback with the same `resetKeys` value does nothing.

### 4. Distinguish root-level observability hooks from per-boundary logging

`onCaughtError`/`onUncaughtError`/`onRecoverableError` are useful for a single, centralized observability wiring point (e.g., forwarding every caught/uncaught error in the app to one telemetry pipeline), but they do not substitute for boundary-specific context. A per-boundary `componentDidCatch` can attach section-specific metadata (which independently-failable unit failed, what retry state existed) that a root-level hook cannot infer on its own. Recommend both where the team has meaningful section-level context to attach.

## Minimal safe review flow

1. For each Error Boundary identified during placement review, confirm `componentDidCatch`/`onError` is implemented and actually calls a logging/observability function — not just present as an empty or commented-out method.
2. Confirm the logging call includes enough context to be actionable (at minimum: the error, `info.componentStack` or equivalent, and ideally which independently-failable section/boundary it came from).
3. If the app defines root-level `onCaughtError`/`onUncaughtError`/`onRecoverableError` (React 19+), confirm they are wired to the same observability pipeline and are not the *only* logging mechanism for boundaries that need section-specific context.
4. For each boundary guarding a plausibly-transient failure, confirm a `resetKeys`-driven (or equivalent) retry path exists and that the retry action actually changes the value(s) being compared.
5. Confirm retry actions that re-trigger a suspending read are wrapped appropriately (e.g., `startTransition`) so the retry doesn't produce a jarring synchronous re-suspend with no pending-state affordance.
6. State the verdict per boundary: logging present/absent, reset/retry present/absent-and-justified, and whether root-level hooks (if any) are additive to or a substitute for missing per-boundary logging.

## High-risk assumptions to kill

- "Our error-tracking SDK auto-catches everything, we don't need to log inside the boundary" — verify this against the actual SDK's instrumentation method; a caught error inside `componentDidCatch` does not bubble to `window` in production, so window-level auto-instrumentation will miss it unless the boundary explicitly re-reports.
- "The fallback shows an error, so we're already observable" — a fallback is a user-facing signal, not a telemetry signal; the two are unrelated unless `componentDidCatch`/`onError` explicitly forwards to logging.
- "Retry just means telling the user to refresh the page" — that is not a retry mechanism, it is asking the user to lose all other in-progress state on the page to recover from one section's transient failure; prefer a scoped `resetKeys` retry wherever the failure is plausibly transient.

## When to push back

Push back if the user asks to:

- ship a boundary's `componentDidCatch`/`onError` as empty "for now, we'll wire up logging later,"
- rely solely on a global `window.onerror` listener as the logging strategy for errors caught by boundaries,
- tell users to reload the page as the only recovery path for a failure that a `resetKeys`-driven retry could resolve in place.

Those trade a small amount of wiring effort now for either a permanently blind failure rate or a needlessly destructive recovery path later.
