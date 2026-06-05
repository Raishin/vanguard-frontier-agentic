# Azure WAF Reliability Review operations

> Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state. Do not paste secrets, identifiers, billing exports, or customer data into commands or files.

## What people get wrong

Reliability is not a SKU checkbox. If the workload has no measured targets, no health model, and no tested recovery path, it is not reliable by design.

## Officially grounded service shape

Microsoft Well-Architected reliability guidance focuses on business requirements, resilience, recovery, operations, and simplicity. Reliability testing guidance stresses deliberate testing against failure modes and recovery targets; service reliability guidance is shared responsibility. That is the key insight: the workload must prove it can detect, withstand, and recover from the failures it claims to tolerate.

## Non-negotiable design rules

### 1. Require business-defined SLOs, SLIs, RTO, and RPO before judging reliability.
### 2. Separate service-level platform capability from workload-level resilience.
### 3. Treat single-instance, single-zone, untested backup, and missing health alerts as material gaps.
### 4. Require restore tests, failover/failback evidence, and deployment rollback paths.
### 5. Use chaos or fault testing only with scoped approval and safety limits.

## Minimal safe implementation flow

1. Classify workload criticality, dependencies, regions, and stated reliability targets.
2. Ground WAF reliability behavior in Microsoft Learn.
3. Review topology, health model, monitoring, alerting, backup, recovery, deployment, and test evidence.
4. Compare tested behavior against SLO/RTO/RPO claims.
5. Return reliability verdict, blockers, safe tests, and open risks.

## High-risk assumptions to kill

- Availability zones alone satisfy HA.
- Configured backup equals successful recovery.
- No incidents means reliable.
- Chaos testing is safe without blast-radius controls.

## Safe command/code verification targets

- SLOs, SLIs, error budgets, RTO/RPO, dependency map, and failure mode analysis.
- Zone/region topology, load balancing, health probes, Azure Monitor alerts, Service Health alerts, and Resource Health signals.
- Backup restore tests, DR drills, failover/failback records, deployment rollback, and chaos test results.

## When to push back

- Reliability claims are undocumented or unmeasured.
- Backups have never been restored.
- No owner accepts data-loss or downtime trade-offs.
- The requested test can harm production without approval.
