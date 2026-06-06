# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html
- https://docs.aws.amazon.com/health/latest/ug/what-is-aws-health.html
- https://docs.aws.amazon.com/awssupport/latest/user/trusted-advisor.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/ce-what-is.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CloudWatch provides operational visibility through metrics, alarms, dashboards, logs, application performance monitoring, infrastructure monitoring, cross-account monitoring, and network/internet monitoring.
- Trusted Advisor inspects AWS environments and can surface recommendations for cost, performance, availability, security, and service limits depending on support plan and feature availability.

Sampled live evidence:
- Read-only regional availability sampling reported `CloudWatch+DescribeAlarms` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- `Health+DescribeEvents` was reported `isAvailableIn` in `us-east-1` and `Not Found` in `us-west-2`, `eu-west-1`, and `ap-southeast-1`; treat Health as account/global-style evidence unless local tooling proves otherwise.

Review implications:
- A daily brief must separate live alarms/incidents, AWS Health events, cost anomalies, deployment changes, security/compliance signals, and backlog risks.
- Do not summarize unknown systems as healthy; say exactly which sources were queried and which were not.
