# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Schema design (primary key type, hotspot risk, interleaving opportunities)
- Index inventory (secondary indexes vs. access patterns, over-indexing signal)
- Processing unit sizing (current PU, peak CPU utilization, autoscaling enabled)
- Instance configuration (single-region vs. multi-region, write latency SLA)
- Transaction patterns (read-write vs. read-only, stale reads for analytics)
- IAM (roles/spanner.databaseUser vs. broader bindings)

## Safe workflow

1. **Frame scope**
   - Project/instance/database:
   - Workload type (OLTP/analytics/mixed):
   - Write throughput and latency SLA:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/DDL, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Does any table use a monotonically increasing primary key?
   - Are parent-child tables interleaved where the access pattern justifies it?
   - Are there secondary indexes that cover columns never used in WHERE clauses?
   - Is processing unit utilization regularly above 65% (scaling headroom risk)?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Spanner Architect: <scope>
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
