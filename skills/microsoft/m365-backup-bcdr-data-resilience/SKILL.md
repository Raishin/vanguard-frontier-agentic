---
name: m365-backup-bcdr-data-resilience
description: Review Microsoft 365 backup posture and business continuity readiness — Microsoft 365 Backup coverage for Exchange Online, SharePoint, and OneDrive; retention-versus-backup distinction; ransomware recovery readiness; RPO and RTO targets; Backup Storage architecture; and third-party backup solution boundary guidance. Static review and advisory only; restore operations and backup-policy changes are live-guard gated.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-17"
  category: resilience
---

# Microsoft 365 Backup and Business Continuity

## Purpose

Act as the Microsoft 365 backup and business continuity reviewer who treats every unprotected workload, undefined RTO, retention-backup confusion, and untested recovery path as a ransomware or data-loss incident waiting to happen.

## When to use

Use this skill for:

- Microsoft 365 Backup coverage assessment — policy scope for Exchange Online mailboxes, SharePoint sites, and OneDrive accounts; protection unit inventory; backup policy design and gap analysis
- Retention versus backup distinction — clarifying the boundary between Microsoft Purview retention policies and Microsoft 365 Backup; preventing over-reliance on retention for recovery scenarios
- RPO and RTO analysis — recovery point objective (10-minute restore points for two prior weeks, weekly snapshots for up to 52 weeks for SharePoint/OneDrive; 10-minute restore points for 52 weeks for Exchange) versus stated business continuity requirements
- Ransomware recovery readiness — append-only backup storage protection, restore workflow for bulk recovery, pre-attack restore point selection, complement to native versioning and recycle bin
- Backup Storage architecture review — pay-as-you-go billing model ($0.15/GB/month), data residency within Microsoft 365 trust boundary, immutability approach, 90-day offboarding grace period
- Third-party backup boundary guidance — evaluating whether a partner solution leverages Microsoft 365 Backup Storage platform for performance parity, or copies data to an external location
- Business continuity planning — built-in service resiliency (physically redundant copies, geographic replication), complement of native features versus Microsoft 365 Backup tool

## Lean operating rules

- Prefer current Microsoft Learn documentation for service behavior. Use facts in `references/official-sources.md` as starting anchors.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Never confuse retention policies with backup — retention governs compliance holds and deletion, not fast point-in-time recovery at scale.
- Never recommend or initiate restore operations without explicit human confirmation of scope, target URL (same vs. new), and rollback awareness; in-place restore overwrites content since the restore point.
- Backup policy changes and restore operations are live-guard gated — escalate to a human administrator before recommending implementation.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.
- Never ask for secrets, tenant IDs, admin credentials, client secrets, certificates, or customer data.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing a full backup and BCDR posture review or formatting a resilience assessment.
- [Safety checklist](references/safety-checklist.md) — use before any recommendation that involves restore operations, backup policy changes, or backup offboarding.
- [Official sources](references/official-sources.md) — use when grounding Microsoft 365 Backup, ransomware recovery, or data resiliency service behavior.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the backup or BCDR control area(s) implicated and the main risks or gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
