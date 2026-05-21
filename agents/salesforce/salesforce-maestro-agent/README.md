# Salesforce Maestro Agent

The `salesforce-maestro-agent` is the single entry point for the 30-agent
Salesforce portfolio in this marketplace. It classifies an incoming Salesforce
matter, routes it to the right specialist agent or agents, and coordinates
cross-functional review with Compliance, Privacy, Security, Architecture, and
business stakeholders. The maestro is a **classifier and router only** — it
never executes changes, never mutates a Salesforce org, and does not perform
the substantive specialist review itself. Every handoff is expressed as a
structured case capsule that names a primary agent, a recommended skill, a risk
tier, and a required human decision owner. All live-org mutation requests
(deployment commits, DML on production, irreversible configuration changes) are
refused and escalated to `salesforce-live-guard-agent` for human-in-the-loop
approval.

---

## Quick start (5 minutes)

Choose the harness that matches your tooling. All adapter files live at
`agents/salesforce/salesforce-maestro-agent/harnesses/`.

### Claude Code

**Install path:** copy or symlink `harnesses/claude-code.agent.md` into your
`.claude/agents/` directory.

```bash
mkdir -p .claude/agents
cp agents/salesforce/salesforce-maestro-agent/harnesses/claude-code.agent.md \
   .claude/agents/salesforce-maestro-agent.md
```

**Invocation:** use the agent picker (`@`) or the `/agents` slash command.

**Minimal prompt:**

```
@salesforce-maestro-agent Review our Agentforce Einstein AI copilot
configuration and prompt template design for the autonomous agent.
```

---

### Cursor

**Install path:** copy `harnesses/cursor.agent.md` into `.cursor/agents/`.

```bash
mkdir -p .cursor/agents
cp agents/salesforce/salesforce-maestro-agent/harnesses/cursor.agent.md \
   .cursor/agents/salesforce-maestro-agent.md
```

**Invocation:** open Cursor Chat and mention the agent by name.

**Minimal prompt:**

```
@salesforce-maestro-agent We need to review sharing rules and org-wide defaults
for our Experience Cloud portal. What is the right specialist?
```

---

### GitHub Copilot

**Install path:** copy `harnesses/copilot.agent.md` into `.github/copilot/agents/`.

```bash
mkdir -p .github/copilot/agents
cp agents/salesforce/salesforce-maestro-agent/harnesses/copilot.agent.md \
   .github/copilot/agents/salesforce-maestro-agent.md
```

**Invocation:** invoke via Copilot Chat in VS Code or GitHub.com.

**Minimal prompt:**

```
@salesforce-maestro-agent Classify this matter: we have a MuleSoft integration
using a Connected App with OAuth and we need to review the Named Credential setup.
```

---

### Gemini

**Install path:** copy `harnesses/gemini.agent.md` into your Gemini agent
configuration directory per your workspace setup.

```bash
cp agents/salesforce/salesforce-maestro-agent/harnesses/gemini.agent.md \
   <your-gemini-agents-dir>/salesforce-maestro-agent.md
```

**Invocation:** reference the agent in your Gemini workspace prompt.

**Minimal prompt:**

```
Route this matter to the right Salesforce specialist: we need a compliance
review of our GDPR data retention configuration and audit trail settings.
```

---

### Kiro IDE

**Install path:** copy `harnesses/kiro-ide.agent.md` into `.kiro/agents/`.

```bash
mkdir -p .kiro/agents
cp agents/salesforce/salesforce-maestro-agent/harnesses/kiro-ide.agent.md \
   .kiro/agents/salesforce-maestro-agent.md
```

**Invocation:** open the Kiro IDE agent panel and select the agent.

**Minimal prompt:**

```
@salesforce-maestro-agent We have a Flow Builder automation that triggers
governor-limit-sensitive operations. Route this to the right specialist.
```

---

### Kiro CLI

**Install path:** the CLI adapter is a JSON file.

