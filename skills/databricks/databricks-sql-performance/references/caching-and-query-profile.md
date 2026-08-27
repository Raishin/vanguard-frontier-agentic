# Caching, Query Profile, And Data Layout

Three-tier caching semantics, query-profile reading for skew and spill detection, and data-layout choices.

- The UI cache persists across warehouse restarts for up to 7 days; the remote result cache survives restart with a 24-hour lifecycle and invalidates on any table schema change; the local disk cache is per-node SSD and auto-invalidates when a file changes. Setting `use_cached_result = false` disables result reuse for a single query.
- Query profile exposes wall-clock duration, aggregated task time summed across cores, peak memory, shuffle and spill sizes, and task-duration percentiles. Task-duration skew (maximum > 75th percentile + 50%) indicates data skew — read the percentile distribution to spot it.
- Liquid clustering is the recommended layout for all new tables; `ALTER TABLE <t> CLUSTER BY (col, ...)` redefines keys without a rewrite, and `CLUSTER BY AUTO` enables automatic clustering. Partitions should be at least 1 GB each.
- Data-skipping statistics are auto-collected for the first 32 columns (configurable by `dataSkippingNumIndexedCols` or explicit columns via `dataSkippingStatsColumns`); statistics are order-dependent and tuned for high-selectivity columns.
- `ANALYZE TABLE <t> COMPUTE STATISTICS NOSCAN` produces byte-size stats only; `FOR ALL COLUMNS` adds column statistics; the `DELTA` variant refreshes Delta log statistics rather than optimizer statistics.
- Materialized views update incrementally on a schedule with seconds-to-minutes latency; some schema changes force a full recompute, and materialized views cannot be CLONEd.

## Sources

- https://docs.databricks.com/aws/en/sql/user/queries/query-caching
- https://docs.databricks.com/aws/en/sql/user/queries/query-profile
- https://docs.databricks.com/aws/en/tables/clustering
- https://docs.databricks.com/aws/en/tables/data-skipping
- https://docs.databricks.com/aws/en/tables/partitions
