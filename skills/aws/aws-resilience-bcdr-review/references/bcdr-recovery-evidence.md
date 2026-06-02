# BCDR Recovery Evidence Guide

Use this reference for AWS resilience, business continuity, disaster recovery, RTO/RPO, failover/failback, backup/restore, game days, runbooks, and recovery validation reviews.

## What people get wrong

The lazy story is:

> We have backups and multi-AZ, so recovery is covered.

Wrong. BCDR is proven by exercised recovery against business objectives. Configuration without restore/failover evidence is a promise, not a capability.

Common bad assumptions:

- Backup success equals restore success.
- Multi-AZ equals disaster recovery.
- Multi-Region replication guarantees low RPO.
- DNS failover is enough for application recovery.
- Runbooks prove operators can execute under pressure.
- Resilience Hub assessment replaces game days.

## BCDR failure modes

- RTO/RPO targets are not defined per business process or data tier.
- Backups are encrypted, retained, or replicated but not restorable by the recovery team.
- Failover path lacks identity, network, DNS, KMS, secrets, quota, or dependency readiness.
- Data replication produces split-brain, stale reads, or unreconciled writes.
- Failback is undefined or riskier than failover.
- Recovery automation depends on the same failed Region/account/service.

## Minimum safe workflow

1. Define workload scope, critical business functions, RTO/RPO, MTPD, and dependency tiers.
2. Map recovery strategy: backup/restore, pilot light, warm standby, active/passive, or active/active.
3. Verify recovery prerequisites: backups, replication, IAM, KMS, DNS, network, quotas, secrets, and runbooks.
4. Demand restore/failover test evidence, not just configuration.
5. Review failback, data reconciliation, customer communication, and post-recovery monitoring.
6. Prioritize gaps by business impact, recovery objective miss, and remediation complexity.
7. Keep destructive failover/failback operations approval-gated.

## Verification targets

- RTO/RPO by workload component and business owner approval
- AWS Backup plans/vaults/copy jobs, restore jobs, restore testing, retention, vault lock, and KMS access
- Resilience Hub assessment, recommendations, and policy target evidence
- Route 53 health checks, Application Recovery Controller routing controls/readiness checks, DNS TTL, and traffic-shift runbooks
- database/storage replication lag, consistency model, point-in-time restore, and reconciliation plan
- game-day results, recovery runbook timestamps, operator roles, and post-test corrective actions

## When to push back

Push back if the user asks to:

- accept backup configuration without restore evidence
- claim RTO/RPO compliance without timed tests
- ignore failback and reconciliation
- perform failover tests without stakeholder approval
- rely on one Region/account for recovery control plane
- hide recovery gaps because the architecture is “multi-AZ”
