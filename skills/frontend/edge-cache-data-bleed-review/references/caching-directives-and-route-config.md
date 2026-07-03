# Caching Directives and Route Segment Config

Load this reference when reviewing `revalidate`, `'use cache: private'`, `generateStaticParams`, or `dynamic` route segment config.

## `revalidate` and per-user data

`export const revalidate = N` (a route segment config export) tells Next.js to treat the rendered output of that route as fresh for up to `N` seconds, then re-render and re-cache it on the next request after the window elapses. This is a single, shared cache entry keyed on the route path (and any static params) -- it is not scoped to an individual visitor. If the render reads `cookies()`, `headers()`, or any other per-request/per-user API, the rendered HTML captured in that cache entry belongs to whichever request happened to trigger the re-render, and every other visitor who hits the same URL within the window receives that same cached, personalized output.

```tsx
// DANGEROUS: shared 60-second cache entry serves one user's session data to
// every visitor who hits this route within the window.
export const revalidate = 60

export default async function DashboardPage() {
  const session = (await cookies()).get('session')?.value
  const user = await db.users.findBySession(session)
  return <Dashboard user={user} />
}
```

The documented fix is not to shorten the window -- it is to remove the route-level `revalidate` export for personalized content and isolate the per-user lookup behind `'use cache: private'` instead (see below), or to make the route fully dynamic.

## `'use cache: private'`

`'use cache: private'` is a directive placed as the first statement inside a function body. It allows the function to read runtime request APIs (`cookies()`, `headers()`, `searchParams`) from within a cached scope, but the result is **never stored on the server** -- it is cached only in the requesting user's own browser, making it per-user by definition and safe to combine with cookie-derived lookups.

```tsx
async function getUser() {
  'use cache: private'
  const session = (await cookies()).get('session')?.value
  return db.users.findBySession(session)
}
```

Two structural checks matter here, not just the directive's presence:

- The directive must be the function's own first statement -- a function that reads `cookies()` and calls out to *another* function that has the directive does not itself get the protection unless the `cookies()` read happens inside the directive-scoped function.
- A function with `'use cache: private'` still needs the actual per-request read (`cookies()`, `headers()`) to happen inside it for the isolation to be meaningful -- a function that has the directive but reads no per-request data isn't wrong, but it also isn't evidence that some *other*, undirected function elsewhere is safe.

## `generateStaticParams` and personalized routes

`generateStaticParams` pre-computes which dynamic-segment values (`[id]`, `[userId]`, etc.) Next.js should statically render for a route. When the enumerated values are user- or account-scoped IDs and the route also renders authenticated, per-user data, pairing this with `revalidate` reproduces the same shared-cache-entry problem as above, except now the cache key includes the user ID segment -- so specifically, requests for `/account/123` from *different* people (e.g. a shared/public device, or a URL passed between users) hitting the route within the same revalidate window get the same cached response for that ID, not a fresh per-request one.

```tsx
// DANGEROUS: authenticated per-user page pre-rendered by ID, then reused for
// 60 seconds regardless of who requests it next.
export const revalidate = 60

export async function generateStaticParams() {
  const users = await db.users.findAll()
  return users.map((u) => ({ userId: u.id }))
}
```

The documented alternative for a route whose content must always reflect the current requester's own authenticated view is `export const dynamic = 'force-dynamic'`, which forces the route to render fresh on every request and skip both the static pre-render and the ISR cache entirely. Do not pair `generateStaticParams` enumerating auth-scoped IDs with `revalidate` on a route that renders session-derived content -- if the enumerated IDs are genuinely public and non-personalized (e.g. public blog post slugs), this pairing is fine and not a finding.

## Reading the trace correctly

When flagging any of the above, name the exact per-request API call (`cookies()`, `headers()`) and its exact reachability path from the flagged route-segment-config export -- a route segment config alone, with no per-request read anywhere in its render tree, is not a finding; conversely, a per-request read with no route-segment-config export at all is still a `missing-private-cache-boundary` finding on its own (see the main workflow reference), because a future caller could wrap it in a shared cache scope.
