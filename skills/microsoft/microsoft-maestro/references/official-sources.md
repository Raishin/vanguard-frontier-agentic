# Official sources

Use this reference only when you need source grounding for Microsoft SaaS service behavior or the detailed source list.

## Microsoft documentation

Use these as starting points, not as proof of the user's live tenant state:
- https://learn.microsoft.com/microsoft-365/admin/admin-overview/admin-center-overview
- https://learn.microsoft.com/compliance/assurance/assurance-governance
- https://learn.microsoft.com/microsoft-365/community/microsoft365-maturity-model--governance-and-compliance
- https://learn.microsoft.com/dynamics365/guidance/overview
- https://learn.microsoft.com/power-platform/guidance/adoption/admin-best-practices

## Grounding rule

Official documentation explains Microsoft SaaS service behavior. It does not prove the user's current tenant, license tier, configuration, identity boundary, or operational state. Prefer read-only Microsoft Graph evidence, admin center evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-16)

Service facts from official docs:
- The Microsoft 365 admin center is the central entry point for managing M365 users, billing, service health, licensing, and security. Specialist workspaces (Security, Compliance, Exchange, SharePoint, Teams, Entra ID) provide deeper control.
- Microsoft's security governance program covers M365, Azure, and Dynamics 365 via the Microsoft Security Policy (MSP) and aligns with GDPR, ISO 27001, HIPAA, and SOC 2.
- The M365 Maturity Model for Governance, Risk, and Compliance defines five maturity levels covering information protection, information governance, insider risk management, and eDiscovery.
- Power Platform governance covers environment strategy, data loss prevention (DLP) policies, managed governance via environment groups, and CoE Starter Kit adoption.
- Dynamics 365 guidance is organized under the Success by Design framework: Discover, Initiate, Implement, Prepare, Operate.

Review implications:
- Microsoft Maestro routing should choose the narrowest sub-maestro based on domain evidence: M365 governance/identity/security, D365 ERP/CRM, Power Platform low-code, or Copilot governance.
- Do not centralize decisions without citing the evidence source and routing rationale.
- Cross-cloud deflection is mandatory: Azure IaaS tasks must be refused and redirected to azure-maestro.
