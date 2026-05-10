# Official sources

Use this reference only when you need source grounding for OVHcloud Managed Kubernetes service behavior or the detailed source list.

## OVHcloud documentation

Use these as starting points, not as proof of the user's live cluster state:

- https://help.ovhcloud.com/csm/en-public-cloud-kubernetes?id=kb_article_view&sysparm_article=KB0049613
- https://registry.terraform.io/providers/ovh/ovh/latest/docs/resources/cloud_project_kube
- https://registry.terraform.io/providers/ovh/ovh/latest/docs/resources/cloud_project_kube_nodepool

## Grounding rule

Official documentation explains OVHcloud Managed Kubernetes version support windows, node pool lifecycle policies, and MCK-specific behavior. It does not prove the user's current cluster version, node pool configuration, PodDisruptionBudget coverage, RBAC state, or network policy posture. Prefer live MCK API evidence or sanitized user-provided cluster config for current-state claims.
