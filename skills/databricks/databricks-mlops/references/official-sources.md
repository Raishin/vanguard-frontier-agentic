# Official Sources

Primary MLflow 3, Unity Catalog model registry, and Model Serving documentation.

Primary sources, verified 2026-08-17 against current official Databricks documentation. Each was fetched and read; a source that could not be reached is not listed here.

- https://docs.databricks.com/aws/en/machine-learning/manage-model-lifecycle/
- https://docs.databricks.com/aws/en/mlflow/
- https://docs.databricks.com/aws/en/mlflow/model-registry-3
- https://docs.databricks.com/aws/en/machine-learning/model-serving/
- https://docs.databricks.com/aws/en/machine-learning/model-serving/inference-tables
- https://docs.databricks.com/aws/en/machine-learning/feature-store/uc/feature-tables-uc
- https://docs.databricks.com/aws/en/machine-learning/automl/

## Source notes

- Feature engineering and SDK client surfaces were cross-checked against the Context7 MCP (`/websites/databricks`, `/databricks/databricks-sdk-py`) in addition to Databricks documentation.

## Authority ranking

1. `FIRST_PARTY` — Databricks documentation, Databricks API/SDK reference, and the provider's own deprecation pages. Every claim in this skill that constrains a decision must trace to one of these.
2. `STANDARD_BODY` — Apache Spark, Delta Lake, MLflow, and OpenTelemetry project documentation for behaviour Databricks inherits rather than defines.
3. `SECONDARY` — blogs, conference talks, and press. Leads only. Never cited as evidence and never sufficient to encode a behaviour claim.

## Grounding rule

Documentation explains how the platform behaves in general. It does not prove the user's workspace configuration, Databricks Runtime version, compute type, region, cloud, edition, or actual grant state. Treat any claim that depends on those as `assumption` until an artifact or a sampled read-only query result confirms it, and name which artifact would settle it.
