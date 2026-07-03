# Acceptance Rubric (write before the rest of the skill — this is the failing test)

This rubric enumerates every defect a correct Vue state-store (Pinia / legacy Vuex) security
review MUST catch, and the benign patterns it MUST NOT flag. `SKILL.md` and the other
references are only "done" once every numbered item below is covered by an explicit operating
rule, decision-tree branch, or reference section. `selfCheck` in the final structured output
must map each item to the rule/reference that satisfies it.

## MUST CATCH

1. **Sensitive data persisted via `pinia-plugin-persistedstate` / `vuex-persistedstate` with no
   `pick`/`paths` restriction.** A store whose `state` includes an auth token, refresh token,
   session identifier, or PII field, and whose `persist: true` (or `persist: {}` with no
   `pick`/`paths` array) config persists the *entire* state object — the sensitive field rides
   along by default. `storage` defaults to `localStorage`, which is readable by any script in
   the page's origin (XSS-exfiltratable, no `HttpOnly`-equivalent protection).

2. **Sensitive data persisted to `localStorage` explicitly (not just by default) when
   `sessionStorage` or a narrower `pick` would suffice.** E.g. `storage: localStorage` combined
   with a `pick` list that still includes `token`/`accessToken`/`user.ssn`/etc.

3. **Store hydration from an untrusted/unvalidated server payload.** Client code that reads
   `window.__pinia`, `window.__INITIAL_STATE__`, or an equivalent global and feeds it directly
   into `pinia.state.value = JSON.parse(...)` (or assigns into a Vuex store's state) with no
   schema/shape validation and no evidence the server-side serialization step escaped the value
   before embedding it in the HTML response.

4. **Un-escaped state serialization into the HTML response on the server side.** Server entry
   code that builds the embedded state string via naive `JSON.stringify(state)` interpolated
   directly into a `<script>` block (e.g. a template literal like
   `` `<script>window.__pinia=${JSON.stringify(state)}</script>` ``) with no escaping of
   `<`, `>`, `/`, or use of a safe-serialization library (`devalue` or equivalent) — a stored
   value containing `</script><script>alert(1)</script>` breaks out of the tag and executes.

5. **SSR store singleton created once at module scope instead of freshly per request.** A
   `const pinia = createPinia()` (or `const store = new Vuex.Store({...})` /
   `createStore({...})` assigned to a module-level `const`/`let`) declared at the top level of
   an SSR entry file (`entry-server.js`/`.ts`, a Nuxt server plugin, an Express/Node request
   handler module) and reused/imported across requests instead of being constructed inside the
   per-request handler function. This leaks one user's cart/auth/session state into another
   concurrent user's response.

6. **A per-request store factory that still closes over module-scope mutable/reactive state.**
   The factory function itself calls `createPinia()`/`createStore()` fresh each invocation, but
   also reads or writes a module-level cache, singleton, or mutable default parameter from
   enclosing scope — the factory call looking correct does not prove isolation.

7. **`$subscribe` / `$onAction` plugin hooks (or a Vuex plugin) that consume `mutation`/`args`
   payloads and act on them (write to a DB, call an external API, log, mutate other stores)
   without validating/sanitizing the payload shape or origin** — especially when the
   store/action is reachable from a component that accepts route params, query strings, or
   other user-controlled input as the action's argument.

8. **Client-side store flags (`isAdmin`, `role`, `isPremium`, `permissions`) used as an
   authorization source of truth for gating a sensitive action or UI-triggered mutation**,
   with no corresponding server-side authorization check on the actual mutating request. A
   client can set/patch its own store state (via devtools, `$patch`, or direct console access),
   so `if (store.isAdmin) { await deleteUser(id) }` with no server-side re-check is a broken
   access-control finding, not merely a UX nicety.

9. **Devtools integration left enabled in a production build.** Vuex's `devtools: true`
   (or an unset/default-true devtools option) shipped in a production store configuration
   exposes full state-tree contents, mutation/action history, and time-travel debugging to
   anyone with browser devtools open — a state/PII exposure risk if the store holds sensitive
   data. (Pinia's devtools integration is dev-only-by-default at build time via the Vue
   devtools plugin; flag only if a codebase explicitly force-enables it in a production
   config — do not invent a `devtools` option on Pinia's `defineStore`/`createPinia` APIs,
   which Context7 does not confirm exists.)

## MUST NOT FLAG (explicit false-positive guards)

10. **An immutable, non-sensitive constant persisted intentionally.** A `pick`-scoped
    persistence of a UI preference (`theme`, `locale`, `sidebarCollapsed`) with no
    auth/session/PII content is not a finding, even though it uses `localStorage` — state this
    explicitly as reviewed-and-cleared rather than omitting it silently.

11. **A store correctly re-created per request.** An SSR entry point whose `createPinia()` (or
    `createStore()`) call sits inside the exported per-request handler function, with no
    closed-over mutable module-scope state, is not a finding — do not flag the mere presence of
    `createPinia()` in an SSR file without checking its enclosing scope.

12. **Server-sent state that is properly escaped/serialized (`devalue` or equivalent) before
    embedding, and validated/schema-checked on the client before hydration.** Not a finding —
    state this explicitly as reviewed.

13. **`$subscribe`/`$onAction` hooks used for read-only observation** (e.g. analytics logging
    of a mutation type, a devtools-only debug logger) with no write/mutating side effect and no
    unsanitized use of the payload in a sink — not a finding, but note it was reviewed.

14. **A client-side role/flag used only for non-authoritative UI purposes** (e.g. hiding a menu
    item) where the actual protected action is independently re-authorized server-side — not a
    finding for that specific action, though the review should still confirm the server check
    exists rather than assuming it.

## Traceability requirement

Every finding above must cite a concrete file:line and the exact data-flow path (store
declaration → persistence config / hydration call / SSR entry scope / action-payload sink /
authorization check) — a finding that says "this store might leak" without naming the specific
`persist`/`pick`/`storage` config, the specific module-scope declaration, or the specific
untraced sink is not valid per this skill's traceability rule.
