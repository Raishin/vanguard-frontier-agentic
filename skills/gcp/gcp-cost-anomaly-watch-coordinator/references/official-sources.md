# Official sources

Use this reference only when you need source grounding for GCP billing and cost management service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/billing/docs/how-to/budgets
- https://cloud.google.com/billing/docs/how-to/budget-api-overview
- https://cloud.google.com/billing/docs/how-to/export-data-bigquery
- https://cloud.google.com/bigquery/docs/best-practices-costs
- https://cloud.google.com/bigquery/docs/reservations-intro
- https://cloud.google.com/run/docs/configuring/max-instances
- https://cloud.google.com/compute/docs/instances/instance-life-cycle
- https://cloud.google.com/recommender/docs/overview
- https://cloud.google.com/billing/docs/how-to/notify

## Grounding rule

Official documentation explains GCP billing, cost management, and recommender service behavior. It does not prove the user's current billing export configuration, BigQuery slot reservation status, Cloud Run max-instances setting, or budget alert notification channels. Prefer sanitized billing export queries, gcloud output, or user-provided configuration for current-state claims.
