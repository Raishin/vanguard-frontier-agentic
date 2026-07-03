# Server-Side Enforcement Patterns

Use this reference only when auditing whether a "protected" route has real server-side enforcement versus client-side-only gating. Load it during step 3 of the review workflow.

> Version note: React Router's `middleware` API and Next.js's Data Access Layer guidance are both relatively recent formalizations of patterns that existed informally before. Verify the installed major version before asserting these exact APIs exist; older codebases may implement equivalent checks through custom loader wrappers or `getServerSideProps`-era patterns.

## What people get wrong

The common bad assumption is:

> "The route component checks `if (!user) redirect('/login')`, so it's protected."

That check runs **inside the client-rendered tree**, after the route has already matched and after any component code (and often any data-fetching hooks) above the redirect has already begun executing. It also does nothing to stop a direct request for the route's data (an API call the component makes, or in Next.js, a Server Component that fetches before the redirect logic runs if the check is misplaced). A client-side redirect is a UX nicety for already-authenticated-but-wrong-state users; it is not an authorization boundary.

## Officially grounded enforcement shape

### React Router: loader (with middleware for forced execution)

- The authoritative enforcement point is a `loader` that throws a `redirect()` (or throws a 401/403 response) when the request is unauthenticated/unauthorized. `redirect` thrown from a loader is the documented pattern for gating protected routes.
- If middleware is used for cross-cutting auth logic, official React Router guidance notes that middleware alone does not run on every client-side navigation unless the route also has a `loader` — pairing an (even trivial, no-op) `loader` with the middleware forces the middleware to execute on every client-side transition into that route. A middleware with no paired loader is not a reliable enforcement point for client-side (SPA) navigations.
- A route whose only "protection" is a check inside its `Component`/element (not its `loader`) is a client-side-only gate — flag it.

Enforcement pointer to look for:

```tsx
export async function loader({ request }: Route.LoaderArgs) {
  if (!isLoggedIn(request)) {
    throw redirect("/login");
  }
  // ... fetch protected data only after the check
}
```

### Next.js: Data Access Layer (not middleware alone)

- Official Next.js guidance frames `middleware`/proxy-based auth checks as an **optimistic** check: it typically reads a session cookie's presence (not full validation) at the edge, primarily to redirect for UX purposes before a page even starts rendering.
- The authoritative check belongs in a **Data Access Layer**: a server-only module (marked with `import 'server-only'` or equivalent) that every Server Component, Server Action, and Route Handler touching sensitive data calls into. This layer performs the real `auth()`/session validation, an authorization check (does this user own/have access to this specific resource — guarding against IDOR), and returns only a minimal safe DTO.
- A route protected only by middleware, with no corresponding Data Access Layer check in the Server Component/Action/Route Handler that actually serves the data, is enforcement-incomplete — flag it as HIGH even if the middleware matcher is correctly scoped.
- Server Actions in particular must re-check authentication and authorization inside the action itself (not rely on the fact that the triggering page was gated) — official production-checklist guidance calls this out explicitly, including checking resource ownership (e.g., `post.authorId !== session.user.id`) to prevent IDOR, not just "is logged in."

Enforcement pointer to look for:

```ts
import 'server-only'
import { auth } from '@/lib/auth'
import { db } from '@/lib/db'

export async function deletePost(postId: string) {
  const session = await auth()
  if (!session?.user) throw new Error('Unauthorized')

  const post = await db.post.findUnique({ where: { id: postId } })
  if (post.authorId !== session.user.id) throw new Error('Forbidden')

  await db.post.delete({ where: { id: postId } })
}
```

## Non-negotiable design rules

1. **Every protected route needs a citable server-side enforcement pointer.** "The nav link is hidden" or "the component redirects" is not a pointer; a `loader` throw, a middleware-with-paired-loader, or a Data Access Layer call is.
2. **Do not treat Next.js middleware as sufficient by itself.** It is a UX-redirect layer over an optimistic check. Require the Data Access Layer check too, and flag the gap if it's missing even when middleware looks airtight.
3. **Check resource-level authorization, not just authentication.** "Is logged in" is necessary but not sufficient for routes/actions scoped to a specific resource (e.g., `/orders/:id`, `deletePost(postId)`) — verify an ownership/permission check exists, or the finding is an IDOR risk, not just a missing-auth risk.
4. **Server Actions and Route Handlers must self-check.** Do not assume a Server Action inherits protection from the page that renders the form that triggers it — the official guidance requires the action to re-verify.
5. **BFF/API-layer enforcement counts too.** If the frontend calls a backend-for-frontend or API layer that itself enforces authorization independent of the loader/DAL, that also satisfies the requirement — but verify it, don't assume "the API probably checks this."

## High-risk assumptions to kill

- "It's not in the nav, so users can't get there."
- "The middleware matcher covers this path, so it's protected." (Next.js — middleware alone is optimistic.)
- "The loader fetches the data, so of course it checks auth first." (Verify the check actually runs before the fetch, not after or not at all.)
- "The Server Action is only called from the protected page's form, so it inherits protection." (It does not; actions are independently callable.)
- "Checking `session?.user` is enough for a resource-scoped route." (Also check resource ownership/permission.)

## Safe verification targets

- Grep every route's `loader`/Server Component/Server Action/Route Handler for an auth check that occurs before any protected data fetch or mutation, and confirm it throws/redirects rather than just setting a flag a component might ignore.
- For Next.js, grep for `middleware.ts`/`proxy.ts` and confirm whether a corresponding Data Access Layer (`server-only` import) exists and is actually called by the routes the middleware matches.
- For React Router, confirm any auth `middleware` has a paired `loader` on the same route (or an ancestor layout route) to force execution on client-side navigations.

## When to push back

Push back if the user asks to:

- ship a route as "protected" based solely on hiding a nav item or a component-level redirect,
- rely on a Next.js middleware matcher alone as the complete authorization boundary,
- skip resource-ownership checks because "the user is already logged in,"
- skip re-checking auth inside a Server Action because "the calling page is already gated."

Those are not shortcuts. They are authorization bypasses waiting for a direct URL, a replayed request, or a changed session state.
