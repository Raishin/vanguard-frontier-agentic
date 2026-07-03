# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load
the two domain references only for the specific defect class the routing code under review
actually raises.

## Prerequisites

- Confirm Vue Router is actually in use (`package.json` — `vue-router`) and identify the major
  version. Guard signatures (`next` third argument vs. return-value guards), redirect object
  shape, and catch-all syntax (`/:pathMatch(.*)*` vs. legacy `*`) differ across Vue Router 3 vs.
  4 — do not apply v4 syntax expectations to a v3 codebase or vice versa.
- Identify whether the app runs in `history` mode (`createWebHistory`) or hash mode
  (`createWebHistory` vs. `createWebHashHistory`) — the server-fallback concern
  (`client-guards-and-control-flow.md`, rule 6) only applies to `history` mode.
- Identify whether this is a meta-framework (Nuxt) rather than raw Vue Router — Nuxt's
  `definePageMeta`/route middleware wraps the same underlying guard concepts but with different
  file-based conventions; note this explicitly rather than assuming `router.beforeEach` exists
  verbatim in a Nuxt app.

## Workflow

1. **Locate every navigation guard.** Grep for `router.beforeEach(`, `beforeEnter:` on route
   definitions, and (for Nuxt) `middleware/` files or `definePageMeta({ middleware: ... })`.
   Read each guard's full body.
2. **For each guard that gates a protected route, look for the server-side counterpart.**
   Identify what API/data endpoint the protected route depends on and search for evidence
   (tests, comments, a BFF layer, an API contract doc) that the endpoint independently enforces
   authorization server-side. Absence of such evidence is the finding — see
   `client-guards-and-control-flow.md`.
3. **Trace every redirect.** For guard-returned redirects, `next(...)` calls, and any
   post-login redirect flow, determine: is the target hardcoded/same-origin, or built from
   `to.query`/`to.params`/`route.query`? If the latter, trace whether it is validated against a
   same-origin/relative-path allowlist before use. See `open-redirect-and-injection.md`.
4. **Enumerate every dynamic `:to`/`:href` binding in scope.** Trace each backward through
   props, computed values, and store state to its origin. Check for a visible protocol
   allowlist on the exact traced path when the origin includes user-reachable input.
5. **Enumerate every `v-html`/`innerHTML` sink in scope that touches `route.params`/
   `route.query`.** Trace backward the same way; check for a named sanitizer call on the exact
   path.
6. **Check guard self-exclusion and route-table ordering.** For every unconditional
   redirect-on-guard, confirm a self-exclusion check exists for the guard's own redirect target.
   Read the full `routes` array in declaration order and check whether a catch-all or broad
   pattern could shadow a route that should require authentication.
7. **Check `history`-mode server configuration when in scope.** If `createWebHistory()` is used,
   look for a server/proxy config file with an SPA-fallback rule; note explicitly if that file
   is outside the review's scope rather than assuming correctness.
8. **Produce ranked findings** using the output contract below.

## Decision tree

- A guard redirects unauthenticated users away from a protected route, and no evidence exists
  that the route's underlying API/data endpoint independently enforces authorization
  server-side → **HIGH** finding, `client-guard-as-sole-authz`. Cite Vue Router's own framing of
  guards as client-side navigation control flow (`repo evidence`) plus general server-side-
  enforcement practice (`inference`/`documentation-based`).
- A guard redirects unauthenticated users away from a protected route, **and** the route's
  server endpoint is confirmed to independently reject unauthorized requests → not a finding;
  state explicitly that the guard is correctly UX-only.
- A post-auth redirect flow passes `route.query.redirect`/`returnUrl` (or equivalent) into
  `router.push`/`window.location` with no same-origin/relative-path validation → **HIGH**
  finding, `open-redirect`.
- The same flow validates the value against a same-origin/relative-path allowlist before use →
  not a finding.
- A dynamic `:to`/`:href` binding's traced source includes user-reachable input with no
  protocol allowlist on the exact path → **HIGH** finding, `scheme-injection` (reachable via a
  crafted profile/link field triggering `javascript:`/`data:` execution on click).
