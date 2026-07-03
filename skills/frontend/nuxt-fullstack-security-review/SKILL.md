---
name: nuxt-fullstack-security-review
description: Statically review Nuxt 3/4 full-stack code for private secrets exposed via runtimeConfig.public/NUXT_PUBLIC_* env vars, useState/module-scope cross-request state pollution in Nitro, server-route SSRF via $fetch/ofetch with blind useRequestHeaders/credential forwarding, NuxtPayload/useState serialization reaching an XSS sink, and missing security response headers (routeRules headers, nuxt-security), grounded in Nuxt's own documentation via Context7.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# Nuxt Fullstack Security Review

## Purpose

Review Nuxt 3/4 full-stack code — `nuxt.config.ts`, `composables/`, `plugins/`,
`server/api/`, `server/routes/`, `server/middleware/`, and templates using
`useState`/`v-html` — for five defect classes Nuxt's own documentation and its
long-lived Nitro server model make security-critical: (a) private secrets placed
under `runtimeConfig.public` (or fed by `NUXT_PUBLIC_*` env vars) so they ship in the
client bundle, (b) `useState`/module-scope reactive or mutable state leaking across
requests in Nitro's single long-lived process, (c) server-route SSRF via
`$fetch`/`ofetch` to a user-controlled URL and blind forwarding of
`useRequestHeaders()`/credentials, (d) `NuxtPayload`/`useState` data reaching an
unsanitized render sink (XSS), and (e) missing security response headers
(`routeRules` `headers`, the `nuxt-security` module, `useResponseHeader`). This
skill exists so the review stays anchored to these five documented, structural
defect classes instead of drifting into a general "Nuxt code review."

## When to use

Use this skill when the user asks to:

- review a Nuxt `nuxt.config.ts` `runtimeConfig`/`routeRules` block for
  secret-exposure or missing-header risk,
- review a `server/api/*` or `server/routes/*` handler that calls out to another
  service (`$fetch`/`ofetch`/`event.$fetch`),
- investigate a report of one user seeing another user's data from a Nuxt app — the
  classic cross-request state pollution symptom,
- assess whether a `useState`/payload value rendered with `v-html` is safe,
- perform a pre-launch security review of a Nuxt 3/4 full-stack application.

Do not use this skill for:

- a Nuxt app's client-only component architecture, composable-extraction quality, or
  reactivity-boundary design with no security angle — use
  `vue-composition-api-architecture-review` instead,
- general Vue SSR concerns (non-Nuxt `entry-server.js`, raw `@vue/server-renderer`
  usage) with no Nuxt-specific API involved — use `vue-ssr-security-review` instead,
- Vuex/Pinia store internals or Vue Router navigation-guard security with no
  Nuxt-specific `runtimeConfig`/`server/`/`useState` surface — use
  `vue-state-store-security-review` or `vue-router-navigation-security-review`
  instead,
- a bug that requires live traffic reproduction (concurrent-request load testing, a
  captured cross-user response, an actual SSRF probe against a running deployment) to
  confirm exploitation — static analysis proves the structural risk, not that it has
  already been exploited in production.

## Context7 Documentation Protocol

- Resolve and query `/websites/nuxt_4_x` (primary; Nuxt 4 prose docs) and
  `/websites/nuxt_3_x` (Nuxt 3 prose docs) before citing any `runtimeConfig`,
  `useState`, `$fetch`/`event.$fetch`/`useRequestFetch`/`useRequestHeaders`,
  payload/`devalue`, `routeRules`, or `useResponseHeader` behavior as fact. Both are
  Nuxt's own documentation site content mirrored into Context7 — treat matches from
  either as `documentation-based`.
