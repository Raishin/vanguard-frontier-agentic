# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific defect class the client code under review actually raises.

## Prerequisites

- Read `package.json` first to confirm which GraphQL client library (Apollo Client, urql, graphql-request, or other) and version are in use. Apollo Client APIs (`connectToDevTools`, `resetStore`, `clearStore`, `SetContextLink`, `PersistedQueryLink`) do not transfer 1:1 to other clients -- confirm the equivalent construct before applying Apollo-specific guidance to a non-Apollo codebase.
- Identify every place the client is instantiated (`new ApolloClient({...})` or equivalent) -- a codebase may construct more than one client (e.g., an authenticated client and a public client), and each needs its own review pass.

## Workflow

1. **Locate every client instantiation.** For each, read the full construction call, including every option passed.
2. **Check `connectToDevTools`.** Is it an unconditional `true`, absent (defaulting per the client's own behavior), or gated on an environment check? See `references/devtools-and-auth-header-risk.md`.
3. **Locate every logout / account-switch / session-teardown handler.** For each, trace whether `resetStore()` or `clearStore()` (or the equivalent for the client in use) is called on the path between the auth-invalidation call and any subsequent navigation. See `references/cache-lifecycle-and-query-surface.md`.
4. **Trace the link chain for persisted-query allowlisting.** Locate the `link` option passed to the client. Determine whether a `PersistedQueryLink` (or equivalent allowlisting link) sits between the application code and the transport link, or whether arbitrary client-authored query documents reach the transport link unfiltered. See `references/cache-lifecycle-and-query-surface.md`.
5. **Trace every auth-header-injection link** (`SetContextLink`, an `ApolloLink` middleware, or equivalent). Determine whether an anti-CSRF header (e.g. `x-csrf-token`) is attached alongside the `authorization` header, and whether the same auth token could also be attached automatically via a cookie-backed session. See `references/devtools-and-auth-header-risk.md`.
6. **Enumerate cache `typePolicies` (or equivalent field-policy config) against every sensitive field reachable by a query in scope.** For fields carrying payment data, `cvv`, SSNs, tokens, or other PII, check for a masking `read()` policy that prevents the raw value from being normalized into the cache. See `references/cache-lifecycle-and-query-surface.md`.
7. **Produce ranked findings** using the output contract below.

## Decision tree

- `connectToDevTools: true` (or any construction that is unconditionally on, with no environment gate) → **HIGH** finding, devtools/introspection exposure. Cite Apollo Client's own devtools-configuration documentation (`repo evidence` via Context7 `/apollographql/apollo-client`).
- `connectToDevTools` gated on a build-time environment check (e.g. `process.env.NODE_ENV === 'development'`) → not a finding.
- A logout/session-teardown handler invalidates the server-side session but never calls `resetStore()`/`clearStore()` on that same path → **HIGH** finding, cross-user cache exposure risk on the client. The risk is structural (shared device, fast re-login, profile switch) regardless of whether it has been reported as an incident.
- A logout handler calls `resetStore()`/`clearStore()` on the exact traced path → not a finding.
- Client's link chain wires application operations directly to a transport link (`HttpLink` or equivalent) with no `PersistedQueryLink` (or equivalent allowlist) in between → **MEDIUM-to-HIGH** finding depending on reachability (public/unauthenticated endpoint is higher; requiring an authenticated session is still a finding, since a legitimate-but-malicious authenticated client can still abuse an unallowlisted surface).
- Client's link chain includes a persisted-query allowlisting link between application code and the transport link → not a finding.
- Auth-header-injection link attaches `authorization` with no anti-CSRF header, and the same token or an equivalent session credential can also travel automatically (cookie-backed) → **MEDIUM-to-HIGH** finding depending on whether the mutation surface is state-changing.
- Auth-header-injection link attaches both `authorization` and an anti-CSRF header (e.g. `x-csrf-token`), or the token is exclusively attached by explicit client code with no cookie-based fallback anywhere in the auth flow (state this explicitly) → not a finding.
- A sensitive field (payment data, `cvv`, SSN, token, other PII) reachable by an in-scope query has no masking `read()` policy in `typePolicies` (or equivalent) and is normalized into the cache as-is → **HIGH** finding, unmasked sensitive data cached -- visible in the devtools inspector for the life of the entry regardless of the current devtools-enablement finding.
- The same sensitive field has a masking `read()` policy (or the field is never requested by any in-scope query) → not a finding, but state this explicitly rather than omitting the field from the review.

## Output contract

Every response from this skill must return:

1. **Scope** -- the client instantiation(s), link chain(s), cache configuration, and/or logout handler(s) reviewed.
2. **Ranked findings** -- each with file:line, defect category (`devtools-exposure` / `cache-not-cleared` / `unallowlisted-query-surface` / `csrf-less-auth-header` / `unmasked-sensitive-field`), the concrete configuration or data-flow trace (naming every hop), and a fix sketch matching Apollo Client's documented pattern.
3. **Cache-clearing status per logout finding** -- an explicit statement of whether `resetStore()`/`clearStore()` is present on the traced path; never infer one exists elsewhere.
4. **Masking status per sensitive-field finding** -- an explicit statement of whether a `typePolicies` masking policy is present for that exact field; never infer downstream handling is safe.
5. **Evidence level per finding** -- `repo evidence`, `documentation-based`, or `inference`. Label structural risk findings as structural risk explicitly -- do not imply confirmed exploitation without live evidence (e.g., a captured cross-user cache read, a load-test reproduction of query-cost abuse).
6. **Verdict** -- approve / approve-with-notes / block.
7. **Open questions or out-of-scope items** -- e.g., "confirming actual query-cost abuse requires live load testing against the GraphQL server, not static review," or "server-side field-level authorization is out of scope for this client-focused skill -- recommend a server-side resolver review if that surface is in question."

## When to push back

Push back if the user asks to:

- approve `connectToDevTools: true` because "it's only in the staging build" without a visible environment gate on that exact literal -- staging today is production tomorrow, and the construct itself is what is being reviewed,
- approve a logout handler because "the cache clears itself on reload" -- a reload is not guaranteed (SPA navigation without a full page reload, a shared-device profile switch) and the handler under review is what must demonstrably clear it,
- treat an authenticated-only GraphQL endpoint as immune to unallowlisted-query-surface risk -- an authenticated session does not prevent a legitimate-but-malicious client from sending expensive, unallowlisted queries,
- clear a CSRF finding on the assumption that "the token is in a header, so cookies aren't involved" without checking whether any part of the auth flow also relies on a cookie-backed session,
- skip masking review for a sensitive field because "we'll disable devtools in production" -- the unmasked cache entry is a data-exposure risk independent of whether devtools happen to be reachable in the environment being reviewed.
