# Workflow and output contract

Use this reference only when performing the full landing zone review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Organization node, billing account linkage, and domain verification
- Folder hierarchy (bootstrap, security, prod, non-prod, sandbox)
- Org policy baseline (SA key creation disabled, member domain restriction, OS Login required, public IP restriction)
- Shared VPC host projects per environment — no workloads in host projects
- Security Command Center tier (Standard vs Premium) and active findings
- Cloud Asset Inventory enabled and export configured
- Audit log types: Admin Activity (always on), Data Access (must be explicitly enabled), System Event
- Centralized log sink to BigQuery or Cloud Storage

## Safe workflow

1. **Frame scope**
   - Organization ID and domain:
   - Existing folder structure and projects:
   - Compliance and regulatory drivers:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Which org policies are missing or only applied at folder level when they should be org-wide?
   - Which Data Access audit logs are disabled for sensitive services?
   - Are workloads deployed in the Shared VPC host project?
   - Is SCC enabled and are critical findings acknowledged?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Landing Zone Architect: <scope>
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
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
