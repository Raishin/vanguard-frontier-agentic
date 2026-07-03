# Untrusted Payloads, Client-Side Authorization Flags, and Devtools Exposure

Use this reference when reviewing `$subscribe`/`$onAction` (Pinia) or plugin/module code
(Vuex) that consumes mutation/action payloads, when checking whether a client-held role/flag is
being used as an authorization boundary, or when checking devtools configuration for
production exposure.

## Defect (d), part 1: store plugins / $subscribe / $onAction acting on untrusted payloads

### What people get wrong

The naive assumption is:

> "This is my own store's hook — the payload came from my own action, so it's already trusted
> by the time it reaches the hook."

Wrong when the action's argument itself originated from user-controlled input (a route param,
a form field, a query string, a request body forwarded into the action call) and the hook does
something with side effects. Pinia's `$subscribe` and `$onAction` hooks are generic
instrumentation points — they receive whatever `mutation`/`args` content the calling code
passed, with no built-in sanitization or validation (this is an architectural fact, not a
defect in Pinia itself: the hook is a generic observation/interception point and the trust
boundary is defined entirely by the caller).

### Officially grounded API shape (do not invent beyond this)

Confirmed via Context7 `/vuejs/pinia` (`documentation-based`):

- `store.$subscribe((mutation, state) => {...})` — the callback receives a `mutation` object
  (`mutation.type`: `'direct' | 'patch object' | 'patch function'`, `mutation.storeId`,
  `mutation.payload` for patch-object mutations) and the current `state`.
- `store.$onAction(({ name, store, args, after, onError }) => {...})` — the callback receives
  the action `name`, the `store` instance, an `args` array of the parameters passed to the
  action, and `after`/`onError` hooks for post-action handling.
- Vuex plugins receive the `store` instance directly (`store => {...}`) and can subscribe to
  mutations/actions via `store.subscribe`/`store.subscribeAction`; a namespaced module's action
  context includes `rootState`/`rootGetters`, giving plugin/action code reach across module
  boundaries (`documentation-based`, via Context7 `/vuejs/vuex`).

Do not describe a hook argument, method, or option beyond what is listed above — if a
reviewed codebase appears to use something not confirmed here, treat the claim about its
*documented* behavior as unverified and say so, rather than asserting a specific contract.

### Review procedure

1. Enumerate every `$subscribe`, `$onAction` (Pinia), or `store.subscribe`/
   `store.subscribeAction`/plugin registration (Vuex) in scope.
2. For each, read the hook body. Classify: does it only observe (log a mutation type, emit an
   analytics event with non-sensitive metadata) or does it act (write to a DB, call an external
   API, mutate a *different* store, log payload content that could be sensitive)?
3. For hooks that act, trace the payload's (`mutation.payload` / `args`) origin backward to the
   action call site. Does the action's argument ever originate from user-controlled input
   (a route param, a form value, a request body) with no validation between that input and the
   hook's consumption of it?
4. If yes and the hook has a side effect on unvalidated content → **HIGH** finding,
   `untrusted-payload`. Name the specific hook, the specific action/mutation, and the specific
   unvalidated hop.
5. If the hook is read-only or the payload is already validated/sanitized before the hook
   consumes it → not a finding; state this explicitly.

### Anti-pattern (do not approve)

```ts
// A plugin acts on unvalidated action args reachable from a route param.
pinia.use(({ store }) => {
  store.$onAction(({ name, args }) => {
    if (name === 'updateProfile') {
      // args[0] flows from a route param with no validation upstream —
      // writing it straight to an external audit-log API with no sanitization.
      auditLogApi.post('/log', { field: args[0].bio })
    }
  })
})
```

## Defect (d), part 2: client-held flags used as an authorization source of truth

### What people get wrong

The naive assumption is:

> "The store says `isAdmin: true` only after the server told us the user is an admin, so
> checking `store.isAdmin` in the client before calling the delete endpoint is safe."

Wrong. Once a value lives in client-side reactive state, it is client-writable — via browser
devtools, a `$patch()` call from the console, direct manipulation of a hydrated store, or a
compromised/malicious browser extension. A store flag is a UI convenience for *display*
decisions; it is never proof of authorization for a *mutating* decision. The only correct
authorization boundary is the server independently re-checking the authenticated user's
permissions when the mutating request actually arrives.

### Review procedure