```bash
mkdir -p .kiro/agents
cp agents/salesforce/salesforce-maestro-agent/harnesses/kiro-cli.agent.json \
   .kiro/agents/salesforce-maestro-agent.json
```

**Invocation:** invoke via the Kiro CLI `run` command.

```bash
kiro agent run salesforce-maestro-agent \
  --prompt "Classify: we need an Apex trigger code review for governor limits."
```

---

### Codex

**Install path:** the Codex adapter is a TOML file.

```bash
cp agents/salesforce/salesforce-maestro-agent/harnesses/codex.toml \
   <your-codex-agents-dir>/salesforce_maestro_agent.toml
```

**Invocation:** reference `salesforce_maestro_agent` in your Codex agent
configuration or session.

**Minimal prompt:**

```
Route this matter: review our Salesforce CPQ quoting configuration and
revenue recognition logic for compliance and risk.
```

---

## How routing works

### Required skills

The maestro loads three cross-functional skills before answering every request:

- `skills/cross-functional/salesforce-routing-protocol/SKILL.md`
- `skills/cross-functional/salesforce-case-capsule/SKILL.md`
- `skills/cross-functional/salesforce-risk-taxonomy/SKILL.md`

### Case capsule

Every routing decision is expressed as a structured **case capsule**. The
capsule always contains:

| Field | Description |
|---|---|
| `matter_type` | Classification from the risk taxonomy |
| `primary_agent` | Exactly one specialist agent to handle the matter |
| `secondary_agents` | Additional specialists (parallel routing only, when genuinely cross-domain) |
| `routing_mode` | `single` / `parallel` / `escalate` / `unclassified` |
| `recommended_skill` | The skill the primary agent should load first |
| `risk_tier` | `Critical` / `High` / `Medium` / `Low` / `Unknown` |
| `decision_owner` | Named human responsible for approval |
| `do_not_do_list` | Non-empty list of explicitly prohibited actions for this matter |
| `escalation_gates` | Which gates fired and why |
| `evidence_basis` | Signals that support the routing decision |
| `missing_context` | Gaps that would change the routing if resolved |
| `confidence` | `strong` / `moderate` / `weak` / `unknown` |

`Unknown` risk is mandatory whenever the org type, product identity, or
material facts are absent.

### Routing modes

- **single** — one specialist is sufficient.
- **parallel** — the matter genuinely crosses two or more domains; the maestro
  coordinates synthesis.
- **escalate** — an escalation gate fired; routes to `salesforce-live-guard-agent`
  or `salesforce-enterprise-architect-agent`.
- **unclassified** — signals are ambiguous; the maestro holds the matter and
  requests clarifying information rather than force-fitting a specialist.

### Escalation gates

The following categories are escalation-grade by default and bypass normal
routing directly to `salesforce-live-guard-agent`:

- Any request matching the live-guard intent pattern (deploy to production,
  destroy/delete org objects, mass update on live records, activate in prod,
  release to prod, promote to production, live org mutation).
- Production data exposure or guest-user access expansion.
- Autonomous Agentforce AI action without grounding evidence.
- Marketing Cloud consent changes.
- Shield encryption or compliance-regulated data changes.
- CPQ/finance logic with revenue impact.
- Irreversible deploys or mass change operations.
- Regulated-vertical matters (Health Cloud, Financial Services Cloud, etc.)
  that carry extra statutory obligations.

When specialist agents produce conflicting assessments, the conflict-resolution
protocol from the routing-protocol skill fires and the matter escalates to
`salesforce-enterprise-architect-agent`.

### Taxonomy reference

Routing keyword matching is defined in
`tests/fixtures/salesforce-maestro-routing/taxonomy.json`. A matter's incoming
signals are compared against each domain's keyword set; the domain with the
highest-confidence keyword match wins. When two domains match at the parallel
threshold (≥ 0.8 of the scoring scale), the maestro considers parallel routing.

