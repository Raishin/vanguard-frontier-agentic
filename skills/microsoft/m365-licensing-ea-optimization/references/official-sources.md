# Official sources

Use this reference only when you need source grounding for Microsoft 365 licensing plans, group-based licensing, EA/CSP/MCA contract characteristics, or license assignment hygiene.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Microsoft 365 license assignment state or contract terms:

- https://learn.microsoft.com/microsoft-365/admin/manage/manage-group-licenses
- https://learn.microsoft.com/entra/identity/users/licensing-admin-center
- https://learn.microsoft.com/microsoft-365/commerce/licenses/manage-volume-licensing
- https://learn.microsoft.com/entra/fundamentals/licensing
- https://learn.microsoft.com/microsoft-365/enterprise/assign-licenses-to-user-accounts
- https://learn.microsoft.com/microsoft-365/enterprise/view-licenses-and-services-with-microsoft-365-powershell
- https://learn.microsoft.com/entra/fundamentals/licensing-groups-resolve-problems
- https://learn.microsoft.com/microsoft-365/commerce/licenses/e3-extra-features-licenses
- https://learn.microsoft.com/azure/cost-management-billing/microsoft-customer-agreement/onboard-microsoft-customer-agreement
- https://learn.microsoft.com/entra/identity/users/licensing-group-advanced

## Grounding rule

Official documentation explains Microsoft 365 licensing plan capabilities and assignment mechanics. It does not prove the user's current license assignment state, actual contract pricing, or renewal terms. Prefer read-only Microsoft 365 admin center evidence, Microsoft Graph license API read output, repository evidence, or sanitized user-provided evidence for current-state claims. Never derive or imply contract pricing from documentation.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**Group-based licensing (per learn.microsoft.com/microsoft-365/admin/manage/manage-group-licenses):**
- Security groups, mail-enabled groups, and Microsoft 365 groups can be used for license assignment
- Maximum of 20 groups assignable at once in Microsoft 365 admin center; API and PowerShell have no such UI limit
- Nested groups are not supported — only first-level group members receive licenses
- Users without a usage location inherit tenant location; always set user location before assignment
- Move users to new group before removing from old group to prevent temporary service loss
- License Administrator role is required for group-based licensing in Microsoft 365 admin center; Group Administrators can use API and PowerShell
- Group-based licensing is available with Microsoft Entra ID P1 or higher (included in E3, E5, F-SKUs, Business Premium)

**Microsoft Entra licensing tiers (per learn.microsoft.com/entra/fundamentals/licensing):**
- Microsoft Entra ID Free: included with Microsoft cloud subscriptions
- Microsoft Entra ID P1: included in Microsoft 365 E3, F1, F3, Enterprise Mobility + Security E3, Business Premium
- Microsoft Entra ID P2: included in Microsoft 365 E5, Microsoft Defender Suite, Enterprise Mobility + Security E5 — adds Identity Protection, Privileged Identity Management, and access reviews
- Microsoft Entra Suite: combines Private Access, Internet Access, ID Governance, ID Protection, Verified ID premium

**SKU landscape:**
- Microsoft 365 E3: Office 365 E3 + Enterprise Mobility + Security E3 + Windows E3 (via EA)
- Microsoft 365 E5: adds E5 Security, E5 Compliance, E5 Voice — includes Microsoft Entra ID P2, Microsoft Defender suite, Microsoft Purview advanced features
- Microsoft 365 F1/F3: Firstline Worker plans for shift workers — reduced feature set, lower per-seat cost
- E3 Extra Features and E5 Extra Features: additional capabilities for EA customers (Avatars for Teams, Windows Autopatch, Customer Lockbox for E5, etc.)
- Add-ons: Microsoft 365 Copilot, Microsoft Defender for Endpoint, Microsoft Purview add-ons — required when base SKU does not include needed capability

**EA, CSP, MCA contract awareness (advisory context only):**
- Enterprise Agreement (EA): designed for 500+ user organizations, annual true-up cycle, 3-year term, volume discount
- Cloud Solution Provider (CSP): partner-managed, monthly flexibility, no long-term commitment
- Microsoft Customer Agreement (MCA): simplified digital agreement, pay-as-you-go, no expiry, automated processing
- Volume Licensing Service Center (VLSC) retired April 2024; VL management moved to Microsoft 365 admin center

**Common failure modes:**
- Manual per-user license assignment at scale — high operational overhead, error-prone, no automatic de-provisioning
- Group-based licensing without usage location set — assignment failures for users without location
- Nested security groups used for license assignment — licenses not inherited by nested members
- E5 assigned to users who only need E3 capabilities — over-assignment cost without capability utilization
- Add-on licenses purchased without confirming base SKU includes prerequisite (e.g., Microsoft Entra ID P1 required for some add-ons)
- EA true-up not tracked — surprise overage costs at annual reconciliation

Review implications:
- Do not recommend removing licenses without confirming the user is inactive or the service is not depended upon — service interruption risk.
- Never commit to savings estimates without the customer's actual contract pricing data.
- Documentation cannot prove the user's actual assigned license counts, group membership, or contract terms.
