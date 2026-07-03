---
name: graphql-client-security-review
description: Statically review GraphQL client configuration (Apollo Client, and urql/similar clients by analogy) for production-enabled devtools/introspection exposure, a normalized cache left uncleared across user sessions, missing persisted-query allowlisting against client-driven query abuse, auth headers attached with no CSRF protection, and sensitive fields cached unmasked -- grounded in Apollo Client's own configuration and security-relevant documentation.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# GraphQL Client Security Review

## Purpose

Review GraphQL client setup code -- client construction, link chains, and cache configuration -- for five documented, security-critical defect classes: production-reachable devtools/schema introspection, a normalized cache that survives a user logout or account switch, client-driven query abuse with no server-side allowlist, an authorization header attached with no accompanying CSRF protection, and sensitive fields written into the normalized cache with no masking policy. This skill exists so the review stays anchored to these five concrete, greppable sinks instead of drifting into a general "is this GraphQL app well-architected" review.

## When to use

Use this skill when the user asks to:

- review an Apollo Client (or comparable GraphQL client) instantiation for production security posture,
- assess whether a logout/session-switch flow correctly clears client-side GraphQL cache state,
- investigate a report that one user's cached GraphQL data appeared in another user's session on the same device,
- audit a GraphQL client's link chain for unallowlisted, client-driven query risk (excessive nesting, alias-based amplification, arbitrary ad hoc documents reaching the server),
- check whether an auth-header link (`SetContextLink` or equivalent middleware) exposes GraphQL mutations to CSRF,
- review cache `typePolicies` (or equivalent) for whether sensitive fields (payment data, `cvv`, SSNs, tokens) are masked before being normalized into the cache.

Do not use this skill for:

- GraphQL server-side schema/resolver authorization review (field-level permission checks, resolver-level access control) -- this skill is scoped to the client, not the server; the server-side defect class needs a different review entirely,
- general Apollo Client / urql architecture or performance review with no security angle (cache normalization strategy quality, fetch-policy tuning, pagination design) -- use a general frontend-architecture review for that,
- a bug that requires live traffic reproduction (capturing a real cross-user cache leak, load-testing a query-cost exploit against a live server) to confirm exploitation -- static analysis proves the structural risk in the client configuration, not that it has already been exploited in production.

## Context7 Documentation Protocol

- Resolve the Apollo Client library ID with `resolve-library-id` (matched result: `/apollographql/apollo-client`) before citing any client-configuration, cache-reset, or link-chain claim.
- `/apollographql/apollo-client` is Apollo Client's own docs-and-source repository. Use `query-docs` against it to ground claims about `connectToDevTools` behavior, `resetStore`/`clearStore` semantics, `SetContextLink` header-injection patterns, and `PersistedQueryLink` allowlisting -- these are `repo evidence` when confirmed there.
- urql is not currently resolvable in Context7. Treat urql-specific findings as `documentation-based` (urql's own published docs) or `inference` (reasoning by analogy from Apollo Client's documented behavior), and say so explicitly -- never state a urql API detail as `repo evidence` without a Context7-confirmed source.
- Read `package.json` first to confirm which GraphQL client (and version) is actually in use before applying any client-specific API name -- `resetStore`/`clearStore` and `SetContextLink` are Apollo Client APIs; do not apply them to a urql or graphql-request codebase without confirming the equivalent construct first.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- All five defect classes default to HIGH severity. This is a security-scoped skill: do not downgrade a production-reachable devtools finding, an uncleared cache after logout, an unallowlisted query surface, a CSRF-less auth header, or an unmasked sensitive cached field to MEDIUM just because it has not been observed exploited yet -- the risk is in the structure, not in whether someone has already hit it.
- Trace every finding to a concrete file:line and a concrete data-flow or configuration path. A finding that says "the cache might leak" or "this could be a CSRF risk" without showing the specific `connectToDevTools` literal, the specific logout handler missing `resetStore`/`clearStore`, or the specific header-construction code is not a valid finding -- it is a guess.
- `connectToDevTools` set to an unconditional `true` (or simply always-on with no environment gate) is a HIGH finding regardless of whether the current deployment happens to be a staging environment -- flag the structural exposure, not just today's environment. `connectToDevTools` correctly gated on a build-time environment check (development-only) is not a finding.
- Do not approve a logout or account-switch handler unless it visibly calls `client.resetStore()` or `client.clearStore()` (or the equivalent for the client library in use) on the exact path between the auth-invalidation call and the navigation/redirect that follows. A cache-clearing call that exists elsewhere in the codebase, in a different handler, does not clear this bar for the specific logout path under review.
- Check every link-chain construction for a `PersistedQueryLink` (or equivalent allowlisting mechanism) between the application's operations and the transport link. A client wired directly to a raw HTTP link with no persisted-query allowlist is a MEDIUM-to-HIGH finding depending on whether the endpoint is public/unauthenticated (higher) or requires an authenticated session to reach (lower, but still a finding) -- an authenticated session does not eliminate cost-abuse risk from a legitimate-but-malicious authenticated client.
- Check every `SetContextLink`/auth-header-injection link for whether it attaches an anti-CSRF header (commonly `x-csrf-token` or an equivalent named header) alongside the `authorization` header. A token-only header construction with no CSRF header is a MEDIUM-to-HIGH finding when the same token can also travel via an automatically-attached credential (a cookie-backed session, a same-site credentialed fetch) -- if the token is exclusively attached by explicit client code with no cookie-based fallback anywhere in the auth flow, state that explicitly and do not manufacture a CSRF finding where the attack surface does not exist.
- Check cache `typePolicies` (or equivalent field-policy mechanism) for every sensitive field type (payment data, `cvv`, SSNs, auth tokens, other PII) that flows through a query in scope. A sensitive field with no masking `read()` policy, normalized into an `InMemoryCache` (or equivalent) with no field-level exclusion, is a HIGH finding -- it is visible in the devtools cache inspector for the life of the cache entry regardless of whether devtools happen to be enabled in the current build.
- Never execute, build, or run application code, and never send live GraphQL requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) -- use for the step-by-step review procedure, the five-defect-class decision tree, and the required output shape.
- [Cache lifecycle and query-surface risk](references/cache-lifecycle-and-query-surface.md) -- load only when reviewing cache clearing on logout, `typePolicies` masking of sensitive fields, or `PersistedQueryLink` allowlisting of the client's query surface.
- [Devtools exposure and CSRF-less auth headers](references/devtools-and-auth-header-risk.md) -- load only when reviewing `connectToDevTools` configuration or a `SetContextLink`/auth-header-injection link for CSRF protection.

## Response minimum

Return, at minimum:

- the client construction, link chain, cache configuration, and/or logout handler(s) in scope,
- ranked findings with file:line evidence, defect category (`devtools-exposure`, `cache-not-cleared`, `unallowlisted-query-surface`, `csrf-less-auth-header`, or `unmasked-sensitive-field`), the concrete configuration or data-flow trace, and a fix sketch matching Apollo Client's documented pattern,
- for every cache-clearing finding, an explicit statement of whether `resetStore()` or `clearStore()` is present on the traced logout path -- never approve on the assumption one exists elsewhere,
- for every sensitive-field finding, an explicit statement of whether a `typePolicies` masking `read()` policy is present for that exact field -- never approve on the assumption the field is handled safely downstream,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming actual query-cost abuse requires live load testing against the GraphQL server, not static review of the client").
