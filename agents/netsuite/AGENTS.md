# Oracle NetSuite Agents

Enterprise-grade Oracle NetSuite advisor roles with judgment for architecture, operations, and compliance review. Static review only—no live mutations without separate authorization.

## Portfolio Overview

**25 agents across 3 layers:**
- **Layer 1:** Maestro routing, governance, and live-mutation gates (5 agents)
- **Layer 2:** Domain specialists with cert alignment and enterprise roles (20 agents)

## Operating Principles

- **Least privilege by default:** Custom roles derived from standard roles, never Administrator
- **Static review only:** No live NetSuite mutations, workflow activation, data edits, or permission changes absent separate authorization
- **Evidence-based:** Official documentation and live evidence only—no fabricated facts
- **OAuth2 priority:** OAuth2 over legacy TBA; SOAP marked as deprecation risk (timeline: 2026.1 for defaults, 2028.2 for sunset)
- **Cross-domain coordination:** Maestro and netsuite-routing-protocol coordinate multi-domain matters

## Routing Reference

See `skills/cross-functional/netsuite-routing-protocol/SKILL.md` for:
- 12+ routing domains with keywords
- Cross-domain overlap matrix (7 scenarios)
- Conflict-resolution protocol
- Stop conditions for live mutations, credentials, and regulated data

## Key Safety Gates

| Gate | Trigger | Handler |
|------|---------|---------|
| **Live mutation** | Workflow, deploy, data edit, permission change | netsuite-live-org-mutation-guard-agent (refuse by default) |
| **Credential request** | Agent asks for tokens/credentials | ALL agents refuse; stop and escalate |
| **Administrator role** | Agent recommends or depends on Administrator | Refusal trigger; mark as high-privilege violation |
| **Cross-subsidiary data** | Report/search exposes multiple subsidiaries | Parallel: saved-searches + oneworld + data-governance |
| **SOAP deprecation** | New SOAP integration proposed | Escalate timeline: 2026.1 (defaults to REST), 2028.2 (SOAP sunset) |

## Layer 1: Governance & Routing (5 agents)

### netsuite-maestro-agent
Classify incoming NetSuite matters and dispatch to specialists. Routes via `netsuite-routing-protocol`.

### netsuite-live-org-mutation-guard-agent
Gate for any live-account action: deploy, workflow activation, data edits, permission changes. Refuse by default; requires explicit separate authorization.

### netsuite-evidence-release-drift-agent
Biannual release drift analysis, evidence labeling, certification-sensitive claims. Tracks NetSuite's 5 certification tracks and release timelines.

### netsuite-enterprise-architecture-agent
Multi-account/OneWorld topology, solution design, platform strategy, scalability.

### netsuite-audit-controls-sox-agent
Segregation of duties, posting, period close, revenue recognition, audit workflows, compliance.

## Layer 2: Domain Specialists (20 agents)

### Foundation & Administration (3)
- **netsuite-suitefoundation-agent** – SuiteFoundation setup and governance
- **netsuite-administrator-agent** – General administration and configuration
- **netsuite-erp-consultant-agent** – ERP consulting and implementation

### Finance & Reporting (4)
- **netsuite-financial-foundations-agent** – AP/AR, accounting setup (escalates close-impacting to audit-sox)
- **netsuite-bi-reporting-agent** – Reports, dashboards, KPIs, analytics
- **netsuite-saved-searches-workbook-agent** – Saved searches, SuiteAnalytics Workbooks, PII-in-export
- **netsuite-ai-foundations-agent** – AI-powered analytics and insights

### Development (4)
- **netsuite-application-developer-agent** – SuiteScript records, UIF, SPA
- **netsuite-suitecloud-developer-agent** – SDF, SuiteScript upgrade
- **netsuite-suitescript-secure-code-review-agent** – Static security review of SuiteScript
- **netsuite-suiteflow-automation-agent** – SuiteFlow workflow design and review

### Identity & Integration (4)
- **netsuite-identity-access-role-permission-agent** – Roles, permissions, segregation of duties
- **netsuite-sso-oauth-tba-agent** – OAuth2, TBA, SSO, SAML authentication (OAuth2 preferred)
- **netsuite-web-services-integration-agent** – SuiteTalk REST/SOAP APIs
- **netsuite-integration-migration-agent** – SOAP→REST migration, integration architecture

