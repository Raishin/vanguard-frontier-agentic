# Code-Splitting and URL State

Use this reference only when reviewing lazy-loading/waterfall regressions in a route tree, or when view-critical state needs to move into the URL. Load it during steps 4–5 of the review workflow.

## What people get wrong

The naive assumption is:

> "I added `lazy: () => import('./route')`, so the route is code-split correctly."

That covers the component, but a route module also has a `loader` and possibly an `action`. If those are fetched with separate sequential `await` statements instead of a combined parallel construct, the route now pays for two or three round trips (component chunk, then loader chunk, then action chunk) instead of one — a self-inflicted waterfall that looks identical in the diff ("we added lazy loading!") but regresses time-to-interactive.

A second common assumption:

> "The filter/tab/page state is in `useState`, so it works fine — I can see it update."

It "works" for the current session in the current tab. It does not survive a refresh, cannot be shared as a link, and breaks the browser back/forward button's expected behavior — all silent failures that only surface as user complaints ("I sent my coworker this link and it didn't have my filters").

## Officially grounded code-splitting shape (React Router)

- The recommended `lazy` route property loads the component and its data functions together. The documented pattern uses `Promise.all` (or the single combined `lazy` module import) specifically so the component and `loader` resolve **in parallel**, not one after another:

```tsx
{
  path: "/app",
  lazy: async () => {
    // load component and loader in parallel before rendering
    const [Component, loader] = await Promise.all([
      import("./app"),
      import("./app-loader"),
    ]);
    return { Component, loader };
  },
}
```

- The simpler single-module `lazy: () => import("./about")` form (importing one module that exports `loader`/`action`/`Component` together) achieves the same parallelism implicitly, because it's one dynamic import resolving one module graph.
- A regression to watch for: someone splits `loader`, `action`, and `Component` into three separate `lazy.loader`, `lazy.action`, `lazy.Component` async functions (a valid, documented granular form) but then has one `await` the result of another inside its own function body instead of letting the router resolve them independently — that reintroduces a sequential dependency the granular form was supposed to avoid.

## Officially grounded code-splitting shape (Next.js)

- Next.js App Router code-splits by route segment automatically; the equivalent regression to check for is a route's `page.tsx` or `layout.tsx` performing sequential `await` calls for independent data instead of using parallel data fetching (e.g., initiating multiple `fetch`/DB calls before awaiting any of them, or using `Promise.all`), and a missing `loading.tsx` for a route whose data-fetching is inherently slow — the documented pattern is to add `loading.tsx` so navigation feels instant while server rendering completes, rather than leaving the user on a blank screen.

## Design rules: code-splitting

1. **Component + loader (+ action) should resolve in parallel for a given lazy route**, not sequentially. Flag `await import(a); await import(b)` sequences that could be `Promise.all([import(a), import(b)])`.
2. **Check for a `loading.tsx`/equivalent fallback** on any route segment whose data-fetching is non-trivial, so the user gets immediate navigation feedback rather than a frozen UI — absence is a MEDIUM UX/performance finding, not a hard block.
3. **Don't flag intentional sequential dependencies** — if a `loader` genuinely needs data only available after a component-level decision (rare, and usually a smell in its own right), sequential loading may be correct; verify the dependency is real before flagging.

## Officially grounded URL-state shape

- View-affecting UI state that should be shareable, bookmarkable, or survive a refresh belongs in the URL: a dynamic path segment for identity-like state (`/blog/[slug]`), and search params for filter/pagination/sort/tab state (`?page=2&sort=recent&tab=comments`).
- React Router and Next.js both provide first-class hooks/utilities for reading and writing search params tied to navigation (`useSearchParams` in both, with framework-specific write patterns) — the presence of one of these being used for a given piece of state is a strong positive signal; the absence of any URL-touching code for filter/tab/pagination UI, combined with a `useState` holding that same data, is the defect pattern to search for.

## Design rules: URL state

1. **Classify each piece of route-level UI state** as either (a) session-only/ephemeral (a dropdown's open/closed state — fine in `useState`) or (b) view-critical/shareable (a filter, page number, active tab, search query, sort order — should be in the URL). Do not flag category (a).
2. **For category (b) state found only in `useState`/component state with no corresponding search-param or path-segment representation**, flag it — severity depends on whether the route is meant to be shareable or bookmarkable (a report/dashboard/search-results route is usually meant to be; an internal wizard step sometimes isn't — verify intent rather than assuming).
3. **Verify state survives a hard refresh conceptually**: if the only place a value lives is a `useState` initializer with no URL/`localStorage`/server-state backing, a refresh loses it — that's the concrete, testable definition of the defect, more useful in a finding than "should be in the URL" alone.

## Adversarial checklist

- Does the lazy route's component and loader (and action, if present) resolve via a parallel construct, or did you find a sequential `await` chain?
- Is there a `loading.tsx`/fallback for routes whose data-fetching is non-trivial?
- For every filter/pagination/tab/search UI element on a route, did you check whether its state is represented in the URL, or did you only check the ones a bug report already named?
- If state is in the URL, is it read back out on load (so a shared link actually reproduces the view), not just written on change?

## Safe verification targets

- Grep for `lazy:` route properties and inspect whether their resolution uses `Promise.all` / a single combined import, or separate `await` statements.
- Grep for `useState` hooks adjacent to filter/tab/pagination UI and cross-reference against `useSearchParams`/`URLSearchParams` usage in the same file.
- Check for `loading.tsx` (Next.js) or an equivalent pending/fallback UI sibling to slow route segments.
