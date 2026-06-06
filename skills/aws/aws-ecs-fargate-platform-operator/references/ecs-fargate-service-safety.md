# ECS Fargate Service Safety Guide

Use this reference for Amazon ECS/Fargate platform reviews covering services, task definitions, task role vs execution role, capacity providers, load balancers, deployment circuit breakers, blue/green, autoscaling, health checks, logs, secrets, networking, and rollback.

## What people get wrong

The lazy story is:

> ECS service stable means the platform is healthy.

Wrong. Stable desired count can hide overprivileged task roles, broken health checks, missing circuit breakers, wrong image tags, secret exposure, capacity-provider drift, and weak rollback.

Common bad assumptions:

- Task role and execution role can share permissions.
- Latest task definition revision is safe.
- Fargate removes capacity and networking concerns.
- ALB health checks prove application correctness.
- Circuit breaker is enough without alarms and rollback validation.
- Secrets in task definition references are safe without IAM/KMS review.

## ECS/Fargate failure modes

- Execution role cannot pull images, publish logs, or fetch secrets; task role has broad app permissions.
- Task definition changes CPU/memory, ports, env vars, logging, secrets, or image tag unexpectedly.
- Deployment min/max healthy percent, circuit breaker, or CodeDeploy blue/green settings cause outage or no rollback.
- Target group health check path/port/grace period hides app startup failure.
- Subnet/security group/assignPublicIp/service discovery settings expose or isolate tasks incorrectly.
- Autoscaling follows CPU while bottleneck is queue depth, latency, memory, or downstream throttling.

## Minimum safe workflow

1. Identify cluster, service, launch type, deployment controller, task definition family, target groups, and capacity provider strategy.
2. Review task definition diffs: image digest/tag, CPU/memory, ports, env/secrets, logs, roles, health checks, and platform version.
3. Separate execution role from task role and verify least privilege for ECR, CloudWatch Logs, Secrets Manager, SSM, KMS, and app APIs.
4. Check deployment safety: circuit breaker, alarms, desired count, min/max healthy percent, blue/green hooks, and rollback revision.
5. Review networking and exposure: subnets, security groups, public IPs, load balancer, service discovery, and VPC endpoints.
6. Validate observability: service events, stopped task reasons, target health, logs, metrics, and deployment state changes.
7. Require approval before update-service, force-new-deployment, task definition registration, or scaling changes.

## Verification targets

- ECS service definition, deployments, events, desired/running/pending counts, deployment controller, and circuit breaker
- task definition image digest, CPU/memory, roles, secrets, logging, health checks, ports, volumes, and platform version
- task execution role and task role IAM/KMS/Secrets Manager/ECR/CloudWatch Logs permissions
- ALB/NLB target groups, health check path/port/grace, listener rules, and target health
- capacity providers, Fargate/Fargate Spot mix, autoscaling policies, CloudWatch alarms, and queue/business metrics
- CodeDeploy AppSpec, lifecycle hooks, alarms, rollback config, and previous task definition target

## When to push back

Push back if the user asks to:

- force new deployment without root-cause evidence
- widen task or execution role blindly
- deploy mutable image tags without digest/provenance
- disable health checks or circuit breakers to reach steady state
- scale desired count to hide crash loops
- treat service stable as proof of security or readiness
