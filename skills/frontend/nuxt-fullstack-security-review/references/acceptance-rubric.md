# Acceptance Rubric (write this first — it is the failing test)

This rubric is the spec for `nuxt-fullstack-security-review`. Every item below must be
covered by an explicit operating rule, decision-tree branch, or reference section before
this skill ships. `SKILL.md` and the other `references/*.md` files are written to satisfy
this list, not the other way around.

## MUST catch (true positives)

1. **Private secret placed under `runtimeConfig.public.*`.** A key holding a secret
   (API key, DB credential, signing secret, third-party token) declared inside the
   `public` block of `runtimeConfig` in `nuxt.config.ts` — or a `NUXT_PUBLIC_*`
   environment variable feeding one — ships to the client JS bundle. HIGH. Evidence:
   Context7 `/websites/nuxt_4_x` confirms only `runtimeConfig.public.*` (and
   `runtimeConfig.app.*`) are exposed to the client; every other key is server-only,
   and only an uppercase `NUXT_`-prefixed env var overrides a *matching* runtimeConfig
   key at runtime.
2. **Private secret with a name/shape suggesting sensitivity kept at top-level
   `runtimeConfig` but actually read from a public-scoped variable, or accidentally
   duplicated into `public` "for convenience" (e.g., a client composable needs the same
   value).** Still HIGH — the fix is a server-only proxy endpoint, not exporting the
   secret to `public`.
3. **`useState` or module-scope `ref()`/`reactive()`/mutable object declared outside any
   composable/component function body**, used to hold per-user or per-request data
   (session, cart, auth token, request-specific query results) in server-rendered Nitro
   code. Because Nitro is a single long-lived process serving concurrent requests, this
   is cross-request state pollution — HIGH, structural, regardless of whether it has
   been observed leaking.
4. **A composable that *looks* per-request (invoked inside `setup()`/an event handler)
   but closes over a shared module-level cache/singleton/mutable default parameter**,
   defeating the apparent isolation. HIGH.
5. **`server/api/*` route (`defineEventHandler`) calling `$fetch`/`ofetch` with a
   URL built from user-controlled input (route param, query string, request body)
   with no host allowlist** — classic SSRF: an attacker can redirect the server's
   outbound request to an internal address or arbitrary host. HIGH.
6. **A server route reading `useRequestHeaders()` (or forwarding `event.node.req.headers`
   directly) and blindly forwarding all headers — or specifically `authorization`/
   `cookie` — to a third-party/user-controlled outbound `$fetch` call** without an
   allowlist of headers and without restricting the destination host. HIGH — this
   leaks the current user's credentials to whatever host the URL resolves to.
7. **`useState`/payload data containing user-controlled/echoed content (a query param,
   a comment body, a profile field) serialized into the SSR payload and then rendered
   unescaped** (e.g., interpolated with `v-html`, or written into an inline
   `<script>`/`<style>` block by custom server middleware bypassing Nuxt's own
   `devalue`-based payload serialization) → XSS. HIGH. (Nuxt's own payload
   serialization via `devalue` is not itself an XSS sink; the finding is about
   *what renders that payload data unescaped afterward* or about a hand-rolled
   payload/script-injection path that bypasses `devalue`.)
8. **No security response headers configured anywhere** (no `routeRules` `headers`
   entries, no `nuxt-security` module, no middleware calling `useResponseHeader`) for
   an app that serves authenticated pages, handles forms, or embeds third-party
   content — missing CSP/X-Frame-Options/X-Content-Type-Options is a MEDIUM-to-HIGH
   finding depending on what the app does (a public informational-only site with no
   auth and no user input is lower severity than a logged-in dashboard).

## MUST NOT flag (false positives to actively avoid)

9. A value correctly declared as a **private** `runtimeConfig` key (outside `public`),
   even if consumed only server-side in `server/api/*` — this is the *correct* pattern
   and must not be flagged just because "secrets in config feel risky."
10. `runtimeConfig.public.*` holding genuinely non-secret data (a base API URL, a
    feature flag, a public analytics ID) — public-by-design values are not findings.
11. `useState`/`ref`/`reactive` declared **inside** a composable function, component
    `setup()`, or plugin factory that runs per-request/per-component-instance — this is
    the documented, safe pattern; do not flag every `useState` call site.
12. A module-scope `const` holding an **immutable** value (a frozen route table, a
    static config object, a compiled regex, a constant lookup map) with no runtime
    mutation path — immutable module-scope constants are safe to share across
    requests.
13. A `server/api/*` route whose `$fetch`/`ofetch` target is a **hardcoded or
    allowlisted** host (e.g., validated against a fixed list of trusted upstream
    domains, or the URL is entirely server-config-derived with no user input in the
    host/path) — do not flag SSRF when the destination is not attacker-influenceable.
14. `event.$fetch(...)` used deliberately to forward request context to another
    **internal** Nitro route for legitimate context propagation, with no external host
    involved — not an SSRF/credential-leak finding by itself; only flag when the
    *destination* is external/user-controlled or when *sensitive* headers are
    forwarded to it without justification.
15. An app that already sets a documented security header mechanism (an explicit
    `routeRules` `headers` block with CSP/HSTS/etc., or the `nuxt-security` module
    configured) — do not flag "missing headers" when a header-setting mechanism is
    present and covers the routes in scope; only flag genuine gaps (e.g., headers set
    for `/` but not for `/admin/**`).

## Evidence discipline

- Every finding must cite file:line and a concrete data-flow trace (declaration →
  reachability, or origin → sink).
- Every framework-behavior claim in the finding write-up must be labeled
  `documentation-based` (Context7-confirmed against `/websites/nuxt_4_x` or
  `/websites/nuxt_3_x`), `repo evidence` (observed directly in the file under review),
  or `inference` (reasonable extrapolation not directly stated in either source —
  e.g., community conventions like the third-party `nuxt-security` module's exact
  default header set, which Context7's Nuxt-core docs do not themselves enumerate).
- Do not invent API names. If Context7 does not confirm an API or default, say so and
  label the claim `inference`.
