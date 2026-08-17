# Official Sources

Primary Databricks system-table, job, disaster-recovery, and compute-limit documentation.

Primary sources, verified 2026-08-17 against current official Databricks documentation. Each was fetched and read; a source that could not be reached is not listed here.

- https://docs.databricks.com/aws/en/admin/system-tables/
- https://docs.databricks.com/aws/en/admin/system-tables/audit-logs
- https://docs.databricks.com/aws/en/resources/limits
- https://docs.databricks.com/aws/en/jobs/configure-task
- https://docs.databricks.com/aws/en/jobs/monitor
- https://docs.databricks.com/aws/en/admin/clusters/policy-definition
- https://docs.databricks.com/aws/en/compute/pools
- https://docs.databricks.com/aws/en/compute/choose-compute
- https://docs.databricks.com/aws/en/admin/disaster-recovery
- https://docs.databricks.com/aws/en/lakehouse-architecture/deployment-guide/ha-dr

## Authority ranking

1. `FIRST_PARTY` — Databricks documentation, Databricks API/SDK reference, and the provider's own deprecation pages. Every claim in this skill that constrains a decision must trace to one of these.
2. `STANDARD_BODY` — Apache Spark, Delta Lake, MLflow, and OpenTelemetry project documentation for behaviour Databricks inherits rather than defines.
3. `SECONDARY` — blogs, conference talks, and press. Leads only. Never cited as evidence and never sufficient to encode a behaviour claim.

## Grounding rule

Documentation explains how the platform behaves in general. It does not prove the user's workspace configuration, Databricks Runtime version, compute type, region, cloud, edition, or actual grant state. Treat any claim that depends on those as `assumption` until an artifact or a sampled read-only query result confirms it, and name which artifact would settle it.
