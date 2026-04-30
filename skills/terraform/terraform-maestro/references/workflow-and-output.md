# Workflow and Output — Terraform Maestro

## Classification Workflow

### Step 1 — Identify the execution intent

| Signal in task | Intent |
|----------------|--------|
| "review", "check", "audit", "analyze", "what's wrong" | review — no live execution |
| "apply", "deploy", "run", "execute", "push" | potential live-guard — check provider |
| "destroy", "delete", "tear down" | live-guard — always gate |
| "plan", "diff", "what would change" | review — plan-only, not live |
| "design", "architect", "how should I" | review or provider-specific advisory |

### Step 2 — Identify the cloud provider(s)

| Keywords | Domain |
|----------|--------|
| aws, ec2, s3, ecs, eks, lambda, cloudformation, cdk, control tower | `aws-iac` |
| azure, arm, bicep, azurerm, aks, cosmos, app service, management group | `azure-iac` |
| oci, oracle, resource manager, oke, autonomous db, compartment | `oci-iac` |
| No cloud keyword, or "all providers", "multi-cloud" | `review` (terraform-reviewer handles cross-cloud) |

### Step 3 — Apply routing rules

| Scenario | Route |
|----------|-------|
| Code/module review only, any cloud | `terraform-reviewer` |
| AWS IaC change safety check before apply | `aws-iac-change-safety-review-agent` |
| AWS IaC patch / targeted change | `aws-iac-patch-executor-agent` |
| AWS landing zone / Control Tower design | `aws-landing-zone-governor-agent` |
| Azure landing zone / management group design | `azure-landing-zone-architect-agent` |
| Code review + AWS safety check together | `terraform-reviewer` + `aws-iac-change-safety-review-agent` (parallel) |
| Live AWS apply / CloudFormation update / CDK deploy | `aws-live-iac-change-guard-agent` (GATE) |
| Live Azure ARM stack apply/modify | `azure-live-arm-deployment-stack-guard-agent` (GATE) |
| Live OCI Resource Manager apply/destroy | `oci-live-resource-manager-stack-guard-agent` (GATE) |

---

## Dispatch Examples

### Example 1 — Pure Terraform review

Task: "Review this Terraform module for security issues and state drift"

```
Route: terraform-reviewer
Reason: IaC code review with no live execution — single review domain.
Mode: single
```

### Example 2 — AWS IaC with blast-radius concern

Task: "I'm about to apply this Terraform change to our AWS prod account — check it first"

```
Route: terraform-reviewer + aws-iac-change-safety-review-agent
Reason: Code quality review (review) + AWS-specific blast-radius analysis (aws-iac) required before live apply.
Mode: parallel (2 specialists)
```

### Example 3 — AWS live apply

Task: "Run terraform apply on the AWS prod workspace"

```
Route: aws-live-iac-change-guard-agent
Mode: live-guard-gate
⚠ STOP — live AWS IaC apply requested. Before dispatching, confirm:
  1. Target workspace/stack and AWS account
  2. Blast-radius: which resources will be created, modified, or destroyed?
  3. Rollback path: prior state snapshot or plan to revert?
```

### Example 4 — OCI destroy

Task: "Tear down the OCI Resource Manager stack for the dev environment"

```
Route: oci-live-resource-manager-stack-guard-agent
Mode: live-guard-gate
⚠ STOP — OCI stack destroy requested. Terraform destroy is irreversible without state backup.
Confirm: stack OCID, compartment, blast-radius (all resources in stack), rollback path (state file backup location).
```

### Example 5 — Multi-cloud IaC design

Task: "Help me design a Terraform landing zone that covers both AWS and Azure"

```
Route: aws-landing-zone-governor-agent + azure-landing-zone-architect-agent
Reason: AWS landing zone design (aws-iac) + Azure landing zone design (azure-iac) — parallel specialists.
Mode: parallel (2 specialists)
```

---

## Output Format

Always lead with the routing decision:

```
Route: <agent-id(s)>
Reason: <one sentence>
Mode: single | parallel (N) | live-guard-gate
```

Then: dispatched specialist output (summarized, not verbatim), then recommended next actions.
