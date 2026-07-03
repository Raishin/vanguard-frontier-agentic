# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load
the domain references only for the specific defect class the store code under review actually
raises.

## Prerequisites

- Identify the store library and version in use (`package.json` — `pinia` +
  `pinia-plugin-persistedstate`, or `vuex` + `vuex-persistedstate`/a custom persistence
  plugin). API names and defaults differ between the two; do not apply one library's names to
  the other.
- Identify whether the app is SSR (an `entry-server.js`/`.ts`, a Nuxt server context, or
  equivalent request-handling entry point exists). If not SSR, defect class (c) — SSR
  singleton pollution — and the server-side half of defect class (b) — un-escaped
  serialization — are out of scope; state this explicitly rather than silently skipping them.

## Workflow

1. **Locate every store definition** (`defineStore(...)` for Pinia, `createStore(...)`/
   `new Vuex.Store(...)`/module objects for Vuex). For each, read the full `state` shape.
2. **Classify every state field by sensitivity.** Auth/refresh tokens, session identifiers,
   API keys, PII (name, email, address, government ID), and any field whose disclosure would
   aid session hijacking or identity theft are sensitive. UI preferences, non-sensitive display
   config, and derived/computed-only values are not.
3. **Trace persistence configuration for every store with a `persist` option (Pinia) or a
   `vuex-persistedstate`/custom persistence plugin registration (Vuex).** For each, determine:
   `storage` target (`localStorage` default vs. `sessionStorage` vs. custom), and whether a
   `pick`/`paths` restriction excludes every sensitive field identified in step 2. See
   `references/persistence-and-hydration.md`.
4. **Trace SSR store creation** (if SSR). Determine whether `createPinia()`/`createStore()` is
   invoked inside the per-request handler function or at module scope, and whether the
   per-request factory closes over any module-scope mutable/reactive reference. See
   `references/persistence-and-hydration.md`.
5. **Trace server-to-client state hydration** (if SSR). Find the server-side serialization of
   store state into the HTML response (e.g. embedding `JSON.stringify(pinia.state.value)` or
   `devalue(...)` output in a `<script>` block) and the client-side consumption
   (`pinia.state.value = JSON.parse(window.__pinia)` or equivalent). Determine whether the
   serialization step escapes the value for safe HTML embedding, and whether the client applies
   any validation before hydrating. See `references/persistence-and-hydration.md`.
6. **Enumerate every store plugin, `$subscribe`, `$onAction` (Pinia), or Vuex plugin/module
   registration.** For each, trace what the hook does with the mutation/action payload — does
   it write, call an external API, log, or mutate another store based on unvalidated payload
   content? See `references/untrusted-payloads-and-authorization.md`.
7. **Search for client-side role/permission flags used to gate a mutating action** — grep for
   patterns like `if (store.isAdmin)`, `if (userStore.role === ...)` guarding a network call or
   a store action that performs a write, and check whether the corresponding server endpoint
   independently re-authorizes. See `references/untrusted-payloads-and-authorization.md`.
8. **Check devtools configuration** for a production build — a Vuex `devtools: true` (or
   unset, since default behavior should be checked against the version in use) in a config
   file reachable by a production build step, or any explicit repo-specific override that
   force-enables a devtools integration in production. See
   `references/untrusted-payloads-and-authorization.md`.
9. **Produce ranked findings** using the output contract below.

## Decision tree

- Store `state` includes a sensitive field (token/session-id/PII) AND `persist` has no
  `pick`/`paths` restriction excluding it (or `persist: true`/`persist: {}`) → **HIGH**
  finding, category `persistence`. Note the `storage` target explicitly (localStorage is worse
  than sessionStorage, but both are XSS-exfiltratable — do not treat `sessionStorage` alone as
  clearing the finding for a sensitive field).
- Store `state` includes a sensitive field AND `pick`/`paths` demonstrably excludes every
  sensitive field → not a finding for those fields; state this explicitly.
- Store `state` contains only non-sensitive fields (UI prefs, non-PII display config) →  not a
  finding regardless of persistence config; state this explicitly rather than omitting the
  store from the review.
- Server embeds serialized store state in the HTML response using naive `JSON.stringify`
  interpolation with no escaping and no `devalue`/equivalent safe-serialization call → **HIGH**
  finding, category `hydration` (XSS via state-serialization breakout).
