# Official sources

Use this reference only when you need source grounding for Microsoft Copilot and Copilot Studio governance behavior or the detailed source list.

## Microsoft Copilot and Copilot Studio documentation

Use these as starting points, not as proof of the user's live tenant state:
- https://learn.microsoft.com/security/zero-trust/copilots/zero-trust-microsoft-365-copilot
- https://learn.microsoft.com/microsoft-365/copilot/secure-govern-copilot-foundational-deployment-guidance
- https://learn.microsoft.com/microsoft-365/copilot/configure-secure-governed-data-foundation-microsoft-365-copilot
- https://learn.microsoft.com/microsoft-copilot-studio/admin-data-loss-prevention
- https://learn.microsoft.com/microsoft-copilot-studio/guidance/sec-gov-phase2
- https://learn.microsoft.com/microsoft-365/copilot/security-microsoft-365-copilot
- https://learn.microsoft.com/microsoft-copilot-studio/requirements-certificates-configuration-values
- https://learn.microsoft.com/microsoft-365/copilot/extensibility/copilot-studio-experience

## Grounding rule

Official documentation explains Copilot governance service behavior. It does not prove the user's current tenant, SharePoint permissions state, sensitivity label coverage, Conditional Access policy configuration, or operational posture. Prefer read-only Microsoft 365 admin center evidence, Purview reports, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-16)

Service facts from official docs:
- The Microsoft 365 Copilot Zero Trust model requires 7 layers: data protection, identity and access, app protection, device management, threat protection, secure Teams collaboration, and user permissions to data.
- Oversharing is the primary Copilot risk: Copilot surfaces content users already have permission to access, so overshared or poorly governed content amplifies exposure. Remediation uses Microsoft Purview DSPM for AI and SharePoint Advanced Management.
- Copilot Studio agents receive Entra Agent IDs; connector API permissions attach to these identities and can be targeted by Entra Conditional Access policies. DLP and Advanced Connector Policies (ACP) gate what connectors agents can call at runtime.
- Publishing a Copilot Studio agent broadly (to Teams app store or org-wide) requires admin approval; this is a live-guard-gated action.
- Granting connector or plugin access to an agent expands its attack surface; each connector adds API permissions to the agent's Entra identity.

Review implications:
- Maestro routing should choose the narrowest Copilot governance specialist based on Zero Trust layer signals: data exposure, identity/access, agent governance, plugin/connector risk, or Copilot Studio ALM.
- Do not centralize decisions without citing the evidence source and routing rationale.
