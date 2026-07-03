# Client-Side Guards as an Authorization Boundary, and Navigation Control-Flow Risks

Use this reference when the review scope includes a `router.beforeEach`/`beforeEnter` (or a
meta-framework's route-middleware equivalent), any redirect returned from a guard, a catch-all
route, or the app's `history` mode configuration. Covers rubric items 1, 5, 6, 7 (and their
"not a finding" counterparts 8, 13, 14, 15).

## What people get wrong

The naive assumption:

> "This route checks `isAuthenticated` in `beforeEach` and redirects to `/login` if it's false,
> so the route is protected."

Wrong in isolation. Vue Router's own guard mechanics confirm exactly what a guard is: a
JavaScript callback that runs *in the browser*, before a navigation is allowed to resolve. It
can return `false` to cancel the navigation, a route location to redirect, or (with the legacy
third argument) call `next()` — all of this is client-side control flow (`repo evidence`,
`/vuejs/router`, navigation-guards guide). None of it executes on a server the attacker doesn't
control. A guard is the correct place for *UX* gating — don't render a protected page's shell
to a user who hasn't logged in — but it is not, and cannot be, the mechanism that stops a
request from reaching protected data, because:

- the bundled JavaScript (and therefore the guard's logic) is fully visible and can be read,
  patched via devtools, or skipped by calling the underlying API/data-fetch directly;
- `isAuthenticated` itself is usually just client-held state (a token's presence, a decoded JWT
  claim, a Pinia/Vuex flag) that the guard reads — it is not an independent judgment the server
  makes on the attacker's actual request.

## Officially grounded requirement

Vue Router's documented guard pattern for authentication redirects is:

```js
router.beforeEach(async (to, from) => {
  if (
    !isAuthenticated &&
    // Avoid an infinite redirect
    to.name !== 'Login'
  ) {
    return { name: 'Login' }
  }
})
```

(`repo evidence`, `/vuejs/router`, navigation-guards guide — including the documented
`to.name !== 'Login'` self-exclusion check, which is the framework's own prescribed loop-avoidance
pattern, not an optional hardening step.) The docs also show the same pattern via `meta` fields:

```js
router.beforeEach((to, from) => {
  if (to.meta.requiresAuth && !auth.isLoggedIn()) {
    return { path: '/login', query: { redirect: to.fullPath } }
  }
})
```

(`repo evidence`, `/vuejs/router`, meta guide.) Nowhere in Vue Router's own documentation is a
guard described as an authorization mechanism that replaces server-side access control — the
docs frame it strictly as navigation control flow (cancel / redirect / proceed). The conclusion
that a guard must not be the *sole* boundary is standard web-security practice (client-side
controls are trivially bypassable; the server must independently authorize every request) rather
than a Vue-Router-specific API claim — treat this specific conclusion as `inference` grounded in
`documentation-based` general security guidance (OWASP: never rely on client-side enforcement
alone), layered on top of the `repo evidence` fact that guards are browser-executed callbacks.

Guards can also programmatically extend the route table and redirect in one motion:

```js
router.beforeEach(to => {
  if (!hasNecessaryRoute(to)) {
    router.addRoute(generateRoute(to))
    return to.fullPath
  }
})
```

(`repo evidence`, `/vuejs/router`, dynamic-routing guide.) If `generateRoute()` builds a route
pattern from untrusted input, the pattern itself becomes an injection surface — treat this the
same as any other user-controlled routing input.

## Non-negotiable design rules

### 1. A guard finding requires evidence of an ABSENT server-side check, not just a present client check

Do not flag "there is a client-side guard" as the defect. The finding is: this route's guard
exists, **and** the review found no evidence that the API/data endpoint the route depends on
independently checks authorization on the server. Look for: a 401/403 test or comment, a BFF
layer, server middleware, or an API contract doc showing enforcement. If such evidence exists,
the guard is correctly UX-only and this is not a finding (rubric item 8).

### 2. Distinguish "guard exists with server enforcement" from "guard is believed to be enough"

Ask explicitly, in the finding write-up, whether the codebase (or the user) asserts the guard
*is* the security boundary ("the API trusts anyone who reached this route") versus treats it as
UX convenience layered over real server checks. The former is the HIGH finding; the latter,
with visible server-side evidence, is not.

### 3. Redirect self-exclusion prevents the documented loop failure mode

Every unconditional-redirect guard (`if (!isAuthenticated) return { name: 'Login' }` with no
exclusion for the login route itself) is a HIGH finding: it produces an infinite redirect loop
the moment the guard runs on the login route's own navigation. The fix is the framework's own
documented pattern (`to.name !== 'Login'` or equivalent). A guard that already includes this
check is correctly implemented (rubric item 13) — do not invent a residual risk without a
concrete counter-path.

### 4. A guard-returned or `next()`-passed redirect target must itself be traced

If the redirect location returned by a guard (or passed to `next(...)`) is built from
`to.query`/`to.params` rather than a hardcoded route, trace that value the same way you would
trace an open-redirect target (see `open-redirect-and-injection.md`) — a guard can reintroduce
the exact same open-redirect defect class it was written to prevent.

### 5. Catch-all routes need their actual render path checked, not just their existence

Vue Router's documented catch-all pattern is `{ path: '/:pathMatch(.*)*', name: 'NotFound',
component: NotFound }` (`repo evidence`, `/vuejs/router`, dynamic-matching guide). This pattern
alone is correct and not a finding. It becomes a finding only when: (a) the matched component
renders `route.params.pathMatch` through an unsanitized sink, or (b) route table ordering
causes the catch-all to shadow a route that should require authentication (read the full routes
array in declaration order — Vue Router matches in the order routes are registered).

### 6. History-mode server configuration is out of the router's control but in scope for the review

`createWebHistory()` (`repo evidence`, `/vuejs/router`, history-mode guide) produces
clean URLs but requires the server to serve the app's `index.html` for any path the SPA should
handle client-side. This is not itself a router-code defect, but the review must check for
evidence of the corresponding server/proxy fallback rule (nginx `try_files`, a framework's SPA
middleware, a static-host rewrite rule) — its *absence* breaks direct links/refreshes, and an
*overly broad* fallback (matching `/api/*` or asset paths that should 404 or route elsewhere)
can leak information or serve the app shell where a different, more restrictive handler was
intended.

## Minimal safe pattern

```js
// Guard: UX-only redirect, with the documented loop-avoidance check.
router.beforeEach((to, from) => {
  if (to.meta.requiresAuth && !authStore.isAuthenticated && to.name !== 'Login') {
    return { name: 'Login', query: { redirect: to.fullPath } }
  }
})

// Server / BFF: the actual authorization boundary (illustrative — lives outside router code).
// GET /api/account -> 401 if session invalid, regardless of any client-side navigation state.
```

Anti-pattern (guard as the only boundary — do not approve without server-side evidence):

```js
router.beforeEach((to, from) => {
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    return { name: 'Login' } // no `to.name !== 'Login'` check -> loop risk
  }
  // No evidence anywhere in the reviewed diff/API that /api/account (or whatever
  // this route fetches) independently checks the session server-side.
})
```

## Adversarial checklist

- If a user disables JavaScript, or calls the route's underlying API directly with curl/devtools,
  does the server independently reject the unauthorized request? If unknown, ask — do not assume
  yes.
- Does every unconditional redirect-on-guard have a self-exclusion for its own target route?
- Is any guard's redirect destination built from `to.query`/`to.params` rather than a hardcoded
  name/path? If so, cross-reference `open-redirect-and-injection.md`.
- Does `router.addRoute()` (if called from within a guard) build its route definition from any
  user-reachable input?
- Read the full `routes` array in registration order — does a catch-all or broad pattern appear
  before a route that should take precedence and require auth?
- Does `createWebHistory()` appear with no server config file in the reviewed diff/repo showing
  the SPA-fallback rule? Flag this as an open question if the server config is out of scope
  rather than assuming it is either correct or missing.

## Verification targets

- Grep for `router.beforeEach(`, `beforeEnter:`, and meta-framework middleware files
  (`middleware/`, `definePageMeta`) and read each guard's full body.
- Grep for `to.name !==`, `from.name !==`, or equivalent self-exclusion checks near a redirect
  return/`next(...)` call inside each guard found above.
- Grep for `router.addRoute(` calls inside guard bodies and trace the argument's construction.
- Grep for `pathMatch(.*)` / `path: '*'` and read the matched component's template for any
  `route.params.pathMatch` usage.
- Grep for `createWebHistory(` and check the repo for a corresponding server config file
  (`nginx.conf`, `vercel.json`, `netlify.toml`, a framework's server entry) with a fallback rule;
  note explicitly if that file is outside the review's scope.
