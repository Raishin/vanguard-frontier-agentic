# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `react` | React, hooks, useState, useEffect, JSX, component architecture, rendering performance, effects correctness, component library |
| `nextjs` | Next.js, App Router, Server Component, Client Component, `use cache`, `fetch` cache config, Route Handler, `revalidate`, ISR |
| `vue` | Vue, Composition API, `<script setup>`, SFC, Vue SSR, script/style injection |
| `angular` | Angular, Signals, change detection, zoneless, Angular SSR/hydration |
| `svelte` | Svelte, SvelteKit, load function, `use:enhance`, form actions, progressive enhancement |
| `ssr-hydration` | hydration mismatch, streaming, Suspense boundary, TTFB, LCP from server render, `suppressHydrationWarning`, `error.js`, `global-error.js` |
| `state` | state management, store shape, normalization, re-render, stale data, client/server state boundary |
| `routing` | route tree, loaders, actions, code-splitting boundary, navigation guard, deep link |
| `css-design-system` | CSS, cascade layers, custom properties, design tokens, specificity, container queries, responsive |
| `design-tokens-governance` | Style Dictionary, Tokens Studio, token source of truth, theming, dark mode, contrast guarantee |
| `visual-regression` | pixel diff, screenshot test, Chromatic, Storybook test-runner, DOM snapshot |
| `testing` | unit test, component test, integration test, E2E, test pyramid, flaky suite |
| `performance-cwv` | Core Web Vitals, LCP, INP, CLS, lab data, field data, performance budget |
| `build-tooling` | Vite, Webpack, Rollup, code splitting, bundle size budget, vendor chunk, duplicate dependency |
| `package-governance` | package.json, lockfile, pnpm catalog, npm override, Renovate, Dependabot, dependency confusion |
| `monorepo-dx` | Turborepo, Nx, workspace topology, remote caching, task graph, stale cache |
| `browser-compat` | Baseline, caniuse, polyfill, graceful degradation, unsupported browser feature |
| `accessibility` | WCAG, ARIA, screen reader, keyboard trap, focus order, alt text, a11y audit |
| `html-semantics` | landmark, heading hierarchy, native element, WHATWG HTML, structured data, semantic markup |
| `i18n-l10n` | ICU MessageFormat, CLDR, plural rule, RTL, locale, translation readiness |
| `security` | XSS, CSP, Trusted Types, DOM sink, client-side supply chain, OWASP |
| `api-bff` | BFF, trust boundary, over-fetching, backend contract, authorization at the edge |
| `observability-rum` | RUM, OpenTelemetry Web, error tracking, sampling, cardinality, PII in telemetry |
| `analytics-experimentation` | A/B test, experimentation, event schema, statistical validity, conversion tracking |
| `pwa-offline` | service worker, web app manifest, offline fallback, installability |
| `typescript` | tsconfig strictness, type contract, narrowing, `any`-laundering, public API surface |
| `migration` | legacy jQuery/AngularJS/Backbone migration, monolith-to-microfrontend, framework major-version upgrade, strangler-fig |
| `finops-cost` | CDN egress, edge/SSR compute cost, image transform cost, build minutes, cost-to-serve |
| `platform-architecture` | module boundary, build/runtime topology, shared-platform contract, technology-adoption gate, cross-team drift |
| `platform-foundation` | new-project scaffolding, framework-vs-platform tradeoff, browser-support baseline, cross-cutting HTML/CSS/JS/TS decision that doesn't fit one specialist |
| `ai-generated-review` | AI-generated code, LLM-generated component, hallucinated API, generated hooks/glue code |
| `red-team` | adversarial review, elevated review bar, red-team pass, second-opinion security/a11y check |

## Full routing table

### Frameworks

| Agent | Domain(s) | Use when… |
|---|---|---|
| `react-specialist-agent` | react | Reviewing React component architecture, hooks/effects correctness, or rendering-performance risk |
| `nextjs-specialist-agent` | nextjs, ssr-hydration | Reviewing Next.js App Router rendering strategy, fetch/cache configuration, or Server/Client Component boundary correctness |
| `vue-specialist-agent` | vue | Reviewing Vue 3 Composition API architecture or Vue SSR security posture |
| `angular-specialist-agent` | angular, ssr-hydration | Reviewing Angular Signals architecture, change-detection strategy, or SSR/hydration correctness |
| `svelte-sveltekit-specialist-agent` | svelte | Reviewing SvelteKit routing/load-function correctness or progressive-enhancement resilience |

### Rendering

| Agent | Domain(s) | Use when… |
|---|---|---|
| `ssr-hydration-streaming-agent` | ssr-hydration | Diagnosing hydration-mismatch errors, slow-data waterfalls, or incorrect Suspense/error-boundary placement |

### State and routing

| Agent | Domain(s) | Use when… |
|---|---|---|
| `state-management-data-flow-agent` | state | Reviewing client/server state boundaries, store shape, normalization, or re-render performance |
| `routing-navigation-agent` | routing | Reviewing route-tree structure, data-loading strategy, code-splitting boundaries, or navigation-guard logic |

### Styling and design systems

| Agent | Domain(s) | Use when… |
|---|---|---|
| `css-architecture-agent` | css-design-system | Reviewing CSS specificity, cascade-layer strategy, or custom-property/design-token architecture |
| `design-systems-governance-agent` | design-tokens-governance | Reviewing design-token pipelines and component-library governance |
| `visual-regression-agent` | visual-regression | Reviewing pixel-diff or DOM-snapshot visual regression pipelines |