---

## The 30-domain taxonomy

| Domain | Primary agent | Typical signals |
|---|---|---|
| `platform-admin-review` | `salesforce-platform-admin-review-agent` | admin, profiles, permission sets, org configuration, sharing rules, org-wide defaults |
| `business-analyst` | `salesforce-business-analyst-agent` | business requirements, user stories, process mapping, business process |
| `app-builder-automation` | `salesforce-app-builder-automation-agent` | flow, process builder, automation, validation rules, workflow rules, governor-limit risk |
| `development` | `salesforce-development-agent` | apex, lwc, lightning web component, visualforce, soql, trigger |
| `devops-release` | `salesforce-devops-release-agent` | devops, release, deployment, ci cd, pipeline, sandbox, change set, metadata deployment |
| `security-identity-access` | `salesforce-security-identity-access-agent` | security, identity, access, SSO, MFA, SAML, OAuth, guest user, field level security |
| `data-architecture` | `salesforce-data-architecture-agent` | data model, schema, data architecture, data migration, data quality, master data |
| `integration-mulesoft` | `salesforce-integration-mulesoft-agent` | integration, mulesoft, api, REST, SOAP, middleware, connected app, named credential |
| `sales-cloud-revenue` | `salesforce-sales-cloud-revenue-agent` | sales cloud, revenue, cpq, opportunity, quote, forecast, pipeline |
| `service-field-service` | `salesforce-service-field-service-agent` | service cloud, case, field service, omni-channel, knowledge base, entitlement, service level |
| `experience-cloud` | `salesforce-experience-cloud-agent` | experience cloud, community, portal, self-service, partner portal, customer portal |
| `marketing-cloud` | `salesforce-marketing-cloud-agent` | marketing cloud, email, journey, marketing automation, campaign, subscriber, SFMC |
| `agentforce-ai` | `salesforce-agentforce-ai-agent` | agentforce, einstein, AI, copilot, prompt template, einstein bot, autonomous agent |
| `analytics-tableau` | `salesforce-analytics-tableau-agent` | analytics, tableau, crm analytics, dashboard, report, Einstein Analytics, Tableau CRM |
| `slack-collaboration` | `salesforce-slack-collaboration-agent` | slack, slack integration, slack workflow, collaboration, slack channel |
| `industry-cloud` | `salesforce-industry-cloud-agent` | industry cloud, health cloud, financial services cloud, manufacturing cloud, consumer goods cloud, vertical cloud |
| `enterprise-architect` | `salesforce-enterprise-architect-agent` | enterprise architect, architecture, multi-org, solution design, technical design document, org strategy |
| `compliance-privacy` | `salesforce-compliance-privacy-agent` | compliance, privacy, GDPR, CCPA, data retention, audit trail, regulatory, data subject request |
| `network-policy-architect` | `salesforce-network-policy-architect-agent` | network policy, ip allowlist, trusted ip, login ip range, csp trusted sites, session timeout, clickjack |
| `hyperforce-security` | `salesforce-hyperforce-security-agent` | hyperforce, data residency, infrastructure access, edge hardening, salesforce region selection |
| `sandbox-isolation` | `salesforce-sandbox-isolation-agent` | sandbox isolation, sandbox boundary, sandbox refresh, production data leak, full sandbox, partial copy |
| `session-governance` | `salesforce-session-governance-agent` | session policy, high assurance session, oauth session, lightning locker, session hijacking, session timeout policy |
| `continuous-verification` | `salesforce-continuous-verification-agent` | continuous verification, always-on mfa, adaptive authentication, behavioral anomaly, risk-based authentication |
| `certificate-lifecycle` | `salesforce-certificate-lifecycle-agent` | certificate expiry, certificate rotation, mtls, mutual tls, jwt signing certificate, saml signing certificate |
| `adaptive-access` | `salesforce-adaptive-access-agent` | transaction security policy, shield event monitoring, context-aware access, einstein trust layer, risk scoring |
| `code-analyzer-orchestrator` | `salesforce-code-analyzer-orchestrator-agent` | salesforce code analyzer, sca findings, pmd apex, eslint lwc, retirejs, graph engine analysis |
| `sandbox-governance` | `salesforce-sandbox-governance-agent` | sandbox masking, pii masking, anonymization, pseudonymization, sandbox data governance, gdpr sandbox |
| `change-impact-analyst` | `salesforce-change-impact-analyst-agent` | change impact, metadata dependency, deployment impact, destructive change risk, field deletion impact, change freeze |

