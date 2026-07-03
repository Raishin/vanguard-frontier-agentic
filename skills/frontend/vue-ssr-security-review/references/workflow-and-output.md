# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific defect class the SSR code or template under review actually raises.

## Prerequisites

- Confirm the app is actually SSR: a request-handling entry point exists (`entry-server.js`/`.ts`, a Nuxt/framework server route, or equivalent) that renders Vue output on the server per incoming request. If no such entry point exists, cross-request state pollution is out of scope — flag this explicitly and move directly to the injection-only concerns (`v-html`, dynamic URL bindings), which apply to any Vue app regardless of rendering mode.
- Identify the Vue major and SSR toolchain in use (`package.json` — `vue`, `vue-server-renderer` for Vue 2, `@vue/server-renderer` or a meta-framework for Vue 3). Per-request creation APIs and exact guidance differ by major.

## Workflow

1. **Locate every SSR entry point.** For each, read the full request-handling function from the top.
2. **Trace app/store/router creation.** For each entry point, determine whether `createSSRApp()` (or the toolchain's equivalent app-factory call) — and any store (Pinia/Vuex) or router instance the app depends on — is created *inside* the per-request handler function, freshly, for every request. See `references/cross-request-state-pollution.md` for the decision tree.
3. **Enumerate module-scope declarations reachable from SSR-rendered components.** Grep for `ref()`, `reactive()`, plain mutable object/array literals, and module-level caches or singletons declared outside any per-request factory function. For each, check reachability: is it imported, read, or written by any component or composable in the SSR-rendered component tree?
4. **Enumerate every `v-html` binding in scope.** For each, trace its data source backward through props, computed values, store state, and API responses to the origin. Determine whether the origin includes user-reachable input (route params, query strings, request bodies, or a third-party API response that itself echoes user input) and whether a named sanitizer call sits on that exact path. See `references/injection-and-dynamic-urls.md`.
5. **Enumerate every dynamic `:href`/`:src` binding in scope.** For each, trace its data source the same way. Check for scheme validation (an allowlist rejecting `javascript:` and other non-`http(s)` schemes) when the source includes user-reachable input.
6. **Produce ranked findings** using the output contract below.

## Decision tree

- SSR entry point creates the app/store/router at module scope (outside the per-request handler) → **HIGH** finding, cross-request state pollution risk. Cite the Vue SSR guide's per-request-instance requirement directly (`documentation-based`).
- SSR entry point's per-request factory function closes over a shared module-level mutable cache, singleton, or default parameter from outer scope → **HIGH** finding — the factory pattern alone does not guarantee isolation; name the specific closed-over reference.
- A mutable or reactive module-scope declaration (not an immutable constant) is reachable — read or written — from any SSR-rendered component's render path → **HIGH** finding regardless of whether pollution has been observed yet; the risk is structural, not conditional on an incident report.
- Module-scope declaration is an immutable, non-reactive constant (static config, compiled route table, frozen lookup object) with no runtime mutation path → not a finding.
- `v-html` binding's traced data source includes user-reachable input and no sanitizer call is present on that exact path → **HIGH** finding, XSS. Do not accept "a sanitizer exists elsewhere in this codebase" as clearing this — the trace must show the sanitizer on the specific path reviewed.
- `v-html` binding's traced data source is fully origin-controlled with no user-reachable input anywhere in the trace (e.g., static marketing copy authored by the app's own CMS with no user-submission path) → not a finding, but state this explicitly in the output rather than silently omitting it.
- Dynamic `:href`/`:src` binding's traced source includes user-reachable input with no scheme allowlist/validation → **MEDIUM-to-HIGH** finding depending on reachability (public unauthenticated surface vs. requiring an authenticated session to trigger).
- Dynamic `:href`/`:src` binding's source is either fully origin-controlled or already passes through scheme validation → not a finding.

## Output contract

Every response from this skill must return:

1. **Scope** — the SSR entry point(s), module-scope declarations, and/or template bindings reviewed.
2. **Ranked findings** — each with file:line, defect category (`state-pollution` / `xss` / `url-injection`), the concrete data-flow trace (the module-scope declaration and its reachability path, or the full origin-to-sink path for the injection finding, naming every hop), and a fix sketch matching Vue's documented pattern.
3. **Sanitizer status per `v-html` finding** — an explicit statement of whether a sanitizer call is present on the traced path; never infer one exists.
4. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`. Label structural risk findings as structural risk explicitly — do not imply confirmed exploitation without live evidence (e.g., a captured cross-user response, a load-test reproduction).
5. **Verdict** — approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** — e.g., "confirming actual cross-request leakage requires concurrent-request load testing, not static review," or "hydration-mismatch risk in this same file is out of scope — recommend a hydration-focused review if the framework is Angular-equivalent, otherwise out of scope for this Vue-focused skill."

## When to push back

Push back if the user asks to:

- approve a `v-html` usage because "we sanitize elsewhere in the app" without a sanitizer call visible on the specific traced path — that is not evidence, it is an assumption,
- treat a per-request `createSSRApp()` call as sufficient in isolation without checking what it closes over — the factory call passing a surface-level check is not the same as proving isolation,
- skip the cross-request-pollution check because "we haven't seen it happen in production" — this defect class is structural and often invisible until concurrent load or a specific request-timing race exposes it; absence of a reported incident is not evidence of absence of the risk,
- downgrade an untraced `v-html` finding to informational because "it's probably fine" — this skill's default is HIGH for exactly this class of unproven claim.
