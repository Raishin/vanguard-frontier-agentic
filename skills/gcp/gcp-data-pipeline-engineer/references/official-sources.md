# Official sources

Use this reference only when you need source grounding for GCP data pipeline behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/dataflow/docs/overview
- https://cloud.google.com/pubsub/docs/overview
- https://cloud.google.com/dataproc/docs/overview
- https://cloud.google.com/composer/docs/concepts/overview
- https://cloud.google.com/dataplex/docs/introduction

## Grounding rule

Official documentation explains GCP data pipeline service behavior. It does not prove the user's current Dataflow job health, Pub/Sub subscription lag, Dataproc cluster state, Composer DAG status, or Dataplex policy bindings. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
