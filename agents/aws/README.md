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

## ✍️ Write-capable execution agents

| Agent | Primary use | Write scope | Must not do by default |
| --- | --- | --- | --- |
| `aws-deployment-hotfix-operator-agent` | rapid deployment corrections | manifests, config, release parameters | deploy, apply, destroy |
| `aws-iac-patch-executor-agent` | bounded IaC fixes | CloudFormation, SAM, CDK config, Terraform files | apply or execute infra changes |
| `aws-pipeline-fix-operator-agent` | CI/CD config correction | pipeline files, buildspecs, workflow files | trigger or bypass live pipeline gates |
| `aws-serverless-rollout-corrector-agent` | serverless rollout definition fixes | Lambda / API / event wiring files | live traffic shifts or deploys |
| `aws-ecs-service-remediation-operator-agent` | ECS/Fargate config correction | task/service definitions and rollout config | force deployments or mutate live services |

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
- 🚫 neither tier should mutate live AWS by default without explicit approval
