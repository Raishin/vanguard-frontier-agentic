# Cross-Request State Pollution

Use this reference only when reviewing an SSR entry point, tracing app/store/router creation, or investigating a suspected cross-request data-leak symptom (a user reporting they saw another user's data on load).

## What people get wrong

The naive assumption is:

> "The server renders each request separately, so state can't leak between users."

Wrong. A Node.js SSR server is a single long-lived process handling many concurrent requests on shared module-level memory. If the Vue app instance, its store, or any reactive state it reads is created once at module load time — rather than freshly for each request — every request after the first reads and mutates the *same* in-memory objects. One user's session data, cart contents, or auth state can render into another user's response. This is not a rare edge case; it is the default failure mode of naively porting client-only Vue code to an SSR entry point.

## Officially grounded requirement

Vue's own server-side-rendering guidance is explicit and non-negotiable on this point: **each incoming request must get a fresh, isolated instance of the root Vue app** (and, by extension, any store or router it depends on) rather than sharing one instance across requests. This is the single most important structural rule for SSR safety and is documented as a requirement, not a recommendation (`documentation-based`, Vue SSR guide).

The practical shape of the rule:

- The SSR entry point exports (or defines inline) a per-request factory function.
- That factory function is invoked once per incoming request.
- Inside that factory, `createSSRApp()` (or the toolchain-equivalent app-creation call) runs fresh — along with any Pinia/Vuex store instance and router instance the app needs — and only *that* freshly created instance is used to render the current request's response.
- Nothing about the freshly created instance is retained, cached, or reused for a subsequent request.

## Non-negotiable design rules

### 1. Trace app creation to its actual call site, not its apparent location

Do not accept "there's a `createSSRApp()` call in this file" as sufficient. Confirm it executes *inside* the function that runs per request, not at module top level where it would execute once when the server process starts.

### 2. Store and router instances follow the same rule as the app instance

A correctly per-request `createSSRApp()` call does not fix a module-scope Pinia store or a module-scope router instance created once and imported into every request's app tree. Check store/router creation with the same rigor as the app instance itself — these are frequently overlooked because the app-creation call "looks right" while the store creation nearby does not.

### 3. Closures over shared mutable state defeat an otherwise-correct factory

A per-request factory function that itself creates a fresh app can still leak if it closes over a module-level cache, singleton, or a mutable default parameter supplied from outer scope. Read the full body of the factory function, not just its `createSSRApp()` line — check every variable it references from enclosing scope, and classify each as immutable/safe or mutable/reactive/shared.

### 4. Immutable constants are not the risk; mutable and reactive state is

A module-scope `const ROUTES = [...]` frozen route table, a static config object with no runtime mutation path, or a compiled template string is safe to share across requests — it never changes after module load. The risk is specifically state that is either declared with `ref()`/`reactive()` (Vue's reactivity primitives, which are designed to be mutated and observed) or is a plain mutable object/array that request-handling code writes to. Classify every module-scope declaration on this axis before flagging it.

### 5. Reachability from the SSR-rendered tree is required for a finding

A mutable module-scope object that no SSR-rendered component or composable ever imports, reads, or writes is not a reachable risk in this review's scope (it may still be dead code or a different kind of bug, but it is not a cross-request pollution finding). Trace the import graph from the flagged declaration to at least one component or composable actually rendered during SSR before calling it a finding.

## Minimal safe implementation pattern

The safe shape, matching Vue's documented guidance:

```js
// entry-server.js
export async function render(url, context) {
  // Fresh, per-request instances — nothing here is created at module scope.
  const { app, router, store } = createApp()

  router.push(url)
  await router.isReady()

  // Populate store state for this request only; store was just created above.
  const html = await renderToString(app, context)

  // app, router, store all go out of scope when this function returns.
  return html
}
```

Anti-pattern (module-scope creation — do not approve):

```js
// entry-server.js — WRONG: created once at module load, shared across all requests
const app = createApp()
const store = createStore()

export async function render(url, context) {
  router.push(url)
  await router.isReady()
  return renderToString(app, context) // every request renders the SAME app/store instance
}
```

## Adversarial checklist

Before clearing an SSR entry point as safe from cross-request pollution, answer these:

- Does the app-creation call execute inside the per-request handler, or at module scope?
- Does the store (if any) get created fresh inside that same per-request scope, or is it a module-level singleton imported into the app?
- Does the router get created fresh inside that same per-request scope?
- Does the per-request factory function close over any variable from its enclosing module scope that is mutable or reactive?
- Is there any module-scope cache (e.g., a component-render cache, a computed-property memoization keyed loosely, a "last user" convenience variable) that a developer might have added for performance and forgotten carries cross-request state?
- If two requests from different users hit this entry point concurrently, is there any shared object either request's handling code could write to that the other request's handling code could read?

If any answer reveals a "yes" to shared mutable reachable state, or the app/store/router creation cannot be confirmed as per-request, the finding is HIGH and structural — report it even without a reproduced incident.

## Verification targets

- Grep the SSR entry file for `createApp(` / `createSSRApp(` / `createStore(` / `createRouter(` and confirm each call site's enclosing scope (module-level `import`/top-level statement vs. inside an exported/invoked function).
- Grep for `let `/`var `/mutable `const` object or array literals at module scope in files imported by the SSR entry point or any SSR-rendered component.
- Grep for `ref(`/`reactive(` calls outside of a component's `setup()` or a composable function body — a `ref()`/`reactive()` call at true module scope (not inside any function) is the clearest structural signal of this defect class.
