# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure, the route-tree mapping table, and the required output shape for a routing/navigation review.

> Version note: React Router's route-module API (`loader`, `action`, `lazy`, `middleware`) and Next.js's App Router conventions (middleware/proxy, parallel/intercepting routes, `loading.tsx`) are version-sensitive. Verify the installed major version (`package.json`) before asserting exact API shape; React Router v5/v6-classic and Next.js Pages Router use materially different patterns than the current framework-mode/App Router guidance this skill is grounded in.

## What people get wrong

The naive assumption is:

> "The route isn't in the nav menu / it redirects to `/login` in the component, so it's protected."

That is incomplete in two distinct ways:

1. A client-side redirect or conditional render still requires the route to match and the component tree to mount before the redirect fires. Nothing stops a user (or a script) from requesting the route's data directly, reading the JS bundle for that route, or racing the redirect. The only real gate is a check that runs **before** the sensitive work happens, on the server: a React Router `loader` that throws/redirects, or a Next.js Server Component/Server Action/Route Handler check.
2. In Next.js specifically, teams often stop at a `middleware`/proxy check and call the route "protected." Official Next.js guidance is explicit that middleware-based checks are an *optimistic* pass (cookie presence, not full session validation) intended for UX redirects — the authoritative authorization check belongs in a server-only Data Access Layer close to the data source. A middleware-only implementation is not done; it is half-done.

## Step-by-step workflow

1. **Map the route tree.** Enumerate every route: path, layout nesting/parent chain, and whether it is an index route. Note which framework/router is in play (React Router framework mode vs. classic `<Routes>`; Next.js App Router vs. Pages Router) — the enforcement and code-splitting patterns differ by framework and by mode.
2. **Classify protection level per route.** For each route, determine whether it is meant to be public or protected using the team's actual authorization model (ask if undocumented; do not assume). Do not infer protection level from the route's name or nav visibility.
3. **Locate the server-side enforcement point for every protected route.** See `references/server-enforcement-patterns.md` for the exact patterns to look for in React Router (`loader`/middleware-with-loader) and Next.js (Data Access Layer). If none exists, this is the finding — do not continue to treat the route as protected for the rest of the review.
4. **Inspect code-splitting boundaries.** For each lazily-loaded route, determine whether its component, loader, and action resolve via a parallel construct or a sequential `await` chain. See `references/code-splitting-and-url-state.md`.
5. **Check URL-reconstructable state.** For each route with filters, pagination, tabs, or other view-affecting UI state, determine whether that state is represented in the URL (path segment or search params) or only in component/memory state. See `references/code-splitting-and-url-state.md`.
6. **Trace focus management and status announcements.** For each route transition in scope, determine what receives focus after navigation and whether an `aria-live` region (or equivalent) announces the change. See `references/focus-and-navigation-blocking.md`.
7. **Check navigation-blocking on form-heavy routes.** Determine whether unsaved-changes protection exists and whether its known SPA-only limits are accounted for. See `references/focus-and-navigation-blocking.md`.
8. **Rank and report findings** per the output shape below.

## Route-tree mapping table

Produce this table before listing findings:

| Path | Protection level | Enforcement pointer (file:line, or "none found") | Code-split boundary | Focus target on transition |
|---|---|---|---|---|
| `/dashboard` | protected | `app/dashboard/page.tsx` calls `getDashboardData()` DAL check, `lib/dal.ts:12` | route-level `lazy` | `<h1>` ref-focused in `layout.tsx:8` |
| `/login` | public | n/a | eager | n/a |

## Decision tree

- Protected route has no server-side enforcement point (client-only redirect/hide, or Next.js middleware check with no matching Data Access Layer check) → **HIGH / block**. State exactly what exists (if anything) and why it is insufficient.
- Protected route's only enforcement is a Next.js `middleware`/proxy check with no corresponding server-only check in the Server Component/Action/Route Handler that touches the data → **HIGH**. Middleware-only is not equivalent to a Data Access Layer check per current Next.js guidance.
- Lazy route's component/loader/action are `await`-ed sequentially instead of via `Promise.all` or the framework's combined `lazy` loader → **MEDIUM**, waterfall regression; cite the measurable extra round trip.
- View-critical state (filter/page/tab) exists only in component state with no URL representation → **MEDIUM/HIGH depending on user impact** (HIGH if the route is meant to be shareable/bookmarkable, e.g., a saved-search or report link).
- Route transition has no defined focus target and no `aria-live` announcement → **HIGH** (WCAG 2.4.3 / 4.1.3), even if the app "looks fine" visually — this is an assistive-technology-only defect class.
- Form-heavy route has no navigation-blocking for unsaved changes → **MEDIUM** data-loss risk; escalate to HIGH if the form represents non-trivial user input (long forms, financial/legal data entry).
- `useBlocker` (or equivalent) is present but the route also needs hard-reload/tab-close protection and has none → **LOW/MEDIUM** note the gap; `useBlocker` explicitly does not cover hard reloads or cross-origin navigation.

## Adversarial checklist

Before closing a review with no HIGH findings, confirm:

- For every route you called "protected," did you find and cite the actual server-side enforcement pointer, or did you accept a nav-hiding/component-redirect as sufficient?
- For every Next.js protected route, did you check for a Data Access Layer check in addition to (or instead of) middleware, per current official guidance?
- Did you verify lazy-loaded routes resolve component/loader/action in parallel, not just that `lazy` is used at all?
- Did you check every route with filter/pagination/tab UI for URL representation, not just the ones an issue report already flagged?
- Did you trace focus behavior from source (refs, `useEffect` on location change, `aria-live` regions) rather than assuming a framework "handles it automatically"?
- If you found zero HIGH findings, is that because none exist, or because you didn't trace enforcement/focus/URL-state far enough?

## Output shape

Every review response must include:

1. **Scope** — routes and files reviewed, framework/router and major version noted.
2. **Route-tree mapping table** — as specified above.
3. **Findings** — ranked HIGH → MEDIUM → LOW, each with `file:line`, the category (enforcement gap / waterfall / URL-state / focus-management / navigation-blocking), and a concrete fix sketch.
4. **Evidence level** per finding: `repo evidence`, `documentation-based`, or `inference`.
5. **Verdict** — approve / approve-with-notes / block. Any protected route with no server-side enforcement point is an automatic block.
6. **Open questions** — anything the review could not verify (unconfirmed framework version, runtime focus behavior not simulated, ambiguous authorization model).
