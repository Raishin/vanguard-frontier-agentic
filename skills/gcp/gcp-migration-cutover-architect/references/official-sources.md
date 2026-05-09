# Official sources

Use this reference only when you need source grounding for GCP migration service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/migrate/virtual-machines/docs/5.0/overview
- https://cloud.google.com/database-migration/docs/overview
- https://cloud.google.com/storage-transfer/docs/overview
- https://cloud.google.com/bigquery-transfer/docs/introduction
- https://cloud.google.com/migrate/virtual-machines/docs/5.0/before-you-begin
- https://cloud.google.com/database-migration/docs/mysql/quickstart
- https://cloud.google.com/database-migration/docs/postgres/quickstart

## Grounding rule

Official documentation explains GCP migration service behavior. It does not prove the user's current replication state, DMS lag, cutover readiness, or source environment configuration. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
