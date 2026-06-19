# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `environment-strategy` | environment, environment type, Managed Environment, default environment, environment routing, environment groups, environment rules, developer environment, sandbox, production environment, tenant strategy, multi-environment, environment lifecycle |
| `dlp-policy` | DLP, data loss prevention, connector classification, business data, blocked connector, tenant-wide policy, connector policy, custom connector, endpoint filtering, data policy, connector governance |
| `dataverse-security` | Dataverse, security role, business unit, field-level security, row-level security, privilege, permission, record access, team, owner team, group team, table permission, column security |
| `alm-pipelines` | ALM, pipeline, deployment, solution, managed solution, unmanaged solution, source control, DevOps, CI/CD, promote, stage, test environment, production deployment, solution checker, pipelines in Power Platform |
| `connector-risk` | connector, custom connector, HTTP connector, Power Automate connector, skill connector, channel connector, Teams connector, SharePoint connector, unauthenticated connector, connector action, advanced connector policy, ACP |
| `citizen-dev-guardrails` | citizen developer, maker, low-code, governance, CoE, Center of Excellence, app catalog, sharing, app sharing, flow sharing, Managed Environments sharing limits, maker portal, default environment security |
| `live-guard` | deploy to production, production deployment, tenant-wide DLP change, DLP policy tenant scope, production environment change, requires human gate |

## Full routing table

### Environment Strategy

| Agent | Domain(s) | Use when… |
|---|---|---|
| `power-platform-solution-architect-agent` | environment-strategy | Designing or reviewing a Power Platform tenant environment strategy, multi-environment topology, or governance framework |
| `power-platform-governance-environment-strategy-lead` | environment-strategy, citizen-dev-guardrails | Leading environment strategy decisions, defining environment types, Managed Environments, environment groups and rules, default environment security |

### DLP Policy

| Agent | Domain(s) | Use when… |
|---|---|---|
| `power-platform-governance-environment-strategy-lead` | dlp-policy, environment-strategy | Designing or reviewing DLP policy strategy, connector classification tiers, tenant-level vs environment-level policy scope |
| `power-automate-automation-risk-reviewer` | dlp-policy, connector-risk | Reviewing DLP policy impact on Power Automate flows, connector risk in existing automations, or evaluating connector change risk |

### Dataverse Security

| Agent | Domain(s) | Use when… |
|---|---|---|
| `dataverse-security-model-architect` | dataverse-security | Designing or reviewing Dataverse security roles, business unit hierarchy, field-level security, row-level security, or privilege scope |
| `power-platform-solution-architect-agent` | dataverse-security, environment-strategy | Reviewing cross-environment Dataverse security design as part of a broader solution architecture |

### ALM / Pipelines

| Agent | Domain(s) | Use when… |
|---|---|---|
| `power-platform-alm-pipelines-engineer` | alm-pipelines | Designing, implementing, or troubleshooting Power Platform ALM pipelines, solution promotion, solution checker, or DevOps integration |
| `power-platform-solution-architect-agent` | alm-pipelines, environment-strategy | Reviewing ALM pipeline topology as part of a broader multi-environment architecture |

### Connector Risk

| Agent | Domain(s) | Use when… |
|---|---|---|
| `power-automate-automation-risk-reviewer` | connector-risk, dlp-policy | Reviewing connector risk in Power Automate flows, assessing impact of connector changes, or evaluating unauthenticated/HTTP connector usage |
| `power-platform-governance-environment-strategy-lead` | connector-risk, dlp-policy | Reviewing tenant-wide connector governance policy, advanced connector policies (ACP), or channel publishing restrictions in Copilot Studio |

### Citizen Dev Guardrails

| Agent | Domain(s) | Use when… |
|---|---|---|
| `power-platform-governance-environment-strategy-lead` | citizen-dev-guardrails, environment-strategy | Establishing citizen developer guardrails, CoE strategy, sharing limits, app catalog governance, or maker onboarding controls |
| `power-platform-solution-architect-agent` | citizen-dev-guardrails | Reviewing low-code governance as part of a broader platform solution design |

### Live-guard (ALWAYS requires human gate)

| Agent | Domain(s) | Use when… |
|---|---|---|
| `power-platform-alm-pipelines-engineer` | live-guard, alm-pipelines | Deploying a solution to a PRODUCTION environment via pipeline — requires blast-radius assessment, rollback path, and explicit human confirmation |
| `power-platform-governance-environment-strategy-lead` | live-guard, dlp-policy | Applying a tenant-wide DLP policy change — requires blast-radius assessment, rollback path, and explicit human confirmation |

## Live-guard gate protocol

Before routing to any live-guard operation, surface all three and wait for explicit written confirmation:

1. **Blast-radius assessment** — what environments, flows, apps, or users are affected if this goes wrong?
2. **Rollback path** — what is the tested rollback procedure and estimated recovery time?
3. **Explicit confirmation** — "I confirm I understand the blast radius and rollback path. Proceed."

If the user cannot supply a rollback path, recommend routing to `power-platform-solution-architect-agent` first to develop the rollback plan.

## Response shape

Every Maestro response begins with the routing header:
```
Route: <agent-name(s)>
Reason: <one sentence>
Mode: <single | parallel (N specialists) | live-guard-gate>
```
Followed by: dispatched specialist output (summarized), then recommended next actions.
