# Server-Route SSRF, Header Forwarding, Payload XSS, and Missing Response Headers

Use this reference when the review scope includes a `server/api/*` or
`server/routes/*` event handler (`defineEventHandler`), any `$fetch`/`ofetch`/
`event.$fetch`/`useRequestFetch` call inside server code, a `useState`/payload
value that renders into a template, or `nuxt.config.ts`'s `routeRules`.

## Part 1 — server route SSRF via $fetch/ofetch + header forwarding

### What people get wrong

The naive assumption is:

> "It's just a server-side fetch, not user input rendered in the browser, so it
> can't be a security issue."

Wrong on two counts. First, if the **target URL** of a server-side `$fetch` call
is built from user-controlled input (a route param, query string, or request
body) with no allowlist, the server itself becomes an attacker-controlled
HTTP client — a Server-Side Request Forgery (SSRF) primitive that can reach
internal network addresses, cloud metadata endpoints, or arbitrary hosts the
attacker chooses. Second, Nuxt's own docs draw a sharp, explicit line around
**header forwarding** that is easy to miss:

- Bare `$fetch(...)` inside a server route does **not** forward the incoming
  request's headers or context by default (`documentation-based`,
  `/websites/nuxt_4_x`, "Forwarding Context & Headers": "By default, neither the
  headers from the incoming request nor the request context are forwarded when
  making fetch requests in server routes.")
- `event.$fetch(...)` is the documented way to forward the request context and
  headers (`documentation-based`, same source): `export default
  defineEventHandler((event) => { return event.$fetch('/api/forwarded') })`.
- `useRequestFetch()` is the documented composable for explicitly forwarding the
  current user's headers and cookies during SSR when plain `$fetch` would not
  include them (`documentation-based`, `/websites/nuxt_4_x`,
  `useRequestFetch` API reference).
