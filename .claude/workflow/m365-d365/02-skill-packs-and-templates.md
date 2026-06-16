# 02 — Skill Packs & Templates

**Workflow:** Microsoft M365 / D365 Agent+Skill Board
**Phase:** Authoring contracts — what gets built, how, and to what standard.
**Ruthlessness level:** Repo-accurate only. No invented fields, no category drift, no tool hallucinations.

---

## A. Skill Pack Inventory

Evidence labels: E2 = repo-verified fact; E3 (verify) = Microsoft product claim, must be live-checked before publication.

### Category mapping rationale

The `category` field is a CLOSED enum (E2). Prohibited values: `microsoft`, `governance`, `business-process`, `productivity`. Each skill below maps to the nearest real enum value with justification.

| Skill ID | Folder Path | category | Primary Agent | Pain Solved | Wave | References Needed |
|---|---|---|---|---|---|---|
| microsoft-maestro | skills/microsoft/microsoft-maestro/ | platform | microsoft-maestro-agent | Multi-domain routing across all Microsoft clouds; prevents mis-dispatch to wrong D365 module or M365 workload | 0 (Phase 0) | official-sources.md, workflow-and-output.md, safety-checklist.md |
| m365-maestro | skills/microsoft/m365-maestro/ | platform | m365-maestro-agent | Routes M365 questions to the narrowest specialist; classifies governance vs. security vs. identity before dispatch | 0 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-maestro | skills/microsoft/d365-maestro/ | platform | d365-maestro-agent | Routes D365 questions across Finance, SCM, BC, CE modules; prevents cross-module mis-routing | 0 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| power-platform-maestro | skills/microsoft/power-platform-maestro/ | platform | power-platform-maestro-agent | Routes Power Platform questions (PP, PA, Copilot Studio, Fabric) to correct specialist | 0 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| copilot-governance-maestro | skills/microsoft/copilot-governance-maestro/ | platform | copilot-governance-maestro-agent | Routes Copilot for M365 / Copilot Studio / AI governance questions; gates live-execution requests | 0 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| m365-tenant-governance | skills/microsoft/m365-tenant-governance/ | compliance | m365-tenant-governance-agent | Tenant hygiene, conditional access baselines, named locations, security defaults, SSPR E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md, identity-policy-map.md |
| m365-identity-zero-trust | skills/microsoft/m365-identity-zero-trust/ | security | m365-identity-zero-trust-agent | Entra ID Zero Trust assessment: MFA, SSPR, PIM eligibility, device compliance gates E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md, identity-policy-map.md |
| m365-endpoint-intune-security | skills/microsoft/m365-endpoint-intune-security/ | security | m365-endpoint-intune-security-agent | Intune device compliance, configuration profiles, app protection, co-management E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| m365-purview-data-security | skills/microsoft/m365-purview-data-security/ | compliance | m365-purview-data-security-agent | Purview sensitivity labels, DLP policies, information barriers, retention, eDiscovery E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md, data-classification-map.md |
| m365-defender-xdr-secops | skills/microsoft/m365-defender-xdr-secops/ | security | m365-defender-xdr-secops-agent | Defender XDR incident triage, hunting queries, MCAS/Defender for Cloud Apps integration E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| m365-copilot-readiness-governance | skills/microsoft/m365-copilot-readiness-governance/ | ai | m365-copilot-readiness-governance-agent | Copilot for M365 readiness: license checks, data oversharing, sensitivity label coverage E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| teams-collaboration-communications-governance | skills/microsoft/teams-collaboration-communications-governance/ | compliance | teams-collab-comms-governance-agent | Teams lifecycle policy, guest access controls, meeting recording retention, DLP in Teams E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-success-by-design-governance | skills/microsoft/d365-success-by-design-governance/ | architecture | d365-success-by-design-governance-agent | FastTrack Success by Design governance gates: fit-gap, solution blueprint, go-live readiness E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-finance-close-to-report | skills/microsoft/d365-finance-close-to-report/ | finance | d365-finance-close-to-report-agent | D365 Finance period close, financial reporting, consolidation ledger, dual currency E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-supply-chain-plan-to-produce | skills/microsoft/d365-supply-chain-plan-to-produce/ | operational | d365-supply-chain-plan-to-produce-agent | D365 SCM MRP, master planning, production orders, warehouse management E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-business-central-smb-operations | skills/microsoft/d365-business-central-smb-operations/ | operational | d365-business-central-smb-operations-agent | BC SMB finance, purchase/sales, inventory, basic manufacturing E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-customer-service-contact-center | skills/microsoft/d365-customer-service-contact-center/ | operational | d365-customer-service-contact-center-agent | D365 Customer Service: case routing, SLA, knowledge, Omnichannel, Copilot in CS E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-field-service-to-cash | skills/microsoft/d365-field-service-to-cash/ | operational | d365-field-service-to-cash-agent | Field Service: WO lifecycle, scheduling (RSO), invoicing, integration with Finance E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-sales-revenue-operations | skills/microsoft/d365-sales-revenue-operations/ | operational | d365-sales-revenue-operations-agent | D365 Sales pipeline hygiene, forecasting, CPQ integration, revenue ops reporting E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-customer-insights-journeys | skills/microsoft/d365-customer-insights-journeys/ | data | d365-customer-insights-journeys-agent | Customer Insights (CDP) unified profile, real-time journeys, segment orchestration E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-integration-dual-write | skills/microsoft/d365-integration-dual-write/ | platform | d365-integration-dual-write-agent | Dual-write setup, table maps, conflict resolution, Finance + CE sync E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md, integration-topology-map.md |
| d365-data-migration-cutover | skills/microsoft/d365-data-migration-cutover/ | data | d365-data-migration-cutover-agent | DMF/DIXF data migration, cutover run-book, go-live freeze/unfreeze, reconciliation E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| d365-security-sod-governance | skills/microsoft/d365-security-sod-governance/ | compliance | d365-security-sod-governance-agent | D365 security roles, duty SoD conflict analysis, audit trail configuration E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| power-platform-governance-dataverse-security | skills/microsoft/power-platform-governance-dataverse-security/ | compliance | power-platform-governance-dataverse-security-agent | DLP policies, Dataverse RBAC, environment strategy, connector governance E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| power-platform-alm-pipelines | skills/microsoft/power-platform-alm-pipelines/ | delivery | power-platform-alm-pipelines-agent | Power Platform ALM: solution layers, managed vs. unmanaged, pipelines, environment promotion E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| copilot-studio-agent-governance-alm | skills/microsoft/copilot-studio-agent-governance-alm/ | ai | copilot-studio-agent-governance-alm-agent | Copilot Studio agent lifecycle, publish gates, channel governance, ALM for AI agents E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| power-automate-automation-risk-review | skills/microsoft/power-automate-automation-risk-review/ | compliance | power-automate-automation-risk-review-agent | Power Automate flow risk: unattended RPA, connector scope, data exfiltration surface E3 (verify) | 1 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| fabric-power-bi-business-insights-governance | skills/microsoft/fabric-power-bi-business-insights-governance/ | observability | fabric-power-bi-business-insights-governance-agent | Fabric workspace governance, Power BI sensitivity label enforcement, row-level security E3 (verify) | 2 | official-sources.md, workflow-and-output.md, safety-checklist.md |
| microsoft-business-impact-value-realization | skills/microsoft/microsoft-business-impact-value-realization/ | finance | microsoft-business-impact-value-realization-agent | Business case framing, KPI baseline, value realization tracking post go-live E3 (verify) | 3 | official-sources.md, workflow-and-output.md, safety-checklist.md, business-impact-kpis.md |

