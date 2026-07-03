# Server Action Authorization and Client Environment Exposure

Use this reference only when the review scope includes a `'use server'` directive (a Server Action/Function) or a `'use client'` module reading `process.env`.

## What people get wrong

The naive assumption for Server Actions is:

> "It's a `'use server'` function, so it only runs on the server, and only my app's own UI calls it — that's authorization enough."

Wrong. A `'use server'` directive controls *where the code executes*, not *who is allowed to invoke it*. Next.js compiles every Server Action into a callable server endpoint; anyone who can reach that endpoint — not just users going through the app's own rendered UI — can invoke it directly with an arbitrary argument. A Server Action with no session check is an unauthenticated mutation endpoint. A Server Action with a session check but no ownership check is an authenticated *Insecure Direct Object Reference* (IDOR): any logged-in user can pass any other user's resource ID.

The naive assumption for client-side env access is:

> "I need a config value in this Client Component, so I'll just read `process.env.SOMETHING` like I would on the server."

Wrong. In a `'use client'` module, only environment variables the framework has explicitly exposed to the browser bundle are ever available at runtime; anything else is `undefined` once the code actually runs in the browser, no matter what it appeared to be when read in server-side tooling or type definitions. Beyond the functional bug, the read itself is a signal the developer's mental model of the server/client boundary is broken — a strong prompt to check the rest of that same module for other assumptions that might leak something real.

## Officially grounded rules

Next.js's own documentation shows the required Server Action authorization pattern directly:

```tsx
'use server'

import { auth } from '@/lib/auth'
import { db } from '@/lib/db'

export async function deletePost(postId: string) {
  const session = await auth()
  if (!session?.user) {
    throw new Error('Unauthorized')
  }

  const post = await db.post.findUnique({ where: { id: postId } })

  // Check that the user owns this resource
  if (post.authorId !== session.user.id) {
    throw new Error('Forbidden')
  }

  await db.post.delete({ where: { id: postId } })
}
```

(`documentation-based`, Next.js docs: Server Action data-security guide.) Both checks are present: a session check (`session?.user`) proving the caller is authenticated, and an ownership check (`post.authorId !== session.user.id`) proving the caller may act on *this specific* resource — the session check alone does not imply the ownership check.

Next.js's documentation is equally direct on client environment access: `'use client'` modules can read environment variables prefixed `NEXT_PUBLIC_`; anything else read server-side (e.g. `process.env.DATABASE_URL`) is a server-only value with no client-side equivalent (`documentation-based`, Next.js docs).

## Non-negotiable design rules

### 1. A session check is necessary but not sufficient for resource-scoped mutations

For a `'use server'` function that takes an ID and mutates/deletes/reads the corresponding resource, require both: a session check (is anyone authenticated at all) and an ownership/authorization check (may *this* authenticated user act on *this specific* resource). A function that checks only the former and trusts the caller-supplied ID unconditionally is IDOR-shaped.

### 2. Do not accept "the UI never lets you delete someone else's post" as a control

A Server Action is a callable endpoint independent of what the rendered UI happens to expose. The fact that the app's own components never construct a request for another user's resource ID says nothing about what a direct request to the compiled action endpoint can supply.

### 3. `NEXT_PUBLIC_` is the only sanctioned bridge for client-visible env values

There is no other supported mechanism for exposing a build-time environment value to `'use client'` code short of an explicit runtime fetch to a server endpoint that itself performs its own authorization. A non-`NEXT_PUBLIC_` read in a `'use client'` file is either dead code that silently resolves to `undefined`, or evidence of a broken assumption worth investigating further in the same module.

### 4. Trace `'use server'` scope precisely

A file-level `'use server'` directive applies to every export in that file; a function-level `'use server'` directive applies only to that function. Confirm which form is in use before asserting a specific exported function is a Server Action in scope for this review.

## Minimal safe implementation pattern

```tsx
'use server'

import { auth } from '@/lib/auth'
import { db } from '@/lib/db'

export async function deletePost(postId: string) {
  const session = await auth()
  if (!session?.user) {
    throw new Error('Unauthorized')
  }

  const post = await db.post.findUnique({ where: { id: postId } })
  if (post.authorId !== session.user.id) {
    throw new Error('Forbidden')
  }

  await db.post.delete({ where: { id: postId } })
}
```

Anti-pattern (do not approve):

```tsx
'use server'

export async function deletePost(postId: string) {
  await db.post.delete({ where: { id: postId } })
}
```

Client environment access:

```tsx
'use client'

// Safe: build-time-injected public variable.
const publicEndpoint = process.env.NEXT_PUBLIC_API_URL
```

```tsx
'use client'

// Unsafe: undefined at runtime, and a signal of a broken server/client mental model.
const apiKey = process.env.DATABASE_URL
```

## Adversarial checklist

Before clearing a `'use server'` function as safe:

- Is there a session check (`session?.user` or the framework's equivalent) before any read/write/delete against the data layer?
- If the function mutates or reads a specific resource by ID, is there a separate ownership/authorization check comparing the resource's owner to the authenticated user?
- Could a caller invoke this function directly (bypassing the app's own rendered UI) with an arbitrary ID and reach the mutation regardless of what the UI would normally allow?

Before clearing a `'use client'` module's environment access as safe:

- Does every `process.env.*` read in this file use the `NEXT_PUBLIC_` prefix (or the framework's equivalent public convention)?
- If not, is the read genuinely unreachable dead code, or does it suggest the developer believed a server-only value would be available here?

If any answer reveals a gap, the finding is HIGH (`action-authz-missing`) or MEDIUM (`env-exposure-in-client`) — do not soften either to "worth double-checking."

## Verification targets

- Grep for `'use server'` (file-level and function-level) and enumerate every exported function in scope.
- For each, grep the function body for an `auth(`/`session` check and, separately, for an ownership comparison (`.authorId`, `.userId`, `.ownerId`, or equivalent) against the authenticated session before any `db.*.delete(`/`db.*.update(`/mutating call.
- Grep for `'use client'` and, within each such file, grep for `process.env.` reads; flag every one not prefixed `NEXT_PUBLIC_`.
