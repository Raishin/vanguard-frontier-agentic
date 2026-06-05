# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, RBAC, quotas, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/aks/upgrade-aks-node-pools-rolling
- https://learn.microsoft.com/azure/aks/upgrade-options
- https://learn.microsoft.com/azure/aks/upgrade-conceptual
- https://learn.microsoft.com/azure/aks/blue-green-node-pool-upgrade
- https://learn.microsoft.com/azure/architecture/operator-guides/aks/aks-upgrade-practices
- https://learn.microsoft.com/azure/aks/concepts-clusters-workloads
- https://learn.microsoft.com/azure/aks/operator-best-practices-cluster-security
- https://kubernetes.io/docs/tasks/run-application/configure-pdb/
- https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#rolling-update-deployment

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says AKS rolling upgrades add surge nodes, cordon and drain old nodes, optionally wait for soak time, reimage nodes, repeat, and remove surge nodes. Production node pools are recommended to use max surge rather than disruptive in-place unavailability where possible. Max unavailable can disrupt workloads and increase failures from unsatisfied PodDisruptionBudgets. Blue-green node-pool upgrade includes a final soak window during which rollback is available before old nodes are removed.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.

## Current Microsoft Learn deltas checked on 2026-06-05

- Production AKS upgrade guidance uses surge capacity as the normal safety path, but more surge is not automatically safer when quota, subnet IPs, or workload capacity are constrained.
- MaxUnavailable behavior is constrained by node-pool type and can deadlock or disrupt workloads when PodDisruptionBudgets, drain timeout, or capacity are wrong.
- Force upgrade bypasses normal disruption protections and must be treated as a high-risk live operation, not a routine rollout fix.
- Rollback is not a generic one-command undo; capture current state and rollback limits before any node-pool or workload rollout mutation.

