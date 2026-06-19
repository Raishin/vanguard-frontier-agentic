# Safety checklist

Use this reference before any recommendation that involves restore operations, backup policy changes, backup offboarding, or any other Microsoft 365 Backup or BCDR configuration action.

## Non-negotiables

- Never initiate or approve restore operations, backup policy changes, or backup offboarding without explicit human confirmation and a documented rollback path.
- Never confuse Microsoft Purview retention policies with Microsoft 365 Backup — retention governs compliance holds, not fast point-in-time bulk recovery.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only Microsoft 365 admin center evidence or Microsoft Graph Backup API read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent backup policy scope, protection unit counts, active restore points, or tested RTO values.
- Require explicit user approval before recommending any restore operation — in-place same-URL restores overwrite all content and metadata since the restore point.
- Always confirm whether the restore target is same URL or new URL; same-URL restores are destructive and cannot be undone once initiated.
- Treat any workload with no active Microsoft 365 Backup policy as unprotected from a ransomware or large-scale data loss scenario until proven otherwise.

## Stress checks

- Which workloads have no active Microsoft 365 Backup policy and rely solely on native versioning, recycle bin, or retention?
- What is the current maximum data loss window (RPO) based on the most recent restore point?
- Has the restore workflow been tested or documented against the organization's stated RTO target?
- Is the difference between Microsoft Purview retention and Microsoft 365 Backup understood by the stakeholders making BCDR decisions?
- Does any third-party backup solution use an external-copy architecture that may not meet RTO for large tenants during a ransomware event?
- Is the 90-day offboarding grace period and multi-admin notification feature configured to prevent accidental backup loss?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual backup policy scope, protection unit coverage, tested recovery time, or ransomware recovery readiness.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Initiating any restore operation for Exchange Online mailboxes, SharePoint sites, or OneDrive accounts
- Creating, modifying, or removing Microsoft 365 Backup policies
- Offboarding from Microsoft 365 Backup (triggers 90-day grace period countdown)
- Changing backup policy scope (adding or removing protection units)
- Configuring or modifying multi-admin notification settings for backup operations
- Recommending a third-party backup solution architecture change that affects recovery capability
