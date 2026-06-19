# Official sources

Use this reference only when you need source grounding for Microsoft 365 Backup, ransomware recovery, data resiliency, or BCDR service behavior.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Microsoft 365 backup configuration or recovery readiness:

- https://learn.microsoft.com/microsoft-365/backup/backup-overview
- https://learn.microsoft.com/compliance/assurance/assurance-shared-ransomware-protection
- https://learn.microsoft.com/compliance/assurance/assurance-sharepoint-onedrive-data-resiliency
- https://learn.microsoft.com/microsoft-365/backup/backup-view-edit-policies
- https://learn.microsoft.com/microsoft-365/security/office-365-security/recover-from-ransomware
- https://learn.microsoft.com/compliance/assurance/assurance-exchange-data-resiliency
- https://learn.microsoft.com/troubleshoot/sharepoint/security/handling-ransomware-in-sharepoint-online
- https://learn.microsoft.com/graph/api/resources/backuprestoreroot
- https://learn.microsoft.com/microsoft-365/backup/backup-offboarding
- https://learn.microsoft.com/defender-xdr/playbook-responding-ransomware-m365-defender

## Grounding rule

Official documentation explains Microsoft 365 Backup and data resiliency service behavior. It does not prove the user's current backup policy scope, protection unit count, active restore points, or tested RTO. Prefer read-only Microsoft 365 admin center evidence, Microsoft Graph Backup API read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**Microsoft 365 Backup overview (per learn.microsoft.com/microsoft-365/backup/backup-overview):**
- Covers Exchange Online mailboxes, SharePoint sites, and OneDrive accounts
- Retention period: 1 year for all three workloads
- Recovery points: 10-minute granularity for two prior weeks; weekly snapshots for weeks 2–52 (SharePoint/OneDrive); 10-minute granularity for 52 weeks (Exchange Online)
- Billing model: pay-as-you-go at $0.15 per GB per month; restores are free
- Data never leaves the Microsoft 365 data trust boundary; honors geographic residency requirements
- Backups use append-only storage — service cannot modify existing backup copies, protecting against ransomware overwrite
- 90-day offboarding grace period allows recovery of backups after policy offboarding
- Retention and deletion policies (Microsoft Purview) do not affect backup retention period
- Multi-admin email notification feature alerts preset admins of potentially harmful backup actions

**Backup architecture and performance:**
- Backup policy initiates within 60 minutes; initial restore points available within 60 minutes of activation
- Restore performance: up to 250 protection units per hour for bulk recovery; in-place same-URL restore is fastest
- Express restore points (recommended in UI) yield fastest single-site or single-mailbox recovery
- Full site restore rolls back to exact state at prior point, overwriting all content and metadata since that point
- Exchange Online restore recovers modified or deleted items to same or new folder within user's mailbox

**Retention versus backup distinction:**
- Microsoft Purview retention policies govern compliance holds and legal preservation — they do not provide fast bulk recovery
- Native features (versioning, recycle bin, Preservation Hold library) provide limited recovery windows (30 days Files Restore, 93-day recycle bin)
- Microsoft 365 Backup extends recovery to 1 year with faster bulk restore, designed for ransomware and large-scale data loss scenarios

**Ransomware recovery (per learn.microsoft.com/compliance/assurance/assurance-shared-ransomware-protection):**
- Native protections: versioning (500+ versions by default), recycle bin (93 days), Preservation Hold library, Exchange single item recovery
- Microsoft 365 Backup provides faster bulk recovery than native tools for large-scale ransomware events
- Partner solutions must use Microsoft 365 Backup Storage platform for comparable restore performance; external-copy solutions may not meet RTO for large tenants

**Common failure modes:**
- No Microsoft 365 Backup policies — relying solely on retention or native versioning for BCDR
- Confusing Microsoft Purview retention with backup — retention does not provide point-in-time bulk recovery at scale
- No tested RTO — backup policy exists but restore time has never been validated against business continuity requirements
- Third-party backup solution that copies data externally rather than using Microsoft 365 Backup Storage platform — slower restore for large tenants
- In-place restore initiated without scope confirmation — overwrites all content since restore point

Review implications:
- Do not treat retention policies as a substitute for backup coverage — retention and backup serve distinct purposes.
- Never approve or initiate restore operations without confirming scope, target URL, and human sign-off.
- Documentation cannot prove the user's actual backup policy scope, tested RTO, or ransomware recovery readiness.
