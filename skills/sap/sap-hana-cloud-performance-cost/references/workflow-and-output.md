# Workflow and output contract — SAP HANA Cloud Performance and Cost Review

Use this reference for all finding classification, risk assignment, remediation path selection, and output formatting.

## Review domain taxonomy

| Domain | Scope | Typical findings |
|--------|-------|-----------------|
| `sizing-and-elasticity` | vCPU and memory tier selection, scale-out node design, compute auto-scaling, scheduled scaling | Under-provisioned memory for in-memory workload, absent auto-scaling for variable load, single-node design where scale-out is warranted |
| `data-modeling` | Column store vs. row store choice, table partitioning, index design, delta compression, sort column selection | Missing partitioning on tables over 100M rows, redundant indexes increasing DML overhead, row store table used for analytic scan workloads, missing sort column on frequently filtered column store table |
| `query-performance` | EXPLAIN PLAN operator analysis, SQL Plan Cache findings, expensive statement trace, optimizer hints | Unexpectedly high row engine operator usage, missing column search operator, suboptimal join order, plan instability between executions, missing access path hint |
| `sqlscript` | Cursor vs. set-based logic, tabular function design, procedure plan stability, SQLSCRIPT PLAN PROFILER findings | Cursor loop iterating over column store table (row-at-a-time anti-pattern), missing tabular function for reusable intermediate result, unstable procedure plan causing variable execution time |
| `cost-management` | Auto-stop configuration, compute tier selection, storage auto-scaling thresholds, data tiering (hot/warm/cold) | Non-production instance without auto-stop, compute tier over-provisioned for non-peak base load, missing warm-tier configuration for infrequently accessed historical data |
| `monitoring` | Expensive statement trace, M_EXPENSIVE_STATEMENTS, workload class definition, thread sampling, M_SQL_PLAN_CACHE | Expensive statement trace not configured, no workload class definitions, thread sampling not enabled, M_SQL_PLAN_CACHE not reviewed for plan evolution |

## Risk severity classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Production availability risk: memory under-provisioning causing OOM failures on production instance, workload resource exhaustion with no limit controls, unrecoverable performance degradation from missing partitioning on production tables at or near the 2-billion-row limit |
| `high` | Performance bottleneck or uncontrolled cost: cursor-based SQLScript anti-pattern on large column store table causing query timeout, non-production instances without auto-stop accumulating unbounded compute cost, missing expensive statement trace making incident diagnosis impossible |
| `medium` | Governance gap or monitoring blind spot: no workload class definitions for multi-tenant resource isolation, missing index on a high-frequency filter column, absent warm-tier configuration for historical data causing unnecessary hot-tier cost |
| `low` | Best practice deviation: missing sort column selection on a column store table, undocumented partition key rationale, procedure without plan stability documentation |

## Remediation path decision tree

For each finding:

1. **Is this a production instance memory under-provisioning risk or a table at or approaching the 2-billion-row limit without partitioning?**
   - Yes → `critical`. For memory: scale up the instance to the next tier immediately; obtain M_LOAD_HISTORY_SERVICE data to confirm memory utilization trend before scaling decision. For partitioning: add range partitioning on the primary date or key dimension; plan for online partition conversion using the HANA Cloud ALTER TABLE ... PARTITION BY syntax.
   - No → continue.

2. **Is this a cursor-based SQLScript anti-pattern on a column store table causing production query timeouts?**
   - Yes → `high`. Rewrite the cursor loop as a set-based INSERT INTO ... SELECT, CREATE TABLE AS SELECT, or tabular function. Use SQLSCRIPT PLAN PROFILER (ALTER SYSTEM START SQLSCRIPT PLAN PROFILER FOR PROCEDURE) to confirm the row engine execution path before and after rewrite.
   - No → continue.

3. **Is this a non-production instance without auto-stop causing unbounded compute cost?**
   - Yes → `high`. Configure auto-stop via HANA Cloud Central with a daily idle timeout appropriate for the environment (for example, stop after 2 hours of inactivity for development, 4 hours for QA). Document the expected schedule and confirm with instance owners.
   - No → continue.

4. **Is this a query performance finding identified from user-provided EXPLAIN PLAN or SQL Plan Cache data?**
   - Yes → `high` or `medium` depending on impact. For suboptimal access paths: add EXPLAIN PLAN hints (CS_ALL_COLUMNS for forced column engine, USE_HEX_PLAN for hexagonal join) and compare execution plan operators. For plan instability: check SQL Plan Cache for plan evolution history and consider UPDATE STATISTICS on the affected tables.
   - No → continue.

5. **Is this a governance, monitoring, or cost optimization gap?**
   - Yes → `medium`. Define workload classes with resource limits for query classes (reporting, planning, ETL). Enable expensive statement trace at the threshold appropriate for the workload profile. Configure warm-tier for data older than the retention threshold. 
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — EXPLAIN PLAN output, SQL Plan Cache exports, expensive statement trace results, monitoring view query outputs, architecture documents, or user descriptions.
2. **Classify each finding** by review domain and finding type.
3. **Assign risk level** (critical / high / medium / low).
4. **Apply remediation decision tree** per finding.
5. **Prioritize** — critical availability and data loss risks first; then high performance and cost findings; then medium governance and monitoring gaps; then low best-practice items.
6. **Return output** per the output contract below.

## Output contract

Return:

1. Review domain and specific finding type
2. Evidence label (documentation-based / user-provided evidence / context7-supplementary / inference)
3. Risk level per finding (critical / high / medium / low)
4. Recommended remediation action with specific implementation guidance (including SQL or SQLScript examples where the finding is code-level)
5. Expected performance or cost posture after remediation
6. Prioritized remediation sequence
7. Escalation trigger if live HANA Cloud instance access, SQL execution, or system view query is required to confirm or apply the finding
