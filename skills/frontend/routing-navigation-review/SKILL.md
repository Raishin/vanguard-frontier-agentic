---
name: routing-navigation-review
description: Reviews route-tree structure, loader/action placement, code-splitting boundaries, and navigation-blocking/focus-management behavior in React Router and Next.js applications for correctness, server-side security enforcement, and accessibility conformance on route transitions.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Routing & Navigation Review

## Purpose

Review a frontend application's route tree — path/layout nesting, loader/action data-fetching placement, code-splitting boundaries, and navigation-blocking/focus-management behavior — without re-litigating what data a loader fetches from the backend contract (that is `api-integration-contract-review`) or SSR streaming/hydration mechanics at a route's data boundary (that is `ssr-hydration-streaming-diagnosis`) in every response. This skill exists because route-level defects hide in three distinct, easily-conflated places: a route that is "protected" only by hiding a nav link or redirecting client-side (an authorization bypass reachable by typing the URL directly), a code-split boundary that accidentally serializes loader→component→action instead of loading them in parallel, and a route transition that silently drops keyboard focus with no status announcement (a WCAG 2.4.3 / 4.1.3 failure that is invisible unless you trace it deliberately).

## When to use

Use this skill when the user asks to:

- review a new route or a route-tree restructuring before merge,
- audit whether routes described as "protected" are actually enforced server-side, not just hidden client-side,
- investigate broken deep-links, lost filter/pagination/tab state on refresh, or "back button doesn't work right" bugs,
- respond to an accessibility audit finding of lost focus or missing status announcements on navigation,
- review code-splitting/lazy-loading changes to a route tree for waterfall regressions.

Do not use this skill for:

- reviewing what data a loader fetches from the backend contract, response shape, or error handling — use `api-integration-contract-review` instead,
- SSR streaming/hydration mechanics at a route's data boundary (Suspense boundaries, streaming HTML, hydration mismatches) — use `ssr-hydration-streaming-diagnosis` instead,
- component-internal state with no route/URL involvement.

## Context7 Documentation Protocol

- Resolve library IDs before citing any framework-specific claim: `/remix-run/react-router` for React Router, `/vercel/next.js` for Next.js. Do not assume API shape from memory — both frameworks' routing/data APIs have changed materially across major versions (React Router v6 object-route API vs. v7 framework-mode route modules; Next.js Pages Router vs. App Router).
- Before asserting server-side enforcement patterns in React Router, query `/remix-run/react-router` for "loader authentication redirect" and confirm the current guidance: a `loader` (or a middleware paired with a `loader` to force it to run on every client-side navigation) is the enforcement point — a component-level redirect or conditional render is not, because React Router still renders/matches the route client-side without a network round trip that a server can gate.
- Before asserting server-side enforcement patterns in Next.js, query `/vercel/next.js` for "data access layer authorization" and confirm the current guidance: official docs explicitly frame `middleware`/proxy-based checks (cookie-presence checks run at the edge) as an *optimistic* first pass for UX/redirect purposes, and require the actual authorization check to live in a server-only Data Access Layer close to the data source (Server Component, Server Action, or Route Handler) — do not treat a middleware matcher as sufficient enforcement on its own.
- Before asserting code-splitting behavior, query `/remix-run/react-router` for "lazy route module" and confirm current guidance: the recommended `lazy` route property loads the component and its `loader`/`action` together (e.g., via `Promise.all`) so they resolve in parallel — a common regression is `await`-ing them in sequence instead.
- Before asserting navigation-blocking behavior, query `/remix-run/react-router` for "useBlocker" and confirm current constraints: `useBlocker` only works within data routers (`createBrowserRouter`/framework mode) and explicitly does not intercept hard reloads or cross-origin navigations — do not present it as a universal unsaved-changes guard.
- Verify the installed major version of React Router or Next.js (`package.json`) before asserting version-specific route-module or App Router conventions; if the repo is on Pages Router or React Router v5/v6 classic mode, framework-mode/App Router guidance does not transfer directly.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- First map the full route tree (paths, layout nesting, index routes) before evaluating any individual route — a route's effective protection or focus behavior can depend on a parent layout's loader or wrapper.
- Classify every route as public or protected using the authorization model the team actually has, not assumption. A route is protected only if there is a server-side enforcement point: a React Router `loader` (or middleware forced to run via a paired loader) that redirects/throws, or a Next.js Data Access Layer check inside a Server Component/Server Action/Route Handler. Hiding a nav link, a client-side `useEffect` redirect, or a component-level conditional render is UX affordance only — treat any "protected" route lacking a paired server-side enforcement point as a blocking (HIGH) security finding, not a style note.
- Do not accept a Next.js `middleware`/proxy auth check as sufficient enforcement by itself; official Next.js guidance frames it as an optimistic edge check. Require the corresponding Data Access Layer check to also exist, and flag a middleware-only implementation as a HIGH finding even if the middleware matcher looks correct.
- When reviewing code-splitting, verify whether loader/component/action for a lazy route resolve via a parallel construct (e.g., `Promise.all`, or the framework's single `lazy` property that loads them together) versus sequential `await` calls that create a waterfall — the latter is a measurable performance regression, not a style preference.
- Treat any view-critical state (active filter, pagination page, selected tab, search query) that lives only in component state/memory as a defect if it should be shareable or survive a refresh — it belongs in the URL (path segment or search params), not only in memory. This is what breaks deep-links and the back button.
- Trace focus management on every route transition: identify what receives focus after navigation (a heading, the main landmark, or nothing) and whether an `aria-live` region announces the route/status change for assistive technology. Absence of either is an accessibility (WCAG 2.4.3 Focus Order / 4.1.3 Status Messages) finding, not a nice-to-have.
- For form-heavy routes, verify navigation-blocking (e.g., React Router `useBlocker`) exists for unsaved changes, and verify its known limits (SPA-only; does not cover hard reloads or cross-origin navigation) are either accepted knowingly or covered by a `beforeunload` handler for the hard-reload case.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only). Do not attempt to open a browser or simulate navigation to "check" focus behavior — trace it from source (component refs, `useEffect` on location change, `aria-live` regions) and label the finding `repo evidence` with a caveat that runtime confirmation was not performed.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the route-tree mapping table, and the required output shape.
- [Server-side enforcement patterns](references/server-enforcement-patterns.md) — load only when auditing whether a "protected" route has real server-side enforcement (React Router loader/middleware, Next.js Data Access Layer) versus client-side-only gating.
- [Code-splitting and URL state](references/code-splitting-and-url-state.md) — load only when reviewing lazy-loading/waterfall regressions or when view-critical state needs to move into the URL.
- [Focus management and navigation blocking](references/focus-and-navigation-blocking.md) — load only when reviewing focus/`aria-live` behavior on route transitions or unsaved-changes navigation blocking.

## Response minimum

Return, at minimum:

- a route-tree table: path, protection level with its server-side enforcement pointer (or "none found"), code-split boundary, and focus-management target,
- ranked findings with file:line evidence,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., runtime focus behavior not simulated, transitive import not read).
