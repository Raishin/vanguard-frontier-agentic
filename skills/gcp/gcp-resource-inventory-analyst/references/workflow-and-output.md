# Workflow and output contract

Use this reference only when performing the full inventory report, stale resource audit, implementation guidance, or cost attribution review.

## Review domains

Check these areas before giving a verdict:
- Scope: org/folder/project hierarchy, asset types to include, change history window (max 35 days)
- Resource count: by type (compute, storage, networking, IAM, etc.) across scoped hierarchy
- Stale resources: unattached static IPs, unattached persistent disks, orphaned firewall rules, unused reservations
- Label coverage: required labels (team, environment, cost-center) per resource type, unlabeled resource count
- Org policy violations: resources violating org policy constraints, violation severity
- Change history: resource creates/updates/deletes in the investigation window, who made changes
- IAM policy inventory: overly broad bindings, project-level vs. org-level policy analysis

## Safe workflow

1. **Frame scope**
   - Org/folder/project:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP Asset Inventory API / gcloud asset read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What stale resources are generating ongoing charges?
   - What resources missing labels cannot be attributed in billing exports?
   - What org policy violations indicate control failures?
   - What evidence is missing (e.g., change history older than 35 days)?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Resource Inventory Analyst: <scope>
## Executive verdict
- Status: CLEAN / CLEAN WITH GAPS / NEEDS REMEDIATION / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
