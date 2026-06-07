# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/devopsagent/latest/userguide/working-with-devops-agent-proactive-incident-prevention.html
- https://docs.aws.amazon.com/devops-guru/latest/userguide/monitoring-cloudwatch.html
- https://docs.aws.amazon.com/devopsagent/latest/userguide/about-aws-devops-agent.html
- https://docs.aws.amazon.com/wellarchitected/latest/operational-excellence-pillar/welcome.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS DevOps Agent proactive incident prevention analyzes incident patterns, ranks improvements, and can generate agent-ready specifications.
- DevOps Guru monitoring guidance exposes insight and usage metrics through CloudWatch; those metrics are evidence inputs for operational skill design, not proof that a skill is effective.

Sampled live evidence:
- Read-only regional availability sampling reported `DevOps Guru+DescribeInsight` and `DevOps Guru+ListInsights` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.

Review implications:
- Skill design must define trigger boundaries, evidence collection steps, output contracts, and evaluation criteria; vague incident prose is not enough.
- Treat DevOps Agent recommendations as candidates that need owner approval, validation, rollback planning, and eval coverage before implementation.
