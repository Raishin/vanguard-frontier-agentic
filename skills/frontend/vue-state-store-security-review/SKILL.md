---
name: vue-state-store-security-review
description: Statically review Pinia and legacy Vuex state stores for sensitive data persisted to localStorage/sessionStorage without scoping, untrusted server-payload hydration (window.__pinia/__INITIAL_STATE__) with un-escaped state serialization, SSR store-singleton cross-request pollution, store plugins/$subscribe/$onAction acting on untrusted payloads, client-held role flags used as an authorization source of truth, and devtools state exposure in production builds.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# Vue State Store Security Review

## Purpose

Review Pinia and legacy Vuex state-store code for the security-critical defect classes that
are specific to *state stores* as a component — not general Vue architecture, not composable
design, and not SSR hydration/injection issues that belong to `vue-ssr-security-review`. This
skill exists to keep the review anchored to five documented defect classes: (a) sensitive data
persisted client-side without scoping, (b) store hydration from an untrusted server payload
with un-escaped serialization, (c) SSR store-singleton cross-request pollution, (d) store
plugins/hooks acting on untrusted payloads and client-held flags used as an authorization
source of truth, and (e) devtools state exposure in production. It does not re-litigate
general reactivity/composable architecture, v-html/URL-injection review (covered by
`vue-ssr-security-review`), or Composition API design quality (covered by
`vue-composition-api-architecture-review`) in every response.

## When to use

Use this skill when the user asks to:

- review a Pinia store definition (`defineStore`) or a legacy Vuex module/store for security
  issues,
- assess whether `pinia-plugin-persistedstate` / `vuex-persistedstate` configuration is safe
  (what gets persisted, and where),
- review SSR store creation/hydration code (`entry-server.js`, a Nuxt server plugin, an
  Express/Node handler that constructs `createPinia()`/`createStore()`) for cross-request
  pollution risk,
- review a store plugin, `$subscribe`/`$onAction` hook, or Vuex plugin that consumes
  mutation/action payloads,
- investigate whether a client-side role/permission flag (`isAdmin`, `role`) is being trusted
  as an authorization boundary,
- perform a pre-launch security review of a Pinia/Vuex-based application's state layer.

Do not use this skill for:

- `v-html`/dynamic-URL injection review or SSR entry-point app/router creation — use
  `vue-ssr-security-review` for those; this skill covers the *store* specifically, not the
  broader SSR rendering surface (load both skills together if the review spans both),
- Composition API/composable architecture quality with no security angle — use
  `vue-composition-api-architecture-review` instead,
- a general "which state-management library should we pick" architecture decision with no
  security defect in scope — use `state-management-decision-review` instead,
- a bug that requires live traffic reproduction (concurrent-request capture, a devtools
  screen-recording of production, live token exfiltration) to confirm exploitation — static
  analysis proves the structural risk, not that it has already been exploited.

## Context7 Documentation Protocol

- Resolve library IDs with `resolve-library-id` before citing any store-behavior claim. This
  skill's three grounding libraries: `/vuejs/pinia` (Pinia core — SSR hydration, plugins,
  `$subscribe`/`$onAction`), `/prazdevs/pinia-plugin-persistedstate` (persistence defaults and
  `pick`/`storage` config), `/vuejs/vuex` (legacy Vuex — `state` as function, `devtools`
  option, plugin/module API).
- Use `query-docs` against `/prazdevs/pinia-plugin-persistedstate` to confirm persistence
  defaults before flagging a persistence config: `storage` defaults to `localStorage` when
  unset; `pick` (an array of dotted state-path strings) restricts persistence to named paths;
  with no `pick`, the entire state is persisted. Cite these as `documentation-based`.
- Use `query-docs` against `/vuejs/pinia` to confirm SSR hydration mechanics before flagging a
  hydration finding: the documented client-side pattern is
  `pinia.state.value = JSON.parse(window.__pinia)`, and Pinia's own SSR guide states escaping
  the serialized state is "**VERY important** if the content of the state can be changed by
  the user, which is almost always the case," recommending `devalue` (or equivalent) over naive
  `JSON.stringify`. Cite as `documentation-based`.
- Use `query-docs` against `/vuejs/pinia` to confirm `$subscribe`/`$onAction` hook signatures
  (mutation object with `type`/`storeId`/`payload`; action object with `name`/`store`/`args`/
  `after`/`onError`) before describing a plugin-hook finding — do not invent a hook name or
  argument Context7 does not confirm.
- Use `query-docs` against `/vuejs/vuex` to confirm the `state: () => ({...})` factory pattern
  (module reusability without shared state) and the `devtools: boolean` store option before
  citing either — do not assume Pinia has an equivalent `devtools` boolean on `defineStore`/
  `createPinia`; Context7 does not confirm that API surface for Pinia, so any Pinia-devtools
  concern must be scoped to "a build-time devtools plugin explicitly force-enabled in
  production," not a Pinia store option, and labeled `inference` unless a repo-specific config
  is found.
