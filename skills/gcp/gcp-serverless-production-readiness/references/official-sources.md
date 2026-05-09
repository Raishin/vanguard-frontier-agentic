# Official sources

Use this reference only when you need source grounding for Cloud Run and Cloud Functions production readiness service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/run/docs/configuring/min-instances
- https://cloud.google.com/run/docs/configuring/concurrency
- https://cloud.google.com/run/docs/configuring/cpu
- https://cloud.google.com/run/docs/configuring/vpc-connectors
- https://cloud.google.com/run/docs/configuring/secrets
- https://cloud.google.com/run/docs/rollouts-rollbacks-traffic-migration
- https://cloud.google.com/run/docs/configuring/cmek
- https://cloud.google.com/functions/docs/concepts/version-comparison
- https://cloud.google.com/run/docs/securing/service-identity

## Grounding rule

Official documentation explains Cloud Run and Cloud Functions service behavior, configuration semantics, and security model. It does not prove the user's current min-instances setting, concurrency limit, VPC connector egress, or whether secrets are in environment variables vs. Secret Manager. Prefer sanitized gcloud run services describe output or user-provided configuration for current-state claims.
