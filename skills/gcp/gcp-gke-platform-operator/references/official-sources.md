# Official sources

Use this reference only when you need source grounding for GKE behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-architecture
- https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
- https://cloud.google.com/binary-authorization/docs/overview
- https://cloud.google.com/kubernetes-engine/docs/concepts/release-channels

## Grounding rule

Official documentation explains GKE cluster behavior. It does not prove the user's current node pool version, Workload Identity binding state, Binary Authorization policy, or release channel enrollment. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