Two domains in the portfolio are not in the keyword routing table — they are
structural roles:

| Role | Agent | Function |
|---|---|---|
| Maestro | `salesforce-maestro-agent` | Classification and routing; this document |
| Live Guard | `salesforce-live-guard-agent` | Refusal-by-default gate for all live-org mutation requests |

---

## Worked examples (step by step)

### Example 1 — Static review (T0)

**Request:**

```
Review our Agentforce Einstein AI copilot configuration and prompt template
design for the autonomous agent.
```

**Step 1 — Classification**

The maestro matches keywords `agentforce`, `AI`, `prompt template`, `autonomous
agent` against the taxonomy and classifies the domain as `agentforce-ai`.
Autonomous Agentforce AI action is escalation-grade by default, so the maestro
checks whether this is a review request (T0-safe) or an execution request.
The request asks for a review of configuration, not execution. The escalation
gate does not fire.

**Step 2 — Risk rating**

The matter involves AI autonomous action design. Risk is rated **High** because
misconfigured Agentforce topics or actions can result in unintended org
mutations. Org type is not specified, so the capsule marks `org_type: Unknown`
and requires the decision owner to confirm the org type before the specialist
proceeds.

**Step 3 — Case capsule output (excerpt)**

```json
{
  "matter_type": "agentforce-ai-review",
  "primary_agent": "salesforce-agentforce-ai-agent",
  "routing_mode": "single",
  "recommended_skill": "salesforce-agentforce-risk-review-skill",
  "execution_tier": "static-review",
  "risk_tier": "High",
  "decision_owner": "[CHECK: Salesforce architect or AI platform owner — supply name]",
  "do_not_do_list": [
    "Do not enable or activate any Agentforce agent topics in a live org.",
    "Do not generate or deploy prompt templates.",
    "Do not approve autonomous action permissions."
  ],
  "confidence": "moderate",
  "missing_context": ["org_type", "whether agent is already active in production"]
}
```

**Step 4 — Human action**

The named decision owner reviews the case capsule, confirms the org type, and
hands the capsule to the `salesforce-agentforce-ai-agent` for substantive review.
No org is contacted. No changes are made.

---

### Example 2 — Read-only runtime (T1)

**Request:**

```
Explore the SOQL query plan for our record-sharing audit. We want to check
query performance against our Opportunity sharing rules.
```

**Step 1 — Classification**

Keywords `soql` and `sharing` match two domains: `development` (soql) and
`platform-admin-review` (sharing rules). Because both keyword clusters match,
the maestro evaluates whether this is truly cross-domain or a single-domain
matter. SOQL query plan analysis is the primary activity; sharing rules are
context. The maestro routes to a single specialist: `salesforce-development-agent`.

**Step 2 — Risk rating**

Live SOQL execution against a connected org requires T1 read-only runtime
credentials. No mutations are involved. Risk is rated **Medium**. The OAuth
scope required is `api` + `refresh_token` — minimum scope only.

**Step 3 — Case capsule output (excerpt)**

