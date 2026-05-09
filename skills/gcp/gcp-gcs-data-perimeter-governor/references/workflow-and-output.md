# Workflow and output contract

Use this reference only when performing the full GCS perimeter review, IAM audit, VPC-SC coverage analysis, or data residency compliance pass.

## Review domains

Check these areas before giving a verdict:
- Public access exposure: allUsers and allAuthenticatedUsers IAM bindings across all buckets in scope
- Uniform bucket-level access: UBL enabled/disabled state, legacy ACL presence
- VPC Service Controls: storage.googleapis.com included in perimeter, dry-run vs. enforcement mode
- Org policy: constraints/storage.publicAccessPrevention enforced at org or folder level
- IAM Conditions: time-bounded bindings, resource path conditions, IP-based conditions
- Object Lifecycle: delete rules with versioning prerequisite check, transition rules review
- Data residency: bucket location (single-region vs. dual-region vs. multi-region) vs. compliance requirement
- Bucket Lock: retention policy existence, lock status for compliance workloads
- HMAC keys: active HMAC keys associated with service accounts (exfiltration risk)

## Safe workflow

1. **Frame scope**
   - Bucket list and project:
   - Data classification (regulated/internal/public):
   - Compliance requirements (GDPR, HIPAA, data residency):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test perimeter**
   - Are any buckets bound to allUsers or allAuthenticatedUsers?
   - Is UBL enabled on all buckets?
   - Is storage.googleapis.com included in the VPC-SC perimeter?
   - Is the org policy constraints/storage.publicAccessPrevention enforced?
   - Are delete lifecycle rules on unversioned buckets?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout (VPC-SC dry-run first), validation, and rollback.
   - Flag allUsers bindings as CRITICAL requiring immediate removal.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP GCS Data Perimeter Governor: <scope>
## Executive verdict
- Status: SECURE / EXPOSED / PARTIAL / NEEDS EVIDENCE
- Public access risk: <clean / CRITICAL — allUsers binding detected>
- Biggest gap:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended remediation actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Data residency and compliance
- Bucket locations: <summary>
- Org policy enforcement: <present / absent / unknown>
- Bucket Lock: <configured / missing / unknown>
## Residual risk
- <risk or explicit none>
```
