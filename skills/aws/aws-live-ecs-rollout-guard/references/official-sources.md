# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-deployment.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/deployment-steps-ecs.html
- https://docs.aws.amazon.com/whitepapers/latest/overview-deployment-options/bluegreen-deployments.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/troubleshooting.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- ECS service deployment history tracks lifecycle states, circuit breaker failures, CloudWatch alarms, rollbacks, and recent deployment history.
- ECS blue/green deployments with CodeDeploy shift traffic between replacement task sets and can use lifecycle hooks; rollback depends on deployment configuration and health signals.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon ECS, AWS Fargate, and AWS CodeDeploy as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `ECS+DescribeServices`, `ECS+DescribeTasks`, `CloudWatch+DescribeAlarms`, and `CodeDeploy+GetDeployment` were reported `isAvailableIn` in those regions.

Review implications:
- Guard live ECS rollout with current service/deployment state, stopped-task reasons, target health, alarms, deployment controller, desired/running counts, and rollback/stop path.
- Regional/API availability does not prove a service is safe to roll forward.
