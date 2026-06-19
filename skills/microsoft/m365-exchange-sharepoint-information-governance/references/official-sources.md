# Official sources

Use this reference only when you need source grounding for Exchange Online and SharePoint Online information governance, SharePoint Advanced Management, external and anonymous sharing controls, Microsoft Purview retention and records management, or Microsoft 365 Copilot oversharing readiness service behavior.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Exchange or SharePoint tenant state:

- https://learn.microsoft.com/sharepoint/advanced-management
- https://learn.microsoft.com/sharepoint/restricted-content-discovery
- https://learn.microsoft.com/sharepoint/data-access-governance-reports
- https://learn.microsoft.com/sharepoint/get-ready-copilot-sharepoint-advanced-management
- https://learn.microsoft.com/sharepoint/turn-external-sharing-on-or-off
- https://learn.microsoft.com/sharepoint/restricted-access-control
- https://learn.microsoft.com/sharepoint/site-lifecycle-management
- https://learn.microsoft.com/purview/retention
- https://learn.microsoft.com/purview/enable-archive-mailboxes
- https://learn.microsoft.com/training/paths/explore-data-governance-microsoft-365/

## Grounding rule

Official documentation explains SharePoint Online, Exchange Online, and Microsoft Purview governance service behavior. It does not prove the user's current sharing settings, site ownership coverage, Restricted Content Discovery enablement, retention policy assignments, or litigation hold state. Prefer read-only SharePoint admin center evidence, Exchange admin center evidence, Microsoft Graph read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**SharePoint Advanced Management (per learn.microsoft.com/sharepoint/advanced-management):**
- Layered oversharing controls: Conditional Access policies (authentication context), Restricted Access Control (RAC) limiting site access to specified groups, Restricted Content Discovery (RCD) preventing high-risk sites from surfacing in Copilot and org-wide search, and block download policies
- Data access governance (DAG) reports: permission state reports, sharing link activity reports, sensitivity label snapshot reports, EEEU (Everyone Except External Users) insights — identify sites with broadest exposure
- Site access reviews: delegate remediation to site owners; initiate from DAG reports
- Site lifecycle management: inactive site policies (simulation and active modes), site attestation, and Microsoft 365 Archive for stale content

**Restricted Content Discovery (per learn.microsoft.com/sharepoint/restricted-content-discovery):**
- Site-level setting that prevents sites from surfacing in org-wide search and Microsoft 365 Copilot Business Chat (unless user had recent interaction)
- Requires at least one user in the org to be assigned a Copilot license; requires SharePoint Advanced Management prerequisites
- Does not affect existing permissions — users with access can still open files directly
- Cannot be applied to OneDrive sites; overuse degrades search and Copilot grounding quality

**External sharing controls (per learn.microsoft.com/sharepoint/turn-external-sharing-on-or-off):**
- Tenant-level settings range from "Anyone" (most permissive) to "Only people in your organization" (most restrictive)
- Site-level settings can be more restrictive than tenant-level but cannot exceed the tenant maximum
- Anyone links: unauthenticated sharing — expiration and permission controls are the primary mitigation
- EEEU (Everyone Except External Users): all internal users including guests — key oversharing vector for Copilot readiness

**Microsoft Purview retention (per learn.microsoft.com/purview/retention):**
- Retention policies apply to Exchange mailboxes, SharePoint sites, OneDrive accounts, Teams messages, and other workloads
- Retention labels enable record declaration, event-based retention, and item-level retention independent of policy
- Adaptive policy scopes — dynamically include users, sites, or groups based on attributes
- Litigation hold and eDiscovery hold preserve content in the Recoverable Items folder even if deleted by users
- Modern recommendation: use Microsoft Purview retention policies and labels; messaging records management (MRM) is legacy but still supported for archive mailbox movement

**Common failure modes:**
- Tenant-level SharePoint sharing set to "Anyone" with no expiration on Anyone links — unauthenticated sharing at scale
- EEEU (Everyone Except External Users) permissions on sensitive sites — entire internal user base including guests can access
- No site lifecycle policy — inactive sites accumulate with orphaned permissions and no owner to attest
- Restricted Content Discovery not applied to high-risk sites before Copilot deployment — sensitive content surfacing in Copilot responses
- No retention policy covering SharePoint Online or Exchange Online — compliance gap and eDiscovery risk
- Archive mailboxes not enabled — users with growing mailboxes hit quota limits; eDiscovery coverage gaps

Review implications:
- Do not approve tenant-wide sharing policy relaxation without blast-radius assessment and EEEU/Anyone link inventory.
- Restricted Content Discovery is a bridge control — it does not replace proper permissions remediation for high-risk sites.
- Documentation cannot prove the user's actual sharing settings, RCD deployment coverage, retention policy assignments, or litigation hold state.
