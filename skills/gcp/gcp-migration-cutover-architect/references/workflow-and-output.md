# Workflow and output contract

Use this reference only when performing the full migration plan, cutover review, implementation guidance, or go/no-go assessment.

## Review domains

Check these areas before giving a verdict:
- Migration scope: source environment type (VMware/AWS/Azure/physical), workload count, data volume
- Tool selection: Migrate to VMs (compute), DMS (database), Storage Transfer (bulk data) — confirm correct tool per workload type
- DMS replication: CDC lag, replication state, last checkpoint, validation query results
- Cutover sequence: lowest-risk workloads first, dependency mapping, timing windows
- Rollback: original source still running, DNS TTL reduced, rollback trigger criteria defined
- Data validation: row count comparison, checksum validation, application smoke tests
- DNS: TTL reduced to 60s at least 24-48h before cutover; revert DNS is the first rollback step

## Safe workflow

1. **Frame scope**
   - Source environment and target GCP environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API and DMS/MigrateOps read-only evidence if available.
   - Otherwise inspect source environment documentation, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What is the rollback procedure if cutover fails at each step?
   - What data validation checks confirm replication fidelity?
   - What dependencies are not yet mapped?
   - What evidence is missing to approve cutover?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Migration Cutover Architect: <scope>
## Executive verdict
- Status: GO / GO WITH CONDITIONS / NO-GO / NEEDS EVIDENCE
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