**Wave key:** 0 = maestro/routing (must ship first); 1 = governance/security (high ROI, lower data risk); 2 = process/operational; 3 = value realization.

**Companion mapping note (E2):** Each skill above MUST be declared via `companion_agents: [<agent-id>]` in its `metadata.json`, and the paired agent's `metadata.json` must declare `companion_skills: [<skill-id>]`. Neither field is optional once the pairing exists.

---

## B. Reference-File Policy

### Mandatory (every skill, no exceptions)

| File | Why mandatory |
|---|---|
| `references/official-sources.md` | Every claim must have a traceable Microsoft Learn URL. No official-sources.md = no verifiable grounding. |
| `references/workflow-and-output.md` | Defines the skill's output contract: what the skill returns, in what format, at which workflow step. Without this, agents can't parse skill output. |
| `references/safety-checklist.md` | Least-privilege and irreversibility gates. Required by the repo's zero-trust stance. Cannot be inlined in SKILL.md because it is too long and is referenced selectively. |

### Optional (add only when they add non-redundant value)

| File | When to add | When NOT to add |
|---|---|---|
| `references/business-impact-kpis.md` | Skills where the agent explicitly surfaces KPIs or ROI metrics (e.g., `microsoft-business-impact-value-realization`). | Every other skill — don't manufacture a KPI file for a security skill that never emits KPIs. |
| `references/role-certification-map.md` | Skills that reason about Microsoft certification paths or role-to-product ownership (e.g., `d365-success-by-design-governance`). | Do not add to skills whose workflow never references cert paths. |
| `references/integration-topology-map.md` | Skills that reason about cross-system integration (e.g., `d365-integration-dual-write`). | Do not add to single-workload skills. |
| `references/identity-policy-map.md` | Skills whose workflow explicitly steps through Entra ID policy evaluation (e.g., `m365-identity-zero-trust`, `m365-tenant-governance`). | Do not add to D365 operational skills. |
| `references/data-classification-map.md` | Skills whose workflow steps through sensitivity label or classification taxonomy (e.g., `m365-purview-data-security`). | Do not add to compute or delivery skills. |

