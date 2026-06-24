# Official sources

Use this reference only when you need source grounding for Microsoft 365 tenant governance, admin roles, Microsoft Secure Score, GDAP, or Message Center service behavior.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Microsoft 365 tenant state:

- https://learn.microsoft.com/microsoft-365/admin/add-users/about-admin-roles
- https://learn.microsoft.com/defender-xdr/microsoft-secure-score
- https://learn.microsoft.com/partner-center/customers/gdap-introduction
- https://learn.microsoft.com/partner-center/customers/gdap-least-privileged-roles-by-task
- https://learn.microsoft.com/microsoft-365/admin/manage/message-center
- https://learn.microsoft.com/entra/identity/role-based-access-control/permissions-reference
- https://learn.microsoft.com/partner-center/customers/gdap-faq
- https://learn.microsoft.com/partner-center/customers/gdap-supported-workloads
- https://learn.microsoft.com/graph/api/resources/delegatedadminrelationships-api-overview
- https://learn.microsoft.com/microsoft-365/admin/manage/agent-roles-perms

## Grounding rule

Official documentation explains Microsoft 365 admin role and governance service behavior. It does not prove the user's current tenant admin role assignments, Secure Score posture, active GDAP relationships, or org-wide settings configuration. Prefer read-only Microsoft 365 admin center evidence, Microsoft Graph read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**Admin roles and RBAC (per learn.microsoft.com/microsoft-365/admin/add-users/about-admin-roles):**
- Microsoft recommends least-privilege: limit Global Administrator accounts to emergency scenarios
- Role-specific admins exist for Exchange, SharePoint, Teams, Security, Compliance, and other workloads
- License Administrator role is sufficient for license assignment tasks
- Security Administrator and Security Reader roles provide access to Microsoft Defender and Microsoft Purview portals
- AI Administrator role governs agent management in Microsoft 365 admin center

**Microsoft Secure Score (per learn.microsoft.com/defender-xdr/microsoft-secure-score):**
- Measures security posture across Microsoft Entra ID, Defender for Endpoint, Exchange Online, SharePoint Online, Teams, and other products
- Improvement actions are recommendations, not mandatory controls — scored against implemented state
- Security Administrator or higher has read-write access; Security Reader has read-only access
- Microsoft Defender XDR Unified RBAC supports custom roles for Secure Score with Exposure Management permissions
- Security defaults enable MFA-related Secure Score improvement actions automatically

**GDAP — Granular Delegated Admin Privileges (per learn.microsoft.com/partner-center/customers/gdap-introduction):**
- Replaces legacy Delegated Administrative Privileges (DAP) which granted standing Global Administrator access to partner tenants
- GDAP provides time-bound, task-scoped, least-privileged partner access following Zero Trust principles
- Customers must explicitly grant GDAP access; partners request specific Microsoft Entra roles for a defined time period
- Least-privileged role guidance per task available at gdap-least-privileged-roles-by-task
- Service support administrator is the minimum role for partner support ticket creation
- Microsoft 365 Lighthouse supports GDAP setup wizard and role recommendations for MSPs

**Message Center (per learn.microsoft.com/microsoft-365/admin/manage/message-center):**
- Central hub for Microsoft 365 service change notifications, planned maintenance, and advisory notices
- Service Support Administrator role is required to view and share Message Center posts
- Changes are categorized as Major updates, Plan for change, and Stay informed
- Integration with Microsoft Planner for change advisory board (CAB) workflows

**Common failure modes:**
- Standing Global Administrator accounts for day-to-day admin tasks instead of least-privileged workload-specific roles
- Active DAP relationships with partners not yet migrated to GDAP — blanket standing Global Administrator access
- No Message Center monitoring workflow — changes missed, tenant updates ungoverned
- Org-wide settings changed without change control or audit trail
- Microsoft Secure Score improvement actions ignored or not prioritized against governance risk

Review implications:
- Do not approve admin role assignments that use Global Administrator where a task-specific role suffices.
- GDAP relationships without time bounds or task scoping are equivalent to legacy DAP and should be flagged as critical.
- Documentation cannot prove the user's actual admin role inventory, GDAP relationship state, or Secure Score posture.
