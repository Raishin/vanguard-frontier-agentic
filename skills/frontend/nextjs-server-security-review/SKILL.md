---
name: nextjs-server-security-review
description: Statically review Next.js middleware, Server Actions, next.config.js, and environment-variable files for four documented server-side defect classes -- middleware matcher exclusions that silently skip auth on Server Functions, Server Actions missing allowedOrigins CSRF protection, secrets leaked via NEXT_PUBLIC_ prefixes, and SSRF/open-redirect via dangerouslyAllowLocalIP or unvalidated rewrite destinations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# Next.js Server Security Review

## Purpose

Review Next.js middleware (`middleware.ts`/`.js`), Server Actions, `next.config.js`, and environment-variable files for the concrete server-side defect classes Next.js's own documentation calls out directly: middleware `matcher` exclusions that silently skip authorization for Server Functions, Server Actions missing `allowedOrigins` CSRF protection, secrets accidentally exposed to the client via a `NEXT_PUBLIC_` prefix, and server-side request forgery / open-redirect risk via `dangerouslyAllowLocalIP` or an unvalidated rewrite destination. This skill exists so the review stays anchored to these documented defect classes instead of drifting into a general "Next.js code review" of component architecture, data fetching patterns, or styling.

## When to use

Use this skill when the user asks to:

- review `middleware.ts`/`middleware.js` and its `matcher` configuration for authorization gaps,
- assess whether a Server Action is protected against CSRF, or whether `next.config.js`'s `serverActions.allowedOrigins` is configured correctly for a reverse-proxy or multi-zone deployment,
- audit `.env`/`.env.production` files or any `process.env.NEXT_PUBLIC_*` usage for accidental secret exposure to the client bundle,
- review `next.config.js` image configuration (`images.dangerouslyAllowLocalIP`) or `rewrites()`/`NextResponse.rewrite()` usage for SSRF or open-redirect risk,
- perform a pre-launch security review of a Next.js server surface (middleware, Server Actions, config, env files).

Do not use this skill for:

- client-only React component logic with no middleware, Server Action, config, or environment-variable surface in scope — there is no server-side sink for this skill to review,
- general Next.js performance, data-fetching-pattern, or App Router/Pages Router migration review with no security angle,
- a bug that requires live traffic reproduction (actually triggering a CSRF request cross-origin, actually confirming an SSRF callback from a deployed image optimizer) to prove exploitation — static analysis proves the structural risk, not that it has already been exploited in production.

## Context7 Documentation Protocol

- Resolve the Next.js library ID with `resolve-library-id` (matched result: `/vercel/next.js`) before citing any middleware, Server Actions, environment-variable, or `next.config.js` behavior claim.
- `/vercel/next.js` is a high-reputation source covering authentication, middleware, Server Actions, environment variables, and `next.config.js` security patterns directly from the framework's own docs and source. Use `query-docs` against it to confirm exact option names (`serverActions.allowedOrigins`, `images.dangerouslyAllowLocalIP`) and documented behavior (`NEXT_PUBLIC_` inlining, matcher exclusion semantics) before writing a finding.
- Read `package.json` first to confirm the Next.js major version and whether the app uses the App Router or Pages Router — `experimental.serverActions` configuration, middleware `matcher` conventions, and `NextResponse.rewrite()` APIs have shifted across major versions; do not apply App Router API names to a Pages Router codebase or vice versa.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- All four defect classes in this skill's scope default to HIGH severity. This is a security-scoped skill: do not downgrade a middleware-matcher authorization gap, a missing CSRF allowlist, a leaked secret, or a structural SSRF/open-redirect path to MEDIUM just because it has not been observed exploited yet — the risk is in the structure, not in whether someone has already hit it.
- Trace every finding to a concrete file:line and a concrete data-flow path. A finding that says "this middleware might not protect everything" or "this env var might leak" without showing the specific `matcher` pattern, the specific `serverActions` config, or the specific variable declaration is not a valid finding — it is a guess.
- Never accept a middleware `matcher` as sufficient authorization coverage in isolation. A `matcher` whose negative-lookahead pattern excludes a path (e.g. `/((?!api|_next).*)`) means middleware — and any auth check it performs — never runs for that excluded path; a Proxy matcher that excludes a path also skips Server Function calls on that path. Confirm the excluded path's own handlers (Server Actions, Route Handlers) independently verify the session themselves.
- Never approve a `next.config.js` with a `serverActions` block configured for a reverse-proxy, multi-zone, or otherwise cross-origin deployment unless `allowedOrigins` is explicitly present in that same block. Next.js's default CSRF protection compares the Server Action request's `Origin` header to the `Host` header and rejects mismatches — a deployment where those two headers legitimately differ (reverse proxy, multi-zone) needs the allowlist or every legitimate request fails, or worse, the mismatch is worked around insecurely elsewhere.
- Flag every environment variable whose name suggests a secret (contains `KEY`, `SECRET`, `TOKEN`, `PASSWORD`, `CREDENTIAL`, or equivalent) and is prefixed `NEXT_PUBLIC_`. Any `NEXT_PUBLIC_` variable is inlined into the JavaScript bundle at build time and shipped to every client, full stop — there is no runtime gate that can retroactively hide it once bundled. The safe fix is to drop the prefix and read the value only from server-side code via `process.env.API_KEY` (or the equivalent unprefixed name), never from a Client Component.
- Flag `images.dangerouslyAllowLocalIP: true` in `next.config.js` as a structural SSRF risk unless the codebase demonstrably validates every dynamic `src` value against a hardcoded external-hostname allowlist before it reaches the `<Image>` component.
- Flag any `NextResponse.rewrite()` call (or `rewrites()` destination) whose target URL is built directly from user-controlled input (query parameters, headers, request body) with no hostname allowlist check on the resolved value before the rewrite call — this is SSRF/open-redirect via the rewrite backend, not a cosmetic routing bug.
- Never execute, build, or run application code, and never send live requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the decision tree per defect class, and the required output shape.
- [Middleware and Server Actions boundary defects](references/middleware-and-server-actions.md) — load only when reviewing `middleware.ts`/`.js`, its `matcher`, or Server Action CSRF configuration.
- [Environment variables and SSRF/redirect surfaces](references/env-and-ssrf-surfaces.md) — load only when the review scope includes `.env*` files, `NEXT_PUBLIC_*` usage, `images.dangerouslyAllowLocalIP`, or a rewrite/redirect destination built from dynamic input. Includes the OWASP SSRF/open-redirect grounding reference; load that citation only when such a finding is actually present.

## Response minimum

Return, at minimum:

- the middleware file(s), Server Action(s), `next.config.js`, and/or environment file(s) in scope,
- ranked findings with file:line evidence, defect category (`middleware-auth-gap`, `csrf-origin`, `secret-leak`, or `ssrf-redirect`), the concrete data-flow trace (the `matcher` pattern and the excluded path's own auth handling, the `serverActions` config and its deployment topology, the environment variable declaration and its usage site, or the origin-to-sink path for the SSRF/redirect finding), and a fix sketch matching Next.js's documented pattern,
- for every middleware-auth-gap finding, an explicit statement of whether the excluded path's own handler independently verifies the session — never approve on the assumption it does,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming actual cross-origin CSRF success requires a live cross-origin request, not static review").
