# Middleware Matcher Exclusions and Server Actions CSRF

Use this reference only when reviewing `middleware.ts`/`.js`, its `matcher` configuration, or a Server Action's CSRF posture in `next.config.js`.

## What people get wrong: matcher exclusions

The naive assumption is:

> "Middleware runs before every request, so anything I check in middleware protects the whole app."

Wrong, for any path the `matcher` excludes. A `matcher` written as a negative lookahead — e.g. `'/((?!api|_next).*)'` — is not just "skip static assets," it is "skip every path matching that lookahead, including API routes hosting Server Actions and Route Handlers." Next.js's own documentation on proxy execution order is explicit and non-negotiable on this point: a Proxy matcher that excludes a path also skips Server Function calls on that path (`documentation-based`, Next.js proxy/middleware docs). If the only auth check in the codebase lives inside middleware, and the matcher excludes `/api`, then every Server Action and Route Handler under `/api` runs with **zero** authorization enforcement from middleware — because middleware never runs for them at all.

## Non-negotiable design rules

### 1. A matcher exclusion is not evidence of anything — it is the absence of evidence

Do not treat "middleware exists in this project" as sufficient. For each path the `matcher` excludes, that path's *own* handler must independently verify the session. There is no such thing as partial credit for "the matcher probably wasn't meant to exclude sensitive routes" — read the actual regex and the actual excluded paths.

### 2. Distinguish an explicit allowlist matcher from an exclusion-style matcher

A `matcher` that explicitly lists the paths middleware should run on (e.g. `['/dashboard/:path*', '/admin/:path*']`) is a fundamentally different risk shape than a negative-lookahead exclusion matcher (e.g. `['/((?!api|_next).*)']`). The explicit-allowlist form makes it obvious which paths get middleware coverage and, by construction, does not accidentally exclude an API route someone forgot about. Recommend this form when a matcher-exclusion gap is found.

### 3. Trace what each Server Action actually checks

For any Server Action (`'use server'` function) reachable from an excluded path, read its full body. A session check must be visible inside that function — not merely assumed because "there's an auth library imported at the top of the file." An import with no corresponding call site inside the action is not a check.

## What people get wrong: Server Actions CSRF

The naive assumption is:

> "Next.js handles CSRF for Server Actions automatically, so I don't need to configure anything."

Half right. Next.js's built-in protection only allows `POST` requests and compares the request's `Origin` header to its `Host` header, aborting on mismatch (`documentation-based`, Next.js data-security guide). That default works for a simple same-origin deployment. It silently breaks down — or gets worked around insecurely — for any deployment where `Origin` and `Host` legitimately differ for real production traffic: an app behind a reverse proxy, a multi-zone setup where one domain fronts several Next.js apps, or a staging environment fronted by a different domain than the app's own `Host`.

## Officially grounded rule

`serverActions.allowedOrigins` in `next.config.js` lets the framework compare the Server Action request's origin against an explicit, safe allowlist instead of only the automatic `Host`-header comparison:

```js
/** @type {import('next').NextConfig} */
module.exports = {
  experimental: {
    serverActions: {
      allowedOrigins: ['my-proxy.com', '*.my-proxy.com'],
    },
  },
}
```

(`documentation-based`, Next.js data-security and multi-zone guides.)

## Non-negotiable design rules

### 1. A `serverActions` block missing `allowedOrigins` is not automatically safe

If the deployment is same-origin only (no reverse proxy, no multi-zone, `Origin` and `Host` always match for real traffic), omitting `allowedOrigins` is the documented safe default — do not manufacture a finding where none exists. But if the deployment topology is cross-origin (reverse proxy, multi-zone, CDN rewriting `Host`), a `serverActions` block that configures other options (`bodySizeLimit`, etc.) but omits `allowedOrigins` is the exact CSRF gap this rule exists to catch. Confirm the deployment topology before judging severity — check `next.config.js` `rewrites()`, deployment docs, or infrastructure config for reverse-proxy/multi-zone evidence.

### 2. `allowedOrigins` entries must be the actual safe origins, not a wildcard-everything pattern

`allowedOrigins: ['*']` (or an equivalent catch-all) defeats the purpose of an allowlist — it accepts CSRF requests from any origin. Flag a wildcard-everything entry the same as a missing `allowedOrigins` key.

## Verification targets

- Grep `middleware.ts`/`.js` for `matcher:` and inspect the pattern for a negative lookahead (`(?!...)`) versus an explicit path list.
- Grep every file the matcher excludes for `'use server'` and read each exported action's full body for a session/auth check.
- Grep `next.config.js` for `serverActions:` and check whether `allowedOrigins` is a sibling key inside that same object.
- Grep deployment config / `rewrites()` / infra docs for reverse-proxy or multi-zone evidence to determine whether a missing `allowedOrigins` is actually a gap or a legitimate same-origin default.
