# Official sources

Use this reference when grounding Scaleway Kapsule service behavior, CNI options, or Kubernetes platform details.

## Scaleway Kubernetes documentation

Use these as starting points — not as proof of the user's live cluster state:

- https://www.scaleway.com/en/docs/kubernetes/ — Kapsule product docs: cluster creation, node pool management, CNI options, version support, admission plugins
- https://www.scaleway.com/en/developers/api/kubernetes/ — Kapsule API reference: cluster, pool, and node resource operations
- https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/k8s_cluster — Terraform `scaleway_k8s_cluster` resource: CNI selection, admission plugins, auto-upgrade settings, cluster type
- https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/k8s_pool — Terraform `scaleway_k8s_pool` resource: node type, zone binding, autoscaling, placement group, taint configuration
- https://kubernetes.io/docs/concepts/workloads/pods/disruptions/ — Kubernetes upstream PodDisruptionBudget concepts and disruption budget behavior

## Grounding rule

Official documentation describes Scaleway Kapsule service behavior and Kubernetes upstream behavior. It does not prove the user's current cluster version, active node pool configuration, CNI in use, or PDB coverage. Prefer repo evidence or sanitized user-provided evidence for current-state claims. CNI and control-plane version immutability warnings must always come from confirmed evidence, not assumption.
