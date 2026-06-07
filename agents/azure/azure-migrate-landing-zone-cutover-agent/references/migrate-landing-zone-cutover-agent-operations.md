# Azure Migrate Landing Zone Cutover operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

Azure Migrate readiness is not cutover readiness. A safe cutover needs tested landing-zone, network, DNS, identity, monitoring, rollback, and owner evidence.

## Officially grounded service shape

Microsoft guidance separates planning, preparation, migration execution, validation, and decommissioning. Landing-zone transition can affect resource IDs, policy, RBAC, monitoring, writes, and remediation. That is the key insight: cutover risk lives in dependencies and operating model, not only replication status.

## Non-negotiable design rules

1. Do not approve cutover from assessment status alone.
2. Require test migration or equivalent rehearsal evidence before production cutover.
3. Verify landing-zone readiness for connectivity, DNS, identity, policy, monitoring, security, naming, tagging, and subscription placement.
4. Treat resource and subscription moves as control-plane changes with RBAC, policy, alerting, and ID impacts.
5. Require rollback, owner, validation, and decommission checkpoints before go-live.

## Minimal safe implementation flow

1. Classify workload, source, target, migration tool path, and cutover window.
2. Review discovery, dependency, readiness, sizing, and cost evidence freshness.
3. Check landing-zone prerequisites: network, DNS, identity, policy, monitoring, security, and routing.
4. Verify rehearsal, rollback, communications, smoke tests, and post-cutover operational controls.
5. Return go/no-go with blocking gaps and the smallest safe remediation.

## High-risk assumptions to kill

- Replication health means cutover readiness.
- A landing zone exists because a subscription exists.
- DNS and routing can be fixed during the cutover window.
- Rollback is optional after stakeholder approval.
- Documentation proves the user's dependencies or route tables.

## Safe command/code verification targets

- Assessment timestamp, dependency map, readiness warnings, sizing, cost assumptions, and unsupported workloads.
- Landing-zone controls: hub-spoke, DNS, firewall rules, routes, identity, policy, monitoring, backup, and security posture.
- Cutover runbook: freeze, test migration, smoke tests, endpoint switch, rollback, owner, and decommission criteria.

## When to push back

- No recent assessment or dependency map exists.
- Cutover omits DNS, identity, monitoring, backup, or rollback checks.
- Source shutdown or decommissioning is irreversible without explicit signoff.
- Landing-zone policy or RBAC drift is unresolved.
