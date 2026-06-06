# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/wellarchitected/latest/reliability-pillar/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/framework/reliability.html
- https://docs.aws.amazon.com/wellarchitected/2023-10-03/framework/rel_planning_for_recovery_disaster_recovery.html
- https://docs.aws.amazon.com/resilience-hub/latest/userguide/resilience-checks.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- The Well-Architected Reliability Pillar focuses on designing, delivering, and maintaining workloads that perform their intended functions correctly and consistently.
- Reliability guidance includes foundations, workload architecture, change management, failure management, and DR strategies to meet recovery objectives.

Sampled live evidence:
- Read-only API availability sampling reported `WellArchitected+GetWorkload` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Read-only product availability sampling also reported AWS Resilience Hub as `isAvailableIn` in those regions, but that does not prove a workload has an assessed or passing resilience policy.

Review implications:
- Require workload dependency map, quotas, autoscaling, health checks, change safety, backup/restore, failover tests, RTO/RPO, and operational runbooks.
- Do not infer resilience from architecture diagrams or API availability without exercised failure and recovery evidence.
