# Official sources

Use this reference only when you need source grounding for Cloud Spanner behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/spanner/docs/whitepapers/life-of-reads-and-writes
- https://cloud.google.com/spanner/docs/schema-design
- https://cloud.google.com/spanner/docs/instances
- https://cloud.google.com/spanner/docs/secondary-indexes

## Grounding rule

Official documentation explains Cloud Spanner schema design and instance behavior. It does not prove the user's current schema hotspot profile, processing unit utilization, index count, or transaction latency. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