- Client hydrates `pinia.state.value = JSON.parse(window.__pinia)` (or Vuex equivalent) with no
  shape/schema validation of the parsed result before use → **HIGH** finding, category
  `hydration` (trusting an untrusted payload), unless the value is proven fully
  server-controlled with no user-reachable content anywhere in its construction.
- Server-side serialization is escaped (`devalue` or equivalent) and/or client-side hydration
  validates shape before use → not a finding; state this explicitly.
- SSR entry creates `createPinia()`/`createStore()` at module scope, reused across requests →
  **HIGH** finding, category `ssr-pollution`.
- SSR entry's per-request factory closes over a module-scope mutable/reactive reference (a
  cache, a singleton, a mutable default parameter) → **HIGH** finding, category `ssr-pollution`
  — name the specific closed-over reference.
- SSR entry creates the store fresh inside the per-request handler with no closed-over mutable
  state → not a finding; state this explicitly.
- A `$subscribe`/`$onAction`/Vuex-plugin hook performs a side effect (write/external
  call/sensitive log/cross-store mutation) using unvalidated payload content that is reachable
  from user-controlled input (route params, form input, an action argument sourced from a
  request) → **HIGH** finding, category `untrusted-payload`.
- A `$subscribe`/`$onAction`/Vuex-plugin hook is read-only (observation, non-sensitive
  analytics logging) → not a finding; state this explicitly.
- A client-side role/permission flag gates whether a mutating network call is made, and no
  server-side re-authorization is visible in the code under review → **HIGH** finding, category
  `client-auth-flag`. If server-side authorization code is out of scope/not provided, state
  this as an explicit open question rather than assuming it exists.
- A client-side role/permission flag gates only UI visibility (hiding a button/menu item) with
  the actual mutating action separately confirmed to be re-authorized server-side → not a
  finding; state this explicitly.
- Devtools integration is explicitly enabled (or left at a default that enables it) in a config
  path reachable by the production build → **HIGH** finding, category `devtools-exposure`. Do
  not invent a Pinia `devtools` store option — ground any Pinia-specific devtools concern in
  the build-time devtools plugin, and label as `inference` unless a concrete repo config is
  found; ground Vuex findings in the documented `devtools: boolean` store option.

## Output contract

Every response from this skill must return:

1. **Scope** — the store definition(s), persistence config, SSR entry point(s), and/or
   plugin/hook code reviewed.
2. **Ranked findings** — each with file:line, defect category (`persistence` / `hydration` /
   `ssr-pollution` / `untrusted-payload` / `client-auth-flag` / `devtools-exposure`), the
   concrete data-flow trace (naming every hop), and a fix sketch matching the grounding
   library's documented pattern.
3. **Persistence coverage statement** — for every persistence finding, which fields are scoped
   out by `pick`/`paths` (if any) and which are not.
4. **Server-authorization statement** — for every client-auth-flag finding, whether a
   server-side re-check was found, not found, or is out of scope for this review.
5. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`.
   Label structural risk findings as structural risk explicitly — do not imply confirmed
   exploitation without live evidence.
6. **Verdict** — approve / approve-with-notes / block.
7. **Open questions or out-of-scope items** — e.g., "confirming actual cross-request leakage
   requires concurrent-request load testing," "server-side authorization for this action was
   not in the provided scope," or "v-html/URL-injection review of this same app is out of scope
   for this skill — see `vue-ssr-security-review`."

## When to push back

Push back if the user asks to:

- approve an unscoped `persist: true` on a store containing a token/session field because "we
  only persist it for convenience" — convenience is not a mitigant; the field must be excluded
  via `pick`/`paths` or persistence removed,
- treat a client-side `isAdmin`/`role` flag as sufficient authorization because "the UI already
  hides the button" — a hidden button does not stop a direct API call or a devtools-patched
  store value; the server must re-check,
- skip the SSR-singleton check because "we haven't seen cross-user leakage in production" —
  this defect class is structural and often invisible until concurrent load exposes it,
- accept naive `JSON.stringify` state embedding as "probably fine since it's just our own data"
  — any state reachable from user-submitted content anywhere upstream needs escaping; assume
  user-reachability unless proven otherwise,
- downgrade an untraced `$subscribe`/`$onAction` finding to informational because "it's probably
  fine" — this skill's default is HIGH for exactly this class of unproven claim.
