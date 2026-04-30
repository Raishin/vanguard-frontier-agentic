# Terraform Agents

# Terraform agent catalog for this marketplace. 😄

## Agent tiers

| Tier | Purpose | Default access | Live execution |
| --- | --- | --- | --- |
| Advisory | Review Terraform code, plans, state assumptions, and drift posture | read-only | not allowed |
| Repo-write execution | Patch Terraform files and workflow config in-repo | workspace-write | not allowed by default |
| Guarded live operation | Help with real Terraform plan/apply flows near live credentials and state | workspace-write | approval-gated only |

## Catalog

| Agent | Tier | Main use |
| --- | --- | --- |
| `terraform-reviewer-agent` | advisory | review plans, state, backend, workspaces, least privilege |
| `terraform-repo-patch-operator-agent` | repo-write execution | patch Terraform code safely without live apply |
| `terraform-live-apply-guard-agent` | guarded live operation | guard real plan/apply flows with lock, backend, and approval discipline |
