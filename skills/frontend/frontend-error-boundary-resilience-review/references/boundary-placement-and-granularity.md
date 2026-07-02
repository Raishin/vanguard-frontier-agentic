# Boundary Placement and Granularity

Use this reference when mapping a component tree to confirm every suspending, async, or third-party-embedded component has an ancestor error boundary, and when the only boundary present is at the app root.

## What people get wrong

The naive story is:

> We have an `ErrorBoundary` wrapping `<App />`, so errors are handled.

Technically true, practically insufficient. An app-root-only boundary means every descendant error — no matter how unimportant the component — takes the entire page down to the same fallback. A broken third-party ad slot and a broken checkout form produce the identical user-facing outcome: total page loss. Error boundaries are not a binary "present or absent" property of an app; they are a placement and granularity design, and the design question is *where the blast radius of each independently-failable section ends*.

## Officially grounded shape

- React does not currently ship a function-component error-boundary primitive. `static getDerivedStateFromError(error)` and `componentDidCatch(error, info)` are class-component lifecycle methods; a component defining both is, by definition, an Error Boundary. Teams that want a component API without writing the class boilerplate use the community-maintained `react-error-boundary` package, which React's own reference docs link to.
- `getDerivedStateFromError` must be a pure function that returns the state used to render the fallback; it should not perform side effects. `componentDidCatch` is where side effects (logging) belong. A component needs both to be a complete, useful Error Boundary — `getDerivedStateFromError` alone renders a fallback with no logging; `componentDidCatch` alone (calling `setState` inside it) is a deprecated pattern.
- An Error Boundary catches errors thrown during rendering by its children, including distant descendants — not just its immediate child.
- A component that suspends via `use()` on a Promise, or any Suspense-triggering data read, propagates a **rejection** to the nearest ancestor Error Boundary, not the nearest Suspense fallback. Suspense and Error Boundaries solve different problems: Suspense handles the pending state; the Error Boundary handles the rejected/thrown state. Official React examples consistently pair them: `<ErrorBoundary fallback={...}><Suspense fallback={...}>{children}</Suspense></ErrorBoundary>`.
- `React.lazy`-loaded components that fail to load (network failure, bad chunk) throw during render and require the same ancestor Error Boundary treatment as any other suspending/rejecting read.

## Non-negotiable design rules

### 1. Every suspending, async, or third-party component needs an ancestor error boundary

This is not optional and not a style preference. Map every component that can throw during render — a `use()` call, a lazy-loaded chunk, a third-party embed's mount/render path — and confirm an Error Boundary exists somewhere above it in the tree. If it does not, the component is one runtime error away from an unhandled crash.

### 2. Boundary granularity must match failure-isolation need, not code convenience

One boundary at the app root is the easiest thing to write and the worst thing to ship when the page mixes critical flows (checkout, primary content, forms) with non-critical, independently-failable sections (recommendations, third-party embeds, secondary widgets). Wrap each non-critical, independently-failable section in its own boundary so its failure cannot propagate upward into a critical flow's render tree.

### 3. Do not conflate "has a boundary somewhere" with "has the right boundary"

A component nested three levels inside a checkout form that shares the checkout form's own Error Boundary is not isolated — an error in that nested component still trips the checkout form's fallback. The relevant question is always: does *this specific* independently-failable unit have a boundary that fails *only that unit*, or does it share a boundary with something the business cannot afford to lose?

### 4. Suspense placement and Error Boundary placement are related but not the same decision

A Suspense boundary controls what shows while content is pending and bounds hydration-mismatch re-render blast radius. It does not handle rejection. When reviewing a tree, evaluate Suspense granularity and Error Boundary coverage as two related but separately-checked properties — a well-placed Suspense boundary with no ancestor Error Boundary is still a crash-to-blank defect on rejection.

## Minimal safe review flow

1. Map every suspending/async read (`use()`, lazy-loaded component, third-party SDK mount) and every third-party embed in the tree under review.
2. For each, walk up the ancestor chain and confirm an Error Boundary exists. If none exists, flag as a defect immediately — do not wait for other findings.
3. For each existing Error Boundary, identify exactly which components it covers and whether that scope matches an independently-failable unit (good) or spans across a critical flow and a non-critical section (defect — too coarse).
4. Confirm no critical flow (checkout, auth, primary content) shares a boundary with a non-critical, independently-failable section.
5. Confirm each Error Boundary implements both `getDerivedStateFromError` (or the `fallback`/`fallbackRender` equivalent) and `componentDidCatch`/`onError` — a boundary with only one of the two is incomplete (see `observability-and-recovery.md` for the logging half).
6. State the verdict per boundary and per uncovered suspending/third-party component, not just for the tree as a whole.

## High-risk assumptions to kill

- "We have an ErrorBoundary at the root, so we're covered" — covered against total blank pages, not against unnecessary blast radius.
- "It's wrapped in Suspense, so errors are handled" — Suspense handles pending state, not rejection; a rejection with no ancestor Error Boundary is still an unhandled crash.
- "Third-party embeds are usually stable, we don't need a boundary just for that widget" — third-party code is exactly the code the application has the least control over and the strongest reason to isolate.
- "React has an error boundary hook now, right?" — verify against current Context7 docs before asserting this; as of official React reference docs, error boundaries remain class-component-only, with `react-error-boundary` as the documented community path for a component API.

## When to push back

Push back if the user asks to:

- ship a suspending or third-party-embedded component with no ancestor Error Boundary "for now, we'll add it later,"
- rely on a single app-root boundary and call the failure-isolation review complete,
- nest a non-critical, independently-failable section inside the same boundary as a critical flow purely to reduce the number of `ErrorBoundary` components in the file.

Those trade a contained, isolated failure for a page-wide one, and the tradeoff is rarely visible until the incident that reveals it.