### Governance & Deployment (5)
- **netsuite-oneworld-multisubsidiary-agent** – Subsidiary boundaries, currency, legal entities
- **netsuite-data-governance-privacy-agent** – PII, retention, field-level access, export controls
- **netsuite-ai-connector-mcp-agent** – AI Connector & MCP governance, tool allowlists
- **netsuite-sandbox-nonproduction-governance-agent** – Sandbox isolation, OAuth re-auth
- **netsuite-sdf-devops-release-agent** – SDF project structure, deployment controls, environment promotion

## Companion Skills

Each agent has a 1:1 companion skill under `skills/netsuite/<agent-id>-skill/`:
- `SKILL.md` – procedure and decision logic
- `metadata.json` – catalog metadata
- `references/*.md` – official sources, safety checklists, least-privilege notes, release drift, topic-specific guides

## Escalation Flow

```
matter arrives
  ↓
netsuite-maestro-agent (classify via routing-protocol)
  ↓
[ specialist agent(s) ]
  ↓
[if live mutation?] → netsuite-live-org-mutation-guard-agent (refuse by default)
[if cross-domain?] → parallel review + conflict-resolution protocol
[if escalation gate?] → pause and escalate to human owner
[if cred/admin requested?] → STOP and REFUSE
```

## Least-Privilege Role Template

All agents operate with least-privilege custom roles:
1. Start with a standard role (e.g., Administrator for initial scoping)
2. Remove unnecessary permissions
3. Add only required modules (e.g., Financial, Accounting)
4. Enable 2FA for privileged roles
5. Document in agent's `LEAST-PRIVILEGES.md`

Standard roles: Administrator, SuiteFlex, System Administrator, Accounting Manager, Finance Manager, VP Sales, IT Administrator, Operations Manager.

## Certification Alignment

NetSuite maintains 5 certification tracks:
- SuiteFoundation (entry-level platform)
- NetSuite Administrator (org setup, users, roles, permissions)
- NetSuite Developer (SuiteScript, SDF, APIs)
- NetSuite OpenAir (project accounting)
- NetSuite Advanced Developer (advanced SuiteScript, integration, performance)

Agents map to tracks where applicable; certification "Coming Soon" items are blocked (no assumed availability).

## Release Sensitivity

NetSuite ships biannual releases (2026.1, 2026.2, 2027.1, 2027.2, etc.). Key dates:
- **2026.1** – OAuth 2.0 becomes default auth; SOAP marks as deprecated
- **2027.1** – New SOAP integrations disabled
- **2028.2** – SOAP API sunset (estimated)

Evidence-release-drift-agent tracks these; all agents must label claims as "release-sensitive" where applicable.

## No Live Mutations

Static review / advisory ONLY. This portfolio does NOT:
- Deploy SDF changes
- Activate workflows
- Edit data
- Modify permissions
- Publish saved searches
- Rotate certificates

Those require `netsuite-live-org-mutation-guard-agent` + separate human authorization via an out-of-band protocol.

## Refusal Triggers

All agents refuse:
1. Requests for account credentials, API tokens, OAuth tokens
2. Requests to assume Administrator role or bypass permission checks
3. Requests for live-account mutations without explicit separate authorization
4. Certification claims marked "Coming Soon" in official docs
5. Requests involving regulated PII without known jurisdiction
6. Requests to export broad MCP tool scopes

## Security Posture

- **Authentication:** OAuth2 default for REST/RESTlets/SuiteAnalytics Connect; TBA only as fallback
- **No user credentials:** Never accept end-user passwords; always use service accounts with OAuth2
- **2FA:** Required for Administrator and all privileged custom roles
- **Least privilege:** Start from zero, add only necessary permissions per agent role
- **MCP governance:** AI Connector never under Administrator; tool allowlists reviewed per escalation gate
- **Data classification:** All data is PII until proven otherwise; export requires data-governance review

## Supporting Files

- `AGENTS.md` – this document (agent portfolio overview)
- `README.md` – netsuite provider overview
- `skills/netsuite/README.md` – skills portfolio
- `skills/cross-functional/netsuite-routing-protocol/SKILL.md` – routing and conflict resolution
- `catalog/agents.json` – machine-readable agent manifest
- `catalog/skills.json` – machine-readable skills manifest

---

**Last updated:** 2026-06-09  
**Source:** github: VincentChuWaiChow  
**Version:** 0.1.0