- The same binding's source is a literal, a named-route object, or already passes a protocol
  allowlist on the traced path → not a finding.
- `route.params`/`route.query` reaches a `v-html` binding or manual `.innerHTML` assignment
  with no named sanitizer call on the exact traced path → **HIGH** finding, `reflected-xss`.
- `route.params`/`route.query` is used only via text interpolation, non-rendering logic, or
  passes through a sanitizer call on the traced path → not a finding.
- An unconditional redirect-on-guard has no self-exclusion check for its own redirect target →
  **HIGH** finding, `redirect-loop` (Vue Router's own docs prescribe the `to.name !== 'Login'`
  self-exclusion pattern as required, not optional).
- The guard already includes a self-exclusion check → not a finding.
- A guard's redirect target (or a `next(...)` argument) is itself built from unvalidated
  `to.query`/`to.params` → **HIGH** finding, `open-redirect` (the guard reintroduces the same
  defect class it exists to prevent) — cross-reference the open-redirect rule above.
- A catch-all route (`/:pathMatch(.*)*`) renders `route.params.pathMatch` through an
  unsanitized sink, or route-table ordering lets it shadow a route that should require auth →
  **MEDIUM-to-HIGH** finding, `catch-all-misconfig`, depending on what is actually exposed.
- A catch-all route renders only a static not-found component with no broader-than-intended
  matching (confirmed via route-table order) → not a finding.
- `createWebHistory()` is used with no evidence of a corresponding server-side SPA-fallback
  rule in scope → note as an **open question**, not a confirmed finding, unless the server
  config is actually in the reviewed diff/repo and shown to be missing or overly broad, in
  which case it is a **MEDIUM** finding, `history-mode-server-misconfig`.
- `createWebHistory()` is paired with a reviewed, correctly scoped server-fallback rule → not a
  finding; state it as reviewed-and-safe.

## Output contract

Every response from this skill must return:

1. **Scope** — the guard(s), redirect flow(s), dynamic link binding(s), route table, and/or
   `v-html`/`innerHTML` sink(s) reviewed.
2. **Ranked findings** — each with file:line, defect category (`client-guard-as-sole-authz` /
   `open-redirect` / `scheme-injection` / `reflected-xss` / `redirect-loop` /
   `catch-all-misconfig` / `history-mode-server-misconfig`), the concrete data-flow trace (every
   hop from origin to sink, or the guard-to-endpoint gap for the authz-boundary category), and
   a fix sketch matching Vue Router's documented pattern.
3. **Server-side enforcement status for every guard finding** — an explicit statement of
   whether evidence of independent server-side authorization was found, and where; never infer
   server enforcement exists without evidence on the traced path.
4. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`.
   Label structural-risk findings (e.g., "no evidence of server enforcement") as structural
   risk, not confirmed exploitation — proving actual bypass requires a live request against the
   real API, which this skill does not perform.
5. **Verdict** — approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** — e.g., "confirming the API layer actually rejects
   unauthorized requests requires a live request or reading server-side code outside this
   diff's scope," or "server SPA-fallback configuration is outside the reviewed files; flag for
   a separate infra review."

## When to push back

Push back if the user asks to:

- approve a route as "protected" solely because a `beforeEach`/`beforeEnter` guard redirects
  unauthenticated users, with no evidence the underlying API/data endpoint independently
  enforces authorization — a client-side redirect is not access control,
- treat a redirect-target validation as done because "we checked it's a string" or "we blocked
  `http://`" — a type check or substring blocklist is not an allowlist and does not clear an
  open-redirect finding,
- approve a dynamic link binding because "Vue Router handles URLs safely" — Vue Router does not
  validate URL schemes; that is application-code responsibility,
- skip tracing a `route.params`/`route.query` value into `v-html` because "it's just the search
  box echoing what the user typed" — a reflected payload via a shared URL is exploitable
  immediately, with no stored/second-order step required,
- downgrade a missing self-exclusion check in a redirect guard to informational because "it
  hasn't looped in testing" — Vue Router's own docs treat this check as required, not optional,
  and the loop condition depends on navigation order that testing may not have exercised.
