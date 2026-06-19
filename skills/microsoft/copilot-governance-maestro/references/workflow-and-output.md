# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `copilot-readiness` | Copilot readiness, oversharing, SharePoint permissions, sensitivity labels, DSPM for AI, data access governance, restricted SharePoint search, restricted content discovery, Copilot exposure, Purview DLP for Copilot |
| `graph-data-exposure` | Graph exposure, Graph API, Microsoft Graph, oversharing, SharePoint Advanced Management, SAM, site access review, broken inheritance, EEEU, Everyone except external users, site ownership, SharePoint governance |
| `agent-governance` | Copilot Studio agent, agent governance, agent registry, Entra Agent ID, agent identity, connector permissions, agent publishing, agent lifecycle, ALM for agents, agent catalog, Teams app store approval |
| `plugin-connector-risk` | plugin, connector, connector action, Advanced Connector Policies, ACP, DLP for agents, HTTP connector, skill connector, channel connector, connector scope, API permission, token, connector governance |
| `copilot-studio-alm` | Copilot Studio ALM, agent environment, agent pipeline, dev environment, test environment, production agent, agent versioning, solution export, agent deployment, managed solution for agents |
| `identity-access` | Conditional Access, Entra, MFA, identity, access policy, Zero Trust identity, JEA, JIT, privileged access, Entra agent identity, Conditional Access for agents |
| `live-guard` | publish agent broadly, org-wide agent publishing, Teams app store publish, grant connector access, plugin access grant, requires human gate |

## Full routing table

### Copilot Readiness / Data Exposure

| Agent | Domain(s) | Use when… |
|---|---|---|
| `m365-copilot-readiness-data-exposure-governor` | copilot-readiness, graph-data-exposure | Assessing or remediating Copilot readiness: oversharing, data access governance reports, DSPM for AI, sensitivity labels, SharePoint Advanced Management, Purview DLP for Copilot |
| `purview-data-security-compliance-officer` | copilot-readiness, graph-data-exposure | Applying Microsoft Purview capabilities for Copilot: DLP policies, sensitivity labels, DSPM for AI, compliance manager, data lifecycle, eDiscovery for Copilot interactions |

### Agent Governance

| Agent | Domain(s) | Use when… |
|---|---|---|
| `copilot-studio-agent-governance-architect` | agent-governance, plugin-connector-risk, copilot-studio-alm | Designing or reviewing Copilot Studio agent governance: Entra Agent IDs, connector permissions, DLP for agents, advanced connector policies, agent lifecycle, publishing controls |
| `purview-data-security-compliance-officer` | agent-governance | Applying Purview audit logs, compliance policies, and retention to Copilot Studio agents and interactions |

### Plugin / Connector Risk

| Agent | Domain(s) | Use when… |
|---|---|---|
| `copilot-studio-agent-governance-architect` | plugin-connector-risk, agent-governance | Reviewing connector and plugin risk for Copilot Studio agents: ACP, DLP policy, connector scope, HTTP connector blocking, channel publishing restrictions |
| `entra-identity-conditional-access-architect` | plugin-connector-risk, identity-access | Reviewing Entra Conditional Access policies targeting agent identities, connector resource policies, or token issuance conditions for agent connectors |

### Copilot Studio ALM

| Agent | Domain(s) | Use when… |
|---|---|---|
| `copilot-studio-agent-governance-architect` | copilot-studio-alm, agent-governance | Designing or reviewing ALM for Copilot Studio agents: environment strategy, solution packaging, deployment pipelines, versioning, rollback |

### Identity and Access

| Agent | Domain(s) | Use when… |
|---|---|---|
| `entra-identity-conditional-access-architect` | identity-access, agent-governance | Reviewing Entra identity and access policies for Copilot scenarios: Conditional Access, MFA, JEA, Entra Agent IDs, scope review |
| `m365-copilot-readiness-data-exposure-governor` | identity-access, copilot-readiness | Reviewing JEA/JIT access scoping to prevent data oversharing through Copilot |

### Live-guard (ALWAYS requires human gate)

| Agent | Domain(s) | Use when… |
|---|---|---|
| `copilot-studio-agent-governance-architect` | live-guard, agent-governance | Publishing or broadly sharing a Copilot Studio agent (Teams app store, org-wide) — requires blast-radius assessment, rollback path, and explicit human confirmation |
| `copilot-studio-agent-governance-architect` | live-guard, plugin-connector-risk | Granting connector or plugin access to a Copilot Studio agent — requires blast-radius assessment, rollback path, and explicit human confirmation |

## Live-guard gate protocol

Before routing to any live-guard operation, surface all three and wait for explicit written confirmation:

1. **Blast-radius assessment** — what users, data, connectors, or systems are affected if this agent is published or connector access is granted without proper governance?
2. **Rollback path** — what is the tested rollback procedure (unpublish, revoke connector, block agent in registry) and estimated recovery time?
3. **Explicit confirmation** — "I confirm I understand the blast radius and rollback path. Proceed."

If the user cannot supply a rollback path, recommend routing to `copilot-studio-agent-governance-architect` to develop the rollback plan first.

## Response shape

Every Maestro response begins with the routing header:
```
Route: <agent-name(s)>
Reason: <one sentence>
Mode: <single | parallel (N specialists) | live-guard-gate>
```
Followed by: dispatched specialist output (summarized), then recommended next actions.
