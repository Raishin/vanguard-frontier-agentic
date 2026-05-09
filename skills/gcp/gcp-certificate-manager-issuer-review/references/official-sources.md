# Official sources

Use this reference only when you need source grounding for GCP Certificate Manager and TLS certificate service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/certificate-manager/docs/overview
- https://cloud.google.com/certificate-manager/docs/deploy-google-managed-dns-auth
- https://cloud.google.com/certificate-manager/docs/reference/certificate-maps
- https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs
- https://cloud.google.com/certificate-manager/docs/monitor-certificate-status
- https://cloud.google.com/load-balancing/docs/ssl-policies-concepts
- https://cloud.google.com/dns/docs/records/caa

## Grounding rule

Official documentation explains GCP Certificate Manager and TLS certificate service behavior, DNS authorization semantics, and certificate map attachment requirements. It does not prove the user's current certificate map configuration, CAA record status, or expiry monitoring setup. Prefer sanitized gcloud certificate-manager output or user-provided configuration for current-state claims.
