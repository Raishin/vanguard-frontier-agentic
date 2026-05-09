# Workflow and output contract

Use this reference only when performing a full IaC change safety review before apply.

## Review domains

Check these areas before giving a recommendation:

- Change summary: resources added, modified, deleted, replaced
- Blast radius: single resource, service-level, account-wide, or Resource Directory org-wide
- Irreversible operations: RDS, OSS, KMS, VPC deletion or replacement
- State backend security: OSS bucket SSE-KMS, public access block, RAM policy scope
- ROS drift detection: was drift check run before generating the change set?
- Rollback plan: documented rollback steps, approval gate, owner assigned
- Cross-account scope: Resource Directory member account enumeration for org-level changes

## Safe workflow

1. **Frame the change**
   - Change set source (terraform plan / ROS change set):
   - Target environment (dev/staging/production):
   - Resources added, modified, deleted:
   - Org-level or account-level scope:
2. **Collect evidence**
   - Prefer sanitized terraform plan output or ROS change set JSON.
   - Otherwise inspect IaC source and stack configuration.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the change**
   - Are any deletions of stateful irreversible resources present?
   - Has ROS drift detection been run before this change set?
   - Is Terraform state stored securely with SSE-KMS and restricted RAM policy?
   - What is the blast radius if this change partially fails mid-apply?
   - Is a rollback plan documented with a tested rollback path?
4. **Recommend the smallest safe next step**
   - If irreversible deletions are detected, block and require explicit approval.
   - If drift is undetected, require drift detection before proceeding.
   - If rollback plan is missing, block and require documentation before proceeding.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud IaC Change Safety Review: <change set identifier>
## Executive summary
- Safety verdict: APPROVE / APPROVE WITH CONDITIONS / BLOCK
- Evidence level:
- Critical findings:
## Change summary
- Resources added:
- Resources modified:
- Resources deleted:
- Resources replaced:
## Blast radius classification
- Classification: low / medium / high / org-wide
- Rationale:
- Affected accounts (if org-wide):
## Irreversible operations
| Resource | Operation | Risk | Required approval |
|---|---|---|---|
## State and drift risks
- Terraform state backend security:
- ROS drift detection status:
- Conflict risks:
## Rollback plan assessment
- Rollback steps documented:
- Approval gate present:
- Owner assigned:
## Safe change sequencing
1. <step> — rationale: <rationale>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
