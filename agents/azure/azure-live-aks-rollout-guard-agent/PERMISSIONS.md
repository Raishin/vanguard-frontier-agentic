# Permissions: Azure Live AKS Rollout Guard

Use least privilege for both the Azure control plane and Kubernetes data plane. Do not request cluster-admin credentials for normal rollout guarding.

## Azure control plane

Required capability is read cluster metadata and obtain user-level cluster credentials only when the session is explicitly authorized to inspect the target cluster. Avoid admin credential retrieval and avoid node-pool or cluster mutation permissions for rollout review.

## Kubernetes data plane

Prefer a namespace-scoped role limited to reading deployments, ReplicaSets, pods, pod logs, events, and PodDisruptionBudgets. Live actions such as pause, resume, undo, patch, update, scale, or apply require explicit human approval and should be scoped to the target namespace and workload.

## Do not assign

- Cluster admin kubeconfig for routine rollout review.
- Kubernetes cluster-admin binding for routine rollout review.
- Node pool delete, cluster delete, or unrelated infrastructure mutation permissions.

## Evidence boundary

Permission evidence is sampled configured-environment evidence only. It does not prove broad tenant, subscription, or cluster posture.
