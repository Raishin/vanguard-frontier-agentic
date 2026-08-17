# Lakehouse Monitoring, Freshness Detection, And Event Log Interrogation

Configuration and interrogation guide for profile/drift metrics, freshness anomaly detection, and event-log quality results.

- Lakehouse Monitoring emits two tables per monitored table: `{output_schema}.{table_name}_profile_metrics` (summary statistics: count, nulls, average, quantiles, distinct counts per column, per time window, per slice, per grouping) and `{output_schema}.{table_name}_drift_metrics` (consecutive and baseline comparisons using chi-square, Kolmogorov-Smirnov, Wasserstein, Jensen-Shannon distance).
- Freshness anomaly detection builds a per-table model predicting the next commit time and marks a table stale when a commit is unusually late; the staleness marker is learned, not a hardcoded wall-clock time.
- Pipeline event logs carry audit logs, data-quality checks, pipeline progress, and lineage. Event types include `update_progress`, `flow_progress` (output rows, upserts/deletes, data-quality results), `flow_definition` (lineage), `operation_progress` (Auto Loader file ingestion), `stream_progress`, `autoscale`, `cluster_resources`, `user_action`, `runtime_details`.
- Event logs are queried via the SQL `event_log(<pipelineId>)` table-valued function, the Pipelines UI, or the Pipelines REST API.
- Pipeline run history and event logs are retained for 60 days; older events are deleted and not queryable.
- Quality metrics are per-table and per-column; they do not cross tables or automatically propagate upstream — consumer quality evidence must be explicitly published.

## Sources

- https://docs.databricks.com/aws/en/lakehouse-monitoring/
- https://docs.databricks.com/aws/en/data-governance/unity-catalog/data-quality-monitoring/
- https://docs.databricks.com/aws/en/ldp/monitor-event-logs
- https://docs.databricks.com/aws/en/ldp/observability
