# Client-Side Persistence and Server-to-Client Hydration

Use this reference when reviewing `pinia-plugin-persistedstate`/`vuex-persistedstate`
configuration, SSR store creation/singleton risk, or server-payload hydration
(`window.__pinia`/`__INITIAL_STATE__`).

## Defect (a): sensitive data persisted to localStorage/sessionStorage without scoping

### What people get wrong

The naive assumption is:

> "I added `persist: true` so refreshes don't lose the user's cart — that's just a convenience
> feature, not a security decision."

Wrong. `persist: true` (Pinia) persists the *entire* store state to `storage` by default, and
`pinia-plugin-persistedstate`'s `storage` option itself defaults to `localStorage`
(`documentation-based`, via Context7 `/prazdevs/pinia-plugin-persistedstate`). If that store
also holds an auth token, a refresh token, a session identifier, or PII — which is common when
the same store that holds "is the user logged in" convenience state also holds the token used
to answer that question — the token rides along into `localStorage`, a plain string store
readable by any script running in the page's origin. Unlike an `HttpOnly` cookie, there is no
mechanism preventing JavaScript (including injected/XSS script) from reading it.

### Officially grounded rules

- `pinia-plugin-persistedstate`'s `storage` option accepts `localStorage` (default),
  `sessionStorage`, or a custom storage object implementing `getItem`/`setItem`
  (`documentation-based`).
- The `pick` option (an array of dotted state-path strings, e.g. `['save.me', 'saveMeToo']`)
  restricts persistence to only the named paths; **with no `pick`, the entire state object is
  persisted** (`documentation-based`). Legacy Vuex's `vuex-persistedstate` exposes the
  equivalent restriction via its `paths` option.
- A single store can use multiple persistence configs (an array of `{ pick, storage }` objects)
  to route different fields to different storage — e.g. a non-sensitive UI field to
  `localStorage` and nothing sensitive persisted at all (`documentation-based`).

### Review procedure

1. Read the full `state()` shape of the store.
2. Read the full `persist` config (or absence of one). If `persist` is absent, there is no
   persistence finding for this store regardless of what it contains.
3. If `persist: true` or `persist: {}` (no `pick`/`paths`) → every field in `state()` is
   persisted. Cross-reference against the sensitivity classification (auth/session/PII vs.
   UI-preference/non-sensitive).
4. If `persist: { pick: [...] }` → only the named paths are persisted. Check whether any
   sensitive field is named in that list. If none are, the persistence finding does not apply
   to this store (note that in the review) — but confirm no sensitive field was missed by
   checking the full list against the full `state()` shape, not just skimming for the word
   "token".
5. Check `storage`. Note explicitly whether it is `localStorage` (default, worse for
   sensitive data) or `sessionStorage`/a custom secure storage adapter — but do not treat
   `sessionStorage` alone as clearing a finding for a sensitive field; both are readable by
   same-origin script, meaning both are XSS-exfiltratable. The `storage` choice affects the
   window of exposure (tab lifetime vs. persistent), not whether the data is exposed to XSS at
   all.

### Minimal safe pattern

```ts
// Only the non-sensitive UI preference is persisted; the auth token is never
// scoped into `pick`, so it never reaches storage.
export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: '',           // sensitive — intentionally NOT in `pick` below
    refreshToken: '',    // sensitive — intentionally NOT in `pick` below
    theme: 'light',      // non-sensitive UI preference
  }),
  persist: {
    pick: ['theme'],
    storage: sessionStorage,
  },
})
```

### Anti-pattern (do not approve)

```ts
export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: '',
    refreshToken: '',
    user: { email: '', ssn: '' },
  }),
  persist: true, // WRONG: entire state, including token/refreshToken/ssn, persisted
                  // to localStorage (the plugin's default storage) with no `pick`.
})
```

## Defect (b): store hydration from an untrusted server payload

### What people get wrong

The naive assumption is:

> "The state blob came from our own server, so it's trusted — `JSON.parse` is safe, and
> embedding it with `JSON.stringify` is just plumbing."

Wrong in two distinct ways, both documented directly by Pinia's own SSR guide
(`documentation-based`, via Context7 `/vuejs/pinia`):

1. **Serialization-side (server → HTML):** naively interpolating
   `JSON.stringify(pinia.state.value)` into an HTML `<script>` block is unsafe if any part of
   that state can be influenced by user-submitted content (a bio field, a comment, any
   value a user or another user previously submitted that flowed into the store) — which is
   "almost always the case." A crafted value containing `</script><script>...` breaks out of
   the tag and executes as script in every subsequent visitor's browser. Pinia's docs
   explicitly recommend a safe-serialization library (`devalue` or equivalent) and call
   escaping "**VERY important**."
2. **Hydration-side (client parse):** the documented client pattern is
   `pinia.state.value = JSON.parse(window.__pinia)` — this assigns the *entire* parsed object
   directly into the live store state with no schema/shape check. If the value was tampered
   with in transit, or the serialization step itself was compromised, this is a direct
   trust-without-validation of a value that traveled through the DOM.

