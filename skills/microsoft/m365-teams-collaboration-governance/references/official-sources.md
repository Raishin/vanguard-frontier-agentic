# Official sources

Use this reference only when you need source grounding for Microsoft Teams governance, lifecycle management, external access, guest sharing, sensitivity labels on Teams and groups, meeting policies, messaging policies, app permission policies, or phone and voice governance service behavior.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Teams tenant state:

- https://learn.microsoft.com/microsoftteams/plan-teams-governance
- https://learn.microsoft.com/microsoftteams/plan-teams-lifecycle
- https://learn.microsoft.com/microsoftteams/guest-access
- https://learn.microsoft.com/purview/sensitivity-labels-teams-groups-sites
- https://learn.microsoft.com/purview/sensitivity-labels-meetings
- https://learn.microsoft.com/microsoftteams/meeting-templates-sensitivity-labels-policies
- https://learn.microsoft.com/microsoftteams/configure-meetings-three-tiers-protection
- https://learn.microsoft.com/entra/id-governance/entitlement-management-access-package-manage-lifecycle
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/ms-700

## Grounding rule

Official documentation explains Microsoft Teams and Microsoft 365 group governance service behavior. It does not prove the user's current Teams external access settings, guest sharing policies, sensitivity label assignments on teams, meeting policy configurations, app permission policy assignments, or expiration policy enforcement. Prefer read-only Teams admin center evidence, Microsoft Graph read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**Teams governance (per learn.microsoft.com/microsoftteams/plan-teams-governance):**
- Group and team creation controls: restrict creation to specific security groups, enforce naming policies (prefixes/suffixes, blocked words), require classification or sensitivity labels
- Expiration policies for Microsoft 365 groups — automatic renewal by active owners or deletion after inactivity period
- Archival and deletion — archived teams remain readable but not writable; deleted teams are recoverable for 30 days
- Guest access — per-tenant and per-team controls; guests can be added by team owners unless restricted

**Teams lifecycle (per learn.microsoft.com/microsoftteams/plan-teams-lifecycle):**
- Lifecycle roles: team owner (up to 100 per team), team member, guest (external email invited)
- Governance decisions before rollout: naming conventions, expiration policies, retention policies, guest access policy
- Microsoft Entra access reviews for groups with guest members — deny blocked from sign-in then account deleted after 30 days inactivity

**Guest access (per learn.microsoft.com/microsoftteams/guest-access):**
- Tenant-wide guest access toggle in Teams admin center — overrides per-team settings
- Guest capabilities: participate in channels, meetings, and chat; cannot access admin features or create teams
- Microsoft Entra entitlement management governs structured guest lifecycle — governed vs. ungoverned guest accounts
- Ungoverned guests remain in tenant indefinitely after last access package assignment expires unless explicitly removed

**Sensitivity labels on Teams and groups (per learn.microsoft.com/purview/sensitivity-labels-teams-groups-sites):**
- Labels can enforce: privacy (public/private), external user access, external sharing from labeled SharePoint sites, Conditional Access for labeled sites, private team discoverability, channel sharing controls
- Labels must be enabled for containers in Microsoft Purview before they appear in Teams group creation
- Channel meeting labels inherit from the group/site label when configured

**Meeting policies and sensitivity labels (per learn.microsoft.com/microsoftteams/meeting-templates-sensitivity-labels-policies):**
- Admin policies determine feature availability; sensitivity labels can enforce features even if admin policy is off
- Templates offer per-meeting flexibility within admin policy constraints; labels enforce specific settings
- End-to-end encryption, watermarking, and automatic recording can be enforced by sensitivity labels
- Three protection tiers: baseline, sensitive, highly sensitive — each with meeting template and label configuration

**Common failure modes:**
- No expiration policy on Microsoft 365 groups — teams accumulate indefinitely with no ownership verification
- Tenant-wide guest access enabled but no per-team review cadence for guest memberships
- Sensitivity labels not enabled for containers — no label governance on Teams or SharePoint sites
- App permission policies set to "Allow all apps" globally — no third-party app trust boundary
- Phone system configured without calling policy review — emergency calling not validated for all users
- Information barriers not implemented in regulated industries requiring communication segment restrictions

Review implications:
- Do not approve tenant-wide external access changes without blast-radius assessment and staged rollout.
- Guest access policies require both technical controls and periodic access review cadence to be effective.
- Documentation cannot prove the user's actual Teams external access settings, expiration policy coverage, or sensitivity label deployment state.
