# Official sources for Azure AKS Platform Operator

Use Microsoft Learn documentation through the user's configured documentation MCP before making AKS platform claims. Documentation proves documented AKS behavior; it does not prove this user's cluster version, node state, RBAC, quotas, add-ons, policies, or workload readiness.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Baseline architecture for AKS](https://learn.microsoft.com/en-us/azure/architecture/reference-architectures/containers/aks/baseline-aks) | Use as the minimum production reference for networking, private API, node pools, image supply chain, identity, GitOps, operations, and upgrade thinking. |
| [AKS start-here architecture guide](https://learn.microsoft.com/en-us/azure/architecture/reference-architectures/containers/aks-start-here) | Use to classify baseline, regulated, microservice, and BCDR variants. |
| [Architecture best practices for AKS](https://learn.microsoft.com/en-us/azure/well-architected/service-guides/azure-kubernetes-service) | Ground reliability, security, operational excellence, cost, and performance recommendations. |
| [AKS upgrade options](https://learn.microsoft.com/en-us/azure/aks/upgrade-options) | Use for upgrade method selection, version skew, node-pool upgrade paths, and maintenance windows. |
| [AKS upgrade practices](https://learn.microsoft.com/en-us/azure/architecture/operator-guides/aks/aks-upgrade-practices) | Use for day-2 upgrade sequencing, preproduction validation, surge, PDB, and rollback expectations. |
| [Workload identity overview](https://learn.microsoft.com/en-us/azure/aks/workload-identity-overview) | Prefer federated workload identity over static secrets for Azure resource access from pods. |
| [Network policy best practices](https://learn.microsoft.com/en-us/azure/aks/network-policy-best-practices) | Use for default deny, application policies, Cilium preference for Linux, and Windows/Calico caveats. |
| [Best practices for AKS cluster reliability](https://learn.microsoft.com/en-us/azure/aks/best-practices-app-cluster-reliability) | Ground pod scheduling, health, scaling, and recovery checks. |

## Source-grounding rules

- For architecture: cite Microsoft Learn and label anything cluster-specific as inference unless sampled.
- For current state: require read-only configured-environment evidence for cluster version, upgrade channels, node-pool state, add-ons, policies, and diagnostics.
- For Kubernetes manifests: treat local YAML as desired state, not proof of live state.
- For kubectl output: accept only sanitized output and never ask for tokens, kubeconfigs, certificates, or private cluster endpoints.

## Current review emphasis

- AKS is a day-2 operations commitment, not just a managed control plane.
- Production clusters need separate system and user node pools, planned upgrades, enough surge capacity, pod disruption planning, observability, policy, and a rollback pattern.
- Network policy engines and Windows node behavior differ; do not write one-size-fits-all policy guidance.
