# AKS Rollout Preflight

Use read-only checks first. Redact cluster, namespace, workload, and identity details before sharing outputs outside the trusted workspace.

## Required checks

1. Confirm target cluster, namespace, workload, intended action, and approval state.
2. Check current rollout status and deployment conditions.
3. Check desired, current, available, and unavailable replicas.
4. Audit PodDisruptionBudgets and allowed disruptions for the workload.
5. Review rolling update strategy, maxSurge, maxUnavailable, probes, and graceful termination settings.
6. Review ReplicaSet history before any undo.
7. Inspect recent events, restarts, scheduling failures, and readiness failures.

## Fail-fast conditions

- Target or kubectl context is ambiguous.
- Approval for a live action is missing.
- PDBs or replica health indicate unsafe disruption.
- Rollback target is unknown.
- Evidence contains secrets, tokens, kubeconfigs, or customer data.
