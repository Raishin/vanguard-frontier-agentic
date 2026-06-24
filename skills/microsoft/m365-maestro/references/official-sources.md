# Official sources

Use this reference only when you need source grounding for M365 service behavior or the detailed source list.

## Microsoft 365 documentation

Use these as starting points, not as proof of the user's live tenant state:
- https://learn.microsoft.com/microsoft-365/admin/admin-overview/admin-center-overview
- https://learn.microsoft.com/microsoft-365/community/microsoft365-maturity-model--governance-and-compliance
- https://learn.microsoft.com/microsoft-365/education/guide/4-advanced/identity/advanced-identity-governance
- https://learn.microsoft.com/microsoft-365-apps/security/compliance-overview
- https://learn.microsoft.com/windows-365/agents/security-overview

## Grounding rule

Official documentation explains M365 service behavior. It does not prove the user's current tenant, license tier, Conditional Access configuration, sensitivity label policy, or operational state. Prefer read-only Microsoft Graph evidence, admin center evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-16)

Service facts from official docs:
- Identity Governance in Microsoft Entra ID enables governance of the identity lifecycle, access lifecycle, and privileged access. Tools include access reviews, entitlement management, PIM, and lifecycle workflows.
- The M365 Maturity Model for Governance, Risk, and Compliance covers information protection (sensitivity labels, DLP, message encryption), information governance (records management, retention), insider risk management, and eDiscovery/Audit.
- M365 compliance certifications include GDPR, ISO 27001, HIPAA, and SOC 2 Type 2. Microsoft Purview provides data governance, Microsoft Entra ID provides identity management, Microsoft Intune enforces device compliance and Conditional Access.
- Windows 365 for Agents is built on a security-first architecture: each Cloud PC is Entra-joined and Intune-enrolled. Microsoft Defender provides threat protection, Microsoft Purview provides data governance and compliance visibility.

Review implications:
- M365 Maestro routing should choose the narrowest specialist based on domain evidence: identity/governance, endpoint, Teams/collaboration, Exchange/SharePoint/OneDrive, Purview/compliance, Defender XDR, Copilot readiness, or licensing.
- Do not centralize decisions without citing the evidence source and routing rationale.
