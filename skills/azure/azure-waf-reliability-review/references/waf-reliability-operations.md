# Azure WAF Reliability Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Using generic uptime targets instead of critical user-flow reliability requirements.
- Assuming redundancy exists because a service is managed by Azure.
- Skipping dependency mapping and blast-radius analysis.
- Treating backup existence as recovery proof.
- Running chaos or failover tests without hypothesis, safety guardrails, and rollback.

## Officially grounded service shape

- Microsoft Learn evidence says reliability requires workloads to be resilient, recoverable, and available according to business promises.
- Reliability design principles cover business requirements, resilience, recovery, operations, and simplicity.
- Reliability guidance emphasizes critical user flows, realistic constraints, fault isolation, redundancy, self-healing, tested recovery plans, observable systems, failure simulation, automation, and avoiding unnecessary complexity.
- Reliability testing verifies the workload can withstand faults, scale under demand, and recover within defined targets; testing must evolve as architecture and incidents reveal new weaknesses.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.

## Non-negotiable design rules

- Define reliability targets per critical user flow before proposing architecture.
- Map dependencies and failure modes before approving redundancy claims.
- Separate high availability, disaster recovery, backup, and operational readiness.
- Require observability and actionable alerts for critical paths.
- Test recovery and resilience safely before claiming readiness.

## Minimal safe implementation flow

- Scope workload, critical flows, business promises, regions/zones, dependencies, and recovery targets.
- Collect architecture, health model, alerts, scaling, backup, deployment, failover, and test evidence.
- Classify gaps in resilience, recovery, operations, simplicity, and service-specific reliability.
- Prioritize fixes by user impact, blast radius, and reversibility.
- Return reliability verdict, blockers, safe tests, target-state changes, and verification checks.

## Safe verification targets

- SLOs, RTO, and RPO are documented per critical flow.
- Dependencies and single points of failure are mapped.
- Health model, metrics, logs, alerts, and ownership cover critical paths.
- Backup, restore, failover, and failback are tested against targets.
- Reliability tests have hypothesis, blast-radius controls, stop conditions, and rollback.

## When to push back

- The user wants reliability approval without critical-flow targets.
- A managed service SLA is used as proof of workload reliability.
- Recovery was never tested.
- Chaos or failover testing would be unsafe or ownerless.
