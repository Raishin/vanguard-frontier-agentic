# Backup Restore Evidence Guide

Use this reference for AWS Backup, backup plans, vaults, vault lock, restore testing, cross-account/cross-Region copy, EBS/RDS/EFS/S3 recovery patterns, retention, lifecycle, encryption, and recoverability evidence.

## What people get wrong

The lazy story is:

> Backup jobs are successful, so the data is protected.

Wrong. Protection means recoverability, immutability where required, correct retention, tested restore permissions, and business-aligned recovery evidence.

Common bad assumptions:

- Successful backup job equals successful restore.
- Cross-Region copy equals disaster recovery.
- Vault Lock protects against every deletion path.
- KMS encryption is operationally invisible during restore.
- Lifecycle cold/archive transitions have no recovery-time impact.
- Native service backups and AWS Backup coverage are equivalent for every resource.

## Backup-specific failure modes

- Backup selection misses resources because tags, opt-in services, or new accounts are not covered.
- Restore role lacks KMS, network, IAM, or target service permissions.
- Vault policies allow deletion, copy disablement, or ransomware-impacting changes.
- Cross-account/cross-Region copies fail or are not monitored.
- Retention/lifecycle conflicts with legal hold, compliance, or RTO.
- Restore test validates infrastructure but not application consistency.

## Minimum safe workflow

1. Identify protected resources, business owner, data classification, RPO/RTO, retention, and compliance requirements.
2. Review AWS Backup plans, selections, vaults, copy actions, lifecycle, encryption, vault policies, and Vault Lock posture.
3. Verify coverage gaps across accounts, Regions, services, tags, and newly created resources.
4. Demand restore evidence: restore job, operator, target environment, application validation, and elapsed time.
5. Check recovery permissions, KMS keys, network, secrets, and dependency readiness.
6. Recommend non-destructive fixes first; deletion, retention reduction, or vault policy changes need explicit approval.
7. Label evidence as configuration, job success, restore test, or business acceptance.

## Verification targets

- AWS Backup plans, backup rules, selections, copy actions, lifecycle, continuous backup/PITR, and protected resource inventory
- backup vault policies, Vault Lock mode, access policy, KMS key policy, and cross-account/cross-Region copy jobs
- restore jobs, restore testing plans, restore role permissions, elapsed restore time, and application validation evidence
- EBS snapshots, RDS/Aurora backups, EFS backups, DynamoDB PITR, S3 versioning/Object Lock/replication, and service-native backup overlap
- AWS Organizations backup policies, Control Tower/account coverage, Config/Backup Audit Manager evidence, and compliance reports
- alerting for failed/missed backup jobs, copy failures, vault policy changes, and backup age

## When to push back

Push back if the user asks to:

- claim recoverability from backup-job success alone
- reduce retention or delete recovery points without compliance approval
- skip restore tests because backups are encrypted and replicated
- ignore KMS/permission dependencies for restore
- treat cross-Region copy as full DR without runbook and application validation
- mutate vault lock or backup policies from advisory evidence alone
