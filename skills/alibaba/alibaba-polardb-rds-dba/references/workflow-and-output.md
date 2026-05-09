# Workflow and output contract

Use this reference only when performing a full DBA review, backup audit, or performance tuning assessment.

## Review domains

Check these areas before giving a verdict:

- Database engine type (PolarDB MySQL/PG/Oracle vs. RDS MySQL/PG/SQL Server/MariaDB) and version
- Cluster topology: primary node, read nodes, GDN cross-region
- DAS diagnostic findings: slow queries, execution plan anomalies, wait events
- Database proxy: connection pooling enabled, read/write splitting configured
- Backup policy: automated backup frequency, retention period, cross-region backup
- Performance metrics: QPS, TPS, connection count, buffer pool hit rate

## Safe workflow

1. **Frame scope**
   - Database type, engine, and version:
   - Environment (dev/staging/prod):
   - Business criticality and RTO/RPO requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live DAS diagnostics or console/API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What happens if the primary node fails?
   - Are backups tested and within RPO?
   - What queries consume the most resources?
   - What DAS findings are unresolved?
4. **Recommend the smallest safe action**
   - Prefer query hint or index addition before schema changes.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Database Review: <scope>
## Executive verdict
- Status: HEALTHY / ATTENTION NEEDED / ACTION REQUIRED
- Biggest risk:
- Evidence level:
## Database type and version
- Engine:
- Version:
- High availability mode:
## Cluster topology
- Primary node:
- Read nodes:
- GDN cross-region:
## DAS diagnostic findings
| Severity | Finding | Query / resource | Recommended action |
|---|---|---|---|
## Proxy configuration
- Connection pooling:
- Read/write splitting:
## Backup policy review
- Automated backup frequency:
- Retention period:
- Cross-region backup:
- Last restore test:
## Performance recommendations
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Open risks
- <risk or explicit none>
```
