# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure, the decision tree across all
five defect classes, and the required output shape. Load the two domain references only
for the specific defect class the code under review actually raises.

## Prerequisites

- Read `package.json` / `nuxt.config.ts` to confirm the Nuxt major (3 vs 4) — API names
  and defaults are stable across 3/4 for everything this skill covers, but confirm
  before citing version-specific behavior.
- Identify the review surface: `nuxt.config.ts` (`runtimeConfig`, `routeRules`,
  `modules`), any `composables/`, `plugins/`, `server/api/`, `server/routes/`,
  `server/middleware/` files, and any template/component using `useState`/`v-html`.

## Workflow

1. **Read `nuxt.config.ts`'s `runtimeConfig` block in full.** Classify every key as
   private (top-level, server-only) or public (nested under `public`, or `app`).
   For every public key, ask whether its name/default value suggests a secret. See
   `references/runtime-config-and-cross-request-state.md`, Part 1.
2. **Grep for `NUXT_PUBLIC_*` and other `NUXT_*` env var names** in `.env*` files and
   deployment config; map each back to the `runtimeConfig` key it overrides and confirm
   the public/private classification holds.
3. **Enumerate module-scope declarations reachable from server-rendered code.** Grep
   `composables/`, `plugins/`, `server/api/`, `server/middleware/` for `useState(`,
   `ref(`, `reactive(`, and mutable object/array literals declared outside any
   function body. Classify each by mutability/reactivity and reachability. See
   `references/runtime-config-and-cross-request-state.md`, Part 2.
4. **Enumerate every server-route outbound `$fetch`/`ofetch`/`event.$fetch`/
   `useRequestFetch` call.** Trace the destination URL to its origin; trace which
   headers are forwarded (via `event.$fetch`'s default context propagation,
   `useRequestHeaders(...)`, or manual header spreading) and to where. See
   `references/ssrf-payload-and-response-headers.md`, Part 1.
5. **Enumerate `useState`/payload values that carry user-controlled or user-echoed
   content**, and trace them forward to any eventual `v-html`/HTML-injection sink.
   Separately check for any custom `definePayloadReducer`/`definePayloadReviver` logic
   that bypasses Nuxt's own `devalue`-based serialization. See
   `references/ssrf-payload-and-response-headers.md`, Part 2.
6. **Check for a security-header mechanism**: `routeRules` `headers` entries, a
   security module in `modules`, or `useResponseHeader(...)` calls, and confirm actual
   route-glob coverage against the app's real surface (auth pages, forms, admin
   routes, embedded third-party content). See
   `references/ssrf-payload-and-response-headers.md`, Part 3.
7. **Produce ranked findings** using the output contract below.

## Decision tree

- A key under `runtimeConfig.public` (or fed by a `NUXT_PUBLIC_*` env var) holds a
  secret, credential, or internal-only value → **HIGH**, `runtimeConfig-exposure`.
- A private `runtimeConfig` key is re-exported into a public-scoped value elsewhere
  in the app → **HIGH**, `runtimeConfig-exposure`.
- A private `runtimeConfig` key stays private and is consumed only server-side → not
  a finding (rubric item 9).
- `runtimeConfig.public.*` holds a genuinely non-secret value (base URL, feature
  flag) → not a finding (rubric item 10).
- `useState`/`ref`/`reactive`/a mutable object is declared at true module scope
  (outside any function) and is reachable from server-rendered code → **HIGH**,
  `cross-request-state-pollution`, structural, regardless of whether leakage has been
  observed.
- Same declaration is invoked inside a composable/`setup()`/plugin factory body → not
  a finding (rubric item 11), unless that body itself closes over a separate
  module-scope mutable reference (still HIGH in that case).
- Module-scope declaration is an immutable constant with no runtime mutation path →
  not a finding (rubric item 12).
- A `server/api`/`server/routes` handler builds its outbound `$fetch`/`ofetch` URL
  (host, path, or query) from user-reachable input with no allowlist → **HIGH**,
  `ssrf`.
- Outbound URL is hardcoded or validated against an explicit host allowlist → not a
  finding (rubric item 13).
- A handler forwards `authorization`/`cookie` (via `event.$fetch` defaults,
  `useRequestHeaders`, or manual spreading) to an external or user-influenceable
  destination with no header allowlist → **HIGH**, `credential-forwarding`.
- `event.$fetch` used to reach an internal Nitro route with no external hop → not a
  finding by itself (rubric item 14); only escalate if sensitive headers are
  forwarded without justification even internally.
- A `useState`/payload value carrying user-controlled or user-echoed content reaches
  a `v-html` (or equivalent unescaped) sink with no sanitizer call on the traced path
  → **HIGH**, `payload-xss`.
- Same value's trace terminates at a safe consumer (text interpolation, a sanitizer
  call on the exact path, or no rendering at all) → not a finding, but state this
  explicitly rather than omitting the traced value from the review.
- No `routeRules` `headers` entry, no security module, and no `useResponseHeader`
  call anywhere cover a route the app actually serves with auth/forms/embeds →
  **MEDIUM-to-HIGH** depending on the app's surface, `missing-security-headers`.
- A header mechanism exists and its glob coverage includes the routes in scope → not
  a finding (rubric item 15) — but call out any gap in coverage for routes it does
  *not* cover.

## Output contract

Every response from this skill must return:

1. **Scope** — the `runtimeConfig`/`routeRules` blocks, module-scope declarations,
   server routes, and/or template bindings reviewed.
2. **Ranked findings** — each with file:line, defect category
   (`runtimeconfig-exposure` / `cross-request-state-pollution` / `ssrf` /
   `credential-forwarding` / `payload-xss` / `missing-security-headers`), the
   concrete data-flow trace (declaration and reachability, or origin-to-sink path),
   and a fix sketch matching the patterns in the relevant reference.
3. **Evidence level per finding** — `documentation-based` (Context7-confirmed against
   `/websites/nuxt_4_x` or `/websites/nuxt_3_x`), `repo evidence`, or `inference`
   (e.g., third-party module defaults Nuxt's own docs do not enumerate). Label
   structural risk findings as structural risk, not as confirmed-exploited.
4. **Verdict** — approve / approve-with-notes / block.
5. **Open questions or out-of-scope items** — e.g., "confirming actual cross-request
   leakage requires concurrent-request load testing, not static review," or "the
   `nuxt-security` module's exact default header set is not documented in Nuxt's own
   Context7 sources — confirm its configured `security:` block directly rather than
   assuming defaults."

## When to push back

Push back if the user asks to:

- approve a `runtimeConfig.public` secret because "it's minified/obfuscated in the
  bundle anyway" — minification is not encryption; the value is trivially readable in
  the browser's network/JS panel,
- clear a module-scope `useState`/`ref` because "we haven't seen cross-user leakage in
  production" — this defect class is structural and often invisible until concurrent
  load exposes it; absence of a reported incident is not evidence of absence,
- approve an SSRF-shaped `$fetch` call because "we trust our users" — an allowlist on
  the destination host is the control, not the trust level of the current user base,
- treat "we sanitize elsewhere" as clearing a specific traced `payload`/`useState` →
  `v-html` path — the sanitizer must be visible on the exact path under review,
- skip the security-headers check because "we'll add nuxt-security later" — report the
  current gap now; a planned future fix is not a mitigating control today.
