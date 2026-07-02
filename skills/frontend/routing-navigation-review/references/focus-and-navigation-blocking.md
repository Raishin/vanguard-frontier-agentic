# Focus Management and Navigation Blocking

Use this reference only when reviewing focus/`aria-live` behavior on route transitions, or unsaved-changes navigation blocking. Load it during steps 6–7 of the review workflow.

## What people get wrong

The naive assumption is:

> "React Router / Next.js handles focus on navigation automatically, like a real page load would."

It does not, by default, in either framework. A client-side route transition in an SPA swaps DOM content without the browser's native full-page-navigation behavior (which resets focus to the document body and lets screen readers announce the new page title). If nothing in the application explicitly moves focus and announces the change, a screen reader user who navigates via a link ends up with focus still anchored to the link they just activated (now possibly removed from the DOM, or pointing at stale content) with no announcement that anything changed — a silent failure sighted users never notice because they can see the visual change.

A second naive assumption:

> "I added a confirm dialog with `window.confirm` in a `beforeunload` handler, so unsaved changes are protected everywhere."

`beforeunload` only fires for hard navigations (reload, closing the tab, typing a new URL, following a link with a full page load). It does **not** fire for client-side SPA navigations triggered by the router — those need a router-level mechanism (e.g., React Router's `useBlocker`), which in turn has the opposite limitation: it does not cover hard reloads or cross-origin navigation. Neither mechanism alone is complete; a form-heavy route that cares about both needs both.

## Officially grounded shape: focus management

- Neither React Router nor Next.js ships automatic focus-to-heading-on-navigation behavior as a default, framework-level guarantee for arbitrary route trees. The established accessible-SPA pattern (consistent with the WAI-ARIA Authoring Practices Guide's page/route-change guidance) is: on route change, move focus to a stable, predictable target — commonly the page's `<h1>` (made programmatically focusable with `tabIndex={-1}`) or a `main` landmark — and pair it with an `aria-live="polite"` (or `assertive`, for urgent changes) region that announces the new page/section so screen reader users get an audible cue equivalent to what a full page load provides natively.
- Look for this pattern implemented via a `useEffect` keyed on the route's location/pathname (React Router: `useLocation`; Next.js: `usePathname`) that calls `.focus()` on a ref, and a live region whose text content updates on the same trigger.

## Design rules: focus management

1. **Every route transition needs a defined focus target.** Absence of any focus-management code (no ref, no `.focus()` call, no live-region update keyed to navigation) is a HIGH finding (WCAG 2.4.3 Focus Order) — this is not cosmetic, it breaks the core navigation model for keyboard/screen-reader users.
2. **The focus target should be stable and meaningful**, not an arbitrary or randomly-remaining-focused element. Flag a focus target that lands on something removed/re-rendered on every transition (causing focus loss anyway) or on an element with no semantic relationship to "the new page."
3. **An `aria-live` announcement (or equivalent, e.g., a visually-hidden status element updated on navigation) should accompany the focus move** for the change to be announced, not just focusable — flag a repo that moves focus but never updates any live-region/status text as a WCAG 4.1.3 (Status Messages) gap, distinct from the 2.4.3 focus-order gap.
4. **Do not assume a component library "handles this."** Verify by tracing the actual `useEffect`/ref/live-region code for the route(s) in scope; a UI kit's individual components (buttons, dialogs) having internal focus management does not imply the app's top-level router transitions do too.

## Officially grounded shape: navigation blocking

- React Router's documented mechanism for blocking in-SPA navigation is `useBlocker(shouldBlock)`, which returns a `Blocker` with `state` (`unblocked` | `blocked` | `proceeding`), and `proceed()`/`reset()` methods to let the confirmation UI resolve the block. It requires a data router (`createBrowserRouter` / framework mode) — it does not exist as a hook on the older `<BrowserRouter>` non-data-router API.
- `useBlocker` explicitly does **not** intercept hard reloads, tab closes, or cross-origin navigations — those require a separate `beforeunload` handler. The two mechanisms are complementary, not interchangeable.
- Next.js does not ship a first-party equivalent to `useBlocker` for App Router client-side transitions as a stable, documented primitive in the same way; verify current docs/Context7 before asserting a specific Next.js API exists, and if none is confirmed, flag the absence of any unsaved-changes protection as a finding rather than assuming a framework default covers it.

## Design rules: navigation blocking

1. **Form-heavy routes need unsaved-changes protection for SPA navigation**, hard reload/tab-close, or both, depending on what's realistic for the app. Absence entirely is a MEDIUM (escalate to HIGH for long/high-stakes forms) data-loss-risk finding.
2. **If only `beforeunload` exists**, flag that in-app SPA navigation (clicking another nav link) is still unprotected — this is the more common real-world path a user takes to accidentally lose data, more so than closing the tab.
3. **If only `useBlocker`/equivalent exists**, flag that hard reload/tab-close/cross-origin navigation is still unprotected, and confirm whether that gap is acceptable for the route's risk profile or needs a `beforeunload` handler added alongside it.
4. **Verify the block condition is based on actual dirty-state tracking** (form value changed from initial), not a coarse "is this route a form" heuristic that blocks navigation even with no changes made — that's a usability regression in the opposite direction (blocking users who made no edits).

## Adversarial checklist

- For each route transition in scope, what element receives focus — did you trace it from an actual ref/`.focus()` call, or assume "it probably works"?
- Is there an `aria-live` region (or equivalent) that updates on the same transition, or does focus move silently?
- For each form-heavy route, does unsaved-changes protection exist at all? If so, does it cover SPA navigation, hard reload, or both — and is the gap (if any) acceptable?
- Is the "unsaved changes" condition tied to real dirty-state, or would it block navigation even when nothing changed?

## Safe verification targets

- Grep for `useLocation`/`usePathname` combined with a `useEffect` and a `.focus()` call or ref assignment.
- Grep for `aria-live` attributes and confirm their content updates on route change (not just present once, statically, with no dynamic text).
- Grep for `useBlocker`, `beforeunload`, or equivalent on routes containing `<form>` elements or form-library state (e.g., react-hook-form's `formState.isDirty`).

## When to push back

Push back if the user says:

- "the browser handles focus, we don't need to do anything" (true for hard navigations, false for SPA route transitions),
- "we'll add focus management later, ship the route now" (this is a compliance gap the moment the route ships, not a deferred nice-to-have),
- "`beforeunload` covers it" for a route whose primary navigation-away path is in-app link clicks, not tab closes.

Those defer a defect users with assistive technology hit immediately, not an edge case.
