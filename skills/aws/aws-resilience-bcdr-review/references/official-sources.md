# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/wellarchitected/2023-10-03/framework/rel_planning_for_recovery_disaster_recovery.html
- https://docs.aws.amazon.com/resilience-hub/latest/userguide/resilience-checks.html
- https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html
- https://docs.aws.amazon.com/route53/latest/developerguide/dns-failover.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Well-Architected reliability guidance says DR strategy should meet recovery objectives using approaches such as backup/restore, standby, or active/active.
- Resilience Hub checks can evaluate RTO/RPO targets and service-specific resilience patterns across services including RDS/Aurora, S3, DynamoDB, EC2, EBS, Lambda, EKS, SNS, SQS, ECS, ELB, API Gateway, Route 53, ARC, and Step Functions.

Sampled live evidence:
- Read-only regional availability sampling reported AWS Resilience Hub, AWS Backup, and Route 53 as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled API `Backup+ListBackupVaults` was reported `isAvailableIn` in those regions.

Review implications:
- BCDR review requires explicit RTO/RPO, dependency map, backup/replication evidence, failover runbook, restore/failover test results, DNS/traffic plan, and rollback criteria.
- Tool checks do not prove business recovery readiness without exercised tests.
