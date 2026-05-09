# Workflow and Output Contract

Use this reference when performing the full WAF Reliability Pillar review or formatting the final assessment.

## Review domains

Work through these five reliability design principles:

1. **Recover automatically from failure** — CloudWatch alarms trigger Auto Scaling, Lambda retries, SQS DLQ routing, and automated EC2 recovery
2. **Test recovery procedures** — AWS FIS experiments, GameDays, chaos engineering, DR drills (Route 53 failover, RDS failover, EC2 ASG replacement)
3. **Scale horizontally** — EC2 ASG with target tracking, ECS/EKS service autoscaling, DynamoDB auto scaling, RDS read replicas, SQS decoupling
4. **Stop guessing capacity** — Service Quotas review, Load Testing, Trusted Advisor limits, AWS Compute Optimizer recommendations
5. **Manage change through automation** — CodeDeploy blue/green, CloudFormation drift detection, rolling updates, deployment circuit breakers

## Safe workflow

1. **Frame scope**: workloads, accounts, Regions, SLO/RTO/RPO targets, and business criticality
2. **Gather evidence**: Auto Scaling config, health check logs, SQS DLQ metrics, backup policy, Multi-AZ status, CloudWatch alarms
3. **Assess each principle**: identify gaps per principle with severity
4. **Prioritize findings**: by RTO/RPO impact × probability × data loss risk
5. **Draft recommendations**: each with rollback path and validation test
6. **Confirm before acting**: require approval for any production autoscaling, backup, or DR change

## Response shape

1. Scope: SLO/RTO/RPO targets confirmed
2. Service Quota and capacity assessment
3. Multi-AZ / multi-region topology
4. Auto Scaling and health check coverage
5. Queue and messaging resilience (DLQ, retries, backoff)
6. Data backup and DR strategy
7. Change management safety (deployments, rollback)
8. Chaos engineering / DR test status
9. Prioritized findings and recommendations
10. Open risks and blockers
