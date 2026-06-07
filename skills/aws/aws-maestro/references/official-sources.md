# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/wellarchitected/latest/operational-excellence-pillar/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/framework/ops_model_ops_model.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html
- https://docs.aws.amazon.com/awssupport/latest/user/trusted-advisor.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- The Well-Architected Operational Excellence pillar focuses on designing, delivering, and maintaining workloads through operations best practices.
- CloudWatch provides metrics, alarms, dashboards, logs, APM, infrastructure monitoring, cross-account monitoring, and network/internet monitoring as operational evidence sources.

Sampled live evidence:
- Read-only regional availability sampling reported `CloudWatch+DescribeAlarms` and `WellArchitected+GetWorkload` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.

Review implications:
- Maestro routing should choose the narrowest AWS skill based on domain evidence: incident, deployment, IAM, network, cost, database, resilience, or compliance.
- Do not centralize decisions without citing the evidence source and routing rationale.
