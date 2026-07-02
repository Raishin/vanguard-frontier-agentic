---
name: vue-ssr-security-review
description: Statically review Vue 3 SSR entry points and templates for cross-request state pollution (module-scope reactive state, non-per-request app/store creation) and injection via unsanitized v-html or unvalidated dynamic href/src bindings, grounded in Vue's own SSR and security-best-practices guidance.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: security
---

# Vue SSR Security Review

## Purpose

Review Vue 3 server-side-rendered entry points, module-scope state, and template bindings for the two SSR-specific defect classes Vue's own documentation calls out directly: cross-request state pollution from state that is not created fresh per request, and injection through unsanitized `v-html` or unvalidated dynamic `:href`/`:src` bindings — without re-litigating composable/reactivity architecture, hydration-mismatch mechanics, or general component design in every response. This skill exists so the review stays anchored to the two documented, security-critical defect classes instead of drifting into a general "SSR code review."

## When to use

Use this skill when the user asks to:

- review an SSR entry point (`entry-server.js`/`.ts`, or equivalent request-handling code that creates the Vue app for rendering),
- assess whether a `v-html` usage is safe,
- investigate a report of users seeing another user's data on first load or on a subsequent request — the classic cross-request state pollution symptom,
- perform a pre-launch security review of an SSR Vue application.

Do not use this skill for:

- a purely client-rendered (non-SSR) Vue app with no server-rendering entry point — cross-request state pollution does not apply, because each browser tab has its own isolated JS realm and there is no shared server process handling concurrent requests,
- Options API/Composition API architecture review with no security angle (composable extraction quality, reactivity-boundary correctness) — use `vue-composition-api-architecture-review` instead,
- a bug that requires live traffic reproduction (concurrent-request load testing, session-replay capture) to confirm exploitation — static analysis proves the structural risk, not that it has already been exploited in production.

## Context7 Documentation Protocol

- Resolve the Vue library ID with `resolve-library-id` (matched result: `/vuejs/vue`) before citing any SSR-mechanism or `v-html`-behavior claim.
- `/vuejs/vue` is Vue's core source-and-test repository (its SSR test fixtures and `server-renderer` package source cover Vue 2's `vue-server-renderer`), not the Vue 3 prose docs site. Use `query-docs` against it only to corroborate low-level rendering mechanics (e.g., that `v-html` compiles directly to setting the `innerHTML` DOM property with no sanitization step). It does not reliably surface the Vue 3 SSR guide's per-request-instance narrative or the Security guide's `v-html`/dynamic-binding rules — for that guidance, use the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based`.
- Before flagging a specific pattern as cross-request state pollution, confirm the app is actually SSR (a request-handling entry point exists that renders on the server) — the pollution risk is specific to SSR's single, long-lived Node.js process handling many requests; it does not apply to a client-only SPA.
- Read `package.json` first to confirm which Vue major and SSR toolchain are in use (`vue-server-renderer` for Vue 2, `@vue/server-renderer` / a meta-framework like Nuxt for Vue 3) — the per-request app/store creation requirement and the exact API names differ by major version and toolchain; do not apply Vue 3 API names to a Vue 2 codebase or vice versa.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- Cross-request state pollution and injection findings default to HIGH severity. This is a security-scoped skill: do not downgrade a structural cross-request-pollution risk or an untraced `v-html` sanitizer gap to MEDIUM just because it has not been observed exploited yet — the risk is in the structure, not in whether someone has already hit it.
- Trace every finding to a concrete file:line and a concrete data-flow path. A finding that says "this might leak state" or "this v-html might be unsafe" without showing the specific module-scope declaration, the specific reachability path, or the specific unsanitized data-flow trace is not a valid finding — it is a guess.
- Do not treat every module-scope declaration as a pollution risk. An immutable, non-reactive constant (a route table, a static config object, a compiled template) declared at module scope is safe. Only *mutable or reactive* state reachable from an SSR-rendered component's render path is the risk — check both properties (mutability/reactivity, and reachability) before flagging.
- Do not approve a `v-html` binding whose data source includes any user-reachable input (route params, query strings, request bodies, third-party API responses that themselves echo user input) unless a named sanitizer call (e.g., DOMPurify) is visibly present on that exact data-flow path. A sanitizer import existing elsewhere in the codebase does not clear this bar — trace the specific path under review.
- Check dynamic `:href`/`:src` bindings for scheme validation (an allowlist rejecting `javascript:` and other non-`http(s)` schemes) whenever the bound value's source includes user-reachable input. An unvalidated dynamic URL binding fed by user input is a MEDIUM-to-HIGH finding depending on reachability from an authenticated or public surface.
- Watch for per-request factory functions that appear correct (a fresh `createApp()`/`createSSRApp()` call per request) but still close over a shared module-level cache, singleton, or default parameter passed in from outer scope — the factory pattern alone does not guarantee isolation if it references mutable shared state from its enclosing scope.
- Never execute, build, or run application code, and never send live requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the state-pollution/injection decision tree, and the required output shape.
- [Cross-request state pollution](references/cross-request-state-pollution.md) — load only when reviewing an SSR entry point, tracing app/store/router creation, or investigating a suspected cross-request data-leak symptom.
- [Injection: v-html and dynamic URL bindings](references/injection-and-dynamic-urls.md) — load only when the review scope includes a `v-html` usage or a dynamic `:href`/`:src` binding. Includes the OWASP XSS grounding reference; load that citation only when a `v-html` finding is actually present.

## Response minimum

Return, at minimum:

- the SSR entry point(s), module-scope declarations, and/or template bindings in scope,
- ranked findings with file:line evidence, defect category (`state-pollution`, `xss`, or `url-injection`), the concrete data-flow trace (module-scope declaration and its reachability, or the origin-to-sink path for the injection), and a fix sketch matching Vue's documented pattern,
- for every `v-html` finding, an explicit statement of whether a sanitizer call is present on the traced path — never approve on the assumption one exists elsewhere,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming actual cross-request leakage requires concurrent-request load testing, not static review").