### The sprawl math

29 skills (5 maestro + 24 specialist) × 5 mandatory+optional files = 145 boilerplate files if every optional file is added to every skill. Real repo skills carry 2-3 reference files (E2: alibaba skills average 2; accounting-maestro carries 0 references, only SKILL.md + metadata.json). The rule: **add a reference file only when a specific workflow step in SKILL.md calls `Load references/X.md`**. If no workflow step loads it, the file is dead weight.

### Reference quality rules

1. Official Microsoft Learn first (`learn.microsoft.com`). Third-party sources only when Learn doesn't cover the topic.
2. No link duplication across unrelated skills. If two skills share a URL, factor it into `official-sources.md` of each independently with distinct justification.
3. Every link in `official-sources.md` must state: (a) which workflow step it supports, (b) what claim it grounds.
4. URLs must be verified at time of `last_verified` date. Stale URLs are a validation failure.

---

## C. Repo-Accurate Folder Tree

Conventions (E2): agents use 7 harness files; skills use SKILL.md + metadata.json + references/. No invented files.

```
vanguard-frontier-agentic/
├── agents/
│   └── microsoft/
│       ├── AGENTS.md
│       ├── README.md
│       ├── microsoft-maestro-agent/          # fully expanded below
│       │   ├── AGENT.md
│       │   ├── PERMISSIONS.md
│       │   ├── metadata.json
│       │   └── harnesses/
│       │       ├── claude-code.agent.md
│       │       ├── codex.toml
│       │       ├── copilot.agent.md
│       │       ├── cursor.agent.md
│       │       ├── gemini.agent.md
│       │       ├── kiro-cli.agent.json
│       │       └── kiro-ide.agent.md
│       ├── m365-maestro-agent/
│       │   └── ...                           # same 7-harness structure
│       ├── d365-maestro-agent/
│       ├── power-platform-maestro-agent/
│       ├── copilot-governance-maestro-agent/
│       ├── m365-tenant-governance-agent/
│       ├── m365-identity-zero-trust-agent/
│       ├── m365-endpoint-intune-security-agent/
│       ├── m365-purview-data-security-agent/
│       ├── m365-defender-xdr-secops-agent/
│       ├── m365-copilot-readiness-governance-agent/
│       ├── teams-collaboration-communications-governance-agent/
│       ├── d365-success-by-design-governance-agent/
│       ├── d365-finance-close-to-report-agent/
│       ├── d365-supply-chain-plan-to-produce-agent/
│       ├── d365-business-central-smb-operations-agent/
│       ├── d365-customer-service-contact-center-agent/
│       ├── d365-field-service-to-cash-agent/
│       ├── d365-sales-revenue-operations-agent/
│       ├── d365-customer-insights-journeys-agent/
│       ├── d365-integration-dual-write-agent/
│       ├── d365-data-migration-cutover-agent/
│       ├── d365-security-sod-governance-agent/
│       ├── power-platform-governance-dataverse-security-agent/
│       ├── power-platform-alm-pipelines-agent/
│       ├── copilot-studio-agent-governance-alm-agent/
│       ├── power-automate-automation-risk-review-agent/
│       ├── fabric-power-bi-business-insights-governance-agent/
│       └── microsoft-business-impact-value-realization-agent/
│
├── skills/
│   └── microsoft/
│       ├── README.md
│       ├── microsoft-maestro/                # fully expanded below
│       │   ├── SKILL.md
│       │   ├── metadata.json
│       │   └── references/
│       │       ├── official-sources.md       # mandatory
│       │       ├── workflow-and-output.md    # mandatory
│       │       └── safety-checklist.md       # mandatory
│       ├── m365-maestro/
│       │   ├── SKILL.md
│       │   ├── metadata.json
│       │   └── references/
│       │       ├── official-sources.md
│       │       ├── workflow-and-output.md
│       │       └── safety-checklist.md
│       ├── m365-identity-zero-trust/
│       │   ├── SKILL.md
│       │   ├── metadata.json
│       │   └── references/
│       │       ├── official-sources.md
│       │       ├── workflow-and-output.md
│       │       ├── safety-checklist.md
│       │       └── identity-policy-map.md    # optional: workflow calls it
│       ├── d365-integration-dual-write/
│       │   ├── SKILL.md
│       │   ├── metadata.json
│       │   └── references/
│       │       ├── official-sources.md
│       │       ├── workflow-and-output.md
│       │       ├── safety-checklist.md
│       │       └── integration-topology-map.md  # optional: workflow calls it
│       ├── microsoft-business-impact-value-realization/
│       │   ├── SKILL.md
│       │   ├── metadata.json
│       │   └── references/
│       │       ├── official-sources.md
│       │       ├── workflow-and-output.md
│       │       ├── safety-checklist.md
│       │       └── business-impact-kpis.md   # optional: workflow emits KPIs
│       └── ...                               # remaining skills (29 total: 5 maestro + 24 specialist), same pattern
│
├── tests/
│   └── fixtures/
│       └── microsoft-maestro-routing/
│           ├── route-m365-identity.json
│           ├── route-d365-finance.json
│           └── route-power-platform.json
│
├── powers/
│   └── vanguard-microsoft/
│       └── ...                               # provider powers, not skills
│
└── .claude/
    └── evals/
        └── microsoft/
            └── ...                           # eval fixtures for microsoft skills
```

