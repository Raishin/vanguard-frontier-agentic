# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Database type (AlloyDB vs. Cloud SQL), engine version, and region
- HA configuration (standby zone, failover tested, connection endpoint stability)
- Connection method (Auth Proxy vs. direct IP, pgBouncer pool size)
- Backup and PITR (retention period, PITR enabled, last successful backup)
- Performance (slow query log, connection count vs. max_connections, Index Advisor findings)
- Maintenance window (scheduled, off-peak, notification contacts)
- IAM and network (private IP vs. public IP, authorized networks, SA permissions)

## Safe workflow

1. **Frame scope**
   - Project/region and instance or cluster name:
   - Database type (AlloyDB/Cloud SQL) and engine version:
   - Workload type (OLTP/OLAP/mixed):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Is the instance using public IP without proper authorized network restrictions?
   - Is PITR disabled for a production database?
   - Is connection pooling absent, risking connection exhaustion?
   - Is the maintenance window unset or during peak hours?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP AlloyDB and Cloud SQL DBA: <scope>
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
