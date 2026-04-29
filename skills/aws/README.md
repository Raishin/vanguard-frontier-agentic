# AWS Skills

<img src="../../assets/logos/cloud/aws/aws-cdnlogo.png" alt="AWS logo" width="140" />


This folder contains AWS-focused skills curated for this marketplace.

## Local marketplace portfolio

As of **2026-04-29**, this folder contains **42** local AWS skills:

- `aws-agentcore`
- `aws-api-edge-delivery-review`
- `aws-bedrock-agent-security-governor`
- `aws-change-impact-advisor`
- `aws-ci-cd-release-engineer`
- `aws-compliance-evidence-mapper`
- `aws-cost-anomaly-watch-coordinator`
- `aws-cost-optimization-governor`
- `aws-daily-operations-briefing-coordinator`
- `aws-data-protection-backup-steward`
- `aws-deployment-hotfix-operator`
- `aws-devops-agent-skill-designer`
- `aws-dynamodb-data-modeling-performance-review`
- `aws-ec2-compute-operations-steward`
- `aws-ecs-fargate-platform-operator`
- `aws-ecs-service-remediation-operator`
- `aws-eks-platform-operator`
- `aws-event-driven-architecture-review`
- `aws-generative-ai-developer`
- `aws-iac-change-safety-review`
- `aws-iac-patch-executor`
- `aws-iam-least-privilege-review`
- `aws-kms-secrets-lifecycle-steward`
- `aws-landing-zone-governor`
- `aws-live-deployment-guarded-operator`
- `aws-live-ecs-rollout-guard`
- `aws-live-iac-change-guard`
- `aws-live-pipeline-approval-operator`
- `aws-live-serverless-release-guard`
- `aws-migration-cutover-architect`
- `aws-network-architect`
- `aws-non-destructive-task-automation-advisor`
- `aws-observability-incident-responder`
- `aws-pipeline-fix-operator`
- `aws-rds-aurora-performance-investigator`
- `aws-resilience-bcdr-review`
- `aws-s3-data-perimeter-governor`
- `aws-security-posture-hardening`
- `aws-serverless-production-readiness`
- `aws-serverless-rollout-corrector`
- `aws-solution-architect`
- `aws-ticket-triage-escalation-coordinator`

## Portfolio posture

Role-based AWS skills for evidence-backed architecture, operations, security, resilience, migration, agentic systems, FinOps workflows, bounded execution, and guarded live-environment operations.

These skills are intentionally conservative:

- prefer `AwsDocumentationMcpServer` via `uvx awslabs.aws-documentation-mcp-server@latest` when available for AWS documentation grounding,
- if `uvx` cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to official AWS docs, Context7, repo evidence, and read-only AWS CLI evidence when available,
- prefer read-only discovery before mutation,
- separate repo patching from live AWS mutation,
- require explicit target confirmation, approval, rollback posture, and verification for guarded live actions,
- challenge broad IAM, public exposure, untested recovery, skipped previews, and unclear ownership,
- use official AWS documentation and Context7 grounding when service behavior matters.

Run `npm run validate` after changing cataloged AWS skills.
