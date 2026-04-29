# AWS Skills

<img src="../../assets/logos/cloud/aws/aws-cdnlogo.png" alt="AWS logo" width="140" />


This folder contains AWS-focused skills curated for this marketplace.

## Local marketplace portfolio

As of **2026-04-29**, this folder contains **32** local AWS skills:

- `aws-agentcore`
- `aws-api-edge-delivery-review`
- `aws-bedrock-agent-security-governor`
- `aws-ci-cd-release-engineer`
- `aws-compliance-evidence-mapper`
- `aws-change-impact-advisor`
- `aws-cost-optimization-governor`
- `aws-cost-anomaly-watch-coordinator`
- `aws-daily-operations-briefing-coordinator`
- `aws-data-protection-backup-steward`
- `aws-devops-agent-skill-designer`
- `aws-dynamodb-data-modeling-performance-review`
- `aws-ec2-compute-operations-steward`
- `aws-ecs-fargate-platform-operator`
- `aws-eks-platform-operator`
- `aws-event-driven-architecture-review`
- `aws-generative-ai-developer`
- `aws-iac-change-safety-review`
- `aws-iam-least-privilege-review`
- `aws-kms-secrets-lifecycle-steward`
- `aws-landing-zone-governor`
- `aws-migration-cutover-architect`
- `aws-network-architect`
- `aws-non-destructive-task-automation-advisor`
- `aws-observability-incident-responder`
- `aws-rds-aurora-performance-investigator`
- `aws-resilience-bcdr-review`
- `aws-s3-data-perimeter-governor`
- `aws-security-posture-hardening`
- `aws-ticket-triage-escalation-coordinator`
- `aws-serverless-production-readiness`
- `aws-solution-architect`

## Portfolio posture

Role-based AWS skills for evidence-backed architecture, operations, security, resilience, migration, agentic systems, and FinOps workflows.

These skills are intentionally conservative:

- prefer `AwsDocumentationMcpServer` via `uvx awslabs.aws-documentation-mcp-server@latest` when available for AWS documentation grounding,
- if `uvx` cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to official AWS docs, Context7, repo evidence, and read-only AWS CLI evidence when available,
- prefer read-only discovery before mutation,
- separate live AWS state from documentation-based guidance,
- challenge broad IAM, public exposure, untested recovery, and unclear ownership,
- require explicit approval before privileged, destructive, traffic-changing, or production-impacting actions,
- use official AWS documentation and Context7 grounding when service behavior matters.

Run `npm run validate` after changing cataloged AWS skills.
