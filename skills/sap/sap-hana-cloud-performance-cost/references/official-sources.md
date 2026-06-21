# Official sources — SAP HANA Cloud Performance and Cost Review

Use this reference when grounding HANA Cloud sizing, data modeling, query performance, SQLScript design, cost management, and monitoring assessments.

**Evidence level**: documentation-based (SAP HANA Cloud Help Portal). No live-system evidence is collected by this skill.

## Sizing and instance management

- Sizing SAP HANA Cloud
  https://help.sap.com/docs/hana-cloud/sap-hana-cloud-getting-started-guide/sizing-sap-hana-cloud
  source_owner: SAP SE
  topic_supported: HANA Cloud instance sizing guidance, vCPU and memory tier selection, sizing rules for in-memory footprint, disk storage sizing, and elasticity configuration
  why_needed: Primary reference for classifying sizing findings including under-provisioned memory for in-memory workloads, incorrectly sized compute tiers, and absent elasticity configuration for variable workloads
  evidence_level: primary
  last_verified: 2026-06-19

- Managing SAP HANA Cloud instances
  https://help.sap.com/docs/hana-cloud/sap-hana-cloud-getting-started-guide/managing-sap-hana-cloud
  source_owner: SAP SE
  topic_supported: HANA Cloud Central instance management, auto-stop configuration, compute scaling (scale-up, scale-out), storage auto-scaling, instance lifecycle governance
  why_needed: Defines auto-stop and compute scaling controls — required to classify non-production instances without auto-stop and uncontrolled compute scaling as cost governance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Data modeling

- Table partitioning in SAP HANA Cloud
  https://help.sap.com/docs/hana-cloud-database/sap-hana-cloud-sap-hana-database-administration-guide/table-partitioning
  source_owner: SAP SE
  topic_supported: Partitioning types (range, hash, round-robin, multilevel), partition key selection, partition pruning behavior, partition count guidelines for large tables
  why_needed: Defines the partitioning model — required to classify missing partitioning on large column store tables as a performance finding and to assess partition key selection for pruning effectiveness
  evidence_level: primary
  last_verified: 2026-06-19

- Indexes in SAP HANA Cloud
  https://help.sap.com/docs/hana-cloud-database/sap-hana-cloud-sap-hana-database-administration-guide/indexes
  source_owner: SAP SE
  topic_supported: Index types (inverted index, composite inverted index, full text index), column store implicit primary key index, index usage in query execution plans, when indexes improve vs. hurt column store performance
  why_needed: Defines the index model for HANA Cloud column store — required to assess index design decisions, identify redundant indexes that increase DML overhead, and evaluate whether missing indexes are causing full-scan performance issues
  evidence_level: primary
  last_verified: 2026-06-19

## Query performance

- Analyzing SQL execution plans in SAP HANA Cloud
  https://help.sap.com/docs/hana-cloud-database/sap-hana-cloud-sap-hana-database-performance-guide/analyzing-sql-execution-plans
  source_owner: SAP SE
  topic_supported: EXPLAIN PLAN statement usage, SQL Plan Cache analysis, plan operator interpretation (column search, row search, join engine), identifying plan regressions, optimizer hints for access path control
  why_needed: Primary reference for interpreting user-provided EXPLAIN PLAN output and SQL Plan Cache data — required to classify query plan issues (wrong join engine, missing column search, suboptimal access path) as performance findings
  evidence_level: primary
  last_verified: 2026-06-19

- Monitoring performance in SAP HANA Cloud
  https://help.sap.com/docs/hana-cloud-database/sap-hana-cloud-sap-hana-database-performance-guide/monitoring-performance
  source_owner: SAP SE
  topic_supported: Expensive statement trace configuration, M_EXPENSIVE_STATEMENTS view, thread sampling, M_SQL_PLAN_CACHE, M_LOAD_HISTORY_SERVICE, workload analysis monitoring views
  why_needed: Defines the monitoring model — required to classify missing expensive statement trace configuration, absent thread sampling, and no workload class monitoring as monitoring blind spot findings
  evidence_level: primary
  last_verified: 2026-06-19

## SQLScript

- SAP HANA Cloud SQLScript reference
  https://help.sap.com/docs/hana-cloud-database/sap-hana-cloud-sap-hana-database-sqlscript-reference/sap-hana-cloud-sap-hana-database-sqlscript-reference
  source_owner: SAP SE
  topic_supported: SQLScript language reference, tabular functions, scalar functions, cursor vs. set-based logic, SQLSCRIPT PLAN PROFILER, procedure call stack analysis
  why_needed: Defines the SQLScript execution model — required to classify cursor-based anti-patterns, missing tabular function use for reusable logic, and procedure plan stability issues as SQLScript performance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Cost management and data tiering

- Workload management in SAP HANA Cloud
  https://help.sap.com/docs/hana-cloud-database/sap-hana-cloud-sap-hana-database-administration-guide/workload-management
  source_owner: SAP SE
  topic_supported: Workload class definition, workload mapping rules, resource limits (CPU, memory, query timeout) per workload class, priority-based execution scheduling
  why_needed: Defines the workload governance model — required to classify missing workload class definitions, absent resource limits, and unmanaged query concurrency as cost and resource governance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP HANA Cloud Help Portal documentation describes the designed behavior of column store tables, partitioning strategies, index types, SQL execution plans, SQLScript procedures, and cost management controls. It does not prove which tables exist in the user's instance, what execution plans are currently active, or whether auto-stop is configured. Users must supply EXPLAIN PLAN output, SQL Plan Cache exports, trace results, monitoring view query outputs, or written descriptions for concrete assessment.
