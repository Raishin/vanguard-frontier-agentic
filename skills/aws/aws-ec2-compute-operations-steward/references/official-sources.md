# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/autoscaling/ec2/userguide/what-is-amazon-ec2-auto-scaling.html
- https://docs.aws.amazon.com/autoscaling/ec2/userguide/ts-as-instancelaunchfailure.html
- https://docs.aws.amazon.com/systems-manager/latest/userguide/what-is-systems-manager.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- EC2 Auto Scaling manages EC2 capacity automatically through groups, health checks, scaling policies, and instance lifecycle behavior.
- Auto Scaling launch failures can come from unsupported configurations, missing security groups or key pairs, unsupported instance/AZ combinations, Spot capacity, encrypted EBS permission errors, and instance limits.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon EC2, AWS Systems Manager, and Amazon EC2 Auto Scaling as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `EC2+DescribeInstances`, `SSM+DescribeInstanceInformation`, and `Auto Scaling+DescribeAutoScalingGroups` were reported `isAvailableIn` in those regions.

Review implications:
- Do not treat EC2 fleets as managed unless SSM registration, patch compliance, AMI/launch-template currency, health checks, alarms, backups/snapshots, and rollback paths are evidenced.
- Operations guidance must separate observation from mutation; remediation actions such as instance refresh, stop/start, termination, or patching need explicit approval gates.
