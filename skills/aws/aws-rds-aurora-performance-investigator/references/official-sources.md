# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.LoggingAndMonitoring.html
- https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_PerfInsights.html
- https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/limitless-monitoring.pi.html
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- RDS logging and monitoring guidance includes CloudWatch, CloudTrail, Enhanced Monitoring, Performance Insights, SNS, and Trusted Advisor as reliability evidence sources.
- Aurora Performance Insights monitors DB load and supports filtering by waits and SQL; AWS docs also flag Performance Insights mode/lifecycle considerations that must be checked for current deployments.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon RDS, Amazon Aurora, and Amazon CloudWatch as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `RDS+DescribeDBInstances` and `PI+DescribeDimensionKeys` were reported `isAvailableIn` in those regions.

Review implications:
- Performance investigation needs time window, DB load/AAS, wait events, top SQL, CPU/IO/memory/network, connection counts, storage, engine version, failover events, and recent changes.
- Availability of RDS APIs does not prove Performance Insights is enabled or that query-level evidence exists.
