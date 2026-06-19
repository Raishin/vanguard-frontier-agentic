# Official sources

Use this reference only when you need source grounding for Microsoft 365 Copilot readiness and data governance service behavior or the detailed source list.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Microsoft 365 tenant state:

- https://learn.microsoft.com/security/zero-trust/copilots/zero-trust-microsoft-365-copilot
- https://learn.microsoft.com/microsoft-365/copilot/secure-govern-copilot-foundational-deployment-guidance
- https://learn.microsoft.com/microsoft-365/copilot/configure-secure-governed-data-foundation-microsoft-365-copilot
- https://learn.microsoft.com/en-us/sharepoint/advanced-management
- https://learn.microsoft.com/en-us/sharepoint/get-ready-copilot-sharepoint-advanced-management
- https://learn.microsoft.com/en-us/purview/ai-microsoft-purview
- https://learn.microsoft.com/en-us/purview/data-security-posture-management-learn-about
- https://learn.microsoft.com/en-us/microsoft-365-copilot/microsoft-365-copilot-blueprint-oversharing
- https://learn.microsoft.com/en-us/microsoft-365-copilot/microsoft-365-copilot-architecture-data-protection-auditing
- https://learn.microsoft.com/en-us/purview/dlp-microsoft365-copilot-location-learn-about

## Grounding rule

Official documentation explains Microsoft 365 and Purview service behavior. It does not prove the user's current tenant configuration, sensitivity label coverage, sharing link state, connector grants, or actual Copilot enablement status. Prefer read-only Microsoft 365 Admin Center or Graph API evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-16)

Key service facts from official Microsoft Learn documentation:

**Zero Trust 7-layer model for Microsoft 365 Copilot (per learn.microsoft.com/security/zero-trust/copilots/zero-trust-microsoft-365-copilot):**
1. Data protection — Microsoft Purview sensitivity labels, DLP policies, DSPM for AI, oversharing controls
2. Identity and access — Microsoft Entra MFA, Conditional Access common policies, access reviews
3. App protection — Microsoft Intune app protection policies, approved client apps
4. Device management and protection — Intune device compliance, Defender for Endpoint
5. Threat protection — Microsoft Defender XDR, Defender for Office 365, EOP
6. Secure collaboration with Teams — Teams sharing policies, guest access controls, channel governance
7. User permissions to data — JEA/JIT, site access reviews, EEEU removal, SharePoint Advanced Management

**Oversharing controls (per Microsoft Learn):**
- Restricted SharePoint Search: temporarily limit Copilot search to an approved site list
- Restricted Content Discovery (SAM): exclude sensitive sites from Copilot and org-wide search
- Data Access Governance reports: identify sites with potentially overshared data or sensitive content
- SAM Content Management Assessment: identify oversized audiences, EEEU usage, broken inheritance, inactive/ownerless sites
- Microsoft Purview DSPM for AI: one-click policies, data risk assessments, AI regulatory compliance

**Microsoft Purview DSPM for AI capabilities:**
- Fix oversharing issues identified through default data risk assessment
- Create default sensitivity label sets
- Create DLP policies
- Detect risky AI interactions
- Guidance for AI regulations via Compliance Manager
- Secure interactions for Copilot experiences

**Everyone Except External Users (EEEU) risk:**
- EEEU grants access to all internal users including guests
- Must be disabled at tenant level before Copilot enablement for high-sensitivity environments
- SAM site access reviews can enforce removal at the site level

Review implications:
- Do not approve Copilot enablement from intent alone. Require evidence of oversharing assessment, DSPM for AI review, sensitivity label coverage, DLP policy scope, SAM controls in place, and EEEU audit completed.
- Documentation cannot prove the user's actual tenant configuration, label coverage, or sharing state.
