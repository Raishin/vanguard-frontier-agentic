# Official sources

Use this reference when grounding Scaleway Kapsule live mutation behavior, Kubernetes version lifecycle, or PDB disruption semantics.

## Scaleway Kapsule and Kubernetes documentation

Use these as starting points for behavioral grounding — not as proof of the user's live cluster state:

- https://www.scaleway.com/en/docs/kubernetes/ — Kapsule product docs: version support windows, upgrade procedures, node pool lifecycle, cluster configuration options
- https://www.scaleway.com/en/developers/api/kubernetes/ — Kapsule API reference: cluster get/update, pool create/delete/update, node operations and state machine
- https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/k8s_cluster — Terraform `scaleway_k8s_cluster` resource: version field, CNI selection (immutable), admission plugins, auto-upgrade settings
- https://kubernetes.io/docs/concepts/workloads/pods/disruptions/ — Kubernetes upstream PodDisruptionBudget: disruption budget semantics, `maxUnavailable`, `minAvailable`, eviction API behavior

## Grounding rule

Official documentation describes Scaleway Kapsule service behavior and Kubernetes upstream disruption semantics. It does not prove the user's live cluster health, current node pool state, PDB coverage, or workload readiness. Documentation-based and inferred claims are insufficient to clear the hard-stop checklist. Require live evidence or user-provided sanitized evidence before any mutation proceeds.
