# Safety checklist — SAP HANA Cloud Performance and Cost Review

Use before making any remediation recommendation, especially for findings that affect production instance sizing, table partitioning, index changes, or SQLScript procedure rewrites.

## Non-negotiables

- Do not access, connect to, or request access to any live HANA Cloud instance, execute SQL statements, invoke HANA Cloud APIs, or access HANA Cloud Central. This skill reviews artifacts only.
- Do not accept or request HANA Cloud instance credentials, database user credentials, SQL user passwords, connection strings, service keys, or OAuth tokens for any HANA Cloud instance.
- Do not recommend online partitioning or index changes on a production HANA Cloud table without first confirming the table size, active connections, and whether a maintenance window is available. Online ALTER TABLE operations for partitioning on large tables consume significant resources and can affect production query performance during execution.
- Do not recommend scaling down a HANA Cloud instance (reducing memory or vCPU) without first confirming current and peak memory utilization from M_LOAD_HISTORY_SERVICE data. Scaling down below the working memory requirement of the currently loaded data causes immediate out-of-memory failures.
- Do not recommend enabling auto-stop on an instance that may be serving production workloads. Auto-stop is appropriate only for explicitly non-production instances. Confirm the instance role with the user before recommending auto-stop.
- Do not recommend SQLScript procedure rewrites on production procedures without a development-environment test and SQLSCRIPT PLAN PROFILER validation of the rewritten procedure's execution plan.
- Do not classify a finding as `critical` without being able to trace the specific availability risk, data loss scenario, or resource exhaustion path from user-provided evidence or official documentation.

## What people get wrong

- **Recommending column store indexes as the default solution for slow queries**: HANA Cloud column store is designed for full-column scans using SIMD-based compression. Adding an inverted index on a column store table improves performance only for highly selective single-value lookups. For range scans and aggregations, indexes on column store tables often have no positive effect and increase DML overhead. EXPLAIN PLAN analysis must confirm whether an index would actually be used before recommending it.
- **Confusing HANA Cloud scale-out nodes with dynamic tiering**: HANA Cloud scale-out adds additional in-memory processing nodes for horizontal partitioning and workload distribution. Dynamic tiering (NSE warm store) moves infrequently accessed data to disk storage within the same instance. They address different problems — scale-out for data volume and concurrency; tiering for cost reduction on cold data.
- **Applying HANA Platform on-premise performance guidance directly to HANA Cloud**: Some HANA Platform on-premise recommendations (manual delta merge tuning, specific internal parameter changes) are either not applicable or not user-configurable in HANA Cloud. HANA Cloud manages many internal parameters automatically. Recommending parameter changes that are not user-accessible in HANA Cloud is incorrect.
- **Treating all cursor-based SQLScript as an anti-pattern**: Cursors in SQLScript are appropriate for row-by-row business logic that genuinely requires procedural iteration (sequential number generation, row-dependent conditional branching). The anti-pattern is using a cursor to iterate over large column store result sets when the same result can be achieved with a set-based INSERT INTO ... SELECT. Not every cursor is a performance problem.
- **Recommending workload class changes without confirming existing class bindings**: Workload classes apply to mapped users and application connections. Recommending new resource limits on an existing workload class without confirming which users and applications are currently mapped to it risks applying resource throttling to unintended production workloads.
- **Missing the plan cache invalidation risk from UPDATE STATISTICS**: Updating table statistics triggers SQL Plan Cache invalidation for all queries referencing the updated table. On a busy production system this can cause a temporary plan recompilation storm. UPDATE STATISTICS on large production tables should be executed during low-traffic windows.

## When to push back

- Push back when the user asks to confirm a query performance finding without providing EXPLAIN PLAN output, SQL Plan Cache data, or expensive statement trace results.
- Push back when the user asks for specific memory sizing numbers without providing current M_LOAD_HISTORY_SERVICE utilization data or a workload description with concurrency and data volume details.
- Push back when the user asks to recommend changes to production tables (partitioning, indexing) without confirming current table size, active connections, and maintenance window availability.
- Push back when a request requires live HANA Cloud instance access, SQL execution, or system view query — state clearly that live inspection is out of scope and ask the user to supply the relevant outputs or descriptions.

## Evidence labels

- `documentation-based` — grounded in SAP HANA Cloud Help Portal documentation covering sizing, modeling, partitioning, indexes, SQL execution plans, SQLScript, cost management, or monitoring
- `user-provided evidence` — EXPLAIN PLAN output, SQL Plan Cache exports, expensive statement trace results, monitoring view query outputs, architecture documents, or descriptions provided by the user
- `context7-supplementary` — grounded in SAP HANA Cloud SQL reference or SQLScript documentation from Context7 (supplementary; applies when SQL statement-level or SQLScript procedure-level guidance is in scope)
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