### Testing and quality

| Agent | Domain(s) | Use when… |
|---|---|---|
| `testing-quality-engineering-agent` | testing | Reviewing or designing frontend test strategy across unit, component, integration, and E2E layers |

### Performance and build

| Agent | Domain(s) | Use when… |
|---|---|---|
| `web-performance-core-vitals-agent` | performance-cwv | Triaging Core Web Vitals (LCP, INP, CLS) using lab and field evidence |
| `build-tooling-bundling-agent` | build-tooling | Reviewing Vite/Webpack/Rollup build configuration, code-splitting strategy, or bundle budgets |
| `package-governance-agent` | package-governance | Reviewing package.json manifests, lockfiles, or dependency version policy |
| `monorepo-dx-agent` | monorepo-dx | Reviewing monorepo task-graph orchestration or remote-caching correctness |

### Compatibility and semantics

| Agent | Domain(s) | Use when… |
|---|---|---|
| `browser-compatibility-agent` | browser-compat | Checking used web-platform features against the org's supported-browser matrix |
| `accessibility-wcag-agent` | accessibility | Auditing markup and components against WCAG 2.2 A/AA success criteria |
| `html-semantics-agent` | html-semantics | Reviewing markup structure, landmark/heading hierarchy, or ARIA application |
| `internationalization-localization-agent` | i18n-l10n | Verifying i18n architecture and l10n readiness |

### Security and boundaries

| Agent | Domain(s) | Use when… |
|---|---|---|
| `frontend-security-agent` | security | Hunting DOM XSS sinks, CSP/Trusted Types gaps, or client-side supply-chain risk |
| `api-integration-bff-agent` | api-bff | Reviewing the contract, ownership, and trust boundary between frontend clients and backend/BFF layers |

### Observability, analytics, and offline

| Agent | Domain(s) | Use when… |
|---|---|---|
| `frontend-observability-rum-agent` | observability-rum | Reviewing or designing RUM instrumentation (Core Web Vitals, OTel Web traces, error tracking) |
| `product-analytics-experimentation-agent` | analytics-experimentation | Reviewing frontend analytics instrumentation and A/B experimentation setups |
| `pwa-offline-capability-agent` | pwa-offline | Validating service-worker caching behavior, manifest installability, or offline-fallback coverage |

### Contracts, migration, and cost

| Agent | Domain(s) | Use when… |
|---|---|---|
| `typescript-contracts-agent` | typescript | Reviewing tsconfig strictness posture, exported type contracts, or narrowing correctness |
| `frontend-migration-modernization-agent` | migration | Planning or de-risking a large-scale frontend migration or framework major-version upgrade |
| `frontend-finops-cost-to-serve-agent` | finops-cost | Quantifying CDN egress, edge/SSR compute, image-transform, or build-minute cost impact |

### Cross-cutting architecture

| Agent | Domain(s) | Use when… |
|---|---|---|
| `frontend-platform-architect-agent` | platform-architecture | Deciding cross-cutting module boundaries, build/runtime topology, or technology-adoption gates |
| `web-platform-foundation-agent` | platform-foundation | Handling a cross-cutting HTML/CSS/JS/TS decision (new-project scaffolding, framework-vs-platform tradeoff, browser-support baseline) that doesn't fit one narrow specialist |
| `javascript-runtime-agent` | ssr-hydration, platform-foundation | Reviewing event-loop/microtask ordering, Promise composition, or DOM event-handling lifecycle for race conditions or listener leaks |

### AI-generated code and red-team

| Agent | Domain(s) | Use when… |
|---|---|---|
| `ai-assisted-frontend-review-agent` | ai-generated-review | Applying an elevated review bar to AI/LLM-generated frontend code |
| `enterprise-red-team-review-agent` | red-team | Running a mandatory or spot-check adversarial second-opinion pass (security review, AI-generated code review, or production incident workflows, per `frontend-board-chair`'s workflow table) |

### Live-guard (none currently cataloged)

No agent in the frontend catalog is currently capable of a live/production mutation (deploy, feature-flag flip in prod, cache purge, rollback trigger). If a task carries a live-guard signal, say so explicitly and stop — do not invent a live-guard agent, and do not route the task to a static-review specialist as a substitute for a production-mutation gate. Re-check `catalog/agents.json` for a `frontend` provider agent whose ID or summary indicates live/production-mutation capability before asserting this is still true.

## Live-guard gate protocol

If a future frontend catalog addition introduces a live-mutation-capable specialist, before routing to it, surface all three and wait for explicit written confirmation:

1. **Blast-radius assessment** — what environments, users, or revenue-generating surfaces are affected if this goes wrong?
2. **Rollback path** — what is the tested rollback procedure and estimated recovery time?
3. **Explicit confirmation** — "I confirm I understand the blast radius and rollback path. Proceed."

## Response shape

Every Maestro response begins with the routing header:
```
Route: <agent-name(s)>
Reason: <one sentence>
Mode: <single | parallel (N specialists) | live-guard-gate | unclassified>
```
Followed by: dispatched specialist output (summarized), then a handoff note to `frontend-board-chair-agent`.
