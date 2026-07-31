---
name: netsuite-routing-protocol
description: Use this skill when a NetSuite matter must be classified and routed to the right specialist agent, when a matter crosses multiple NetSuite domains and needs parallel review, when a live-account mutation intent must be gated, or when specialist agents disagree and the conflict must be resolved. It defines routing rules per matter type, the cross-domain overlap matrix covering finance × developer × identity × integration × analytics × OneWorld × AI-connector × compliance, and the conflict-resolution protocol. Does not give NetSuite or business advice; routing is a recommendation only and never makes a binding routing decision on behalf of a human owner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: platform
  lifecycle: experimental
---

# NetSuite Routing Protocol

## Purpose
This skill defines how a NetSuite matter is classified, routed to the right
specialist agent, and coordinated when it crosses multiple NetSuite domains. It
exists so agents never work in silos when risk crosses a domain boundary, and so
every handoff is controlled and auditable rather than free-form. It does not give
NetSuite or business advice; routing is a recommendation that a human owner
confirms.

## When to use
- The `netsuite-maestro-agent` must classify an incoming NetSuite matter and dispatch it.
- A specialist agent finds a matter has crossed into another NetSuite domain.
- A matter triggers an escalation gate and must be paused.
- Specialist agents reach conflicting recommendations.
- A matter is ambiguous and cannot be confidently assigned to a single domain.

## When not to use
- The matter is already classified and a specialist agent is actively working it.
- The matter is outside NetSuite scope entirely.
- You need direct live-account mutation handling — that is owned by
  `netsuite-live-org-mutation-guard-agent`, not this routing skill.

## Communication principles
- One accountable human owner per matter. One primary agent per matter.
- Parallel review only when the matter genuinely crosses domains.
- No agent makes a final configuration, deployment, posting, period-close,
  or compliance decision.
- Every high-risk cross-domain matter produces a **pause and escalate**
  recommendation unless sufficient documented controls already exist.
- Every agent defaults to least-privilege access (never the Administrator role)
  and minimum-necessary, sanitized data.

## Routing rules

### Governance / evidence / release drift
Evidence labelling, release-sensitive claims, biannual NetSuite release drift →
**netsuite-evidence-release-drift-agent**.

### Enterprise architecture
Multi-account/OneWorld topology, solution design, platform strategy →
**netsuite-enterprise-architecture-agent**.

### Audit / SOX / financial controls
Segregation of duties, posting, period close, revenue recognition, approval
workflows, audit trail → **netsuite-audit-controls-sox-agent**.

### Foundation / administration / consulting
SuiteFoundation setup, administration, ERP consulting and configuration →
**netsuite-suitefoundation-agent**, **netsuite-administrator-agent**, or
**netsuite-erp-consultant-agent**.

### Finance
Financial setup, AP/AR, accounting configuration →
**netsuite-financial-foundations-agent** (escalate close-impacting items to
audit-controls-sox).

### BI / reporting / saved searches
Reports, dashboards, KPIs, data-source semantics →
**netsuite-bi-reporting-agent**; saved search and SuiteAnalytics Workbook
mechanics, PII-in-export → **netsuite-saved-searches-workbook-agent**.

### Development
SuiteScript records and UIF/SPA → **netsuite-application-developer-agent**;
SDF and SuiteScript upgrade → **netsuite-suitecloud-developer-agent**; static
SuiteScript security review → **netsuite-suitescript-secure-code-review-agent**;
SuiteFlow workflow review → **netsuite-suiteflow-automation-agent**.

### Identity / access / auth
Roles, permissions, segregation of duties → **netsuite-identity-access-role-permission-agent**;
OAuth 2.0 / TBA / SSO / SAML authentication → **netsuite-sso-oauth-tba-agent**.

### Integration
SuiteTalk REST/SOAP record APIs → **netsuite-web-services-integration-agent**;
end-to-end integration architecture and SOAP→REST migration →
**netsuite-integration-migration-agent**.

### OneWorld / multi-subsidiary
Subsidiary, currency, tax jurisdiction, legal entity, intercompany boundaries →
**netsuite-oneworld-multisubsidiary-agent**.

### Data governance / privacy
PII exposure, retention, field-level access, export controls →
**netsuite-data-governance-privacy-agent**.

### AI Connector / MCP
AI Connector and MCP governance, tool allowlists, prompt-injection testing →
**netsuite-ai-connector-mcp-agent**.

### Sandbox / non-production governance
Sandbox vs release-preview vs production separation, sandbox OAuth
re-authorization → **netsuite-sandbox-nonproduction-governance-agent**.

### SDF / DevOps / release
SuiteCloud Development Framework project structure, deployment controls,
environment promotion → **netsuite-sdf-devops-release-agent**.

## Cross-domain overlap matrix

| Overlap scenario | Primary agent | Secondary agents | Escalation gate |
|---|---|---|---|
| Saved search exposes cross-subsidiary data | netsuite-saved-searches-workbook-agent | netsuite-oneworld-multisubsidiary-agent, netsuite-data-governance-privacy-agent | cross-subsidiary-data |
| SuiteScript change + auth/secret handling | netsuite-suitescript-secure-code-review-agent | netsuite-sso-oauth-tba-agent | production-data |
| AI Connector tool scope + permissions | netsuite-ai-connector-mcp-agent | netsuite-identity-access-role-permission-agent | autonomous-AI-action |
| Role change + segregation of duties | netsuite-identity-access-role-permission-agent | netsuite-audit-controls-sox-agent | segregation-of-duties |
| SDF deploy + live mutation | netsuite-sdf-devops-release-agent | netsuite-live-org-mutation-guard-agent | irreversible-deploy |
| SOAP integration + migration timeline | netsuite-integration-migration-agent | netsuite-web-services-integration-agent | soap-deprecation |
| Reporting logic + period close impact | netsuite-bi-reporting-agent | netsuite-audit-controls-sox-agent | finance/revenue |

## Conflict-resolution protocol
When specialist agents disagree, follow this order and stop at the first unmet step:
1. Freeze any irreversible action.
2. Preserve evidence; preserve sandbox state if applicable.
3. State the disagreement precisely — what each agent concluded and why.
4. Separate technical risk from financial/operational risk; do not collapse them.
5. Identify the accountable human owner.
6. Escalate to a qualified NetSuite architect or control owner.
7. Document unresolved assumptions and open questions.
8. Produce options, not a single conclusion.
9. Require human approval before any action.
10. Log the decision path in the audit log.

## Security notes
- Routing is a recommendation, never an authorization. The protocol never
  approves, denies, or directs a deployment, posting, or configuration action.
- A matter is routed on sanitized signals. Never request account credentials,
  tokens, the Administrator role, customer PII, or production account IDs to classify.
- When classification is ambiguous, mark the matter `unclassified` rather than guessing.
- This protocol does not authorize live-account mutations; those route to
  `netsuite-live-org-mutation-guard-agent`.

## Audit log fields
- matter_id, agent_id, agent_version, invoked_by, input_hash, evidence_quality, output_verdict, escalation_fired, timestamp

## Stop conditions
- A live-mutation action is requested — stop and route to netsuite-live-org-mutation-guard-agent.
- Classification requires production account credentials — stop and refuse.
- Matter involves regulated personal data and jurisdiction is unknown — stop and escalate.
