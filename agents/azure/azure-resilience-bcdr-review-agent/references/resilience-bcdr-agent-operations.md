# Azure Resilience BCDR Review operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

Zero-downtime and zero-data-loss claims are expensive, rare, and usually false without tested architecture evidence. Untested DR is not DR.

## Officially grounded service shape

Microsoft guidance says realistic RTO/RPO require stakeholder agreement, architecture support, and testing; DR plans need runbooks, communications, escalation, failover and failback design, backups, and drills. That is the key insight: resilience is a tested operating capability, not a diagram.

## Non-negotiable design rules

1. Do not accept RTO or RPO claims without test evidence.
2. Separate Azure platform resilience features from workload recovery obligations.
3. Require failover, failback, backup, restore, and communications runbook evidence.
4. Map every critical dependency to a recovery owner and validation signal.
5. Treat untested single-region dependencies as material risk.

## Minimal safe implementation flow

1. Classify workload criticality, dependencies, regions, and recovery targets.
2. Ground reliability and DR behavior in Microsoft Learn.
3. Review architecture, backup, replication, failover, failback, runbook, and drill evidence.
4. Compare tested recovery results against RTO/RPO claims.
5. Return verdict with blockers, safe next drills, and open questions.

## High-risk assumptions to kill

- Zone redundancy equals disaster recovery.
- Backups prove restore capability without restore tests.
- Failover is enough without failback planning.
- Cloud provider status replaces workload recovery planning.

## Safe command/code verification targets

- RTO, RPO, MTTR, failure mode analysis, business criticality, and stakeholder signoff.
- Backup policy, restore tests, replication mode, regional dependencies, and data consistency checks.
- DR runbook, war room, communication plan, escalation path, drill records, and failback plan.

## When to push back

- Recovery targets are aspirational but untested.
- Runbooks are unavailable during regional outage.
- Failback is undefined.
- Data loss risk is hidden from business owners.
