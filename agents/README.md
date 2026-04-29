# Agents

Role definitions for repeatable review, architecture, operations, and bounded execution work.

## Provider catalog

| Provider | Current status | Notes |
| --- | --- | --- |
| AWS | active | includes advisory and bounded write-capable execution agents |
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

See `agents/aws/README.md` for the AWS-specific catalog.
