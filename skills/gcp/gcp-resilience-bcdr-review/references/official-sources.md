# Official sources

Use this reference only when you need source grounding for GCP resilience and BCDR service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/architecture/disaster-recovery
- https://cloud.google.com/sql/docs/postgres/high-availability
- https://cloud.google.com/spanner/docs/instance-configurations
- https://cloud.google.com/kubernetes-engine/docs/concepts/multi-cluster-ingress
- https://cloud.google.com/compute/docs/disks/scheduled-snapshots
- https://cloud.google.com/run/docs/multiple-regions
- https://cloud.google.com/load-balancing/docs/https/setting-up-https

## Grounding rule

Official documentation explains GCP resilience service behavior. It does not prove the user's current HA configuration, snapshot schedule, recovery test history, or failover posture. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
