# Official Sources

Primary Databricks Lakeflow, Delta, and table-optimization documentation.

Primary sources, verified 2026-08-17 against current official Databricks documentation. Each was fetched and read; a source that could not be reached is not listed here.

- https://docs.databricks.com/aws/en/ldp
- https://docs.databricks.com/aws/en/ldp/concepts/where-is-dlt
- https://docs.databricks.com/aws/en/delta-live-tables/develop
- https://docs.databricks.com/aws/en/delta-live-tables/configure-pipeline
- https://docs.databricks.com/aws/en/tables/clustering
- https://docs.databricks.com/aws/en/delta/optimize
- https://docs.databricks.com/aws/en/optimizations/predictive-optimization
- https://docs.databricks.com/aws/en/ingestion/cloud-object-storage/auto-loader/
- https://docs.databricks.com/aws/en/lakehouse/medallion

## Source notes

- Pipelines and Delta table-feature surfaces were cross-checked against the Context7 MCP (`/websites/spark_apache_api_python`, `/delta-io/delta`). Where the Apache Spark or Delta Lake library documentation and the Databricks service documentation describe different layers, this skill names the layer instead of merging them.

## Authority ranking

1. `FIRST_PARTY` — Databricks documentation, Databricks API/SDK reference, and the provider's own deprecation pages. Every claim in this skill that constrains a decision must trace to one of these.
2. `STANDARD_BODY` — Apache Spark, Delta Lake, MLflow, and OpenTelemetry project documentation for behaviour Databricks inherits rather than defines.
3. `SECONDARY` — blogs, conference talks, and press. Leads only. Never cited as evidence and never sufficient to encode a behaviour claim.

## Grounding rule

Documentation explains how the platform behaves in general. It does not prove the user's workspace configuration, Databricks Runtime version, compute type, region, cloud, edition, or actual grant state. Treat any claim that depends on those as `assumption` until an artifact or a sampled read-only query result confirms it, and name which artifact would settle it.
