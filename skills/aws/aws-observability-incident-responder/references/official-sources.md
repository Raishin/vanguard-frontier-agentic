# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Install-CloudWatch-Agent.html
- https://docs.aws.amazon.com/xray/latest/devguide/aws-xray.html
- https://docs.aws.amazon.com/health/latest/ug/what-is-aws-health.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CloudWatch provides metrics, alarms, dashboards, logs, APM, infrastructure monitoring, cross-account monitoring, and network/internet monitoring.
- The CloudWatch agent can collect metrics, logs, and traces from EC2 and on-premises servers via StatsD, collectd, OpenTelemetry, and X-Ray SDKs.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon CloudWatch and AWS X-Ray as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CloudWatch+DescribeAlarms` and `XRay+GetTraceSummaries` were reported `isAvailableIn` in those regions.

Review implications:
- Incident response must separate symptoms, time window, affected services, alarms, logs/traces, recent changes, customer impact, and unknown telemetry gaps.
- Absence of queried alarms is not proof of health if logs, traces, canaries, dashboards, or AWS Health were not checked.