1. Grep for role/permission-flag reads gating a mutating call: patterns like
   `if (store.isAdmin)`, `if (userStore.role === 'admin')`, `v-if="store.canDelete"` guarding an
   action dispatch or a direct API call that performs a write (delete, update, privilege
   change, financial transaction).
2. For each match, determine what the guarded code actually does:
   - Gates only UI visibility (hides/shows a button, disables a form field) with the
     underlying mutating action's authorization enforced elsewhere → not a finding for that
     specific gate, but confirm (or explicitly flag as unconfirmed) that the actual mutating
     endpoint re-authorizes server-side.
   - Gates whether the mutating network call is *made at all*, with no evidence the server
     independently re-checks authorization on that request → **HIGH** finding,
     `client-auth-flag`, regardless of how the flag was originally populated.
3. If the server-side code implementing the endpoint is not in scope/not provided, say so
   explicitly as an open question rather than assuming a check exists — do not clear the
   finding on the assumption that "surely the backend checks this too."

### Anti-pattern (do not approve)

```ts
// store.isAdmin is client-side reactive state — devtools-patchable.
async function deleteUser(id: string) {
  if (userStore.isAdmin) {           // WRONG: client flag treated as the auth boundary
    await api.delete(`/users/${id}`) // if the endpoint doesn't re-check, this is a
  }                                   // full broken-access-control vulnerability
}
```

### Minimal safe pattern

```ts
// Client-side check is UX-only (avoids showing an error after the fact);
// the actual authorization decision is made server-side on every request.
async function deleteUser(id: string) {
  if (userStore.isAdmin) {
    // Optimistic UX gate only — the server independently re-verifies the
    // caller's role/permissions before performing the deletion.
    await api.delete(`/users/${id}`)
  }
}
```

Note: the client code above is nearly identical in shape to the anti-pattern — the difference
is entirely in the server-side endpoint, which is why this defect class requires confirming
(or explicitly flagging as unconfirmed) the server-side check rather than judging the client
code in isolation.

## Defect (e): devtools state exposure in production builds

### What people get wrong

The naive assumption is:

> "Devtools only matter in development — production builds strip that stuff automatically."

Not reliably true without an explicit build-time step, and Vuex exposes an *explicit runtime
option* that can force devtools integration on regardless of environment.

### Officially grounded rule

Vuex's store options API documents a `devtools: boolean` option that "activates or deactivates
devtools integration for a Vuex instance," called out specifically as useful when running
multiple stores on a single page (`documentation-based`, via Context7 `/vuejs/vuex`). If a
production build's store configuration sets `devtools: true` (or leaves a config path that
defaults to enabling it reachable in production), the full state tree, mutation/action
history, and time-travel debugging are exposed to anyone with browser devtools open — a
meaningful exposure if the store holds any sensitive data (tokens, PII, internal flags).

Context7 does not confirm an equivalent `devtools` boolean option on Pinia's `defineStore`/
`createPinia` APIs — Pinia's devtools integration is wired through the Vue devtools browser
extension/plugin rather than a store-constructor option. Do not assert a Pinia-specific
`devtools` config option exists. If a codebase under review appears to force-enable a devtools
integration in a production build (e.g., an explicit plugin registration reachable in the
production bundle), flag that as a repo-evidence finding scoped to the specific code found —
do not generalize it into a claim about a Pinia API that Context7 does not confirm.

### Review procedure

1. Grep store-configuration files for a `devtools:` option (Vuex) and determine whether the
   value is `true`, absent (check documented default for the version in use), or explicitly
   tied to an environment check (`process.env.NODE_ENV !== 'production'` or equivalent).
2. If `devtools: true` (or an unguarded default) is reachable in a production build path →
   **HIGH** finding, `devtools-exposure`.
3. If gated behind an environment check that correctly excludes production → not a finding;
   state this explicitly.
4. For Pinia codebases, check only for an explicit, repo-specific forced-enable of a devtools
   plugin in a production bundle path — do not invent or assume a `devtools` constructor
   option exists on `createPinia()`/`defineStore()`.

### Minimal safe pattern

```js
// Vuex — devtools explicitly disabled outside development
const store = createStore({
  // ...
  devtools: process.env.NODE_ENV !== 'production',
})
```

### Anti-pattern (do not approve)

```js
// Vuex — WRONG: devtools unconditionally enabled, including in production builds
const store = createStore({
  // ...
  devtools: true,
})
```
