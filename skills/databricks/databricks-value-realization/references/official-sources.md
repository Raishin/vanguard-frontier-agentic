# Official Sources

Primary Databricks architecture, billing system-table, usage-management, and data-quality documentation underpinning the cost-side evidence in a value case.

Primary sources, verified 2026-08-17 against current official Databricks documentation. Each was fetched and read; a source that could not be reached is not listed here.

- https://docs.databricks.com/aws/en/lakehouse-architecture/well-architected
- https://docs.databricks.com/aws/en/admin/system-tables/billing
- https://docs.databricks.com/aws/en/admin/system-tables/
- https://docs.databricks.com/aws/en/admin/usage/
- https://docs.databricks.com/aws/en/data-governance/unity-catalog/data-quality-monitoring/

## Source notes

- Only the cost side of a value case is groundable in Databricks documentation. The benefit side is grounded in the organisation's own financial and operational systems, which no documentation page can substitute for.
- `system.billing.usage` and `system.billing.list_prices` are GA and are the only defensible Databricks-side source for spend.
- Workload evidence used to build efficiency ratios draws on system tables of mixed maturity; a ratio resting on a PUBLIC PREVIEW table inherits that table's schema risk and must say so.

## Authority ranking

1. `FIRST_PARTY` — Databricks documentation, Databricks API/SDK reference, and the provider's own deprecation pages. Every claim in this skill that constrains a decision must trace to one of these.
2. `STANDARD_BODY` — Apache Spark, Delta Lake, MLflow, and OpenTelemetry project documentation for behaviour Databricks inherits rather than defines.
3. `SECONDARY` — blogs, conference talks, and press. Leads only. Never cited as evidence and never sufficient to encode a behaviour claim.

## Grounding rule

Documentation explains how the platform behaves in general. It does not prove the user's workspace configuration, Databricks Runtime version, compute type, region, cloud, edition, or actual grant state. Treat any claim that depends on those as `assumption` until an artifact or a sampled read-only query result confirms it, and name which artifact would settle it.
