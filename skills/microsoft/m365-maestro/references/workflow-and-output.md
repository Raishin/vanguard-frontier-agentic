# Routing table and domain taxonomy

Use this reference when classifying an M365 task or selecting the right specialist.

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `identity-governance` | Entra ID, identity lifecycle, access reviews, entitlement management, PIM, SSPR, lifecycle workflows, identity governance, provisioning, HR-driven provisioning |
| `conditional-access` | Conditional Access policies, named locations, MFA enforcement, sign-in risk, device compliance, grant controls, session controls, authentication strengths |
| `endpoint` | Intune, MDM, MAM, device enrollment, compliance policies, configuration profiles, app protection, Windows Autopilot, iOS/Android management |
| `teams-collaboration` | Microsoft Teams, channels, meetings, live events, Teams governance, external access, guest access, Teams voice, Direct Routing, Teams Rooms |
| `exchange-sharepoint-onedrive` | Exchange Online, mailbox management, shared mailboxes, mail flow rules, SharePoint Online, site collections, OneDrive for Business, external sharing, information architecture |
| `purview-compliance` | Microsoft Purview, sensitivity labels, DLP, records management, retention policies, eDiscovery, audit, insider risk management, communications compliance, compliance manager |
| `defender-xdr` | Microsoft Defender XDR, Defender for Identity, Defender for Office 365, Defender for Endpoint, Defender for Cloud Apps, Microsoft Sentinel, threat hunting, incident response |
| `copilot-readiness` | M365 Copilot readiness, oversharing risk, data exposure assessment, Copilot Studio governance, Copilot adoption, SharePoint permissions hygiene |
| `licensing` | M365 licensing, E3 vs E5, Copilot add-on, Teams licensing, Frontline worker licensing, license optimization, license assignment, cost analysis |
| `tenant-governance` | Tenant configuration, admin roles, delegated administration, Microsoft 365 admin center, service health, message center, change management |

## Full routing table

### Identity and Governance

| Agent | Domain(s) | Use when… |
|---|---|---|
| `m365-tenant-governance-architect` | tenant-governance | Designing or reviewing M365 tenant architecture, admin role structure, delegated administration, or multi-tenant governance |
| `entra-identity-conditional-access-architect` | identity-governance, conditional-access | Designing or reviewing Entra ID identity governance, access reviews, PIM, entitlement management, or Conditional Access policies |

### Endpoint

| Agent | Domain(s) | Use when… |
|---|---|---|
| `intune-endpoint-administrator-agent` | endpoint | Managing Intune device enrollment, compliance policies, configuration profiles, app protection, or Windows Autopilot |

### Collaboration

| Agent | Domain(s) | Use when… |
|---|---|---|
| `teams-collaboration-communications-architect` | teams-collaboration | Designing or governing Microsoft Teams, channels, guest access, external access, Teams voice, or Teams Rooms |
| `exchange-sharepoint-onedrive-information-steward` | exchange-sharepoint-onedrive | Managing Exchange Online, SharePoint, OneDrive, mail flow, external sharing, or information architecture |

### Compliance and Security

| Agent | Domain(s) | Use when… |
|---|---|---|
| `purview-data-security-compliance-officer` | purview-compliance | Designing or operating Microsoft Purview: sensitivity labels, DLP, retention, records management, eDiscovery, or compliance manager |
| `defender-xdr-security-operations-analyst` | defender-xdr | Investigating threats, incidents, or alerts across Defender XDR, Defender for Office 365, Defender for Identity, or Microsoft Sentinel |

### Copilot and Licensing

| Agent | Domain(s) | Use when… |
|---|---|---|
| `m365-copilot-readiness-data-exposure-governor` | copilot-readiness | Assessing M365 Copilot readiness, auditing oversharing or data exposure risk, or governing Copilot Studio agents |
| `m365-licensing-value-realization-analyst` | licensing | Reviewing M365 licensing posture, optimizing SKU assignments, evaluating E3/E5 delta, or planning Copilot licensing |

### Adoption and Change

| Agent | Domain(s) | Use when… |
|---|---|---|
| `m365-adoption-change-enablement-lead` | tenant-governance | Planning or executing M365 adoption programs, change management, training strategy, or end-user enablement |

## Live-guard gate protocol

Before routing to any live-guard agent (Conditional Access changes, MFA enforcement, mailbox or SharePoint sharing policy changes), surface all three and wait for explicit written confirmation:

1. **Blast-radius assessment** — what users, policies, or access paths are affected if this goes wrong?
2. **Rollback path** — what is the tested rollback procedure and estimated recovery time?
3. **Explicit confirmation** — "I confirm I understand the blast radius and rollback path. Proceed."

## Response shape

Every Maestro response begins with the routing header:
```
Route: <agent-name(s)>
Reason: <one sentence>
Mode: <single | parallel (N specialists) | live-guard-gate>
```
Followed by: dispatched specialist output (summarized), then recommended next actions.
