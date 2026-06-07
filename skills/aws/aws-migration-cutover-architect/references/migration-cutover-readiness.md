# Migration Cutover Readiness Guide

Use this reference for AWS migration and cutover planning across Application Migration Service, Migration Hub, wave planning, test launches, DNS, data consistency, acceptance tests, downtime windows, rollback, and post-cutover validation.

## What people get wrong

The lazy story is:

> Replication is healthy, so the cutover is ready.

Wrong. Healthy replication is only one prerequisite. Cutover fails on dependencies, identity, DNS, data consistency, acceptance criteria, rollback authority, and stakeholder timing.

Common bad assumptions:

- Application Migration Service launch success proves application acceptance.
- DNS TTL change is a small operational detail.
- Rollback means power on the source again.
- Migration Hub status proves business readiness.
- Test launch can be skipped if the workload is simple.
- Data consistency is a database team problem only.

## Migration-specific failure modes

- Dependency map misses batch jobs, file shares, identity providers, DNS, certificates, licenses, firewall rules, or third-party integrations.
- Test launch differs from production cutover subnet, IAM, security group, DNS, or load balancer path.
- Source and target accept writes simultaneously without reconciliation plan.
- Cutover window lacks owner, go/no-go criteria, rollback decision point, or communication plan.
- Route 53/DNS, TTL, cache, certificate, and client endpoint behavior are untested.
- Post-cutover monitoring cannot distinguish migration regression from normal workload behavior.

## Minimum safe workflow

1. Define wave scope: applications, servers, databases, dependencies, owners, downtime budget, and business acceptance criteria.
2. Verify replication health, test launches, target configuration, security controls, and performance baseline.
3. Build cutover runbook with timestamps, owners, commands, validation checks, decision points, and rollback criteria.
4. Plan DNS/traffic switch, data freeze, final sync, app startup, smoke tests, and stakeholder communication.
5. Define rollback before execution: source state, write reconciliation, DNS reversal, target shutdown, and data-loss risk.
6. Validate post-cutover: functional tests, latency/errors, logs, security controls, backup, monitoring, and user acceptance.
7. Keep destructive migration steps approval-gated and evidence-based.

## Verification targets

- Application Migration Service source server state, replication health, launch template, test launch, cutover launch, and post-launch actions
- Migration Hub wave/application status, owner mapping, dependencies, and automation unit outputs where used
- source/target network, IAM, security groups, routes, DNS, certificates, load balancers, and secrets
- database/file/data consistency plan, freeze window, final sync, and reconciliation evidence
- acceptance tests, smoke tests, performance baseline, monitoring dashboards, and rollback runbook
- stakeholder communication plan, support coverage, change record, and go/no-go approvals

## When to push back

Push back if the user asks to:

- approve cutover from replication health alone
- skip test launch or acceptance tests
- change DNS without TTL/cache rollback plan
- run source and target writes without reconciliation
- hide rollback data-loss risk from stakeholders
- treat Migration Hub status as business acceptance