**What is NOT in this tree:**
- No `domain/` subdirectory under skills (E2: not a repo pattern).
- No `AGENT.md` inside skill directories (agents and skills are separate trees).
- No `README.md` per skill (E2: alibaba and accounting skills do not carry per-skill READMEs inside the skill dir — some providers have a provider-level README.md only).
- No cross-functional skill directory unless a skill genuinely spans providers (none of the 29 skills here do; all are `provider: microsoft`).

---

## D. Corrected SKILL.md Template

The brief's template had three fatal errors (E2):
1. `category: microsoft` — not a valid enum value.
2. `domain:` — field does not exist in the schema.
3. `Context7` in `allowed-tools` — not a real tool; tools must be real Claude Code tool names.

### Corrected template

```markdown
---
name: <skill-id-in-kebab-case>
description: >
  <50-1500 chars. What the skill does, for whom, and under what conditions.
  Be specific enough that routing agents can classify without reading the body.>
allowed-tools:
  - Read
  - Grep
  - Glob
  - WebFetch
metadata:
  author: "github: <handle>"
  version: "0.1.0"
  updated: "YYYY-MM-DD"
  category: <one of: security|platform|data|finops|ai|delivery|observability|compliance|resilience|networking|storage|database|compute|architecture|messaging|serverless|cost-management|operational|generation|devsecops|finance>
  lifecycle: experimental
---

# <Human-Readable Skill Name>

## Purpose

<Two to four sentences. What this skill does. What judgment it applies.
What it never does. State the zero-trust and least-privilege stance.>

## When To Use

Use this skill for:

- <specific trigger 1>
- <specific trigger 2>
- <specific trigger 3>

## When NOT To Use

- Do not use this skill to make irreversible changes to production tenants without explicit human approval.
- Do not use this skill if you already know the exact specialist agent ID — bypass maestro skills when the destination is certain.
- Do not use this skill to accept raw credentials, tenant secrets, or PII beyond what is required to classify the task.
- <skill-specific exclusions>

## Lean Operating Rules

1. **Prefer official docs over memory.** Verify claims against `learn.microsoft.com` before asserting feature availability. E3 (verify) any Microsoft product behavior claim.
2. **Least privilege.** Request only the minimum read scopes needed. Never infer write access is acceptable.
3. **No secrets in outputs.** Strip or refuse tenant IDs, client secrets, connection strings, and PII from all outputs.
4. **No prod changes without approval.** Any workflow step that would mutate a production tenant requires an explicit human approval gate before proceeding.
5. **Evidence labels.** Mark all claims: `documentation-based`, `inference`, or `user-provided`. Never present inference as fact.
6. **Current-doc verification.** Microsoft product behavior changes frequently E3 (verify). Load `references/official-sources.md` and check `last_verified` before citing feature behavior.
7. **Human approval for irreversible actions.** Conditional access policy changes, DLP policy deployment, and data deletion are irreversible without backup. Gate these explicitly.
8. **Rollback and owner.** Every recommendation must identify the rollback path and the human owner responsible for executing it.
9. **Cross-agent handoff.** When routing to a specialist agent, emit the routing header: `Route: <agent-id> | Reason: <one sentence> | Mode: single | parallel(N) | live-guard-gate`.

## Required Context

Before proceeding, collect:

- [ ] Microsoft 365 / Dynamics 365 tenant ID (do NOT log or retain)
- [ ] Workload or module in scope (list specifically)
- [ ] Current configuration evidence (screenshot, export, or admin center excerpt)
- [ ] Change intent: advisory review only, or implementation guidance?
- [ ] Environment type: production / UAT / sandbox
- [ ] <skill-specific context items>

## Workflow

### Step 1 — <name>

<What the skill does first. What evidence it reads. What it outputs.>

Load references only when needed:
- Load `references/official-sources.md` when grounding a Microsoft product feature claim.

### Step 2 — <name>

<Next step.>

### Step 3 — Assessment and Recommendations

<What the final output looks like. Categorized findings, ranked by risk.>

Load `references/workflow-and-output.md` for the output format contract.

### Step 4 — Safety Gate

Load `references/safety-checklist.md` before emitting any recommendation that touches production.

## Safety Checklist

Before every output:
- [ ] No secrets or credentials appear in the output.
- [ ] All product behavior claims are labeled `documentation-based` or `E3 (verify)`.
- [ ] Any irreversible action is gated on explicit human approval.
- [ ] Rollback path is identified for every recommendation.
- [ ] Output is labeled `advisory` — not authoritative Microsoft guidance.
- [ ] No tenant-specific data beyond classification minimum was retained.

## Response Minimum

Return, at minimum:

- Assessment scope and evidence reviewed.
- Findings categorized by risk (Critical / High / Medium / Low / Informational).
- Specific recommendations with rollback path and human owner.
- Open questions that must be resolved before implementation.
- Advisory disclaimer.

## References

Load only when needed:

- [Official sources](references/official-sources.md) — grounding for Microsoft Learn URLs supporting this skill's claims.
- [Workflow and output](references/workflow-and-output.md) — output format contract; load when formatting the final assessment.
- [Safety checklist](references/safety-checklist.md) — irreversibility gates; load before any production-touching recommendation.
- [Optional reference if applicable](references/<optional>.md) — load only when Step N explicitly requires it.
```

