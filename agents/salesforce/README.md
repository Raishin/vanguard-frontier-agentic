# Salesforce Agents

<p align="center">
  <img src="../../assets/logos/cloud/salesforce/salesforce.svg" alt="Salesforce logo" width="200" />
</p>

Salesforce platform governance, admin review, architecture, security, integration,
revenue operations, service operations, marketing operations, Agentforce/AI risk,
compliance, DevSecOps, and zero-trust agent catalog for this marketplace.

**30 agents** — 20 Wave 1 domain specialists plus 10 Wave 3 infrastructure security
and DevSecOps specialists.

Companion skill portfolio: [`skills/salesforce/`](../../skills/salesforce/README.md) — 25 skills across four execution tiers.

---

## Salesforce agent ecosystem

A three-layer ecosystem: the `salesforce-maestro-agent` classifies and routes;
29 specialists cover every major Salesforce surface; the `salesforce-live-guard-agent`
acts as a refusal-by-default authority gate for any live-org mutation request. All
agents are static-review (T0) — they triage, analyze, and escalate; they never mutate
a Salesforce org, execute SFDX/Salesforce CLI commands, deploy metadata, or run Apex.

---

## Wave 1 — Domain specialist agents (20)

### Routing and live-guard authority

| Agent | Layer | Summary |
|---|---|---|
| [`salesforce-maestro-agent`](salesforce-maestro-agent/README.md) | maestro | Classifies a Salesforce matter, routes it to the right specialist(s), and coordinates multi-agent review — [step-by-step user guide](salesforce-maestro-agent/README.md) |
| `salesforce-live-guard-agent` | live-guard | Refusal-by-default authority gate for any request that would mutate a live Salesforce org — documents the refusal, demands human approval evidence
|

### Platform and development

| Agent | Summary |
|---|---|
| `salesforce-platform-admin-review-agent` | Org configuration review — permission sets, profiles, sharing model, object schema, field-level security |
| `salesforce-business-analyst-agent` | Requirements, user stories, process mapping, acceptance criteria, Salesforce feature fit, stakeholder alignment |
| `salesforce-app-builder-automation-agent` | Flow Builder, validation rules, approval processes, declarative automation safety and governor-limit risk |
| `salesforce-development-agent` | Apex classes/triggers, LWC, Visualforce, SOQL/SOSL, code quality, test coverage, governor limits |
| `salesforce-devops-release-agent` | SFDX / Salesforce CLI pipelines, change sets, sandbox strategy, release readiness, regression risk |

### Security and identity

| Agent | Summary |
|---|---|
| `salesforce-security-identity-access-agent` | Org security posture, SSO/MFA, Connected Apps, Named Credentials, IP allowlisting, Shield, Event Monitoring
|

### Architecture and integration

| Agent | Summary |
|---|---|
| `salesforce-data-architecture-agent` | Data model design, object relationships, master data strategy, Data Cloud / Data 360
, archival, volume risk |
| `salesforce-integration-mulesoft-agent` | MuleSoft API review, Salesforce API surface, integration patterns, error handling, idempotency, OAuth flows |
| `salesforce-enterprise-architect-agent` | Org strategy, multi-org topology, platform boundaries, capability roadmap, technical debt, architect-tier review |

### Cloud-specific surfaces

| Agent | Summary |
|---|---|
| `salesforce-sales-cloud-revenue-agent` | Sales Cloud process review, CPQ, Revenue Cloud
, quoting, opportunity management, forecasting, pipeline governance |
| `salesforce-service-field-service-agent` | Service Cloud cases, entitlements, SLAs, omni-channel, Field Service Lightning
scheduling, knowledge governance |
| `salesforce-experience-cloud-agent` | Experience Cloud sites, guest-user access, community sharing rules, digital experience security posture |
| `salesforce-marketing-cloud-agent` | Marketing Cloud Engagement, Account Engagement (MCAE/Pardot)
, consent management, journey review, data extensions |
| `salesforce-agentforce-ai-agent` | Agentforce agent topics/actions, prompt templates, Prompt Builder, Einstein AI feature risk, autonomous action review
|
| `salesforce-analytics-tableau-agent` | CRM Analytics, Tableau, Einstein Discovery
, dashboard governance, KPI definitions, data lineage review |
| `salesforce-slack-collaboration-agent` | Slack integration with Salesforce, workflow automations, Slack Connect governance, notification design |
| `salesforce-industry-cloud-agent` | Router-to-vertical-counsel — routes to vertical counsel for Health Cloud, Financial Services Cloud, Education Cloud, Nonprofit Cloud, and others |
| `salesforce-compliance-privacy-agent` | Data residency, GDPR/CCPA obligations in Salesforce context, field-level PII mapping, retention, Shield encryption |

---

## Wave 3 — Infrastructure security and DevSecOps agents (10)

Ten specialized agents for Hyperforce infrastructure security, zero-trust controls,
and DevSecOps pipeline governance. These agents complement the Wave 1 platform
specialists with deeper infrastructure and security posture coverage.

| Agent | Summary |
|---|---|
| `salesforce-adaptive-access-agent` | Reviews contextual and risk-based access controls — Transaction Security policies, device trust, continuous authentication posture |
| `salesforce-certificate-lifecycle-agent` | Reviews Salesforce certificate and key management — self-signed and CA-signed certificates, expiry, rotation policy, Named Credential cert binding |
| `salesforce-change-impact-analyst-agent` | Performs adversarial pre-deployment change impact analysis — dependency graph, blast radius estimation, rollback path, regression risk |
| `salesforce-code-analyzer-orchestrator-agent` | Reviews and triages Salesforce Code Analyzer findings across PMD, ESLint, RetireJS, and Graph Engine rules for prioritization and remediation planning |
| `salesforce-continuous-verification-agent` | Reviews continuous identity and session verification controls — adaptive MFA, session timeout policy, high-assurance session enforcement |
| `salesforce-hyperforce-security-agent` | Reviews Hyperforce deployment security posture, data residency commitments, HIPA
controls, and infrastructure isolation posture |
| `salesforce-network-policy-architect-agent` | Reviews org-level network security policies — IP allowlisting, TLS enforcement, Connected App network restrictions, and perimeter control posture |
| `salesforce-sandbox-governance-agent` | Reviews sandbox data governance — PII masking strategy, Connected App scope in non-production environments, sandbox refresh cadence |
| `salesforce-sandbox-isolation-agent` | Reviews sandbox environment types, data isolation enforcement, production-data contamination risk, and environment topology |
| `salesforce-session-governance-agent` | Reviews session security settings, High Assurance session requirements, login hour restrictions, concurrent session limits, and timeout posture |

---

## Operating notes

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
- The `salesforce-maestro-agent` routes incoming matters to the right specialist(s)
  and coordinates multi-agent review for complex, cross-domain requests.
- Escalation-grade matters (guest-user exposure, Shield encryption decisions,
  cross-border data transfer, autonomous Agentforce action, and consent-mapping
  failures) are flagged immediately and routed to a qualified human owner.
- Cross-domain matters use the `salesforce-routing-protocol` and related
  cross-functional protocol skills; see `skills/cross-functional/`.
- **Verify before merge:** Salesforce product names, certification names, and
  Agentforce / Data Cloud terminology change frequently. All credential names and
  product terms in these agents are marked `
`.
  Confirm against [Salesforce Credentials](https://trailhead.salesforce.com/credentials/administratoroverview)
  and official release notes before treating any name as current.
- Never supply org credentials, session tokens, tenant IDs, customer data, PII,
  or any other secrets to these agents.
