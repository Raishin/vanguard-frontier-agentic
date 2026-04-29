# ☁️ AWS Agents

<p align="center">
  <img src="../../assets/logos/cloud/aws/aws-cdnlogo.png" alt="AWS logo" width="140" />
</p>

# AWS agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live AWS mutation |
| --- | --- | --- | --- |
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Execution / correction agents | Patch repo files, deployment config, IaC, and workflow definitions | workspace-write | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real AWS environments | workspace-write | approval-gated and target-confirmed only |

## ✍️ Write-capable execution agents

| Agent | Primary use | Write scope | Must not do by default |
| --- | --- | --- | --- |
| `aws-deployment-hotfix-operator-agent` | rapid deployment corrections | manifests, config, release parameters | deploy, apply, destroy |
| `aws-iac-patch-executor-agent` | bounded IaC fixes | CloudFormation, SAM, CDK config, Terraform files | apply or execute infra changes |
| `aws-pipeline-fix-operator-agent` | CI/CD config correction | pipeline files, buildspecs, workflow files | trigger or bypass live pipeline gates |
| `aws-serverless-rollout-corrector-agent` | serverless rollout definition fixes | Lambda / API / event wiring files | live traffic shifts or deploys |
| `aws-ecs-service-remediation-operator-agent` | ECS/Fargate config correction | task/service definitions and rollout config | force deployments or mutate live services |

## 🚦 Guarded live-AWS operators

| Agent | Primary use | Default live posture | Must refuse when |
| --- | --- | --- | --- |
| `aws-live-deployment-guarded-operator-agent` | generic live deployment actions | preview + approval + rollback required | account, region, target, or approval is ambiguous |
| `aws-live-iac-change-guard-agent` | live CloudFormation/SAM/CDK/Terraform-backed execution | change set/plan + drift + rollback posture first | execute is requested without preview or resource protection |
| `aws-live-pipeline-approval-operator-agent` | live CodePipeline approvals and gated resumes | exact execution + approver scope required | evidence or approver authority is weak |
| `aws-live-serverless-release-guard-agent` | live Lambda/serverless rollout actions | alias/deployment config + alarms + rollback required | traffic shift is requested without a clear rollout plan |
| `aws-live-ecs-rollout-guard-agent` | live ECS/Fargate rollout actions | circuit breaker or alarms + health evidence first | rollout safety signals are weak or contradictory |

Per-agent IAM least-privilege guidance:

- [`aws-live-deployment-guarded-operator-agent/IAM-PERMISSIONS.md`](aws-live-deployment-guarded-operator-agent/IAM-PERMISSIONS.md)
- [`aws-live-iac-change-guard-agent/IAM-PERMISSIONS.md`](aws-live-iac-change-guard-agent/IAM-PERMISSIONS.md)
- [`aws-live-pipeline-approval-operator-agent/IAM-PERMISSIONS.md`](aws-live-pipeline-approval-operator-agent/IAM-PERMISSIONS.md)
- [`aws-live-serverless-release-guard-agent/IAM-PERMISSIONS.md`](aws-live-serverless-release-guard-agent/IAM-PERMISSIONS.md)
- [`aws-live-ecs-rollout-guard-agent/IAM-PERMISSIONS.md`](aws-live-ecs-rollout-guard-agent/IAM-PERMISSIONS.md)

## 👀 Read-only advisory examples

| Agent | Focus |
| --- | --- |
| `aws-observability-incident-responder-agent` | incident review and observability evidence |
| `aws-cost-anomaly-watch-coordinator-agent` | proactive cost watch and escalation |
| `aws-change-impact-advisor-agent` | pre-change blast-radius and rollback review |
| `aws-compliance-evidence-mapper-agent` | audit evidence mapping |
| `aws-solution-architect-agent` | broad architecture judgment |

## 🛡️ Operating note

Have fun, but keep the contract sharp:

- 😄 advisory agents stay read-only by default
- ✍️ execution agents can patch repo files
- 🚦 guarded live operators may work near real AWS authority, so they must confirm target, approval, rollback, and verification before mutation
- 🚫 no tier should treat vague production intent as permission
