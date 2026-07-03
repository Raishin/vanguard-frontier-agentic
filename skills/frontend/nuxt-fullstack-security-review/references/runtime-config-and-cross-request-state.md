# runtimeConfig Exposure and Cross-Request State Pollution

Use this reference when the review scope includes `nuxt.config.ts`'s `runtimeConfig`
block, any `.env`/`NUXT_*` variable naming, or `useState`/module-scope reactive
declarations reachable from server-rendered code.

## Part 1 — runtimeConfig: private vs public split

### What people get wrong

The naive assumption is:

> "I put it in `runtimeConfig`, so it's server-only."

Wrong — only *part* of `runtimeConfig` is server-only. Nuxt's own config API
distinguishes two zones inside the same object:

```ts
export default defineNuxtConfig({
  runtimeConfig: {
    // Private keys are only available on the server
    apiSecret: '123',

    // Public keys that are exposed to the client
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE || '/api',
    },
  },
})
```

(`documentation-based`, Context7 `/websites/nuxt_4_x`, `useRuntimeConfig`/
`nuxt.config` API reference and the runtime-config guide.) Everything nested under
the `public` key (and, per the same source, the `app` key) is serialized into the
client bundle and readable by anyone who opens dev tools. Everything else in
`runtimeConfig` stays server-only.

### The environment-variable trap

Context7 confirms two hard rules for env-var overrides (`documentation-based`,
`/websites/nuxt_4_x`, "Runtime Config > Exposing > Environment Variables" and the
migration guide):

1. Only variables **already declared** in `nuxt.config`'s `runtimeConfig` can be
   overridden at runtime — arbitrary env vars are never auto-exposed.
2. Only an **uppercase environment variable prefixed `NUXT_`**, using `_` to
   separate nested keys, overrides the matching config path. A `NUXT_PUBLIC_*`
   variable maps to a key under `public`; a `NUXT_*` variable without `PUBLIC`
   maps to a private top-level key.

The security-relevant consequence: a developer who names an env var
`NUXT_PUBLIC_API_SECRET` — intending it to *feel* private because of the word
"SECRET" — has just told Nuxt to expose it to the client, because the naming
convention that controls exposure is structural (`public.*` nesting / the
`PUBLIC` segment), not the variable's English-language name.

### Verification targets

- Grep `nuxt.config.*` for `runtimeConfig:` and read the full block, including the
  nested `public: { ... }` object.
- For every key under `public`, ask: does its name or the value it defaults to
  (`process.env.*`) suggest a secret, credential, signing key, or internal
  hostname? If yes → HIGH finding, move it out of `public` (or behind a
  server-only proxy endpoint that reads the private key and returns only what the
  client needs).
- For every key **outside** `public` (private), confirm it is never re-exported
  into a public-scoped value elsewhere (e.g., a plugin or composable doing
  `useRuntimeConfig().public.foo = useRuntimeConfig().apiSecret` — an anti-pattern
  that defeats the private/public split at runtime).
- Grep `.env`, `.env.*`, and deployment/CI config for `NUXT_PUBLIC_*` variable
  names; confirm each maps to a genuinely public value in `nuxt.config`.
- Do not flag a private key merely for existing — that is the correct, safe
  pattern (rubric item 9). Only flag actual public-zone exposure or private→public
  leakage.

### Fix sketch

```ts
// nuxt.config.ts — correct
export default defineNuxtConfig({
  runtimeConfig: {
    apiSecret: '', // NUXT_API_SECRET — server-only, never reaches the client
    public: {
      apiBase: '', // NUXT_PUBLIC_API_BASE — fine to expose
    },
  },
})
```

```ts
// server/api/proxy.ts — client needs data derived from the secret,
// so it calls a server route, never the secret itself
export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig(event)
  const result = await $fetch('https://upstream.example.com/data', {
    headers: { Authorization: `Bearer ${config.apiSecret}` },
  })
  return result // only the derived, non-secret payload leaves the server
})
```

## Part 2 — useState / module-scope cross-request state pollution

### What people get wrong

The naive assumption is:

