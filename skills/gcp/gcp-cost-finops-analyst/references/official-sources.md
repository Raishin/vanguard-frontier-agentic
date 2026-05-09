# Official sources

Use this reference only when you need source grounding for GCP billing and cost optimization service behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP billing state:

- https://cloud.google.com/billing/docs/how-to/export-data-bigquery — billing export setup, dataset schema (`gcp_billing_export_v1`), and query patterns
- https://cloud.google.com/docs/cuds — Committed Use Discount overview, resource-based vs. spend-based mechanics, 1-year vs. 3-year terms
- https://cloud.google.com/billing/docs/how-to/budgets — budget alert setup, threshold types, notification channel options
- https://cloud.google.com/bigquery/pricing — BigQuery on-demand vs. slot edition pricing, break-even guidance
- https://cloud.google.com/kubernetes-engine/docs/concepts/autopilot-overview — GKE Autopilot pricing model, per-Pod resource billing, comparison with Standard
- https://cloud.google.com/compute/docs/sustained-use-discounts — Sustained Use Discount mechanics, qualifying instances, calculation method
- https://cloud.google.com/billing/docs/how-to/labels-best-practices — label strategy for cost attribution, recommended label keys
- https://cloud.google.com/recommender/docs/recommenders/overview — Recommender API for rightsizing recommendations (Compute, GKE, Cloud SQL)

## Grounding rule

Official documentation explains GCP billing service behavior and pricing models. It does not prove the user's current billing export configuration, label coverage percentage, CUD utilization, or spend breakdown. Prefer sanitized billing export query results, Cloud Billing Console exports, or structured user descriptions for current-state claims. Never project savings from documentation pricing without grounding in the user's actual usage pattern.
