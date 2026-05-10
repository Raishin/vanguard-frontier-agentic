# Official sources

Use this reference only when grounding IONOS managed Kubernetes cluster behavior, node pool configuration, or IaC definitions.

## IONOS Cloud Kubernetes documentation

Use these as starting points, not as proof of the user's live cluster state:

- https://docs.ionos.com/cloud/managed-kubernetes — cluster lifecycle, node pool concepts, supported Kubernetes versions, and upgrade behavior
- https://registry.terraform.io/providers/ionos-cloud/ionoscloud/latest/docs/resources/k8s_cluster — IaC cluster resource definition, version arguments, and supported attributes
- https://registry.terraform.io/providers/ionos-cloud/ionoscloud/latest/docs/resources/k8s_node_pool — IaC node pool resource definition, autoscaling configuration, and LAN attachment
- https://docs.ionos.com/cloud/ — general platform reference for datacenter regions and GDPR residency

## Grounding rule

Official IONOS documentation describes supported Kubernetes versions, node pool configuration options, and cluster lifecycle behavior. It does not prove the user's current cluster state, PDB definitions, workload placement, or node pool health. Prefer live `kubectl` output or sanitized user-provided evidence for current-state claims. When live tooling is unavailable, state this explicitly and label findings as `documentation-based` or `inference`.
