# Workflow and output contract

Use this reference only when performing the full IaC change safety review, blast radius assessment, or approval gate evaluation.

## Review domains

Check these areas before giving a verdict:
- Change summary: resource types, counts, and target projects/folders/org
- Blast radius: low (single resource, no cross-project impact), medium (multiple resources, same project), high (multi-project or recreate), org-wide (org policy, Shared VPC, org IAM)
- Destroy and recreate operations: any resource with "must be replaced" or "will be destroyed" flags
- Cross-project and org-level impact: Shared VPC host/service project bindings, org policy constraints, org-level IAM bindings
- State file conflicts: stale remote state, held state locks, workspace mismatch
- Org policy drift: declared constraints vs. live gcloud org-policies output
- Rollback plan: pre-apply state backup, reverse plan feasibility, documented recovery steps, owner

## Safe workflow

1. **Frame scope**
   - Target project/folder/org and environment:
   - Change type (Terraform plan / Deployment Manager preview):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized terraform plan output or Deployment Manager preview as live evidence.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What resources are being destroyed or recreated?
   - What changes cross project or org boundaries?
   - Is the state file locked or stale?
   - Is a rollback plan documented and feasible?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence or approval, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP IaC Change Safety Review: <scope>
## Executive verdict
- Status: SAFE TO APPLY / APPLY WITH CONDITIONS / BLOCKED / NEEDS EVIDENCE
- Blast radius: low / medium / high / org-wide
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Destroy and recreate operations
- <resource or explicit none>
## Rollback plan assessment
- Pre-apply state backup: <confirmed / not confirmed>
- Reverse plan feasible: <yes / no / unknown>
- Recovery steps documented: <yes / no>
## Approval requirements
- Required approvers: <single / dual / escalation>
- Rationale:
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
