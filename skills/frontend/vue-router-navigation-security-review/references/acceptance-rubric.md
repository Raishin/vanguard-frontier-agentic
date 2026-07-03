# Acceptance Rubric (write first, satisfy second)

This is the failing test for this skill. Every item below must be traceable to a specific
operating rule or reference file in this skill before the skill is considered complete. The
`selfCheck` returned by the authoring agent must map each item to what satisfies it.

## MUST CATCH — concrete defects a correct review of this domain must flag

1. **Client-side guard as sole authorization boundary.** A `router.beforeEach`/`beforeEnter`
   (or a meta-framework's `definePageMeta`/middleware equivalent) checks `isAuthenticated` or a
   role flag and redirects if false/absent, but no server-side check exists on the
   corresponding API/data-fetch call the protected route triggers. Reachable/exploitable by:
   directly calling the underlying API endpoint (curl, browser devtools fetch) while bypassing
   the SPA entirely, or by disabling JavaScript / patching the bundled guard function in
   devtools before navigation resolves. The guard only ever gates the client-side route
   render; the server has no independent memory of "this user passed the guard."

2. **Open redirect via `route.query.redirect`/`returnUrl`.** A login flow reads
   `to.query.redirect` (or `route.query.returnUrl`) and passes it unvalidated into
   `router.push(redirectTarget)` or `window.location.href = redirectTarget` after
   authentication succeeds. Reachable by: crafting a login URL such as
   `/login?redirect=https://evil.example.com` (or a protocol-relative `//evil.example.com`),
   which sends an authenticated user's session to an attacker-controlled origin post-login —
   a classic phishing/token-leak primer, since the URL often still carries a valid session
   token or referrer header to the destination.

3. **`javascript:`/`data:` scheme injection via dynamic `:to`/`:href`.** A `<router-link :to="...">`
   or a plain `<a :href="...">` built from user-reachable input (route params, query string,
   API response echoing user content) is bound without scheme validation. Reachable by:
   submitting a value like `javascript:alert(document.cookie)` or a `data:text/html;base64,...`
   payload as the "profile link"/"website" field, which executes when a victim clicks the
   rendered link.

4. **Route params/query interpolated into `v-html`/`innerHTML` (reflected XSS through the
   router).** A component reads `route.params.x` or `route.query.q` and renders it via
   `v-html` (or manually assigns `.innerHTML`) with no sanitizer call on that exact path.
   Reachable by: a crafted URL like `/search?q=<img src=x onerror=alert(1)>` shared with a
   victim — the payload never touches a database; it reflects straight from the URL into the
   rendered DOM.

5. **`next(unvalidatedPath)` / guard-returned redirect creating a loop or bypass.** A
   `beforeEach` guard redirects unconditionally (e.g., every unauthenticated user to `/login`)
   without excluding the target route itself (`to.name !== 'Login'`), producing an infinite
   redirect loop; or a guard's redirect target is itself built from unvalidated `to.query`/
   `to.params`, letting a crafted URL redirect through/around the intended destination.

6. **Overly-broad catch-all route with no security review.** A `{ path: '/:pathMatch(.*)*' }`
   (or legacy `*`) catch-all route is wired to a component or redirect that assumes it only
   ever receives "not found" traffic, but the component actually renders `route.params.pathMatch`
   unsanitized, or the catch-all silently matches and serves a path that should have 404'd
   (e.g., shadowing a more specific route that was supposed to require auth).

7. **`history` mode (`createWebHistory`) with missing/incorrect server SPA fallback.** The app
   uses `createWebHistory()` (clean URLs) but the review has no evidence the server is
   configured to serve `index.html` for unmatched paths — either producing a broken
   direct-link/refresh experience, or (the security-relevant variant) exposing raw source/
   config files at paths the SPA never intended to be routable because the server's fallback
   rule is too permissive (e.g., serving `index.html` for `/api/*` or static-asset-looking
   paths that should 404 or hit a different handler).

## MUST NOT FLAG — benign patterns that must not produce a finding

8. A `beforeEach`/`beforeEnter` guard used purely for UX redirection (e.g., "send anonymous
   users to `/login` so they don't see a blank protected page") **when the review has evidence
   the corresponding server endpoint/API independently enforces authorization** (a 401/403 on
   direct call, a documented server-side check, or a BFF that re-validates the session). This
   is the correct pattern, not a finding — do not flag "client guard exists" alone as a defect;
   flag only the *absence* of the server-side counterpart.

9. A redirect target that is a static, hardcoded, same-origin path (e.g.,
   `redirect: { name: 'Login' }` or `router.push('/dashboard')`) — no user input reaches the
   destination, so there is no open-redirect surface regardless of how the redirect is
   triggered.

10. A redirect that reads `to.query.redirect`/`returnUrl` but **validates it against an
    allowlist of same-origin relative paths** (e.g., confirms the value starts with a single
    `/` and is not protocol-relative `//`, or resolves it with `new URL(value, window.location.origin)`
    and confirms the resolved origin matches) before using it. Validated same-origin redirects
    are the documented-safe pattern, not a finding.

11. A `:to`/`:href` binding whose value is a compile-time literal, a named-route object
    (`{ name: 'user', params: { id } }`), or a value confirmed to have already passed scheme
    allowlisting — do not re-flag a binding that already has a visible scheme check on its
    exact data-flow path.

12. `route.params`/`route.query` values consumed only via text interpolation (`{{ }}`), passed
    to `router.push`'s own `path`/`name`/`params` fields, used in non-rendering logic (API
    query params, conditional branching), or rendered through a sanitizer call on the traced
    path — Vue's default template interpolation auto-escapes; only the `v-html`/`innerHTML`
    sink is the defect, not the mere presence of `route.params`/`route.query` in a component.

13. A `beforeEach` guard with an explicit self-exclusion check (`to.name !== 'Login'`, or
    equivalent) before redirecting — the documented loop-avoidance pattern is correctly
    applied; do not flag it as a residual risk without a concrete path that still loops.

14. A catch-all route (`/:pathMatch(.*)*`) that renders only a static "not found" component
    with no rendering of `route.params.pathMatch` and no broader-than-intended matching
    behavior (confirmed by reading the actual route table ordering) — this is the documented,
    correct catch-all pattern.

15. `createWebHistory()` paired with confirmed server-side SPA-fallback configuration (e.g., a
    reviewed nginx `try_files`/framework adapter config that serves `index.html` only for
    non-asset, non-API paths) — not a finding; state it as reviewed-and-safe.

## Mapping requirement

Every one of items 1–7 must be covered by a decision-tree rule in
`references/workflow-and-output.md` and a detailed rule in one of the two domain reference
files. Every one of items 8–15 must appear as an explicit "not a finding" branch in the same
places — silence is not sufficient; the skill must say *why* the benign pattern is not flagged.
