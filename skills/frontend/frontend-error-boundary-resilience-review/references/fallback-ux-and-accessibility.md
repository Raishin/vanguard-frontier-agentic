# Fallback UX and Accessibility

Use this reference when reviewing fallback UI copy, information-disclosure risk, accessible alert semantics, or the layout-shift impact of an error-boundary fallback swap.

## What people get wrong

The naive story is:

> The fallback just needs to say "Something went wrong" instead of crashing — copy is a low-priority polish item.

Incomplete on two fronts. First, what the fallback *contains* is a security surface, not just a copy surface — rendering `error.message` or `error.stack` verbatim is a common shortcut during development that quietly ships to production and leaks internal implementation detail (library names, internal endpoint paths, stack frames) to anyone who can trigger the error. Second, what the fallback *is perceivable as* is an accessibility surface — a sighted user visually notices a DOM region change to an error message; a screen-reader user does not, unless the fallback is marked up so assistive technology announces it.

## Officially grounded shape

- `getDerivedStateFromError(error)` receives the thrown error and is responsible for producing the state that renders the fallback. Because JavaScript allows throwing any value (not guaranteed to be an `Error` instance), this method — and any fallback UI derived from it — must handle non-`Error` thrown values defensively rather than assuming `error.message` and `error.stack` are always present and safe to display.
- `componentDidCatch(error, info)` is the intended place to send the error report to an analytics/observability service, separate from the render path. `info.componentStack` contains the component stack trace. This is exactly the kind of implementation detail that belongs in observability tooling, not in end-user-visible DOM.
- The `react-error-boundary` package's `fallback` / `fallbackRender` / `FallbackComponent` props exist specifically to let the fallback be a *designed* UI component, not a raw dump of the caught error — official React examples pair a fallback like `<p>⚠️ Something went wrong</p>` with a `componentDidCatch`/`onError`-style logging call, keeping the two concerns (user-facing message, internal diagnostic detail) separate.
- ARIA APG's Alert pattern specifies that an alert is "a type of live region that renders important, and usually time-sensitive, information" and should not require or expect user focus, but must be programmatically exposed via `role="alert"` (implying `aria-live="assertive"` and `aria-atomic="true"`) or an equivalent `aria-live` region so assistive technology users are notified without needing to navigate to the changed region themselves.

## Non-negotiable design rules

### 1. Never render raw error detail to end users in production

Block any fallback UI that interpolates `error.message`, `error.stack`, or `info.componentStack` directly into user-facing markup. This is an information-disclosure defect regardless of how unlikely the error is to be triggered by an untrusted actor — treat it the same as any other unintended internal-detail leak, not as a debugging convenience that's fine to leave in.

### 2. The fallback must be programmatically announced, not just visually present

A fallback that swaps in silently — no `role="alert"`, no `aria-live` region, no focus management — is invisible to screen-reader users even though it's visually obvious to sighted users. Apply the ARIA APG alert pattern so the failure is announced. Do not require the fallback to steal focus; per the APG pattern, an alert should not expect or require focus by default.

### 3. Provide a recovery affordance where the failure is plausibly transient

A fallback that only says "something went wrong" with no path forward forces the user to reload the entire page (losing any other in-progress state) to recover from a single section's failure. Where the underlying data can plausibly be retried (a `use()` rejection, a lazy chunk load failure), prefer a fallback with an explicit retry action wired through the boundary's reset mechanism (see `observability-and-recovery.md`) over a dead-end message.

### 4. Account for layout shift when the fallback replaces content

A fallback that renders at a meaningfully different size than the content it replaces (or than the loading-state placeholder that preceded it) causes a visible layout shift at the moment of failure. This is a measurable Cumulative Layout Shift (CLS) regression, not a cosmetic detail — CLS is one of the Core Web Vitals used to assess real-world UX quality, and an error-triggered shift is exactly the kind of field-data surprise this skill should catch during review rather than after a production incident report.

## Minimal safe review flow

1. For each fallback UI found, check whether it interpolates any property of the caught error object or component stack into rendered output. Any such interpolation reaching production is a block-level finding.
2. Confirm the fallback element or its container carries `role="alert"` or is inside an `aria-live` region, per the ARIA APG alert pattern.
3. Confirm the fallback does not forcibly steal focus (unless there is a specific, justified reason tied to the failure's severity) — the APG pattern does not require focus for alerts.
4. Check whether the underlying failure is plausibly transient (network blip, chunk load failure, a retryable fetch) and, if so, confirm a retry affordance exists and is wired to the boundary's reset mechanism rather than requiring a full page reload.
5. Compare the fallback's rendered footprint against the content/placeholder it replaces; flag a likely CLS regression if the size or position differs meaningfully with no reserved space.
6. State the verdict per fallback instance, since a tree can have multiple boundaries with inconsistent fallback quality.

## High-risk assumptions to kill

- "We only show the raw error in dev mode, it's fine" — confirm the actual build/environment gate is correct and cannot leak into a production bundle or misconfigured environment; do not accept a verbal assurance as proof.
- "The fallback is only shown briefly, so its accessibility doesn't matter" — an error state is exactly the moment a user most needs a clear, perceivable signal; brevity does not reduce the accessibility requirement.
- "Any fallback message is better than a blank page" — a fallback that leaks stack traces or causes a jarring layout shift can be a worse outcome for security and UX than a well-designed, generic message.

## When to push back

Push back if the user asks to:

- show the caught error's raw message or stack "just for this internal tool" without a hard boundary confirming it can never reach an external or lower-trust surface,
- skip accessible alert markup because "it's an edge case, users rarely see it,"
- ship a fallback with no retry path for a plausibly-transient failure purely to avoid wiring the reset mechanism.

Those trade a small implementation cost now for an information-disclosure risk, an accessibility gap, or a worse recovery experience later.