- Confirm which Nuxt major the target repo uses (`package.json`'s `nuxt` dependency)
  before assuming version-specific defaults; the APIs this skill covers are stable
  across 3/4, but state which major was confirmed when citing a claim.
- The third-party `nuxt-security` module's exact default header set and
  configuration surface is **not** covered by Nuxt's own Context7-indexed docs —
  never state a specific default for it as `documentation-based`. Confirm its
  presence/config by reading the repo's `nuxt.config.ts` directly, and label any
  claim about its behavior `inference` unless corroborated by the module's own
  documentation (not currently in scope for this skill's Context7 grounding).
- Do not invent API names. If Context7 does not confirm an API or default, say so
  explicitly and label the claim `inference`.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's
  `metadata.json` and label the claim `documentation-based, unverified against
  current release`.

## Lean operating rules

- Every finding in this skill's five defect classes defaults to HIGH severity
  (missing-security-headers may be MEDIUM-to-HIGH depending on the app's actual
  surface — see `references/ssrf-payload-and-response-headers.md`, Part 3). Do not
  downgrade a structural `runtimeConfig` exposure, cross-request state leak, SSRF
  path, or payload-XSS trace to informational because it has not been observed
  exploited yet.
- Trace every finding to a concrete file:line and a concrete data-flow path. A
  finding that says "this might leak the secret" or "this fetch call could be
  SSRF" without showing the specific config key, the specific module-scope
  declaration and its reachability, or the specific origin-to-sink trace is a
  guess, not a finding.
- Classify every `runtimeConfig` key by its actual nesting (top-level = private,
  under `public` = client-exposed) — never by variable name alone. A key named
  `apiSecret` sitting inside `public` is exposed; a key named `baseUrl` sitting
  outside `public` is still private and not itself a finding.
- Classify every module-scope declaration on two axes before flagging it:
  mutability/reactivity (only `useState`/`ref`/`reactive`/mutable objects are at
  risk; immutable constants are not), and reachability from server-rendered code
  (a declaration no server-rendered path ever touches is not a finding in this
  scope).
- Do not clear a `$fetch`/`ofetch`/`event.$fetch` call in a server route as safe
  from SSRF just because it "looks like an API call" — trace the destination URL
  to its origin and confirm either a hardcoded host or an explicit allowlist check
  before the request fires.
- Do not clear a header-forwarding call (`event.$fetch`'s default forwarding,
  `useRequestHeaders(...)`, or manual header spreading) as safe just because
  Nuxt documents the mechanism — the mechanism existing is not the same as its
  use being scoped to only the headers actually needed and only trusted
  destinations.
- Do not approve a `useState`/payload value reaching a `v-html` binding unless a
  named sanitizer call is visibly present on that exact traced path — a sanitizer
  existing elsewhere in the codebase does not clear this bar.
- Do not report "no security headers" as cleared just because *a* mechanism
  exists somewhere in the config — confirm the `routeRules` glob (or module
  config) actually covers the routes in scope before crediting it.
- Never execute, build, or run application code, and never send live requests, as
  part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use
  for the step-by-step review procedure, the full decision tree across all five
  defect classes, and the required output shape.
- [runtimeConfig exposure and cross-request state pollution](references/runtime-config-and-cross-request-state.md)
  — load when reviewing `nuxt.config.ts`'s `runtimeConfig` block, `NUXT_PUBLIC_*`
  env vars, or `useState`/module-scope reactive/mutable declarations reachable
  from server-rendered code.
- [Server-route SSRF, header forwarding, payload XSS, and missing response headers](references/ssrf-payload-and-response-headers.md)
  — load when reviewing a `server/api`/`server/routes` handler's outbound
  `$fetch`/`ofetch`/`event.$fetch`/`useRequestFetch` calls, a `useState`/payload
  value that renders somewhere, or `routeRules`/security-module configuration.
- [Acceptance rubric](references/acceptance-rubric.md) — the authoritative list of
  defects this skill must catch and the false positives it must not raise; consult
  when unsure whether a pattern is in scope.

## Response minimum

Return, at minimum:

- the `runtimeConfig`/`routeRules` blocks, module-scope declarations, server
  routes, and/or template bindings in scope,
- ranked findings with file:line evidence, defect category
  (`runtimeconfig-exposure` / `cross-request-state-pollution` / `ssrf` /
  `credential-forwarding` / `payload-xss` / `missing-security-headers`), the
  concrete data-flow trace, and a fix sketch matching Nuxt's documented pattern,
- for every `useState`/payload → render-sink finding, an explicit statement of
  whether a sanitizer call is present on the traced path — never approve on the
  assumption one exists elsewhere,
- evidence level per finding (`repo evidence`, `documentation-based`, or
  `inference`), with structural risk findings explicitly labeled as structural
  risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming actual
  cross-request leakage requires concurrent-request load testing, not static
  review").
