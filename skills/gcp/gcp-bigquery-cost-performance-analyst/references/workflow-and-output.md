# Workflow and output contract

Use this reference only when performing a full BigQuery cost and performance review, incident triage, or implementation guidance.

## BigQuery areas to check

- Billing mode: on-demand vs. slot reservation; slot tier (Standard/Enterprise/Enterprise Plus); autoscaling configuration
- Slot utilization: peak vs. average consumption; reservation assignment scope (project/folder/org); spillover to on-demand
- BI Engine: reservation size vs. query patterns; cache hit rate; accelerated vs. non-accelerated queries
- Partitioning and clustering: partition columns, pruning effectiveness, clustering key selection, partition expiration
- Query cost drivers: top queries by bytes billed from `INFORMATION_SCHEMA.JOBS`; SELECT * usage; unfiltered large table scans; wildcard table patterns
- Dataset governance: expiration policies; IAM role bindings (`roles/bigquery.admin` findings); authorized views and row-level security
- Cross-region transfers: dataset locations; JOIN patterns across regions; egress cost exposure
- Materialized views: usage, refresh schedule, and cost-vs-freshness trade-offs

## Safe workflow

1. **Frame scope** — confirm project, dataset, billing mode, environment, and explicit non-goals
2. **Collect evidence** — prefer live state from `INFORMATION_SCHEMA` and Cloud Billing export; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius? what queries or jobs fail? what cost spike is possible? what is missing?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# GCP BigQuery Cost and Performance: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
