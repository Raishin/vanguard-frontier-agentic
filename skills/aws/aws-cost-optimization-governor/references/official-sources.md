# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/compute-optimizer/latest/ug/what-is.html
- https://docs.aws.amazon.com/compute-optimizer/latest/ug/savings-estimation-mode.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/understanding-rr-calc.html
- https://docs.aws.amazon.com/wellarchitected/latest/cost-optimization-pillar/welcome.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Compute Optimizer analyzes resource configuration and utilization for EC2 instances, Auto Scaling groups, Lambda functions, EBS volumes, and ECS services on Fargate, then reports optimization recommendations.
- Cost Explorer rightsizing recommendations use usage and billing context; savings estimates can be affected by On-Demand hours, RI/Savings Plans coverage, and payer/member-account scope.

Sampled live evidence:
- Read-only regional availability sampling reported `Compute Optimizer+GetEC2InstanceRecommendations` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- `TrustedAdvisor+ListRecommendations` was reported `isAvailableIn` in `us-east-1`, `us-west-2`, and `eu-west-1`, and `Not Found` in `ap-southeast-1`; treat this as sampled API availability, not proof of support-plan entitlement or recommendation quality.

Review implications:
- Do not delete, downsize, or purchase commitments from recommendation output alone. Require utilization windows, performance/SLO impact, reservation or Savings Plans interaction, owner approval, rollback path, and change calendar context.
- Cost tools can identify candidates; they do not prove business criticality or safe remediation.
