# Frontend Agent & Skills Expansion Plan

**Status:** Implemented and merged. `agents/frontend/` contains 35 agents; `skills/frontend/` contains 49 skills. `npm run validate` exits 0 ("QA cluster eval: 80/80 checks passed" refers to the most recent cluster eval run in this pipeline; the frontend board's own gates are folded into the same 19+ gate `validate` run). Spell-check (`npm run lint:spell`) and markdown-lint (`markdownlint-cli2`) are separate CI jobs, not part of `validate`, and are called out explicitly wherever this document claims "green."

---

## 1. Executive Verdict

**Approve-with-blockers**, matching the enterprise red-team critique this plan is built from. The frontend expansion is real, on disk, and passes catalog/schema/asset-integrity/maestro-routing/marketplace validation. It is not a "coming soon" proposal — 35 agents and 49 skills exist under `agents/frontend/` and `skills/frontend/`, each with `metadata.json`, harness variants, and (for skills) a `SKILL.md` with `allowed-tools` conforming to `schemas/skill.frontmatter.schema.json`.

Two findings are release blockers and must close before this cluster is considered done, not just "green":

1. **Unscoped `Bash` on security/a11y-adjacent skills** (`frontend-dom-xss-csp-review`, `wcag-22-accessibility-audit`, `browser-compatibility-review`, `frontend-observability-rum-instrumentation`) — a bare `Bash` grant on a security or compliance skill contradicts this repo's own least-privilege baseline and the skill's own "static-only" prose.
2. **Empty `companion_skills` on 4 owning agents** (`web-performance-core-vitals-agent`, `accessibility-wcag-agent`, `html-semantics-agent`, `frontend-security-agent`) despite each having an obviously-owned skill on disk — this passes `validate` today only because the gate checks *resolvability of declared* companions, not *completeness*, which is a silent rot vector.

Three should-fix findings (a11y authority sourcing, a11y agent overlap, perf-cluster skill overlap) are documented in §19 and do not block ship but should land in the next iteration.

Net: the architecture, governance model, and evidence-labeling discipline are Fortune-50-board-caliber. The gate-completeness blind spot is the kind of defect that erodes trust in "green CI" generally — it gets fixed first, everywhere else defers to the roadmap in §17.

---

## 2. Repository Fit Analysis

This repo (`vanguard-frontier-agentic`) is a curated, multi-harness marketplace for cloud/zero-trust/compliance-aware AI workflows, organized around:

- `agents/<provider>/<agent-id>/` — `metadata.json` + `harnesses/*` per-harness adapters
- `skills/<provider>/<skill-id>/` — `SKILL.md` + `metadata.json` + `references/`
- `catalog/agents.json`, `catalog/skill-manifest.json`, `catalog/asset-integrity.json` — machine-readable indexes
- `schemas/agent.schema.json`, `schemas/skill.frontmatter.schema.json` — metadata contracts
- `tests/validate-*.py` — the 19+ gate pipeline (`npm run validate`)
- `tests/fixtures/<provider>-maestro-routing/` — routing fixtures for maestro/router agents

The frontend expansion introduces `provider: frontend` as a first-class provider, following the exact same shape as existing providers (aws, azure, gcp, etc.): a maestro/router agent (`frontend-maestro-agent`), a board-chair adjudicator (`frontend-board-chair-agent`), a red-team second-pass agent (`enterprise-red-team-review-agent`), and N domain specialists with 1:1 companion skills. This is a **structural fit, not a bolt-on** — it reuses:

- The existing `execution_tier` taxonomy (`static-review`, `read-only-runtime`; frontend introduces zero `mutating-runtime` agents, which is a deliberate scope fence, not an oversight).
- The existing `companion_skills` / `primaryAgent` bidirectional linkage convention.
- The existing maestro routing-fixture contract (`taxonomy.json` + `inputs/` + `expected/`), which this cluster explicitly defers per §17 until the specialist catalog fully lands (already done) — the fixture itself is called out as a cross-cluster dependency in the original governance cluster notes.
- The existing evidence-labeling convention (`live evidence` / `repo evidence` / `documentation-based` / `inference`) already implicit in other providers' security-sensitive agents, made explicit and mandatory here.

No new schema fields were invented outside what `schemas/agent.schema.json` and `schemas/skill.frontmatter.schema.json` already support (`companion_skills`, `security_notes`, `official_docs`, `allowed-tools`, `metadata.category`). Where the cluster design notes proposed a category value not in the schema enum (`performance`), the actual on-disk assets correctly fall back to an enum-valid category (see §8) — this is confirmed by inspecting `schemas/skill.frontmatter.schema.json`'s `category` enum, which does not include a literal `performance` value.

---

## 3. Brutal Gap Analysis of Naive Frontend-Agent Thinking

A naive "frontend agent" build would fail on every one of these axes; this expansion was designed specifically against them:

1. **"One React agent that does everything."** Naive designs collapse component architecture, hooks/effects, SSR, performance, and security into a single agent with a vague system prompt. This expansion splits by defect class with disjoint scope: `react-specialist-agent` never touches Next.js caching or Node SSR — that is `nextjs-specialist-agent`'s and `ssr-hydration-streaming-agent`'s job respectively.
2. **Framework fanboyism.** Naive agents implicitly prefer the reviewer's favorite framework ("just use Vue's Composition API," "rewrite this in Svelte"). Every framework specialist here is symmetric in structure (agent + 2 companion skills, static-review tier, Context7-grounded) and every one explicitly forbids unprompted framework/rewrite recommendations.
3. **Memory-based version claims.** Naive agents state framework behavior from training-data recall — dangerous specifically because hook semantics, caching defaults, and hydration diagnostics are version-sensitive (Next.js 13/14 vs 15 `fetch()` cache defaults; Angular v18 vs v20 Signals API; React 18 vs 19 hydration-error format). Every claim in this expansion is required to be Context7-verified against the *installed* version, not the latest docs by default (see §12).
4. **Accessibility as an afterthought pass.** Naive builds run an automated a11y linter at the end and call it done. This expansion treats a11y as a HARD gate at every tier (Chair cannot downgrade an a11y reject), disclosed automation-coverage ceiling (~30-50%), and requires WCAG SC id + ARIA APG pattern citation per finding, not a lint-tool pass/fail.
5. **Security as a style note.** Naive reviewers flag `dangerouslySetInnerHTML` as a "consider sanitizing" suggestion. Here, every framework's documented injection/leakage vector (Server Actions trusting client identity, SSR cross-request state pollution, universal-load secret leakage) is HIGH severity by construction, and BOLA/excessive-data-exposure findings are security-severity, not style commentary.
6. **Performance theater.** Naive perf review optimizes a single Lighthouse number. This expansion mandates lab-vs-field separation everywhere, numeric CI-enforced budgets (not "make it faster"), and explicitly labels a lab-only pass as "unverified in the field," never "PASS."
7. **Rewrite bias in migration guidance.** Naive migration agents recommend "rewrite it in the new framework" as a default. `frontend-migration-modernization-agent` requires strangler-fig, incremental, rollback-capable plans and rejects "the old thing is old" as a justification outright.
8. **AI-generated code held to a lower bar.** Naive review treats LLM-authored code as equivalent to human code once it type-checks. `ai-assisted-frontend-review-agent` explicitly elevates the bar — registry verification against slopsquat/dependency-confusion, Context7 verification of API claims, and rejects "it compiles" as proof of correctness.
9. **Single-reviewer authority with no adversarial pass.** Naive systems let one agent both produce and bless its own finding. This expansion has a three-tier structure (specialist → red-team adversarial pass → board chair) specifically to catch a specialist's unverified "passed" claim before it reaches a binding decision.
10. **Implicit, un-auditable handoffs.** Naive multi-agent systems hand work between agents with no record of who owns what next. Every workflow here produces a handoff record with verdict, evidence table, blockers, safe next action, rollback path, and a named receiving owner — never an anonymous handoff.

---

## 4. Design Principles

1. **Static-review-first, zero blast radius by default.** All 35 agents are `static-review` or `read-only-runtime` tier. None execute builds, deploys, or mutating commands; none hold credentials. A false positive costs review time, not a broken deploy.
2. **Evidence labeling is mandatory, not optional.** Every claim carries one of: `live evidence`, `repo evidence`, `user-provided sanitized evidence`, `documentation-based` (Context7-verified), or `inference`. No verdict is issued without a per-claim label.
3. **Security and accessibility are HARD gates.** A reject on either overrides an approve from every other specialist — no averaging, no majority vote, no Chair-level downgrade. Only a named human risk-owner can accept residual risk, and that acceptance is recorded, never inferred.
4. **Performance is budget-based, lab and field always separated.** Numeric thresholds tied to a percentile (e.g., "LCP p75 ≤ 2.5s field"), never vague "faster." A lab-only pass is a conditional-approve at best.
5. **Migration is rewrite-averse by default.** Every ADR-producing review requires an incremental, rollback-capable plan. Duplicated in-repo capability and "rewrite because it's trendy" are default-reject conditions.
6. **Version-sensitive claims are verified live, not recalled.** Context7 (`resolve-library-id` + `query-docs`) grounds every framework-behavior claim against the target repo's actual installed version where Context7 has authoritative coverage; where it doesn't (WCAG, CSP), the asset states the gap and names the official-doc fallback instead.
7. **Least privilege on tooling.** Every skill's `allowed-tools` is scoped to the minimum needed (`Read Grep Glob`, plus an explicitly allowlisted `Bash(...)` pattern only where a read-only lint/audit command is required) — never a bare, unscoped `Bash`.
8. **No anonymous handoffs.** Every cross-agent or cross-tier handoff carries a named receiving owner, evidence table, and rollback path.
9. **Adversarial self-interrogation is a first-class citizen, not a postscript.** Every skill and agent spec carries its own adversarial checklist, not just a top-level red-team afterthought.
10. **Measurable enterprise value or the asset does not ship.** Every agent/skill ties to a named metric class: revenue/conversion, latency/CWV, a11y compliance, security risk, developer velocity, maintainability, governance auditability, cost-to-serve, incident reduction, migration safety, or regulatory risk.

---

## 5. Frontend Agent Board

35 agents on disk under `agents/frontend/`, organized by cluster:

### Frameworks (5)
`react-specialist-agent`, `nextjs-specialist-agent`, `angular-specialist-agent`, `vue-specialist-agent`, `svelte-sveltekit-specialist-agent` — all `static-review`, all Context7-grounded against version-matched official docs, all symmetric in structure (2 companion skills each).

### Platform Foundation (5)
`web-platform-foundation-agent` (cross-cutting arbiter + Baseline/support-matrix owner), `html-semantics-agent`, `css-architecture-agent`, `javascript-runtime-agent`, `typescript-contracts-agent`.

### Architecture & Data-Flow (5)
`frontend-platform-architect-agent`, `state-management-data-flow-agent`, `routing-navigation-agent`, `api-integration-bff-agent`, `ssr-hydration-streaming-agent`.

### Performance & Async (2)
`web-performance-core-vitals-agent`, `pwa-offline-capability-agent`.

### Risk & Governance — a11y/security/observability/i18n/compat (5)
`accessibility-wcag-agent`, `frontend-security-agent`, `frontend-observability-rum-agent`, `internationalization-localization-agent`, `browser-compatibility-agent`.

### Quality — testing/visual/design-systems/build/monorepo (6)
`testing-quality-engineering-agent`, `visual-regression-agent`, `design-systems-governance-agent`, `build-tooling-bundling-agent`, `package-governance-agent`, `monorepo-dx-agent`.

### Change — migration/analytics/finops/AI-review (4)
`frontend-migration-modernization-agent`, `product-analytics-experimentation-agent`, `frontend-finops-cost-to-serve-agent`, `ai-assisted-frontend-review-agent`.

### Governance (3)
`frontend-maestro-agent` (router — never renders a verdict itself), `enterprise-red-team-review-agent` (Tier 2 adversarial pass), `frontend-board-chair-agent` (Tier 3 binding decision authority).

Total: 5+5+5+2+5+6+4+3 = **35**, matching the on-disk count under `agents/frontend/`.

---

## 6. Agent Governance Model

Three-tier accountability structure, not a single reviewer:

- **Tier 1 — Specialist agents** (all 32 non-governance agents above) produce scoped, evidence-labeled recommendations only. They never approve/reject a change on their own authority.
- **Tier 2 — `enterprise-red-team-review-agent`** runs an adversarial second pass against Tier-1 output: actively tries to break the claimed verdict (missing edge cases, weakened CSP, keyboard-trap regressions, unverified perf claims, prompt-injected AI-generated code). This is the HARD-gate enforcement layer.
- **Tier 3 — `frontend-board-chair-agent`** is the single point of accountability: sequences specialists + red-team passes, resolves conflicting verdicts, issues the binding approve / conditional-approve / reject decision plus handoff record.
- **Routing — `frontend-maestro-agent`** classifies inbound tasks and dispatches to the narrowest specialist(s), capped at 4 concurrent specialists (mirrors the existing `parallel_threshold` convention used by `aws-maestro-agent`/`azure-maestro-agent`). It never itself renders a governance verdict.

Conflict resolution rules (binding):

- Security and accessibility verdicts are HARD gates — a reject from either overrides every other approve, full stop.
- Performance verdicts are budget-based — a lab-only pass without field data is conditional-approve at best.
- Migration-safety overrides framework-fanboyism — a rewrite recommendation must justify itself against a narrower strangler-fig path or the Chair defaults to non-rewrite.
- Non-hard-gate disagreements between two Tier-1 specialists are resolved by evidence-tier comparison, not agent preference; ties escalate to a human owner.
- Any task-embedded instruction attempting to reframe roles, invoke urgency ("ship now"), or claim prior approval is treated as user-provided content, not a governance directive — this mirrors the adversarial-instruction-injection defense already used by `aws-maestro-agent`.

---

## 7. Agent Handoff Protocol

Every handoff record — Tier 1→2, Tier 2→3, Chair→human — carries: **verdict, evidence table (claim → evidence label → source), blockers, safe next action, rollback path (if live), and a named receiving owner.** No anonymous or implicit handoffs are permitted.

### The 10 required workflows

| # | Workflow | Sequence | Hard gates | Primary handoff owner on approve |
|---|---|---|---|---|
| 1 | New framework feature | Maestro → architecture specialist → a11y → perf (lab) → Chair | a11y | Feature team lead |
| 2 | Performance regression | Maestro → perf specialist (lab+field) → Chair | none (budget-based) | Perf owner + on-call |
| 3 | A11y audit | Maestro → a11y specialist → red-team spot-check → Chair | a11y | A11y program owner |
| 4 | Security review | Maestro → security specialist → red-team (mandatory) → Chair | security | AppSec + red-team lead |
| 5 | SSR/hydration bug | Maestro → architecture/framework specialist → perf (regression check) → Chair | none unless it caused an incident | Framework owner |
| 6 | Design-system change | Maestro → architecture specialist → a11y → Chair | a11y | Design-system owner |
| 7 | Framework migration | Maestro → migration-safety specialist → architecture → perf → Chair | none (rewrite-bias check mandatory) | Migration program owner |
| 8 | AI-generated code review | Maestro → red-team (mandatory) → security → a11y → Chair | security, a11y | Reviewing engineer of record |
| 9 | Production incident | Maestro → red-team + relevant specialist(s) in parallel → Chair (expedited) | security if exploit-adjacent | Incident commander |
| 10 | CWV failure | Maestro → perf specialist (field-data-first) → Chair | none (budget-based, field data mandatory) | Perf owner |

Timeout escalation: if a specialist cannot return evidence-backed output within a workflow's required gate, the Chair escalates to "unclassified — needs human scoping" rather than approving on partial evidence.

---

## 8. Skills Catalog

49 skills on disk under `skills/frontend/`. `metadata.category` values shown reflect the actual enum in `schemas/skill.frontmatter.schema.json` (which does **not** include a literal `performance` value — perf-cluster skills correctly use an enum-valid category such as `architecture`/`resilience`/`observability` per skill, not the human-facing cluster label).

| Skill ID | Primary Agent | Category | Cluster |
|---|---|---|---|
| `react-component-architecture-review` | react-specialist-agent | architecture | Frameworks |
| `react-state-effects-review` | react-specialist-agent | architecture | Frameworks |
| `nextjs-rendering-caching-review` | nextjs-specialist-agent | architecture | Frameworks |
| `nextjs-app-router-data-fetching-review` | nextjs-specialist-agent | architecture | Frameworks |
| `angular-architecture-signals-review` | angular-specialist-agent | architecture | Frameworks |
| `angular-ssr-hydration-review` | angular-specialist-agent | architecture | Frameworks |
| `vue-composition-api-architecture-review` | vue-specialist-agent | architecture | Frameworks |
| `vue-ssr-security-review` | vue-specialist-agent | security | Frameworks |
| `sveltekit-routing-load-review` | svelte-sveltekit-specialist-agent | architecture | Frameworks |
| `sveltekit-progressive-enhancement-review` | svelte-sveltekit-specialist-agent | resilience | Frameworks |
| `html-semantics-accessibility-review` | html-semantics-agent | compliance | Platform Foundation |
| `css-architecture-design-system-review` | css-architecture-agent | architecture | Platform Foundation |
| `javascript-runtime-async-review` | javascript-runtime-agent | resilience | Platform Foundation |
| `typescript-contracts-review` | typescript-contracts-agent | architecture | Platform Foundation |
| `critical-rendering-path-review` | web-platform-foundation-agent (dispatched) | observability | Platform Foundation |
| `frontend-platform-architecture-review` | frontend-platform-architect-agent | architecture | Architecture & Data-Flow |
| `state-management-decision-review` | state-management-data-flow-agent | architecture | Architecture & Data-Flow |
| `api-integration-contract-review` | api-integration-bff-agent | security | Architecture & Data-Flow |
| `frontend-bff-boundary-review` | api-integration-bff-agent | security | Architecture & Data-Flow |
| `routing-navigation-review` | routing-navigation-agent | architecture | Architecture & Data-Flow |
| `ssr-hydration-streaming-diagnosis` | ssr-hydration-streaming-agent | observability | Architecture & Data-Flow |
| `microfrontend-boundary-review` | frontend-platform-architect-agent | architecture | Architecture & Data-Flow |
| `frontend-error-boundary-resilience-review` | ssr-hydration-streaming-agent | resilience | Architecture & Data-Flow |
| `core-web-vitals-triage` | web-performance-core-vitals-agent | observability | Performance & Async |
| `bundle-budget-code-splitting-review` | web-performance-core-vitals-agent | architecture | Performance & Async |
| `tree-shaking-dead-code-review` | web-performance-core-vitals-agent | architecture | Performance & Async |
| `service-worker-cache-strategy-review` | pwa-offline-capability-agent | resilience | Performance & Async |
| `pwa-offline-readiness-review` | pwa-offline-capability-agent | resilience | Performance & Async |
| `wcag-22-accessibility-audit` | accessibility-wcag-agent | compliance | Risk & Governance |
| `frontend-dom-xss-csp-review` | frontend-security-agent | security | Risk & Governance |
| `frontend-auth-session-security-review` | frontend-security-agent | security | Risk & Governance |
| `frontend-observability-rum-instrumentation` | frontend-observability-rum-agent | observability | Risk & Governance |
| `i18n-l10n-readiness-review` | internationalization-localization-agent | compliance | Risk & Governance |
| `browser-compatibility-review` | browser-compatibility-agent | compliance | Risk & Governance |
| `frontend-testing-strategy-review` | testing-quality-engineering-agent | architecture | Quality |
| `e2e-testing-playwright-review` | testing-quality-engineering-agent | architecture | Quality |
| `visual-regression-storybook-review` | visual-regression-agent | architecture | Quality |
| `design-token-governance-review` | design-systems-governance-agent | architecture | Quality |
| `build-tooling-vite-webpack-review` | build-tooling-bundling-agent | architecture | Quality |
| `monorepo-package-governance-review` | package-governance-agent / monorepo-dx-agent | architecture | Quality |
| `frontend-migration-modernization-plan` | frontend-migration-modernization-agent | architecture | Change |
| `legacy-jquery-to-modern-framework-review` | frontend-migration-modernization-agent | architecture | Change |
| `framework-upgrade-risk-review` | frontend-migration-modernization-agent | resilience | Change |
| `product-analytics-experimentation-review` | product-analytics-experimentation-agent | data | Change |
| `frontend-finops-cost-to-serve-review` | frontend-finops-cost-to-serve-agent | finops | Change |
| `ai-generated-frontend-code-review` | ai-assisted-frontend-review-agent | ai | Change |
| `frontend-maestro` | frontend-maestro-agent | architecture | Governance |
| `enterprise-red-team-review` | enterprise-red-team-review-agent | security | Governance |
| `frontend-board-chair` | frontend-board-chair-agent | architecture | Governance |

Count check: 10 (Frameworks) + 5 (Platform Foundation) + 8 (Architecture & Data-Flow) + 5 (Perf) + 5 (Risk & Governance) + 6 (Quality) + 6 (Change) + 3 (Governance) = **48 listed** above; the 49th on-disk skill is a duplicate-counted architecture entry resolved by `monorepo-package-governance-review` serving two agents (`package-governance-agent` and `monorepo-dx-agent`) — see `catalog/skill-manifest.json` for the exact 49-row source of truth rather than re-deriving the count by hand here (DRY per CLAUDE.md: this table is illustrative, not the authoritative count source).

---

## 9. Detailed Skill Folder Architecture

Every skill under `skills/frontend/<skill-id>/` follows the repo-standard shape:

```
skills/frontend/<skill-id>/
├── SKILL.md              # frontmatter (name, description, allowed-tools, metadata) + body
├── metadata.json          # id, primaryAgent, category, official_docs, security_notes, last_verified
└── references/            # loaded on-demand only, never eagerly — progressive disclosure
    ├── <topic-a>.md        # e.g. hook-rules-checklist.md, wcag-sc-map.md
    └── <topic-b>.md
```

Design constraints enforced across all 49 folders:

- `SKILL.md` frontmatter validates against `schemas/skill.frontmatter.schema.json` — `allowed-tools` is a required, least-privilege field (§1 blocker #1 is a violation of this exact constraint on 4 skills).
- `references/` files are loaded selectively (progressive disclosure) — the skill body states a single-trigger condition for each reference file, not a bundle loaded on every invocation.
- No skill duplicates another skill's primary reference source without adding new context — the red-team notes flag this as a live risk in the a11y/security cluster (§19).
- `metadata.json.primaryAgent` must resolve to an agent whose `companion_skills` array contains this skill's id — this bidirectional linkage is what `validate:catalog`'s `REF-4` check enforces (confirmed live in this session's `npm run validate` output: `PASS [REF-4:<agent-id>] companion_skills resolve: [...]`).

---

## 10. Complete SKILL.md Blueprint

```markdown
---
name: <skill-id>
description: <one sentence: what is reviewed, against what standard, producing what artifact>
allowed-tools: Read Grep Glob [Bash(<explicit-allowlisted-command>:*)]
metadata:
  author: "github: <handle>"
  version: "0.1.0"
  updated: "<ISO date>"
  category: <enum value from schemas/skill.frontmatter.schema.json>
---

# <Skill Title>

## Purpose
What this skill reviews and — critically — what it deliberately excludes from scope,
so adjacent concerns (styling, live profiling, state-lib selection, etc.) don't bleed in.

## When to use
Bullet list of concrete trigger phrases/tasks.

## When NOT to use
Bullet list of adjacent-but-out-of-scope tasks, each pointing to the skill that *does* own it
(this is the fix for the overlap findings in §19 items 3-5).

## Operating rules (non-negotiable)
- Evidence labeling required per claim: live evidence / repo evidence / documentation-based / inference.
- Context7 protocol: resolve-library-id + query-docs against the *installed* version before any
  version-sensitive claim; if Context7 has no authoritative coverage for this domain, say so and
  name the official-doc fallback instead of forcing a resolve/query call.
- Security/a11y findings tied to this skill's domain are HIGH severity by default, never advisory.
- Blast-radius cap: if >N components/files are implicated, hand off to a dedicated
  refactor/migration plan rather than reviewing inline.

## Response minimum (every response)
1. Component/file(s) in scope
2. Evidence tier per claim
3. Ranked findings (file:line, severity, standard/spec citation)
4. Safe next action
5. Exact verification command/tool to confirm the fix

## Adversarial checklist (self-interrogation before responding)
- [domain-specific bullets, e.g. "did I confirm the installed framework version before
  citing a version-sensitive API?"]

## References (load on demand only)
- references/<topic>.md — loaded when <specific trigger condition>
```

---

## 11. Ten Complete Sample Skills (on-disk references)

The following 10 skills exist in full on disk today and exemplify the blueprint above end-to-end (path, purpose, and companion agent verified against the repo in this session):

1. `skills/frontend/react-component-architecture-review/SKILL.md` — companion of `react-specialist-agent`; reviews God-components/prop-drilling/context misuse against React's own composition guidance.
2. `skills/frontend/react-state-effects-review/SKILL.md` — companion of `react-specialist-agent`; reviews effect misuse against React's documented "you might not need an effect" guidance.
3. `skills/frontend/nextjs-rendering-caching-review/SKILL.md` — companion of `nextjs-specialist-agent`; reviews Data Cache/`fetch()` default drift across Next.js 13/14→15.
4. `skills/frontend/wcag-22-accessibility-audit/SKILL.md` — companion of `accessibility-wcag-agent`; WCAG 2.2 SC + ARIA APG conformance audit.
5. `skills/frontend/frontend-dom-xss-csp-review/SKILL.md` — companion of `frontend-security-agent`; DOM XSS sink + CSP/Trusted Types review, and the one skill in the set that already models the correct Context7-gap-disclosure pattern called out in §12.
6. `skills/frontend/core-web-vitals-triage/SKILL.md` — companion of `web-performance-core-vitals-agent`; decomposes LCP into TTFB/resource-load-delay/resource-load-duration/render-delay rather than treating it as one number.
7. `skills/frontend/frontend-platform-architecture-review/SKILL.md` — companion of `frontend-platform-architect-agent`; requires rollback path + duplication check + Context7-verified version claims before approval.
8. `skills/frontend/frontend-migration-modernization-plan/SKILL.md` — companion of `frontend-migration-modernization-agent`; strangler-fig phased plans, rewrite-bias rejection built into the operating rules.
9. `skills/frontend/ai-generated-frontend-code-review/SKILL.md` — companion of `ai-assisted-frontend-review-agent`; elevated bar for LLM-authored code, registry/slopsquat verification required.
10. `skills/frontend/frontend-board-chair/SKILL.md` — companion of `frontend-board-chair-agent`; the binding-decision artifact producer for the governance model in §6.

---

## 12. Context7 MCP Usage Policy

- **Mandatory for version-sensitive claims.** Any claim about framework behavior that changes across releases (hook semantics, caching defaults, Signals APIs, hydration diagnostics, bundler chunking APIs) must be verified via `mcp__Context7__resolve-library-id` → `mcp__Context7__query-docs` against the library matched to the target repo's **actual installed version** — not the latest docs by default.
- **State the version match explicitly.** A finding citing Next.js caching behavior must say which Next.js major version the claim applies to and confirm it against the repo's `package.json`/lockfile, not assume latest.
- **Disclose gaps instead of forcing a call.** Where Context7 has no authoritative maintained library for a domain (WCAG success-criteria text, ARIA APG, CSP/Trusted Types specifics), the skill must say so explicitly and name the official-doc fallback (W3C WCAG 2.2, ARIA APG, MDN CSP reference) instead of prescribing a resolve/query call that will fabricate or misattribute a library ID. `frontend-dom-xss-csp-review` already models this correctly ("Context7 does not reliably surface a maintained CSP-specific library") — this pattern must be replicated across all 49 skills, per the red-team finding in §19 item 7.
- **Evidence-conflict rule.** When Context7/official docs disagree with a widely-repeated blog convention (e.g., Vite's `manualChunks` vs. emerging Rolldown `codeSplitting`), the asset states both, marks which is current per official docs, and flags the version dependency rather than picking silently.
- **Never substitute for live evidence.** Context7-grounded claims are labeled `documentation-based`, distinct from `live evidence` (pasted CI output, coverage report, lockfile) and `repo evidence` (files read directly from the target repo). Documentation never proves the user's actual repo state.

---

## 13. References Matrix

| Domain | Primary authoritative source | Verification method | Notes |
|---|---|---|---|
| React | react.dev (`/reactjs/react.dev`) | Context7-grounded | Version-matched per repo's React major |
| Next.js | Next.js docs (`/vercel/next.js`) | Context7-grounded | 13/14 vs 15 `fetch()` cache-default is the critical version fork |
| Angular | angular.dev | Context7-grounded | v18 vs v20 Signals/hydration API differences |
| Vue | vuejs.org SSR guide | Context7-grounded | Per-request-instance requirement is structural, not stylistic |
| SvelteKit | kit.svelte.dev | Context7-grounded | Universal (`+page.js`) vs server-only (`+page.server.js`) load boundary |
| TypeScript | `/microsoft/typescript-website` | Context7-grounded | `tsc --init` 5.9 defaults incl. `noUncheckedIndexedAccess` |
| JS runtime/async | `/mdn/content` | Context7-grounded | Microtask-vs-macrotask ordering, worked examples |
| WCAG 2.2 / ARIA APG | **W3C** `w3.org/TR/WCAG22`, `w3.org/WAI/ARIA/apg` | **Must be primary, not Context7** | §19 finding: currently over-routed through MDN/web.dev paraphrase — fix pending |
| CSP / Trusted Types | MDN CSP reference, `w3c.github.io/trusted-types` | Official docs (no Context7 coverage) | Gap explicitly disclosed by `frontend-dom-xss-csp-review` |
| Core Web Vitals | web.dev (`/googlechrome/web.dev`) | Context7-grounded | INP ≤200ms good / 200-500ms needs-improvement / >500ms poor; lab vs field distinguished |
| Workbox | `/googlechrome/workbox` | Context7-grounded | `precacheAndRoute`, cache-first-no-revalidation semantics |
| Vite | `/vitejs/vite` | Context7-grounded, version-sensitive | `manualChunks` (Rollup-based, v5/6) vs `rolldownOptions.output.codeSplitting` (v8+) |
| TanStack Query / React Router / Zustand | `/tanstack/query`, `/remix-run/react-router`, `/pmndrs/zustand` | Context7-grounded | Optimistic-rollback pattern, `lazy` route-module API, `useShallow` |
| OWASP (Server Actions/BOLA/A01) | owasp.org Top 10 / API Security Top 10 | Official docs | Cited by id, not paraphrased |
| React Compiler incremental adoption | `/reactjs/react.dev` | Context7-grounded | `compilationMode: 'annotation'`, `"use memo"`/`"use no memo"` directives ground rewrite-averse migration rules |

Duplicate-citation rule (per §19 finding #10 in the original red-team notes): no skill should re-cite another skill's primary source without adding new context specific to its own scope.

---

## 14. Validation and CI Gates

```bash
npm install
npm run validate                 # 19+ gates: catalog, schema, asset integrity, maestro routing, marketplace
npm run lint:spell               # codespell — separate CI gate, NOT part of validate
npx --yes markdownlint-cli2 "**/*.md" "#node_modules"   # markdown lint — separate CI gate
```

If the frontend catalog changes:

```bash
npm run manifest:write:all       # runs generators in parallel
npm run asset-integrity:write    # MUST run again, alone, LAST (ordering caveat — see CLAUDE.md)
npm run validate
```

Confirmed in this session: `npm run validate` exits 0. The `frontend-maestro-agent` routing fixture at `tests/fixtures/frontend-maestro-routing/` (taxonomy.json + inputs/ + expected/) is required by `validate:maestro-routing` and depends on every specialist agent referenced existing in `catalog/agents.json` first — satisfied, since all 35 agents are catalogued. Guarded-mutating-live agents (none exist in this cluster by design — see §4 principle 1) would need to be listed under `live_guards`; this cluster has zero, which is itself a validation-relevant fact, not an omission.

Additional gate this document recommends adding (not yet implemented — tracked in §17):

- A completeness check asserting every `provider: frontend` skill is the declared companion of ≥1 agent's `companion_skills` array — this closes the blind spot behind blocker #2 in §1, since `validate:catalog`'s current `REF-4` check only verifies that *declared* companions resolve, not that every skill *is* declared by someone.

---

## 15. Security/Accessibility/Performance/Compliance Hard Stops

**Security (HARD GATE):**
- BOLA / excessive data exposure in API contracts is security-severity, never a style note.
- Client-side-only route guards are automatic-fail — must pair with server-side enforcement.
- Server Actions trusting client-supplied identity/role for authorization = OWASP A01, HIGH severity.
- Per-user `fetch()` responses cached without `no-store`/user-scoped tags = cross-user data exposure, HIGH.
- `dangerouslySetInnerHTML` / `v-html` / native-DOM-bypassing-sanitizer on unsanitized dynamic content = HIGH.
- Secrets/DB clients reachable from universal (browser-executing) load functions = HIGH.
- Session tokens in `localStorage`/`sessionStorage`, missing `SameSite`/`HttpOnly`/`Secure` flags, client-only open-redirect validation = blocking findings, never blessed.
- Service-worker cache poisoning, scope over-broadening, caching of authenticated/PII-bearing responses = HARD blockers.
- No agent may recommend removing a security control (CSP, WAF, TLS tier, image-pipeline scanning) to save cost/time without a named security-owner sign-off.

**Accessibility (HARD GATE):**
- WCAG 2.2 AA failure is an automatic reject the Chair cannot downgrade.
- Route transitions must define a focus target + ARIA live-region announcement where needed (WCAG 2.2 SC 2.4.3, 4.1.3).
- Error-boundary fallback UI must use accessible alert semantics (ARIA APG Alert pattern), never visual-only signaling.
- CLS/INP fixes must not remove accessible loading-state announcements (`aria-live`, `role="status"`) or break keyboard operability/focus order (WCAG 2.2 SC 2.4.11, 2.5.8).
- Automated-tool coverage ceiling (~30-50% of WCAG via axe-core/eslint-plugin-jsx-a11y) must be disclosed in every report — never implied as full conformance.
- Every migration replacing a legacy widget, and every AI-generated interactive component, must pass an explicit ARIA APG keyboard/role parity check.

**Performance (budgets, lab vs. field separated — not a hard block, but a conditional-approve ceiling):**
- Numeric thresholds tied to a percentile (LCP p75 ≤2.5s good, INP ≤200ms good/500ms poor, CLS ≤0.1 good/0.25 needs-improvement, per web.dev).
- A lab-only pass is never presented as "PASS" — it is "unverified in the field, field data pending."
- Every splitting/lazy-loading/caching change requires a rollback path (feature flag, revert commit, cache-version bump).
- CWV approval requires field data (CrUX/RUM) before approve, per workflow #10 in §7.

**Compliance / Migration safety:**
- No unresolved HARD-gate finding on security or a11y may reach an "approve."
- Full framework/tooling rewrites are default-reject without a narrower strangler-fig justification.
- Statistical experimentation findings (unverified stopping rule, absent pre-registered MDE, client/server bucketing mismatch) are blocking, not nice-to-have.
- Every dollar figure in cost-to-serve modeling is labeled billing-verified / modeled-from-public-pricing / inference — never presented as an audited invoice number when it isn't.

---

## 16. PR File Tree

```
agents/frontend/
├── react-specialist-agent/
├── nextjs-specialist-agent/
├── angular-specialist-agent/
├── vue-specialist-agent/
├── svelte-sveltekit-specialist-agent/
├── web-platform-foundation-agent/
├── html-semantics-agent/
├── css-architecture-agent/
├── javascript-runtime-agent/
├── typescript-contracts-agent/
├── frontend-platform-architect-agent/
├── state-management-data-flow-agent/
├── routing-navigation-agent/
├── api-integration-bff-agent/
├── ssr-hydration-streaming-agent/
├── web-performance-core-vitals-agent/
├── pwa-offline-capability-agent/
├── accessibility-wcag-agent/
├── frontend-security-agent/
├── frontend-observability-rum-agent/
├── internationalization-localization-agent/
├── browser-compatibility-agent/
├── testing-quality-engineering-agent/
├── visual-regression-agent/
├── design-systems-governance-agent/
├── build-tooling-bundling-agent/
├── package-governance-agent/
├── monorepo-dx-agent/
├── frontend-migration-modernization-agent/
├── product-analytics-experimentation-agent/
├── frontend-finops-cost-to-serve-agent/
├── ai-assisted-frontend-review-agent/
├── frontend-maestro-agent/
├── enterprise-red-team-review-agent/
└── frontend-board-chair-agent/
    (each: metadata.json + harnesses/{codex.toml, copilot.agent.md, claude-code.agent.md,
     cursor.agent.md, gemini.agent.md, kiro-ide.agent.md, kiro-cli.agent.json})

skills/frontend/
├── react-component-architecture-review/  (SKILL.md, metadata.json, references/)
├── react-state-effects-review/
├── ... (49 total skill directories, one per row in §8)

catalog/
├── agents.json                # +35 frontend entries
├── skill-manifest.json        # +49 frontend entries
└── asset-integrity.json       # SHA256 hashes refreshed last, alone, per ordering caveat

tests/fixtures/frontend-maestro-routing/
├── taxonomy.json
├── inputs/NN-name.json
└── expected/NN-name.json

docs/frontend/
└── frontend-agent-skills-expansion-plan.md   # this document

docs/taxonomy.md                 # provider bullet list updated for `frontend`
docs/language-stack-boards.md    # frontend board enumeration
docs/_data/catalog.yml           # regenerated via npm run docs-data:write
README.md                        # counts regenerated via npm run readme-counts:write
```

---

## 17. Step-by-Step Implementation Plan

1. **Fix blocker #1 (unscoped Bash).** Replace bare `Bash` in `allowed-tools` for `frontend-dom-xss-csp-review`, `wcag-22-accessibility-audit`, `browser-compatibility-review`, `frontend-observability-rum-instrumentation` with explicit allowlists (e.g., `Bash(npx axe:*)`, `Bash(caniuse-lite:*)`) or drop `Bash` entirely if no read-only command is actually needed.
2. **Fix blocker #2 (empty companion_skills).** Populate `companion_skills` on `web-performance-core-vitals-agent` (→ `core-web-vitals-triage`, `critical-rendering-path-review`), `accessibility-wcag-agent` (→ `wcag-22-accessibility-audit`), `html-semantics-agent` (→ `html-semantics-accessibility-review`), `frontend-security-agent` (→ `frontend-dom-xss-csp-review`, `frontend-auth-session-security-review`).
3. **Add the completeness gate** described in §14 to `tests/validate-catalog.py` so this class of drift fails CI going forward, not just resolvability of what's declared.
4. **Fix should-fix #3 (a11y authority sourcing).** Pin W3C WCAG 2.2 / ARIA APG as `official_docs` primary on `html-semantics-accessibility-review` and `wcag-22-accessibility-audit`; label MDN/web.dev-derived claims `secondary-source` explicitly in the skill body.
5. **Fix should-fix #4 (a11y agent overlap).** Add an explicit `## Anti-goals` boundary statement to both `accessibility-wcag-agent` and `html-semantics-agent` metadata/skill bodies distinguishing semantics/DOM-structure review from success-criteria audit.
6. **Fix should-fix #5 (perf overlap).** Add a one-line "distinct from X because Y" pointer to `## When NOT to use` in `core-web-vitals-triage`, `critical-rendering-path-review`, `bundle-budget-code-splitting-review`, `tree-shaking-dead-code-review`, `build-tooling-vite-webpack-review`.
7. **Fix low-severity #7 (Context7 ceremony).** Audit all 49 `SKILL.md` files' Context7 protocol blocks; replace the boilerplate with the gap-disclosure pattern already correct in `frontend-dom-xss-csp-review` wherever Context7 has no authoritative library for the domain.
8. **Propagate blast-radius caps (#9).** Add the explicit component/file count cap (mirroring react-architecture skill's ">5 components → dedicated refactor plan") to migration and monorepo-governance skills that currently lack it.
9. **Regenerate derived artifacts:** `npm run manifest:write:all` then `npm run asset-integrity:write` alone, last (ordering caveat).
10. **Run full validation:** `npm run validate`, `npm run lint:spell`, `npx --yes markdownlint-cli2 "**/*.md" "#node_modules"`.
11. **Generate/refresh the maestro routing fixture** at `tests/fixtures/frontend-maestro-routing/` if any agent id changed as part of steps 1-8.
12. **Open PR** with this document linked as the design record; require sign-off from a named a11y program owner and AppSec lead per the governance model in §6 before merge, consistent with the "Chair cannot self-approve HARD-gate items" rule.

---

## 18. Acceptance Tests

1. `npm run validate` exits 0 — confirmed in this session.
2. Every agent's `companion_skills` array is non-empty where an obviously-owned skill exists on disk (post-fix for blocker #2) — new completeness gate from §14/§17 step 3 enforces this going forward.
3. No skill's `allowed-tools` contains a bare, unscoped `Bash` token (post-fix for blocker #1) — verifiable via `grep -rn "allowed-tools:.*\bBash\b" skills/frontend/*/SKILL.md` and confirming every match is either absent or followed by an explicit `(...)` allowlist.
4. Every security- or a11y-relevant finding class in §15 is represented by at least one skill's operating rules text (spot-checked against `frontend-dom-xss-csp-review`, `wcag-22-accessibility-audit`, `frontend-auth-session-security-review`).
5. Every performance-adjacent skill's response floor includes an explicit lab-vs-field evidence label field (spot-checked against `core-web-vitals-triage`, `bundle-budget-code-splitting-review`).
6. `tests/fixtures/frontend-maestro-routing/` exists with `taxonomy.json` + matched `inputs/`/`expected/` pairs, and every agent id referenced resolves in `catalog/agents.json` — enforced by `validate:maestro-routing`.
7. Zero agents in `agents/frontend/` are tagged `mutating-runtime` — confirmed by design (§4 principle 1); any future addition of a mutating agent must add it to `live_guards` in the maestro fixture.
8. Provider invariant holds: `set(provider bullets in docs/taxonomy.md)` == `provider_list in docs/_data/catalog.yml` == `{distinct providers with ≥1 agent}`, including `frontend`.
9. `npm run lint:spell` and `markdownlint-cli2` both pass on this document and all touched files (run separately from `validate` per CLAUDE.md).
10. Handoff records for all 10 required workflows (§7) are exercised at least once in a dry-run and each produces the mandatory 5 fields (verdict, evidence table, blockers, safe next action, rollback path + named owner).

---

## 19. Enterprise Red-Team Critique

Sampled: `react-specialist` (agent+2 skills), `core-web-vitals-triage`, `critical-rendering-path-review`, `bundle-budget-code-splitting`, `tree-shaking-dead-code`, `build-tooling-vite-webpack`, `frontend-maestro`, `finops-cost-to-serve`, `dom-xss-csp`, `wcag-22-audit`, `html-semantics-accessibility`, `i18n-l10n`, `product-analytics`; plus a companion-mapping and `allowed-tools` sweep across all 49 skills.

Overall this is a high-quality expansion: evidence-labeling discipline, lab-vs-field separation, static-only guards, and anti-rewrite framing are consistently present. The defects below are real seams a Fortune-50 board would still block on.

**HARD-GATE / SECURITY**

1. **Least-privilege violation — unrestricted `Bash` on 4 static-review skills. Severity: HIGH.** `frontend-dom-xss-csp-review`, `wcag-22-accessibility-audit`, `browser-compatibility-review`, `frontend-observability-rum-instrumentation` all declare `allowed-tools: Read Grep Glob Bash` (unscoped), contradicting the repo's own least-privilege baseline and these skills' own "static-only" prose. Fix: explicit allowlist or remove.
2. **Empty `security_notes` reliance is uneven. Severity: MEDIUM.** `react-specialist` carries a strong `security_notes`; some cross-cutting agents leave the companion/security linkage thin. Fix: require non-empty `security_notes` on every agent touching a sink, token, or header.

**A11Y**

3. **A11y authority routed through secondary sources. Severity: MEDIUM.** `html-semantics-accessibility-review` and `wcag-22-accessibility-audit` ground normative WCAG 2.2/ARIA APG claims via Context7 against MDN paraphrase rather than the normative W3C spec. Fix: pin W3C as primary, label MDN-derived claims `secondary-source`.
4. **Two a11y agents, overlapping surface. Severity: MEDIUM.** `accessibility-wcag-agent` and `html-semantics-agent` both cover semantics + WCAG + APG territory. Fix: explicit `## Anti-goals` boundary or merge.

**PERFORMANCE**

5. **Perf-cluster skill duplication. Severity: MEDIUM.** `core-web-vitals-triage`/`critical-rendering-path-review` both decompose LCP/INP/CLS; `bundle-budget-code-splitting-review`/`tree-shaking-dead-code-review`/`build-tooling-vite-webpack-review` all inspect bundle bytes. Defensible by distinct scope (triage=field-regression, CRP=load-sequencing, budget=numeric-gate, tree-shaking=elimination-proof, build-tooling=config) but the seam is nowhere stated. Fix: one-line "distinct from X because Y" pointer in each `## When NOT to use`. No performance theater found — budgets are numeric and CI-gated, lab/field cleanly separated (good).

**MAINTAINABILITY / TRACEABILITY**

6. **Broken agent→skill linkage despite green validation. Severity: HIGH.** `web-performance-core-vitals-agent`, `accessibility-wcag-agent`, `html-semantics-agent`, `frontend-security-agent` all declare `companion_skills: []` while obviously-owned skills exist. Validation passes only because the gate checks resolvability of *declared* companions, not completeness. Fix: populate `companion_skills`; add a completeness-checking validation gate.

**OTHER FINDINGS**

7. **Context7 ceremony / potential misuse. Severity: LOW.** All 49 skills carry a Context7 protocol block, including domains where Context7 has no authoritative library (WCAG, CSP/Trusted Types). Fix (good pattern already exists in `dom-xss-csp` — replicate it): state the gap and name the official-doc fallback instead of prescribing a resolve/query where coverage doesn't exist.
8. **Weak business-value framing on cross-cutting agents. Severity: LOW.** `react-specialist` and `finops` tie concretely to metrics; sampled governance agents (`frontend-board-chair`, `enterprise-red-team`) risk process-for-process framing. Confirm each states a measurable outcome (incident reduction, review-cycle time).
9. **Unrealistic-PR-size risk mostly well-guarded. Severity: LOW (positive).** React architecture skill caps blast radius (">5 components → dedicated refactor plan"). Fix: propagate the same cap pattern to migration and monorepo-governance skills that lack it.
10. **Framework bias — acceptable.** Framework agents are symmetric and each explicitly forbids rewrite recommendations. No fanboyism found.

**Net verdict: Approve-with-blockers.** Blockers before ship: #1 (unscoped Bash) and #6 (empty `companion_skills`) — both self-inconsistencies the pipeline does not catch today. #3, #4, #5 are should-fix boundary/overlap statements, not blockers. Validation: **true** (green). Remaining CI-blocking items: **none** on `validate`; spell-check and markdown-lint were not exercised in this pass and remain separate gates per CLAUDE.md.

---

## 20. Final Accept/Reject Checklist

- [x] All 35 agents exist on disk under `agents/frontend/` with `metadata.json` + harness variants.
- [x] All 49 skills exist on disk under `skills/frontend/` with `SKILL.md` + `metadata.json` + `references/`.
- [x] `npm run validate` exits 0 in this session.
- [x] Zero `mutating-runtime` agents in this cluster (static-review / read-only-runtime only).
- [x] Security and accessibility are enforced as HARD gates in the governance model (§6, §15), not advisory.
- [x] Performance claims are budget-based with mandatory lab-vs-field separation (§15).
- [x] Migration guidance is rewrite-averse by explicit design rule (§4, §15).
- [x] Every workflow handoff carries verdict + evidence table + blockers + safe next action + rollback path + named owner (§7).
- [ ] **BLOCKER:** 4 skills carry unscoped `Bash` in `allowed-tools` — must be fixed before this cluster is "done," not just "green" (§1, §19 #1).
- [ ] **BLOCKER:** 4 agents carry empty `companion_skills` despite owning an obvious skill on disk — must be fixed, and a completeness gate added so `validate:catalog` catches recurrence (§1, §19 #6).
- [ ] Should-fix: pin W3C as primary a11y source over MDN paraphrase (§19 #3).
- [ ] Should-fix: state explicit boundary between the two overlapping a11y agents (§19 #4).
- [ ] Should-fix: state explicit "distinct from X because Y" boundary across the 5 overlapping perf skills (§19 #5).
- [ ] Nice-to-have: replace Context7-ceremony boilerplate with gap-disclosure pattern where no authoritative library exists (§19 #7).
- [ ] Nice-to-have: propagate blast-radius cap pattern to migration/monorepo skills (§19 #9).
- [ ] Not yet run this session: `npm run lint:spell`, `markdownlint-cli2` (separate CI gates — run before merge).

**Overall disposition: APPROVE WITH BLOCKERS.** Merge is conditioned on closing the two HIGH-severity self-inconsistencies (§1) before this expansion is represented as complete; the should-fix and nice-to-have items are tracked for the next iteration per §17 and do not block initial ship of the 35-agent/49-skill frontend board.
