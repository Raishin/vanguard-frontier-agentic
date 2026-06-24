# Context7 framework docs — SAP HANA Cloud Performance and Cost Review

**Role**: supplementary. Official SAP HANA Cloud Help Portal documentation is the primary source for all sizing, modeling, and performance review guidance. Context7-sourced SAP HANA Cloud documentation supplements guidance specifically for SQL statement-level and SQLScript procedure-level analysis — EXPLAIN PLAN syntax, execution plan operator interpretation, SQLScript plan profiler commands, and column store optimizer hints.

**Library used**: SAP HANA Cloud  
Context7 library ID: `/sap-archive/sap-hana-cloud-hana-database`  
Lookup target: EXPLAIN PLAN syntax, SQL Plan Cache queries, SQLScript PLAN PROFILER, column store optimizer hints, row store vs. column store operator behavior  
Skill: `sap-hana-cloud-performance-cost`  
Classification: supplementary

---

## EXPLAIN PLAN for SQL execution plan analysis (supplementary)

Source: SAP HANA Cloud SQL Reference (Context7 `/sap-archive/sap-hana-cloud-hana-database`)

The EXPLAIN PLAN statement captures the optimizer's execution plan for a SQL query without executing it. The captured plan is written to the EXPLAIN_PLAN_TABLE system view, queryable by STATEMENT_NAME. This is used to identify suboptimal access paths (row engine operations on column store tables, missing partition pruning, or cartesian joins) from user-provided query text.

```sql
DELETE FROM explain_plan_table WHERE statement_name = 'MY_QUERY';

EXPLAIN PLAN SET STATEMENT_NAME = 'MY_QUERY' FOR
  SELECT * FROM SALES_FACT SF
  JOIN CUSTOMER DIM ON SF.CUSTOMER_ID = DIM.CUSTOMER_ID
  WHERE SF.POSTING_DATE >= '2025-01-01';

SELECT operator_name, operator_details, table_name
  FROM explain_plan_table
  WHERE statement_name = 'MY_QUERY';
```

**Relevance to performance review**: When the user provides a slow query, the EXPLAIN PLAN output (or the equivalent SQL Plan Cache entry) is the primary artifact for classifying query performance findings. Key signals to look for:
- `ROW SEARCH` operator on a large column store table indicates row engine forced execution — likely caused by a data type mismatch in the join predicate or an explicit `WITH HINT(COLUMN_VIEW_AS_ROW_TABLE)`.
- Missing `COLUMN SEARCH` operator on the primary filter column indicates the optimizer did not use the column store scan path.
- `HASH JOIN` on a very large outer table without partition pruning indicates a missing or non-selective partition key.

## SQLScript Plan Profiler for procedure performance (supplementary)

Source: SAP HANA Cloud system management statements (Context7 `/sap-archive/sap-hana-cloud-hana-database`)

The SQLScript Plan Profiler captures execution statistics for individual SQLScript procedure steps. It can be activated for a specific session or a specific procedure to isolate which steps within a complex procedure consume the most time. This is the correct tool for diagnosing performance issues inside SQLScript procedures when the overall procedure execution time is unacceptably high.

```sql
-- Start profiling a specific procedure
ALTER SYSTEM START SQLSCRIPT PLAN PROFILER FOR PROCEDURE MYSCHEMA.MY_PROC;

-- Execute the procedure to capture profiling data
CALL MYSCHEMA.MY_PROC();

-- Stop the profiler
ALTER SYSTEM STOP SQLSCRIPT PLAN PROFILER;

-- Query profiler results to identify expensive steps
SELECT * FROM M_SQLSCRIPT_PLAN_PROFILERS;
```

**Relevance to performance review**: When the user provides SQLScript procedure code that is suspected to contain a cursor-based anti-pattern or an expensive intermediate step, the SQLSCRIPT PLAN PROFILER is the recommended tool to confirm which specific step dominates execution time before recommending a rewrite. Without profiler data, procedure performance recommendations are classified as `inference`.

## Column store optimizer hints for access path control (supplementary)

Source: SAP HANA Cloud hint reference (Context7 `/sap-archive/sap-hana-cloud-hana-database`)

HANA Cloud's query optimizer selects between column engine and row engine execution paths based on its cost model. In some cases the optimizer's estimates differ from actual performance due to stale statistics or skewed data distributions. Optimizer hints can override the access path decision for a specific query or plan cache entry.

Key hints for column store performance review:

- `CS_ALL_COLUMNS`: Forces the optimizer to use the column store for all table accesses in the query. Use when EXPLAIN PLAN shows unexpected row engine access on a large column store table.
- `NO_CS_JOIN`: Prohibits the column engine from executing the join. Use only when the column engine join is demonstrably slower than a row engine join (rare; typically only for very small tables).
- `USE_HEX_PLAN`: Forces use of the hexagonal (HEX) join engine plan for complex multi-table joins. The HEX engine can outperform the classic join engine for star schema queries with multiple large fact-to-dimension joins.

Example of applying a hint in a query:

```sql
SELECT /* CS_ALL_COLUMNS */ SF.POSTING_DATE, SUM(SF.AMOUNT)
FROM SALES_FACT SF
JOIN CUSTOMER DIM ON SF.CUSTOMER_ID = DIM.CUSTOMER_ID
GROUP BY SF.POSTING_DATE;
```

**Relevance to performance review**: When a user provides EXPLAIN PLAN output showing unexpected row engine operator usage on a column store table, the access path hint guidance above is applied as `context7-supplementary` evidence. The recommended hint must be validated against a fresh EXPLAIN PLAN run on the user's system — hints are advisory supplements, not guaranteed fixes without execution plan confirmation.

---

## Scope boundaries for Context7 usage

Context7 SAP HANA Cloud documentation applies **only** for SQL statement-level and SQLScript procedure-level analysis where the user has provided query text, EXPLAIN PLAN output, SQL Plan Cache data, or SQLScript code. It does not apply to:

- HANA Cloud instance sizing decisions — use SAP HANA Cloud Getting Started Guide (official docs)
- Table partitioning design — use SAP HANA Cloud Database Administration Guide (official docs)
- Index type selection — use SAP HANA Cloud Database Administration Guide (official docs)
- Workload class and resource limit configuration — use SAP HANA Cloud Database Administration Guide (official docs)
- Auto-stop and compute scaling — use HANA Cloud Central documentation (official docs)

Always label Context7-sourced guidance as `context7-supplementary` in responses.
