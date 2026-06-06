# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html
- https://docs.aws.amazon.com/aws-backup/latest/devguide/logicallyairgappedvault.html
- https://docs.aws.amazon.com/aws-backup/latest/devguide/vault-lock.html
- https://docs.aws.amazon.com/aws-backup/latest/devguide/cross-account-backup.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS Backup logically air-gapped vaults are isolated vaults intended to secure backups, support cross-account sharing, multi-party approval recovery, and cross-Region copies.
- AWS Backup vault lock, cross-account backup, lifecycle, encryption, and restore workflows are design inputs; recoverability is proven only by successful restore evidence against the required RTO/RPO.

Sampled live evidence:
- Read-only regional availability sampling reported AWS Backup as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `Backup+ListBackupVaults` and `Backup+ListRecoveryPointsByBackupVault` were reported `isAvailableIn` in those regions.

Review implications:
- Backup existence is not recovery readiness. Require restore-test results, restore IAM/KMS access, backup selection scope, lifecycle/retention evidence, vault lock posture, copy status, and recovery-account access.
- Treat ransomware-resilience claims as unproven without isolation, immutability or lock controls, cross-account/cross-Region strategy, and tested recovery.
