# Official sources

Use this reference only when you need source grounding for AlloyDB or Cloud SQL behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/alloydb/docs/overview
- https://cloud.google.com/sql/docs/postgres/overview
- https://cloud.google.com/sql/docs/postgres/high-availability
- https://cloud.google.com/alloydb/docs/auth-proxy/overview

## Grounding rule

Official documentation explains AlloyDB and Cloud SQL behavior. It does not prove the user's current HA configuration, PITR status, connection method, backup retention policy, or maintenance window schedule. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