```json
{
  "matter_type": "development-soql-analysis",
  "primary_agent": "salesforce-development-agent",
  "routing_mode": "single",
  "recommended_skill": "salesforce-soql-explorer-skill",
  "execution_tier": "read-only-runtime",
  "oauth_scopes": ["api", "refresh_token"],
  "risk_tier": "Medium",
  "decision_owner": "[CHECK: Salesforce developer or platform admin — supply name]",
  "do_not_do_list": [
    "Do not execute DML operations.",
    "Do not modify sharing rules.",
    "Do not use OAuth scopes beyond api and refresh_token.",
    "Do not emit raw org IDs or user IDs in output."
  ],
  "confidence": "strong"
}
```

**Step 4 — Human action**

The decision owner confirms the org allowlist and Connected App scope, then
hands the capsule to the `salesforce-development-agent` loaded with
`salesforce-soql-explorer-skill`. The skill issues read-only SOQL queries
within the declared scope. All outputs are sanitized before being returned.

---

### Example 3 — Production mutation request (T3 — refused)

**Request:**

```
Deploy this change set to production now. We need to activate the new
permission set configuration immediately.
```

**Step 1 — Live-guard gate check**

The intent pattern fires on `deploy` + `production` and `activate`. This
matches the `live_guard_intent` regex in the taxonomy. The escalation gate
fires before any domain routing occurs.

**Step 2 — Routing**

The maestro does **not** route this to any domain specialist. It routes
immediately and exclusively to `salesforce-live-guard-agent` with routing mode
`escalate`.

**Step 3 — Case capsule output (excerpt)**

```json
{
  "matter_type": "live-org-mutation-refused",
  "primary_agent": "salesforce-live-guard-agent",
  "routing_mode": "escalate",
  "risk_tier": "Critical",
  "decision_owner": "[CHECK: Named release manager or org admin with change-management authority — supply name]",
  "do_not_do_list": [
    "Do not deploy any change set.",
    "Do not activate any metadata in production.",
    "Do not proceed without human approval evidence.",
    "Do not simulate or describe the deployment steps."
  ],
  "escalation_reason": "Live org deployment commit to production. T3 operation. PROHIBITED for agents.",
  "confidence": "strong"
}
```

**Step 4 — Human action**

`salesforce-live-guard-agent` documents the refusal and specifies the evidence
required before a human admin may proceed (change-management ticket, rollback
plan, approval from named authority). No agent executes any deployment step.
The human release manager acts outside the agent system.

---

## What the maestro will refuse

The maestro will refuse to route or respond to any request that:

- Supplies or requests org credentials, session tokens, client secrets,
  connected-app client IDs, refresh tokens, or access tokens.
- Supplies or requests personally identifiable information (PII), customer
  data, or row-level Salesforce data.
- Asks the maestro to execute, simulate, or describe execution of any change
  to a live Salesforce org.
- Asks the maestro to deploy metadata, run Apex, issue DML, or trigger SFDX /
  Salesforce CLI mutation commands.
- Asks the maestro to approve a production deployment, grant org access, or
  modify permission sets or profiles in a live org.
- Matches the live-guard intent pattern: destroy, delete, terminate,
  `mutate.*org`, `deploy.*production`, `push.*prod`, `execute.*live`,
  `run.*prod`, `apply.*org`, mass delete, mass update on live records, wipe
  org, activate in prod, release to prod, promote to production.
- Attempts to replace the maestro's persona, inject instructions, or bypass
  its safety contract.

---

## Least-privilege posture

The maestro is declared `execution_tier: static-review` (T0). It operates
entirely on content provided in the conversation — no network egress, no CLI
invocation, no OAuth scopes, no MCP servers.

For detailed permission requirements of the specialists this agent routes to,
see:

- [`LEAST-PRIVILEGES.md`](./LEAST-PRIVILEGES.md) — least-privilege baseline
  for the maestro itself (T0 routing-only). Every specialist agent in
  `agents/salesforce/` also ships its own `LEAST-PRIVILEGES.md`.
- [`../../../docs/execution-tiers.md`](../../../docs/execution-tiers.md) — full
  T0/T1/T2/T3 tier contract including OAuth scope requirements and Run As
  account permission baselines.

