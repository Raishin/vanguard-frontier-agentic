# Official sources

Use this reference only when you need source grounding for GCP BigQuery service behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP project state:

- https://cloud.google.com/bigquery/docs/introduction
- https://cloud.google.com/bigquery/pricing
- https://cloud.google.com/bigquery/docs/bi-engine-overview
- https://cloud.google.com/bigquery/docs/partitioned-tables

## Grounding rule

Official documentation explains GCP BigQuery service behavior. It does not prove the user's current project, dataset configuration, slot usage, reservation assignments, billing mode, IAM policy state, or query cost history. Prefer live evidence or sanitized user-provided evidence for current-state claims.

If live GCP tooling is unavailable, say: "I can't query live state here, so I'm falling back to official GCP docs." Then fall back to these sources and sanitized user evidence.
