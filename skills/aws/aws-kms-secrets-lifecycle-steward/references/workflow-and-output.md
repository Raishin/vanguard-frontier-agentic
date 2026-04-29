# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, incident triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Key purpose, data classification, owning service, key policy, IAM policy, grants, aliases, tags, deletion window, and break-glass path
- Rotation strategy, multi-Region behavior, imported key material, replica policies, CloudTrail evidence, and service integration constraints
- Secret type, rotation pattern, Lambda permissions, retry/client caching, replication, resource policy, VPC endpoint condition, and monitoring
- Recovery risk: backups, cross-account restore, DR Region, disabled/deleted keys, stale credentials, and operator ownership

## Safe workflow

1. **Frame scope**
   - Workload/account/Region/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live AWS MCP read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official AWS docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What can expose data?
   - What can escalate privilege?
   - What can break production or block rollback?
   - What can create unbounded cost?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# AWS KMS Secrets Lifecycle Steward: <scope>
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
