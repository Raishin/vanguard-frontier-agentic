# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/troubleshooting.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-deployment.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_execution_IAM_role.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/deployments-rollback-and-redeploy.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- ECS troubleshooting guidance covers task, service, agent, Docker, EBS, Service Connect, Fargate, and throttling errors as distinct failure classes.
- ECS service deployments track lifecycle states, circuit breaker failures, CloudWatch alarms, rollbacks, and service history for recent deployments.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon ECS, AWS Fargate, and AWS CodeDeploy as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `ECS+DescribeServices`, `ECS+DescribeTasks`, and `CodeDeploy+GetDeployment` were reported `isAvailableIn` in those regions.

Review implications:
- Repo-side fixes must identify the failing service/task-definition field, expected deployment behavior, validation command, and rollback diff; they must not force live service updates by default.
- Do not infer root cause from one ECS error string; correlate service events, stopped-task reasons, target health, deployment controller, task/execution roles, image pull, secrets, and logs.
