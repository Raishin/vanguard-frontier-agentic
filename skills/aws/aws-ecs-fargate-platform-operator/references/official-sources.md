# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/AWS_Fargate.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_execution_IAM_role.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-deployment.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/troubleshooting.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Fargate for ECS provides serverless container management with task definitions, platform versions, capacity providers, service load balancing, and usage metrics.
- The ECS task execution role lets the ECS agent perform AWS API calls such as pulling ECR images, retrieving Secrets Manager or Systems Manager values, and accessing configured storage; it is distinct from the application task role.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon ECS and AWS Fargate as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `ECS+DescribeServices` and `ECS+DescribeTasks` were reported `isAvailableIn` in those regions.

Review implications:
- Require evidence for task role vs execution role separation, image-pull path, secrets access, network mode/security groups, load balancer health, deployment controller, circuit breaker/rollback, logs, and autoscaling.
- Fargate availability does not prove platform-version compatibility, quota, subnet capacity, or service health in the user's account.