---

## E. Corrected metadata.json Template

The brief's template omitted required fields and used placeholder values that would fail schema validation. The corrected template below is repo-valid (E2).

```json
{
  "id": "m365-identity-zero-trust",
  "name": "M365 Identity Zero Trust Advisor",
  "version": "0.1.0",
  "type": "skill",
  "provider": "microsoft",
  "harnesses": [
    "claude-code",
    "codex",
    "copilot",
    "cursor",
    "gemini",
    "kiro"
  ],
  "summary": "Assess Microsoft Entra ID Zero Trust posture: MFA coverage, PIM eligibility, Conditional Access policy gaps, device compliance integration, and SSPR readiness. Read-only advisory — never mutates tenant configuration.",
  "source_type": "original",
  "official_docs": [
    "https://learn.microsoft.com/en-us/entra/identity/conditional-access/overview",
    "https://learn.microsoft.com/en-us/entra/id-governance/privileged-identity-management/pim-configure",
    "https://learn.microsoft.com/en-us/entra/identity/authentication/concept-mfa-howitworks",
    "https://learn.microsoft.com/en-us/microsoft-365/security/microsoft-secure-score",
    "https://learn.microsoft.com/en-us/entra/identity/conditional-access/concept-conditional-access-policies"
  ],
  "security_notes": "Read-only advisory skill. Never requests Global Administrator or write-delegated tokens. All Conditional Access policy recommendations require human review before deployment. Tenant IDs and user principal names must not appear in skill outputs or logs. All product behavior claims against Entra ID must carry E3 (verify) until confirmed against live documentation at last_verified date.",
  "last_verified": "2026-06-16",
  "path": "skills/microsoft/m365-identity-zero-trust",
  "category": "security",
  "lifecycle": "experimental",
  "execution_tier": "read-only-runtime",
  "companion_agents": ["m365-identity-zero-trust-agent"],
  "oauth_scopes": [],
  "mcp_servers": []
}
```

