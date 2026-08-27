# Official Sources

Primary MLflow Tracing, Evaluation, Judge, and observability documentation.

Primary sources, verified 2026-08-17 against current official Databricks documentation. Each was fetched and read; a source that could not be reached is not listed here.

- https://docs.databricks.com/aws/en/mlflow3/genai/
- https://docs.databricks.com/aws/en/mlflow3/genai/tracing
- https://docs.databricks.com/aws/en/mlflow3/genai/eval-monitor/
- https://docs.databricks.com/aws/en/mlflow3/genai/eval-monitor/concepts/scorers
- https://docs.databricks.com/aws/en/mlflow3/genai/eval-monitor/concepts/judges/
- https://docs.databricks.com/aws/en/mlflow3/genai/getting-started/
- https://docs.databricks.com/aws/en/ai-gateway/cost-observability
- https://docs.databricks.com/aws/en/admin/system-tables/

## Source notes

- MLflow client API surfaces in this skill were cross-checked against the Context7 MCP (`/websites/mlflow_genai` and `/mlflow/mlflow`, which carries a v3.1.4 entry) in addition to Databricks documentation. Where the two could differ, Context7 library documentation is authoritative for the client API signature and Databricks documentation is authoritative for service behaviour.

## Authority ranking

1. `FIRST_PARTY` — Databricks documentation, Databricks API/SDK reference, and the provider's own deprecation pages. Every claim in this skill that constrains a decision must trace to one of these.
2. `STANDARD_BODY` — Apache Spark, Delta Lake, MLflow, and OpenTelemetry project documentation for behaviour Databricks inherits rather than defines.
3. `SECONDARY` — blogs, conference talks, and press. Leads only. Never cited as evidence and never sufficient to encode a behaviour claim.

## Grounding rule

Documentation explains how the platform behaves in general. It does not prove the user's workspace configuration, Databricks Runtime version, compute type, region, cloud, edition, or actual grant state. Treat any claim that depends on those as `assumption` until an artifact or a sampled read-only query result confirms it, and name which artifact would settle it.
