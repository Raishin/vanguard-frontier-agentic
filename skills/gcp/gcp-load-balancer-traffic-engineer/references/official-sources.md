# Official sources

Use this reference only when you need source grounding for GCP load balancing service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/load-balancing/docs/load-balancing-overview
- https://cloud.google.com/armor/docs/cloud-armor-overview
- https://cloud.google.com/armor/docs/security-policy-overview
- https://cloud.google.com/load-balancing/docs/health-check-concepts
- https://cloud.google.com/load-balancing/docs/backend-service
- https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs
- https://cloud.google.com/load-balancing/docs/ssl-policies-concepts
- https://cloud.google.com/load-balancing/docs/enabling-connection-draining

## Grounding rule

Official documentation explains GCP load balancing service behavior, type capabilities, and configuration semantics. It does not prove the user's current health check configuration, Cloud Armor policy attachment, or connection draining settings. Prefer sanitized gcloud compute output or user-provided configuration for current-state claims.