- Read `package.json` first to confirm which store library is in play (`pinia`,
  `pinia-plugin-persistedstate`, `vuex`, `vuex-persistedstate`) and its major version — API
  names and defaults differ between Pinia and Vuex and across Vuex 3/4; do not apply one
  library's API names to the other.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's
  `metadata.json` and label the claim `documentation-based, unverified against current
  release`.

## Lean operating rules

- Findings in every defect class below default to HIGH severity: unscoped sensitive-data
  persistence, untrusted/un-escaped state hydration, SSR store-singleton pollution, untrusted
  payload handling in store plugins/hooks, client-side-flag-as-authorization, and production
  devtools exposure. Do not downgrade a structural finding to MEDIUM because it has not been
  observed exploited — the risk is in the structure.
- Trace every finding to a concrete file:line and a concrete data-flow path. "This store might
  leak sensitive data" or "this hydration looks risky" without naming the specific
  `persist`/`pick`/`storage` config, the specific module-scope `createPinia()`/`createStore()`
  declaration, or the specific untraced payload sink is not a valid finding — it is a guess.
- Before flagging a persistence config, read the full `state()` shape and the full `persist`
  config together. A `persist: true` (or `persist: {}`) with no `pick`/`paths` array persists
  the entire state — treat every sensitive field in that state as persisted unless a `pick`
  list demonstrably excludes it. A `pick` list that omits the sensitive field(s) clears the
  finding for those fields specifically (not for the whole store, if other sensitive fields
  remain unscoped).
- Before flagging SSR store creation as cross-request pollution, confirm mutability/reachability
  the same way `vue-ssr-security-review` requires for app instances: is the `createPinia()`/
  `createStore()` call inside the per-request handler, and does that handler close over any
  module-scope mutable/reactive reference? An immutable module-scope constant (a frozen config
  object, a static route table) is not the risk; a store instance or mutable cache is.
- Do not approve a client-side role/permission flag (`isAdmin`, `role`, `permissions`) as an
  authorization boundary for a mutating action unless the review also confirms (via visible
  server-side code, or an explicit statement that server-side authorization is out of scope for
  this static review) that the actual mutation is re-checked server-side. A store flag gating
  only UI visibility (hiding a button) is not itself a finding; a store flag gating whether a
  mutating network call is *made* is a finding regardless of UI-layer intent, because the flag
  is client-writable.
- Do not treat every `$subscribe`/`$onAction`/Vuex-plugin hook as risky. Only hooks that act on
  the payload with a side effect (write, external call, log of sensitive data, mutate another
  store) and lack payload validation are findings; read-only observation (e.g., analytics event
  naming) is not.
- Never execute, build, or run application code, and never send live requests, as part of this
  review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the
  step-by-step review procedure, the decision tree across all five defect classes, and the
  required output shape.
- [Client-side persistence and hydration](references/persistence-and-hydration.md) — load when
  reviewing `pinia-plugin-persistedstate`/`vuex-persistedstate` configuration, SSR store
  creation/singleton risk, or server-payload hydration (`window.__pinia`/`__INITIAL_STATE__`).
- [Untrusted payloads and authorization](references/untrusted-payloads-and-authorization.md) —
  load when reviewing `$subscribe`/`$onAction`/Vuex-plugin hooks, client-held role/permission
  flags used for access control, or devtools production exposure.
- [Acceptance rubric](references/acceptance-rubric.md) — the enumerated defect/false-positive
  list this skill's rules were authored against; load if you need the underlying catch list
  rather than the operating rules derived from it.

## Response minimum

Return, at minimum:

- the store definition(s), persistence config, SSR entry point(s), and/or plugin/hook code in
  scope,
- ranked findings with file:line evidence, defect category (`persistence`, `hydration`,
  `ssr-pollution`, `untrusted-payload`, `client-auth-flag`, or `devtools-exposure`), the
  concrete data-flow trace (state field → persist config, or server payload → hydration call,
  or module-scope declaration → per-request reachability, or payload → sink), and a fix sketch
  matching the grounding library's documented pattern,
- for every persistence finding, an explicit statement of which state fields are covered by
  `pick`/`paths` (if any) and which are not,
- for every client-side-flag finding, an explicit statement of whether a server-side
  authorization re-check was found, not found, or is out of scope for this static review,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`), with
  structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming actual cross-request
  leakage requires concurrent-request load testing," or "v-html/URL-injection review of this
  same app is out of scope for this skill — see `vue-ssr-security-review`").
