# Safety checklist

Use this reference before any recommendation that changes tenant-wide SharePoint or OneDrive sharing settings, retention policies, litigation or eDiscovery holds, Restricted Content Discovery settings, Restricted Access Control policies, site lifecycle policies, or Exchange Online mailbox configuration affecting data preservation.

## Non-negotiables

- Never recommend weakening tenant-wide sharing policies, removing retention holds, or disabling Restricted Content Discovery to accelerate Copilot deployment, reduce friction, or unblock delivery. State this refusal plainly.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only SharePoint admin center, Exchange admin center, or Microsoft Graph read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent SharePoint sharing settings, site ownership coverage, RCD deployment state, retention policy assignments, or litigation hold coverage.
- Require explicit user approval before recommending tenant-wide sharing policy changes, retention policy creation or modification, litigation or eDiscovery hold changes, site lifecycle policy activation, or RCD/RAC policy deployment to production sites.
- Keep remediation least-privilege, reversible, staged (simulation mode before active, pilot sites before org-wide), and scoped to the requested site or workload boundary.
- Treat any tenant with SharePoint sharing set to "Anyone" and no Anyone link expiration as high risk for unauthenticated data exposure.
- Treat any high-risk site (sensitive data, EEEU or Anyone access, no owner) as a Copilot readiness blocker until protected by RCD, RAC, or permission remediation.
- Treat any mailbox or site under legal obligation that lacks a litigation hold or retention policy as a critical compliance gap.

## Stress checks

- What sharing configuration allows unauthenticated access (Anyone links) or org-wide access (EEEU) to sensitive site content without expiration or permission review?
- What high-risk site will surface unintended sensitive content in Microsoft 365 Copilot Business Chat because RCD has not been applied?
- What inactive or orphaned site holds sensitive data with no active owner, no lifecycle policy, and no attestation requirement?
- What workload (Exchange Online, SharePoint Online, Teams messages) has no applicable Microsoft Purview retention policy — creating eDiscovery or regulatory compliance gaps?
- What mailbox belonging to a departed employee lacks litigation hold or inactive mailbox policy — allowing content to be purged before legal hold expires?
- What rollback path exists if a tenant-wide sharing restriction or RCD policy breaks existing business workflows relying on anonymous or org-wide sharing?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live SharePoint sharing settings, RCD deployment coverage, retention policy assignments, litigation hold state, or inactive mailbox policy coverage.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Changing tenant-wide SharePoint or OneDrive sharing settings (especially relaxing from current level)
- Creating, modifying, or removing Microsoft Purview retention policies or retention labels affecting production content
- Adding, modifying, or releasing litigation holds or eDiscovery holds on mailboxes or sites
- Enabling, modifying, or disabling Restricted Content Discovery settings on production sites
- Enabling, modifying, or disabling Restricted Access Control policies on production sites
- Enabling or modifying site lifecycle management policies in active mode (moving from simulation to active)
- Purging content from inactive mailboxes or deleting SharePoint site collections with content under hold
