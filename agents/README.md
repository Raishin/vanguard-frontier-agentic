# Agents

Role definitions for repeatable review, architecture, operations, and bounded execution work.

## Provider catalog

| Provider | Current status | Notes |
| --- | --- | --- |
| AWS | active | includes advisory, repo-write execution, and guarded live-AWS operator agents |
| Azure | active | read-only role agents |
| OCI | active | read-only role agents |
| Terraform | active | generic IaC review |
| GCP | reserved | no provider portfolio yet |
| Multi-cloud | limited | generic architecture roles |
| Security | limited | generic domain roles |

## AWS first: easy catalog

### AWS advisory agents

Read-only by default. Use for review, diagnosis, planning, briefing, triage, and non-destructive coordination.

### AWS execution agents

Workspace-write in Codex, but still non-destructive toward live AWS by default.

| Agent | Type | Default access | Intended use |
| --- | --- | --- | --- |
| `aws-deployment-hotfix-operator-agent` | execution | workspace-write | rapid repo-side deployment corrections |
| `aws-iac-patch-executor-agent` | execution | workspace-write | bounded IaC patching |
| `aws-pipeline-fix-operator-agent` | execution | workspace-write | CI/CD config fixes |
| `aws-serverless-rollout-corrector-agent` | execution | workspace-write | serverless rollout file corrections |
| `aws-ecs-service-remediation-operator-agent` | execution | workspace-write | ECS/Fargate config remediation |

### AWS guarded live operators

Workspace-write in Codex, but these roles are designed for repos or shells that may be connected to real AWS credentials or real deployment authority. They must confirm target identity, require explicit approval, prefer preview or dry-run evidence, and define rollback plus post-change verification before mutation.

| Agent | Type | Default access | Intended use |
| --- | --- | --- | --- |
| `aws-live-deployment-guarded-operator-agent` | guarded-live | workspace-write | generic live deployment actions with approval gates |
| `aws-live-iac-change-guard-agent` | guarded-live | workspace-write | live IaC preview and execution discipline |
| `aws-live-pipeline-approval-operator-agent` | guarded-live | workspace-write | live pipeline approval and gated resume handling |
| `aws-live-serverless-release-guard-agent` | guarded-live | workspace-write | live Lambda/serverless release actions |
| `aws-live-ecs-rollout-guard-agent` | guarded-live | workspace-write | live ECS/Fargate rollout actions |

See `agents/aws/README.md` for the AWS-specific catalog.
