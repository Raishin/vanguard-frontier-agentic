# Well-Architected Reliability Review Guide

Use this reference for AWS Well-Architected Framework Reliability Pillar reviews. In this repository, `aws-waf-reliability-review` means Well-Architected Framework review, not AWS Web Application Firewall configuration.

## What people get wrong

The lazy story is:

> Multi-AZ plus backups equals reliable.

Wrong. Reliability is measured against explicit objectives and exercised recovery. Architecture diagrams, service defaults, and unused backups do not prove resilience.

Common bad assumptions:

- Multi-AZ deployment proves failure isolation.
- Auto Scaling means capacity headroom exists.
- Backups prove restore capability.
- DR plan proves RTO/RPO compliance.
- Queue/DLQ presence proves downstream recovery.
- Well-Architected questionnaire answers equal implementation evidence.

## Reliability-specific failure modes

- No explicit SLO, RTO, RPO, dependency tier, or error budget.
- Single points of failure hidden in DNS, KMS, NAT, VPC endpoints, IAM roles, data stores, or third-party dependencies.
- Quotas, throttling, and retry storms are not modeled.
- Change safety lacks canaries, rollback alarms, deployment circuit breakers, or game-day evidence.
- Backup retention exists but restore order, permissions, and application consistency are untested.
- Regional failover path conflicts with data replication lag, identity, DNS TTL, or runbook ownership.

## Minimum safe workflow

1. Confirm workload scope, business criticality, SLO, RTO, RPO, peak load, and dependency map.
2. Classify failure domains: Availability Zone, Region, account, service dependency, data plane, control plane, and human/operator path.
3. Review workload architecture, quotas, health checks, scaling, backpressure, retries, queues, and stateful components.
4. Inspect change-management controls: staged rollout, rollback, alarms, deployment history, and incident correlation.
5. Verify backup/restore and DR test evidence against RTO/RPO, not just configuration.
6. Produce findings with severity, evidence level, risk, recommended validation, and owner.
7. Do not mark reliability ready without exercised failure/recovery evidence.

## Verification targets

- SLO/RTO/RPO and customer-impact definitions
- dependency map for compute, network, identity, data, DNS, KMS, queues, and third parties
- quotas, scaling policies, circuit breakers, alarms, synthetic checks, and runbook links
- backup policy, restore test record, replication lag, and data-consistency notes
- deployment strategy, rollback alarms, canary/blue-green config, and recent change failure history
- Resilience Hub assessment, Well-Architected workload answers, incident/postmortem evidence where available

## When to push back

Push back if the user asks to:

- call the workload reliable without SLO/RTO/RPO
- accept untested backups or DR as evidence
- ignore quota/throttling risk because current traffic is low
- treat multi-AZ as equivalent to multi-Region DR
- recommend availability changes without owner and rollback path
- confuse this Well-Architected Framework review with AWS Web Application Firewall tuning
