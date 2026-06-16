# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right sub-maestro or specialist.

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `m365-governance` | M365 tenant governance, identity lifecycle, access reviews, entitlement management, PIM, Entra ID, Conditional Access, MFA, Intune, endpoint compliance, MDM, MAM |
| `m365-security-compliance` | Purview, sensitivity labels, DLP, records management, retention, eDiscovery, audit, insider risk, communications compliance, Defender XDR, Microsoft Sentinel |
| `m365-collaboration` | Teams, Exchange, SharePoint, OneDrive, Viva, collaboration governance, external sharing, guest access, information architecture |
| `m365-copilot` | M365 Copilot readiness, oversharing risk, Copilot Studio governance, data exposure, Copilot adoption |
| `m365-licensing` | M365 licensing, SKU comparison, license optimization, E3/E5, Teams licensing, Copilot licensing |
| `d365-erp` | Dynamics 365 Finance, Supply Chain Management, Business Central, Commerce, Human Resources, Project Operations, ERP, general ledger, accounts payable, accounts receivable, inventory |
| `d365-crm` | Dynamics 365 Sales, Customer Service, Field Service, Customer Insights, Marketing, Journeys, CRM, opportunity, case management, contact center |
| `d365-implementation` | Success by Design, solution blueprint, data migration, cutover, UAT, performance testing, go-live readiness, FastTrack, integration, dual-write |
| `power-platform` | Power Apps, Power Automate, Power BI, Dataverse, Power Pages, CoE, environment strategy, DLP policy, ALM, solution packaging, connectors |
| `copilot-governance` | Copilot Studio, agent governance, AI agent readiness, copilot oversharing, copilot security posture |

## Sub-maestro routing table

### M365 Surface

| Sub-Maestro | Domain(s) | Route when… |
|---|---|---|
| `m365-maestro-agent` | m365-governance, m365-security-compliance, m365-collaboration, m365-copilot, m365-licensing | Any M365 task: identity, governance, security, compliance, Teams, Exchange, SharePoint, OneDrive, Purview, Defender, Copilot readiness, licensing |

### D365 Surface

| Sub-Maestro | Domain(s) | Route when… |
|---|---|---|
| `d365-maestro-agent` | d365-erp, d365-crm, d365-implementation | Any Dynamics 365 task: Finance, Supply Chain, Business Central, Sales, Customer Service, Field Service, Customer Insights, implementation, data migration, integration, go-live |

### Power Platform Surface

| Sub-Maestro / Agent | Domain(s) | Route when… |
|---|---|---|
| `power-platform-maestro-agent` | power-platform | Power Apps, Power Automate, Dataverse, Power BI, CoE, environment strategy, DLP, ALM |

### Copilot Governance Surface

| Sub-Maestro / Agent | Domain(s) | Route when… |
|---|---|---|
| `copilot-governance-maestro-agent` | copilot-governance | Copilot Studio governance, agent readiness, oversharing risk, AI agent security posture |

## Cross-cloud deflection table

| Incoming request | Action |
|---|---|
| Azure IaaS, VMs, AKS, VNets, storage accounts, Azure Kubernetes, Azure networking | REFUSE. Tell user to use `azure-maestro`. |
| Generic cloud infrastructure not M365/D365/Power Platform | REFUSE. Identify the right cloud maestro and redirect. |
| Hybrid tasks mixing M365 admin and Azure AD (Entra ID) infrastructure | Route the M365 portion to `m365-maestro-agent`; flag any Azure IaaS portion for `azure-maestro`. |

## Live-guard gate protocol

Before routing to any live-guard agent, surface all three and wait for explicit written confirmation:

1. **Blast-radius assessment** — what tenant resources, users, policies, or environments are affected if this goes wrong?
2. **Rollback path** — what is the tested rollback procedure and estimated recovery time?
3. **Explicit confirmation** — "I confirm I understand the blast radius and rollback path. Proceed."

## Response shape

Every Maestro response begins with the routing header:
```
Route: <sub-maestro or agent name(s)>
Reason: <one sentence>
Mode: <single | parallel (N specialists) | live-guard-gate>
```
Followed by: dispatched specialist output (summarized), then recommended next actions.