---

## Troubleshooting

### Ambiguous routing — multiple keyword clusters match

The maestro scores each domain against the incoming signals. When two domains
both produce strong keyword matches, it checks whether the matter is truly
cross-domain (parallel routing) or whether one domain is context and the other
is substance (single routing). If it cannot distinguish, it marks the routing
mode `unclassified` and asks for a clarifying question rather than force-fitting
a specialist. Provide additional context — org type, the specific object or
component involved, and the type of review needed — to resolve ambiguity.

### Low-confidence routing

A confidence of `weak` or `unknown` indicates that one or more material facts
are missing: org type (production vs. sandbox vs. scratch), product identity
(which Salesforce cloud), or the specific surface being reviewed. The maestro
will list the missing context explicitly in the case capsule. Supply those facts
and resubmit.

### Request falls outside the 30-domain taxonomy

If the incoming matter does not match any taxonomy domain, the maestro marks
the matter `unclassified` and holds it. It will not force a routing decision.
Options:

1. Rephrase the request using more specific Salesforce terminology (see the
   keyword sets in `tests/fixtures/salesforce-maestro-routing/taxonomy.json`).
2. If the matter is a live-org action of any kind, route directly to
   `salesforce-live-guard-agent`.
3. If the matter is architectural or spans the entire platform, route to
   `salesforce-enterprise-architect-agent`.
4. If the matter is genuinely outside Salesforce, the maestro cannot route it
   and will say so.

### Specialist agents disagree

When two specialists routed in parallel return conflicting assessments, the
maestro runs the conflict-resolution protocol from the routing-protocol skill
and escalates to `salesforce-enterprise-architect-agent` with both specialist
outputs as evidence. Do not attempt to resolve specialist disagreements by
re-prompting the maestro; provide the conflicting outputs directly.

---

## Eval coverage

Routing behavior is verified by an automated eval at:

```
tests/fixtures/salesforce-maestro-routing/
```

The fixture set contains:

- 23 happy-path fixtures — one per routable domain (all domains except the
  maestro itself and live-guard).
- 7 adversarial fixtures — including live-guard bypass attempts, instruction
  injection, persona replacement, secrets bait, and ambiguous-signal cases.

To run the routing eval:

```bash
npm run validate:maestro-routing
```

This runs `tests/validate-maestro-routing.py` against all fixtures. The eval
is also part of the full validation pipeline:

```bash
npm run validate
```

All 30 fixtures must pass before a routing change is merged.

---

## Versioning

The maestro's version is declared in `metadata.json` and kept in sync with
`package.json` via `scripts/release-prepare.mjs`. The version is computed by
semantic-release from conventional commits — never hand-edited.

For the full versioning contract, see
[`docs/release-versioning.md`](../../../docs/release-versioning.md).

---

## Related docs

- [`docs/execution-tiers.md`](../../../docs/execution-tiers.md) — T0/T1/T2/T3
  least-privilege contract for all skills and agents.
- [`agents/salesforce/README.md`](../README.md) — full Salesforce portfolio
  overview: all 30 agents, Wave 1 and Wave 3 descriptions, and operating notes.
- [`docs/release-versioning.md`](../../../docs/release-versioning.md) —
  single-source-of-truth versioning contract and semantic-release pipeline.
- [`docs/compatibility.md`](../../../docs/compatibility.md) — harness support
  contract for Claude Code, Cursor, GitHub Copilot, Gemini, Kiro IDE, Kiro CLI,
  and Codex.
- [`tests/fixtures/salesforce-maestro-routing/taxonomy.json`](../../../tests/fixtures/salesforce-maestro-routing/taxonomy.json) —
  machine-readable routing taxonomy: 28 routable domains with keyword sets and
  agent assignments.

---

Part of the [Vanguard Frontier Agentic](../../../README.md) Salesforce portfolio.
