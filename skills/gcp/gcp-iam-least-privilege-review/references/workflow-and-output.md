# Workflow and output contract

Use this reference only when performing the full IAM review, implementation guidance, incident triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Resource hierarchy scope (org, folder, project) and binding inheritance chain
- Principal types: user, group, Service Account, allUsers, allAuthenticatedUsers, workload identity pool
- Role types: basic (owner/editor/viewer), predefined, custom — flag basic roles in production
- Permission-level risks: privilege escalation, SA impersonation, org-level admin roles
- Workload Identity Federation: pool configuration, provider conditions, SA bindings
- Org policy constraints: key creation disabled, domain restriction, WIF cluster creation
- IAM conditions: presence, correctness, resource scoping
- Service Account key inventory: age, usage, rotation status

## Safe workflow

1. **Frame scope**
   - Org/folder/project and environment (prod/staging/dev):
   - Business criticality and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized `gcloud projects get-iam-policy`, `gcloud organizations get-iam-policy`, or Terraform state exports.
   - Otherwise inspect repository IaC/config or structured user descriptions.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What bindings grant org-level admin roles?
   - What SA keys are active and how old are they?
   - Which workloads can use WIF but still use key-based auth?
   - What org policies are missing that would prevent privilege escalation?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# GCP IAM Least Privilege Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Overprivileged bindings
| Resource | Principal | Role | Condition | Risk |
|---|---|---|---|---|
## SA key inventory
| Service Account | Key age | Used recently | WIF available | Action |
|---|---|---|---|---|
## Org policy gaps
| Constraint | Status | Risk if absent |
|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
