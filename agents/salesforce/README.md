# Salesforce Agents

<p align="center">
  <img src="../../assets/logos/cloud/salesforce/salesforce.svg" alt="Salesforce logo" width="200" />
</p>

Salesforce platform governance, admin review, architecture, security, integration,
revenue operations, service operations, marketing operations, Agentforce/AI risk,
and compliance agent catalog for this marketplace.

## Salesforce agent ecosystem

A three-layer ecosystem: a maestro that classifies and routes, nineteen specialists
covering every major Salesforce surface, and a shared cross-functional protocol layer.
All agents are static-review — they triage, analyze, and escalate; they never mutate
a Salesforce org, execute SFDX commands, deploy metadata, or run Apex.

| Agent | Primary use | Layer |
|---|---|---|
| `salesforce-maestro-agent` | Classifies a Salesforce matter, routes it to the right specialist, coordinates multi-agent review | maestro |
| `salesforce-platform-admin-review-agent` | Org configuration review, permission sets, profiles, sharing model, object schema, field-level security | specialist |
| `salesforce-business-analyst-agent` | Requirements, user stories, process mapping, acceptance criteria, Salesforce feature fit, stakeholder alignment | specialist |
| `salesforce-app-builder-automation-agent` | Flow Builder, Process Builder, approval processes, declarative automation safety and governor-limit risk | specialist |
| `salesforce-development-agent` | Apex classes/triggers, LWC, Visualforce, SOQL/SOSL, code quality, test coverage, governor limits | specialist |
| `salesforce-devops-release-agent` | SFDX / Salesforce CLI pipelines, change sets, sandbox strategy, release readiness, regression risk | specialist |
| `salesforce-security-identity-access-agent` | Org security posture, SSO/MFA, Connected Apps, Named Credentials, IP allowlisting, Shield, Event Monitoring | specialist |
| `salesforce-data-architecture-agent` | Data model design, object relationships, master data strategy, Data Cloud / Data 360, archival, volume risk | specialist |
| `salesforce-integration-mulesoft-agent` | MuleSoft API review, Salesforce API surface, integration patterns, error handling, idempotency, OAuth flows | specialist |
| `salesforce-sales-cloud-revenue-agent` | Sales Cloud process review, CPQ, Revenue Cloud, quoting, opportunity management, forecasting, pipeline governance | specialist |
| `salesforce-service-field-service-agent` | Service Cloud cases, entitlements, SLAs, omni-channel, Field Service Lightning scheduling, knowledge governance | specialist |
| `salesforce-experience-cloud-agent` | Experience Cloud sites, guest-user access, community sharing rules, digital experience security posture | specialist |
| `salesforce-marketing-cloud-agent` | Marketing Cloud Engagement, Account Engagement (MCAE/Pardot), consent management, journey review, data extensions | specialist |
| `salesforce-agentforce-ai-agent` | Agentforce agent topics/actions, prompt templates, Prompt Builder, Einstein AI feature risk, autonomous action review | specialist |
| `salesforce-analytics-tableau-agent` | CRM Analytics, Tableau, Einstein Discovery, dashboard governance, KPI definitions, data lineage review | specialist |
| `salesforce-slack-collaboration-agent` | Slack integration with Salesforce, workflow automations, Slack Connect governance, notification design | specialist |
| `salesforce-industry-cloud-agent` | Industry Cloud vertical fit review — routes to vertical counsel; covers Health Cloud, Financial Services Cloud, Education Cloud, Nonprofit Cloud, and others | specialist (router-to-vertical-counsel) |
| `salesforce-enterprise-architect-agent` | Org strategy, multi-org topology, platform boundaries, capability roadmap, technical debt, architect-tier review | specialist |
| `salesforce-compliance-privacy-agent` | Data residency, GDPR/CCPA obligations in Salesforce context, field-level PII mapping, retention, Shield encryption | specialist |
| `salesforce-live-guard-agent` | Refusal-by-default gate for any request that would mutate a live Salesforce org — documents the refusal, demands human approval evidence | live-guard |

## Operating note

- These agents perform **static review only** — they read sanitized configuration
  excerpts, metadata XML, anonymized data models, and sanitized code; they surface
  risks, assumptions, evidence gaps, and escalation paths. They never execute SFDX
  or Salesforce CLI commands, deploy metadata packages, run Apex, or contact a live
  Salesforce org.
- **This repo is not a Salesforce DX executor.** No agent produces deployable
  metadata packages, change sets, or unlocked package versions.
- No agent approves a production deployment, grants org access, modifies permission
  sets or profiles in a live org, or configures a Connected App — every live-org
  mutation routes to a named human admin with change-management evidence.
- `salesforce-live-guard-agent` refuses all live-org mutation requests by default
  and documents the refusal with the evidence required before a human may proceed.
- Escalation-grade matters (guest-user exposure, Shield encryption decisions,
  cross-border data transfer, autonomous Agentforce action, and consent-mapping
  failures) are flagged immediately and routed to a qualified human owner.
- Cross-domain matters use the `salesforce-routing-protocol` and related
  cross-functional protocol skills; see `skills/cross-functional/`.
- **Verify before merge:** Salesforce product names, certification names, and
  Agentforce / Data Cloud terminology change frequently. All credential names and
  product terms in these agents are marked `[VERIFY]`. Confirm against
  [Salesforce Credentials](https://trailhead.salesforce.com/credentials/administratoroverview)
  and official release notes before treating any name as current.
- Never supply org credentials, session tokens, tenant IDs, customer data, PII,
  or any other secrets to these agents.

## Install

> **Wave 2 note:** The `salesforce-portfolio-architect` role is not yet registered
> in `catalog/install-roles.json`. It will be added in Wave 2 alongside full catalog
> entries for all 20 agents. Until then, agents can be referenced individually.

```bash
# Wave 2 (pending catalog registration)
npx vfa-export-agents --platform claude-code --role salesforce-portfolio-architect --repo .
```
