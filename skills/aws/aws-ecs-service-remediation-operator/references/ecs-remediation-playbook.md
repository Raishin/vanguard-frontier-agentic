# ECS Remediation Playbook

Use this reference when correcting ECS/Fargate task definitions, services, deployment parameters, load balancer health checks, secrets, logging, capacity providers, or CodeDeploy blue/green wiring in repository files.

## What people get wrong

The lazy story is:

> ECS service is unhealthy, so tweak desired count or task definition and redeploy.

Wrong. ECS failures often come from task role/execution role confusion, image pull, secrets, target health, capacity provider, deployment controller, or container health checks.

Common bad assumptions:

- Task role and execution role are interchangeable.
- Increasing desired count fixes failing tasks.
- Health check grace period can hide root cause.
- Latest task definition revision is automatically safe.
- Container logs are enough without service events and target health.
- A repo service definition fix authorizes forcing a new deployment.

## Failure-mode map

- **Image pull:** ECR permissions, VPC endpoints, registry creds, platform architecture.
- **Startup:** env var/secrets, command/entrypoint, health check, dependency readiness.
- **Networking:** subnet, security group, assignPublicIp, target group, service discovery.
- **IAM:** task role for app calls, execution role for pull/log/secrets.
- **Capacity:** Fargate quota, capacity provider weights/base, CPU/memory mismatch.
- **Deployment:** circuit breaker, min/max healthy percent, CodeDeploy task sets, alarms.

## Minimum safe workflow

1. Identify service, cluster, launch type, deployment controller, and task definition family.
2. Correlate service events, stopped-task reasons, target health, and logs before patching.
3. Patch only the repo field tied to the evidence.
4. Preserve rollback task definition or previous service config.
5. Validate IaC/task-definition schema and project tests.
6. State whether force-new-deployment/register-task-definition/update-service is still required and approval-gated.

## Verification targets

- task definition: image, CPU/memory, roles, secrets, logs, health checks, ports
- service definition: desired count, deployment config, circuit breaker, capacity providers, subnets/security groups
- load balancer target group health and health check path/port
- CodeDeploy AppSpec and task set config for blue/green
- CloudWatch logs and ECS service events/stopped-task reasons
- validation commands for CloudFormation/CDK/Terraform/task definition JSON

## When to push back

Push back if the user asks to:

- force a new deployment without root-cause evidence
- disable health checks or circuit breaker to get green status
- swap task/execution role permissions blindly
- increase desired count to mask crash loops
- remove secrets/logging to simplify a task definition
- deploy latest task revision without rollback target

