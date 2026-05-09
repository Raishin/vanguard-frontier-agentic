# Workflow and output contract

Use this reference only when performing a full IaC change safety review, blast radius assessment, or rollback plan audit.

## Review domains

Check these areas before giving a verdict:

- Change scope: which resources are added, modified, replaced, or deleted
- Blast radius: how many accounts, Enterprise Projects, and resource types are affected
- Irreversible operations: deletions, key schedules, policy replacements without rollback path
- Organizations SCP: which accounts and Enterprise Projects are in scope; any SCP denial side effects
- State file security: backend OBS bucket encryption, IAM access restriction to CI/CD agency only
- RFS drift: drift detection status before applying; known vs actual state alignment
- Enterprise Project boundaries: confirm billing/attribution scope vs actual IAM enforcement boundary
- Rollback plan: documented steps, tested reversal, approval gates met

## Safe workflow

1. **Frame scope**
   - IaC tool (Terraform or RFS) and version:
   - Target resources and change type:
   - Current-state evidence (plan output or change set):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized terraform plan output or RFS change set preview.
   - Otherwise inspect IaC code, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test change**
   - What is deleted or replaced and can it be reversed?
   - Does the change touch Organizations SCP — what accounts are affected?
   - Is the state file backend encrypted with SSE-KMS and access-restricted?
   - Has RFS drift detection been run before this apply?
   - Is the Enterprise Project boundary a security boundary or just a billing label?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud IaC Change Safety Review: <scope>
## Change summary and target resources
## Blast radius classification (low/medium/high/org-wide)
## Deletion and irreversible operations detected
## Organizations SCP and cross-account scope
## State drift and conflict risks
## Enterprise Project boundary clarity
## Rollback plan and approval gate completeness
```

Each section must include an evidence level label.
