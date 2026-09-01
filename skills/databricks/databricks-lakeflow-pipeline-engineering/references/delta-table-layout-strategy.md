# Delta Table Layout And Optimization Strategy

Comprehensive guidance on table clustering (liquid vs Z-order), deletion vectors (DBR 15.4+), and Predictive Optimization (tier-dependent).

- Databricks recommends liquid clustering for all new tables instead of Z-ordering because OPTIMIZE under liquid clustering is incremental — it rewrites only data that needs clustering — whereas Z-order performs a full rewrite of affected files.
- Deletion vectors mark rows invalidated instead of rewriting files and require Databricks Runtime 15.4 LTS or above; on older DBR versions, OPTIMIZE rewrites files to remove deleted rows.
- Predictive Optimization automatically runs OPTIMIZE, VACUUM, and ANALYZE on Unity Catalog managed tables and is available only on Standard, Premium, and Enterprise tiers — Community tier does not support it.
- Z-order is retained for backward compatibility and complex multi-column predicates, but new table designs should prefer liquid clustering for its efficiency and simplicity.
- Partitioning is still appropriate for tables with very large volume (multiple TiB) where filtering by partition key dramatically reduces scan scope; liquid clustering and partitioning can coexist but are not equivalent — partition pruning precedes clustering.

## Sources

- https://docs.databricks.com/aws/en/tables/clustering
- https://docs.databricks.com/aws/en/delta/optimize
- https://docs.databricks.com/aws/en/optimizations/predictive-optimization
