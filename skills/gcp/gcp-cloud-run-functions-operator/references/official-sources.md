# Official sources

Use this reference only when you need source grounding for Cloud Run or Cloud Functions behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/run/docs/overview/what-is-cloud-run
- https://cloud.google.com/run/docs/rollouts-rollbacks-traffic-migration
- https://cloud.google.com/functions/docs/concepts/overview
- https://cloud.google.com/eventarc/docs/overview

## Grounding rule

Official documentation explains Cloud Run and Cloud Functions behavior. It does not prove the user's current revision traffic splits, minimum instance configuration, concurrency settings, or VPC egress setup. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