- Nuxt's data-fetching guide states the caution directly: "Exercise caution when
  proxying headers to external APIs, only including those that are strictly
  necessary. Headers like 'host', 'accept', 'content-length', 'content-type',
  and various 'x-forwarded' or 'cf-' headers should generally not be proxied."
  (`documentation-based`, `/websites/nuxt_4_x`, "$fetch > Pass Client Headers to
  the API.")
- `useRequestHeaders(['authorization'])` is documented specifically to proxy the
  `authorization` header to an internal isomorphic `$fetch` call during SSR
  (`documentation-based`, `/websites/nuxt_3_x`, `useRequestHeaders` API
  reference, "Proxying Authorization Header in SSR").

The security-relevant synthesis: **there is no built-in URL validation on
`$fetch`/`ofetch`/`event.$fetch` targets**, and **there is no built-in header
allowlist** on what `event.$fetch`/`useRequestFetch`/manually-forwarded headers
send onward. Both are the calling code's responsibility. A route that (a) builds
its outbound URL from user input and (b) forwards the current user's
`authorization`/`cookie` header to that user-influenceable destination combines
SSRF with credential exfiltration.

### Non-negotiable design rules

1. **Trace the outbound URL to its origin.** For every `$fetch`/`ofetch`/
   `event.$fetch`/`useRequestFetch` call in a `server/api`/`server/routes`
   handler, find where the URL string comes from. If any segment (host, path,
   or query) is built from `getQuery(event)`, `getRouterParam(event, ...)`,
   `readBody(event)`, or an equivalent user-reachable source, with no allowlist
   check against a fixed set of trusted hosts before the call — HIGH SSRF
   finding.
2. **A hardcoded or allowlisted host is not a finding.** If the destination host
   is a literal string, an env-config value with no user input in it, or the
   user-supplied portion is validated against an explicit allowlist of trusted
   hosts before the request fires, do not flag it (rubric item 13).
3. **Trace what headers actually get forwarded, and to where.** `event.$fetch`
   forwards request context/headers by default — check whether the destination
   is internal (same-origin Nitro route) or external. Forwarding to an internal
   route with no external network hop is not a credential-leak finding by
   itself (rubric item 14). Forwarding `authorization`/`cookie` to an external
   or user-influenceable host — with no allowlist limiting which headers cross
   that boundary — is HIGH.
2b. **Manual, unfiltered header spreading is worse than `event.$fetch`.** Code
   that reads `event.node.req.headers` (or `getHeaders(event)`) and spreads the
   entire object into an outbound `$fetch`'s `headers` option forwards
   everything, including headers Nuxt's own docs say should generally not be
   proxied (`host`, `accept`, `content-length`, `content-type`,
   `x-forwarded-*`, `cf-*`) — flag this even before considering the destination,
   since it also risks breaking the outbound request's own framing/routing
   semantics, and flag it as HIGH if any sensitive header (`authorization`,
   `cookie`) is in the spread and the destination is external or
   user-influenceable.

### Verification targets

- Grep `server/api/` and `server/routes/` for `$fetch(`, `ofetch(`,
  `event.$fetch(`, `useRequestFetch(`, `useRequestHeaders(`, `getHeaders(`.
- For each `$fetch`-family call, read backward to the URL argument's full
  construction; flag string concatenation/template literals combining a fixed
  base with a user-reachable variable, or a bare user-supplied URL.
- For each `useRequestHeaders(...)` or `getHeaders(...)` call, check what keys
  are requested/forwarded and where the result is used downstream.

### Fix sketch

```ts
// BAD — server/api/proxy.ts: user controls the entire outbound host
export default defineEventHandler(async (event) => {
  const { url } = getQuery(event)
  return $fetch(url as string) // SSRF: attacker can point this at internal services
})
```

```ts
// GOOD — allowlisted upstream host, no user control over destination
const ALLOWED_HOSTS = new Set(['api.trusted-partner.com'])

export default defineEventHandler(async (event) => {
  const { path } = getQuery(event)
  const target = new URL(String(path), 'https://api.trusted-partner.com')
  if (!ALLOWED_HOSTS.has(target.host)) {
    throw createError({ statusCode: 400, statusMessage: 'Invalid target' })
  }
  return $fetch(target.toString())
})
```

## Part 2 — NuxtPayload / useState serialization into rendered HTML

### What people get wrong

The naive assumption is:

> "Nuxt serializes my state into the payload automatically, so whatever I put in
> `useState` is handled safely by the framework."

Partially wrong. Nuxt 3/4's payload mechanism (`nuxtApp.payload`, covering
`data` from `useFetch`/`useAsyncData` and `state` from `useState`) is serialized
for transfer from server to client using `devalue`, which supports "advanced
data types beyond basic JSON, such as Dates, Maps, Sets, refs, reactives, and
NuxtErrors" (`documentation-based`, `/websites/nuxt_3_x`, "Serializing Data From
Server to Client" / `useNuxtApp` payload reference). `devalue`-based
serialization itself is not an HTML-injection sink — it produces a JS
expression, not raw HTML concatenation. The actual risk this skill flags is
**what happens after** payload data is revived on the client:

