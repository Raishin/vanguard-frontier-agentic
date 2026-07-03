# Workflow Routing Table

Use this reference to classify the workflow first, then dispatch exactly the specialists it requires — no more, no fewer. Picking the wrong sequence makes every downstream verdict cargo cult: a security review sequenced like a design-system change will miss the mandatory red-team pass; a design-system change sequenced like a security review wastes a specialist's time on an irrelevant threat model.

Two agents are **standing HARD-gate members on every workflow**, regardless of which row below applies:

- `accessibility-wcag-agent` — WCAG 2.2 AA conformance. A reject here is never downgradeable except by a named human risk-owner's recorded acceptance (see `conflict-resolution.md`).
- `frontend-security-agent` — security posture (XSS/CSP, auth/session, dependency risk). Same non-downgradeable rule.

If a workflow's row already lists one of these explicitly, that is because it is the *primary* concern for that workflow, not an exception to the standing rule — both remain active either way.

## The 10 governed workflows

### 1. New framework feature

- **Primary specialist(s):** the framework specialist matching the stack in scope — `react-specialist-agent`, `vue-specialist-agent`, `angular-specialist-agent`, `svelte-sveltekit-specialist-agent`, or `nextjs-specialist-agent`.
- **Supporting:** `typescript-contracts-agent` (contract soundness), `testing-quality-engineering-agent` (coverage of the new surface).
- **Tier-2 red-team:** spot-check only (not mandatory) unless the feature touches auth, payment, or PII surfaces — then treat as workflow 4 (security review) instead.
- **Gate emphasis:** standard HARD gates only.

### 2. Performance regression

- **Primary specialist(s):** `web-performance-core-vitals-agent`.
- **Supporting:** `build-tooling-bundling-agent` (bundle/code-split root cause), `frontend-observability-rum-agent` (field confirmation).
- **Tier-2 red-team:** spot-check.
- **Gate emphasis:** performance-budget gate — require both lab data (synthetic profiling) and field data (RUM) before full approve. Lab-only is conditional-approve at most; see `conflict-resolution.md` for the lab-vs-field rule.

### 3. Accessibility audit

- **Primary specialist(s):** `accessibility-wcag-agent`.
- **Supporting:** `html-semantics-agent` (semantic structure feeding assistive tech).
- **Tier-2 red-team:** spot-check, escalate to mandatory if the audit's own evidence is automated-tooling-only (axe-core/Lighthouse) with no manual keyboard/screen-reader pass — automated tooling structurally cannot detect keyboard traps, focus-order regressions, or meaningful alt-text quality.
- **Gate emphasis:** HARD gate; this workflow's own primary specialist output feeds it.

### 4. Security review

- **Primary specialist(s):** `frontend-security-agent`.
- **Supporting:** `frontend-bff-boundary-review`-scope work routes through `api-integration-bff-agent` if the review touches a BFF/API boundary.
- **Tier-2 red-team:** **mandatory** — `enterprise-red-team-review-agent` must run and report before adjudication; a security review without a red-team pass is an incomplete workflow, not a fast one.
- **Gate emphasis:** HARD gate; this workflow's own primary specialist output feeds it.

### 5. SSR/hydration bug

- **Primary specialist(s):** `ssr-hydration-streaming-agent`.
- **Supporting:** the matching framework specialist (`react-specialist-agent`, `nextjs-specialist-agent`, `vue-specialist-agent`, or `svelte-sveltekit-specialist-agent`), `javascript-runtime-agent` (event-loop/timing root cause).
- **Tier-2 red-team:** spot-check.
- **Gate emphasis:** standard HARD gates. Verify any hydration-mismatch or error-boundary claim against Context7 (`/reactjs/react.dev`, `/vercel/next.js`) — do not accept a specialist's unverified claim about `error.js` Client Component requirements, `global-error.js` `<html>`/`<body>` requirements, or `suppressHydrationWarning` semantics at face value.

### 6. Design-system change

- **Primary specialist(s):** `design-systems-governance-agent`.
- **Supporting:** `css-architecture-agent` (token/architecture consistency), `visual-regression-agent` (visual-diff evidence).
- **Tier-2 red-team:** spot-check.
- **Gate emphasis:** standard HARD gates (a design-system change that regresses contrast ratios or focus-visible styling is an accessibility HARD-gate finding, not a stylistic nit).

### 7. Framework migration

- **Primary specialist(s):** `frontend-migration-modernization-agent`.
- **Supporting:** the target-framework specialist, `testing-quality-engineering-agent` (regression coverage across the migration boundary).
- **Tier-2 red-team:** spot-check.
- **Gate emphasis:** rewrite-bias check — see `conflict-resolution.md`. A full rewrite recommendation without a documented narrower-path (adapt/strangler-fig) evaluation is a blocker, not a stylistic preference.

### 8. AI-generated code review

- **Primary specialist(s):** `ai-assisted-frontend-review-agent`.
- **Supporting:** `typescript-contracts-agent` (contract/type soundness of generated code).
- **Tier-2 red-team:** **mandatory** — AI-generated code carries a distinct failure class (plausible-looking but subtly wrong auth checks, over-broad dependencies, prompt-injected instructions embedded in comments) that a single Tier-1 pass structurally under-detects.
- **Gate emphasis:** standard HARD gates, plus provenance requirement — the workflow's required inputs must include which parts of the diff were AI-generated; missing provenance is a blocker, not an assumption to fill in.

### 9. Production incident

- **Primary specialist(s):** `frontend-observability-rum-agent`.
- **Supporting:** the specialist matching the suspected root cause domain (`ssr-hydration-streaming-agent`, `web-performance-core-vitals-agent`, `frontend-security-agent`, etc. — classify from the incident signal before dispatching).
- **Tier-2 red-team:** **mandatory** — incidents require adversarial verification that the proposed fix addresses the actual root cause and does not mask a security or a11y regression.
- **Gate emphasis:** require an explicit blast-radius assessment and rollback path as part of required inputs; a production-incident workflow without one is incomplete regardless of how confident the specialist is.

### 10. Core Web Vitals field failure

- **Primary specialist(s):** `web-performance-core-vitals-agent`.
- **Supporting:** `frontend-observability-rum-agent` (field-data source of truth), `build-tooling-bundling-agent` (bundle-weight root cause where applicable).
- **Tier-2 red-team:** spot-check.
- **Gate emphasis:** field data is mandatory, not optional, for this workflow by definition — a CWV field-failure workflow adjudicated on lab data alone is a contradiction in terms; reject or escalate rather than approve.

## Classifying an ambiguous request

If the task text does not map cleanly to one row, do not guess a sequence and proceed silently. State the ambiguity, propose the closest-matching row(s), and either ask for the missing signal or route to the union of the plausible rows' primary specialists plus both standing HARD-gate members, explicitly marked as a provisional dispatch pending clarification.