> "Nuxt handles SSR for me, so any state I declare is automatically per-request."

Wrong. Nitro (Nuxt's server engine) is a single long-lived Node.js process handling
many concurrent requests. Context7 confirms the exact failure mode directly
(`documentation-based`, `/websites/nuxt_4_x`, "Auto-imports > Built-in Auto-imports"):
Vue/Nuxt tracks the current component instance (and Nuxt's own `nuxtApp`) via a
transient global reference specifically to avoid "cross-request state pollution
(leaking a shared reference between two users)." That same guide's State
Management page states the rule as a best practice directly: never define
`const state = ref()` outside of `<script setup>` or a `setup()` function — doing
`export myState = ref({})` "would result in state shared across requests on the
server and can lead to memory leaks." (`documentation-based`,
`/websites/nuxt_4_x`, "State Management > Best Practices.")

`useState` itself is the documented safe replacement: an "SSR-friendly `ref`
replacement" whose "value will be preserved after server-side rendering (during
client-side hydration) and shared across all components using a unique key"
(`documentation-based`, same source) — but `useState` is only safe when invoked
inside a component/composable/plugin function body, where Nuxt's per-request
context tracking scopes it correctly. Calling `useState` (or any `ref`/`reactive`)
at true module scope defeats that scoping.

### Non-negotiable design rules

1. **Classify every module-scope declaration by mutability/reactivity, then by
   reachability** — same two-axis test as any SSR framework: is it
   `ref()`/`reactive()`/`useState()`/a mutable object or array literal (risk), or an
   immutable constant (safe)? Is it imported/read/written by any
   server-rendered composable, plugin, or `server/api` handler (reachable), or
   truly dead/unused (not a finding in this scope)?
2. **A composable wrapper does not automatically fix it.** `const useX = () =>
   useState('x')` is the documented safe pattern *only* because `useState`'s
   internal key-based lookup is itself scoped per Nuxt-app-instance/per-request.
   A hand-rolled module-scope cache object that a composable merely *reads* from
   (rather than routing through `useState`/a genuinely per-request store) is not
   fixed by wrapping the read in a function — trace what the function actually
   touches, not just its outer shape.
3. **Server-side (`server/api`, `server/middleware`) module-scope mutable state is
   just as much at risk as client/universal composable state** — Nitro's event
   handlers run in the same long-lived process. A module-level `let cache = {}`
   written to inside a `defineEventHandler` and read by a later request from a
   different user is a textbook cross-request leak, independent of Vue/useState
   entirely.
4. **Closures over shared mutable state defeat an otherwise-correct factory** — a
   `defineEventHandler` that looks correct because it doesn't declare state at
   module scope can still close over a module-level mutable variable from its
   enclosing file.

### Verification targets

- Grep for `useState(` calls and confirm each is inside a `<script setup>` block,
  `setup()` function, composable function body, or Nuxt plugin factory — not at
  a file's top level.
- Grep for `ref(`/`reactive(` outside any function body across `composables/`,
  `plugins/`, `server/api/`, `server/middleware/`, and any file imported by them.
- Grep `server/` for `let `/mutable `const {}`/`const []` declarations at module
  scope; check whether any `defineEventHandler` in the same file (or importing
  the module) reads or writes them.
- For each hit, trace reachability: is it imported by any server-rendered route,
  composable, or middleware that runs per-request? If no server-rendered code
  path reaches it, it is not a finding in this review's scope (rubric item 12 for
  immutable data; otherwise flag as dead code, not a security finding).

### Fix sketch

```ts
// BAD — server/api/session.ts: module-scope mutable object shared across all requests
const lastUser: Record<string, unknown> = {}
export default defineEventHandler((event) => {
  lastUser.id = getQuery(event).userId // every concurrent request writes the SAME object
  return lastUser
})
```

```ts
// GOOD — no shared mutable state; everything is derived fresh per request
export default defineEventHandler((event) => {
  const userId = getQuery(event).userId
  return { id: userId }
})
```

```ts
// GOOD — universal/component state via useState, invoked inside a composable
export const useCounter = () => useState('counter', () => 0)
```