- If a `useState`/payload value holding user-controlled content (an echoed
  query param, a comment body, a profile field from an API that itself echoes
  other users' submissions) is later rendered with `v-html` anywhere in the app,
  that is a standard unsanitized-`v-html` XSS finding — the payload is simply
  the transport, not the sink. Trace the value from `useState`/payload through
  to its eventual render, exactly as in an XSS review of any Vue app.
- A custom payload plugin (`definePayloadReducer`/`definePayloadReviver`,
  documented in `/websites/nuxt_3_x`'s `useNuxtApp` reference) that hand-builds
  a serialized representation without going through `devalue` — or any custom
  server middleware that writes directly into the rendered HTML's inline
  `<script>` tag instead of relying on Nuxt's own payload injection — is a
  distinct, higher-risk pattern: it bypasses the framework's serialization path
  entirely and must be checked for proper escaping by hand (`inference`: Nuxt's
  docs describe the reducer/reviver extension point but do not themselves
  discuss its injection risk, so grade this claim as inference, not
  documentation-based).

### Verification targets

- Grep for `useState(` call sites; for each, trace the value's origin (is it
  seeded from `getQuery`, `readBody`, or an API response that echoes
  user-submitted content?) and its eventual consumers (any `v-html` binding, any
  `innerHTML`/`dangerouslySetInnerHTML`-equivalent sink).
- Grep for `definePayloadReducer(`/`definePayloadReviver(` and read the custom
  serialization logic for any manual string concatenation that lands in
  rendered HTML.
- Apply the same sanitizer-on-exact-path standard as any `v-html` review: a
  sanitizer existing elsewhere in the codebase does not clear a specific traced
  path (same principle as Vue's own `v-html` guidance).

### Fix sketch

```vue
<script setup>
import DOMPurify from 'dompurify'

// comment.body is user-submitted content that round-tripped through
// useAsyncData's payload cache — the payload is the transport, this
// sanitizer call is what actually clears the finding.
const { data: comment } = await useAsyncData('comment', () => $fetch(`/api/comments/${id}`))
const safeBody = computed(() => DOMPurify.sanitize(comment.value?.body ?? ''))
</script>

<template>
  <div v-html="safeBody" />
</template>
```

## Part 3 — missing security response headers

### What people get wrong

The naive assumption is:

> "Nuxt is a modern framework, so it must ship secure headers (CSP, X-Frame-Options,
> etc.) by default."

Wrong. Context7 confirms the mechanisms available, but not that any are
enabled by default:

- `routeRules` in `nuxt.config.ts` supports a `headers` property that "allows
  adding specific HTTP headers to sections of your site" (`documentation-based`,
  `/websites/nuxt_4_x`, "Hybrid Rendering > Route Rules"), and a documented
  example shows `cors: true` adding CORS headers to an API route glob, further
  customizable via the same `headers` property. Nothing in the confirmed docs
  shows CSP/X-Frame-Options/HSTS enabled by default — these must be configured
  explicitly per route or globally (`'/**': { headers: { ... } }`).
- `useResponseHeader(name)` is a documented composable for setting any server
  response header from within pages, components, or plugins, including on a
  per-page basis (`documentation-based`, `/websites/nuxt_4_x`,
  `useResponseHeader` API reference).
- A dedicated community security module (commonly referred to as
  `nuxt-security`) exists to apply a curated default header set (CSP, HSTS,
  X-Frame-Options, and related hardening) in one step. Context7's Nuxt-core
  documentation set (`/websites/nuxt_4_x`, `/websites/nuxt_3_x`) does not itself
  document this third-party module's API or defaults — treat any specific claim
  about its default header values as `inference`, not `documentation-based`;
  confirm its presence/config directly in the repo (`nuxt.config.ts` `modules`
  array and any `security:` config block) rather than asserting what it does
  from memory.

### Non-negotiable design rules

1. **Absence of any header-setting mechanism is the finding, not the absence of
   a specific module.** Check for *any* of: a `routeRules` block with a
   `headers` entry covering the routes in scope, a `nuxt-security`-style module
   in the `modules` array, or middleware/plugin code calling
   `useResponseHeader(...)`. If none exist anywhere in the app, and the app
   handles authentication, forms, or third-party embeds, flag MEDIUM-to-HIGH
   depending on what the app does (rubric item 8).
2. **Partial coverage is a real, narrower finding.** If headers are set for `/`
   but not for `/admin/**` or `/api/**`, do not report "headers exist, fine" —
   report the specific gap, citing the routeRules glob(s) that are covered vs.
   not.
3. **Do not flag an app that already has a working mechanism covering the
   routes in scope** (rubric item 15) — verify the glob pattern's actual
   coverage before crediting it, since a `'/blog/**': { headers: {...} }` rule
   does not cover `/admin/**` or the site root.

### Verification targets

- Grep `nuxt.config.*` for `routeRules` and inspect every entry for a `headers`
  key; note which route globs are covered.
- Grep `nuxt.config.*` `modules` array for a security-module name (e.g.,
  `nuxt-security`) and read its adjacent config block if present.
- Grep the codebase for `useResponseHeader(` calls in plugins/middleware/pages.
- Cross-reference coverage against the app's actual surface (auth pages,
  forms, admin routes, embedded third-party widgets) to size the severity.

### Fix sketch

```ts
// nuxt.config.ts
export default defineNuxtConfig({
  routeRules: {
    '/**': {
      headers: {
        'X-Frame-Options': 'DENY',
        'X-Content-Type-Options': 'nosniff',
        'Content-Security-Policy': "default-src 'self'",
      },
    },
  },
})
```