### Review procedure

1. Find the server-side code that serializes store state for hydration. Grep for
   `JSON.stringify(` combined with a variable referencing `pinia.state`, a Vuex store's
   `state`, or an equivalent root-state accessor, especially where the result is concatenated
   into an HTML template string or a `<script>` tag.
2. Determine whether that serialization uses a safe-serialization library (`devalue` or
   equivalent) or applies explicit escaping of `<`, `>`, `/` before embedding. If it is a bare
   `JSON.stringify` with no escaping step anywhere in the call chain → **HIGH** finding.
3. Find the client-side hydration code — grep for `window.__pinia`, `window.__INITIAL_STATE__`,
   or an equivalent global, and the assignment into `pinia.state.value` (or a Vuex store's
   `replaceState`/direct state assignment).
4. Check whether the parsed value is validated (a schema check, a shape guard, a runtime type
   check) before being assigned/used, or is trusted as-is. Trusting as-is is a finding whenever
   the state's contents could have been influenced by any user's prior input — treat this as
   the default assumption unless the review can show the state is fully server-computed with no
   user-submitted content anywhere upstream.

### Minimal safe pattern

```js
// server: escape before embedding
import devalue from 'devalue'
// ... after rendering, pinia.state.value holds this request's root state
const serialized = devalue(pinia.state.value) // escapes for safe HTML embedding
// embed `serialized` in the response, e.g. `<script>window.__pinia=${serialized}</script>`
```

```js
// client: hydrate only after confirming the shape looks like what's expected
const raw = JSON.parse(window.__pinia)
if (isValidPiniaStateShape(raw)) {
  pinia.state.value = raw
}
```

### Anti-pattern (do not approve)

```js
// server — WRONG: naive stringify with no escaping, and no validation downstream
const html = `<script>window.__pinia=${JSON.stringify(pinia.state.value)}</script>`
```

```js
// client — WRONG: entire parsed payload trusted with no shape/schema check
pinia.state.value = JSON.parse(window.__pinia)
```

## Defect (c): SSR store singleton cross-request pollution

### What people get wrong

The naive assumption is:

> "Each HTTP request gets its own function call, so a module-level `const pinia =
> createPinia()` is fine — it's just one variable."

Wrong. A Node.js SSR process is long-lived and handles many concurrent requests over shared
module memory. A `createPinia()`/`createStore()` call sitting at module scope (executed once
when the module is first `import`ed) is shared by every request the process ever handles after
that — one user's cart, auth state, or session data written into that store during request A's
handling can be read back during request B's handling, concurrently or on a subsequent request.

### Officially grounded rules

- Pinia's SSR pattern is to construct a fresh `createPinia()` per request and pass it into a
  freshly created app instance for that request; the state is retrieved from that specific
  instance's `pinia.state.value` after rendering (`documentation-based`, via Context7
  `/vuejs/pinia`).
- Pinia's own "outside-component usage" guidance states explicitly that in SSR apps, the pinia
  instance must be passed explicitly to `useStore()` calls "to prevent the unintended sharing
  of global state between different application instances during the server-side rendering
  process" (`documentation-based`).
- Vuex's module-reusability pattern requires `state` to be declared as a factory function
  (`state: () => ({...})`) rather than a plain object literal, specifically so that each module
  instance gets its own isolated state object rather than sharing one object reference
  (`documentation-based`, via Context7 `/vuejs/vuex`) — the same "fresh instance per use"
  principle Pinia's SSR guidance requires at the store-instance level.

### Review procedure

1. Grep the SSR entry file(s) for `createPinia(`, `createStore(`, `new Vuex.Store(` and
   determine the enclosing scope of each call site: module top level (executes once at import
   time) vs. inside an exported/invoked per-request handler function.
2. If found at module scope and reused across requests → **HIGH** finding, `ssr-pollution`.
3. If found inside a per-request handler, read the full body of that handler function and list
   every variable it references from enclosing/module scope. Classify each as immutable (safe)
   or mutable/reactive (risk). A per-request `createPinia()` call that also reads/writes a
   module-level cache or default object is still a finding — name the specific closed-over
   reference.
4. For Vuex modules intended to be reusable/instantiated more than once, confirm `state` is a
   function, not a plain object literal — a plain object literal shared across module
   instantiations is the Vuex-specific version of this same defect class.

### Minimal safe pattern

```js
// entry-server.js
export async function render(url) {
  const pinia = createPinia()          // fresh per request
  const app = createApp(App)
  app.use(pinia)
  // ...populate state for this request only, then render...
  return { html, piniaState: pinia.state.value }
}
```

### Anti-pattern (do not approve)

```js
// entry-server.js — WRONG: created once at module load, shared across every request
const pinia = createPinia()

export async function render(url) {
  const app = createApp(App)
  app.use(pinia) // every request shares the SAME pinia instance's state
  // ...
}
```
