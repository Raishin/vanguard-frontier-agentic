# OCI WAF Reliability Review Operations Reference

## What people get wrong

- Multi-AD deployment automatically means highly available application behavior.
- Backups prove recovery without a restore drill.
- A DR plan is valid because it exists.
- Load balancer health is the same as end-to-end transaction health.
- Regional services remove the need for dependency mapping.

## Officially grounded service shape

- OCI regions contain availability domains and fault domains; placement evidence must be tied to the service’s actual regional or AD-scoped behavior.
- Full Stack Disaster Recovery orchestrates DR configurations and plans for supported OCI resource types and Oracle recommends cross-region DR for region-wide outage protection.
- Backups, replication, monitoring alarms, and DR plans are separate evidence classes; none alone proves the application can meet RTO/RPO.
- Monitoring alarm listing is compartment-scoped, can traverse subcompartments only with tenancy-level permission, and is subject to documented service limits.

## Non-negotiable design rules

- Force explicit RTO, RPO, dependency graph, failover owner, rollback owner, test date, and evidence source.
- Require approval before failover, DNS change, scaling, replication reconfiguration, backup deletion, or DR plan execution.
- Label observed alarms, backups, or DR objects as sampled current-state evidence, not proof of production readiness.
- Treat single-region dependencies, manual runbooks, stale alarms, untested restores, and quota gaps as blockers.
- Never commit tenancy, region-subscription, compartment, resource, or customer topology identifiers.

## Minimal safe implementation flow

- Define workload and reliability target.
- Ground region, AD, fault-domain, backup, and Full Stack DR behavior in official docs.
- Use OCI API evidence through the user’s configured read-only OCI MCP for sanitized alarm, backup, and resource-list shape where relevant.
- Map single points of failure and untested assumptions.
- Return blockers, safe next actions, and evidence gaps before recommending failover or mutations.

## High-risk assumptions to kill

- Documentation proves service behavior; it does not prove the user's deployed posture.
- Sampled API evidence proves only the sampled command shape or observation.
- Read-only discovery is not approval for mutation.
- Missing evidence is a blocker, not a detail to smooth over.

## Safe command/code verification targets

- Prefer schema, manifest, link, and asset-integrity validation for repository edits.
- Prefer read-only list/get/help operations for cloud evidence.
- Redact or omit identifiers and sensitive values from notes and reports.

## Safe verification targets

- Official OCI documentation URL is attached to each service-behavior claim.
- Sampled API evidence is labeled with scope and limitation.
- Approval gates are explicit for every proposed mutation.
- Evidence gaps are listed as open questions.

## When to push back

- The user equates resource distribution with application recovery.
- The request asks to execute or alter DR plans without explicit approval.
- No restore, failover, or monitoring test evidence exists for the claimed RTO/RPO.