### Validation notes on this template

| Field | Rule | Common error |
|---|---|---|
| `summary` | >= 20 chars, must state what the skill does AND what it never does | "Identity zero trust skill" fails — too short, no constraint |
| `official_docs` | >= 1 URI, all must be `learn.microsoft.com` or equivalent official source | Do not use blog posts or partner sites as primary |
| `security_notes` | >= 20 chars, must name at least one specific constraint | "Be careful with security" fails |
| `last_verified` | YYYY-MM-DD, must be a real date | Must be updated when URLs are re-checked |
| `category` | Closed enum only | `microsoft`, `governance`, `business-process` all invalid |
| `provider` | Must be registered in Phase 0 before any skill validates | Skills with `provider: microsoft` will fail until Phase 0 is complete |
| `companion_agents` | Optional but MUST be set if a 1:1 agent exists | Omitting creates a broken cross-reference in the catalog |
| `oauth_scopes` / `mcp_servers` | Only include if skill is live-execution tier | Read-only skills should set these to empty arrays or omit |

---

## Phase sequencing reminder

Before authoring any skill in the inventory above:

1. **Phase 0** must register `provider: microsoft` and create the maestro routing layer (Wave 0 skills).
2. `npm run manifest:write` after each skill is added — not after all 29.
3. `npm run validate` before finishing any PR — 19+ gates will catch category drift, missing fields, and stale SHA256 hashes.
4. `python3 tests/validate-asset-integrity.py --write` after any root-level or catalog file change.

---

Next: [03-routing-matrix-and-protocols.md](03-routing-matrix-and-protocols.md) — maestro routing table, dispatch rules, live-guard gate protocol, and cross-domain boundary resolution for the full 29-skill graph.
