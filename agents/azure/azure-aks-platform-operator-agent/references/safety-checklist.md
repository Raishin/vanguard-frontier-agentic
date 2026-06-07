# Safety checklist

Use before AKS platform operations recommendations that affect access, cost, network exposure, data, compliance, production availability, or automation.

## Non-negotiables

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for Azure service behavior.
- Use read-only configured-environment evidence only when available and label it as sampled evidence.
- Never ask for credentials, tokens, tenant IDs, subscription IDs, connection strings, certificates, private keys, or customer data.
- Require explicit approval before recommending or executing mutations, deletes, privilege changes, secret-bearing reads, or production-impacting operations.
- State what is unknown; documentation proves service behavior, not the user's deployed state.
- Keep action/tool permissions least-privilege and scoped to the task.
- Require rollback or disablement path for production-impacting recommendations.
- Verify owner, scope, and evidence label before presenting a go/no-go verdict.

## Component risks

- **Upgrade disruption:** missing PDBs, blocked drains, bad surge values, deprecated APIs, unavailable rollback, and untested add-on compatibility.
- **Identity and access:** cluster-admin sprawl, static kubeconfigs, weak Microsoft Entra/Kubernetes RBAC mapping, and missing workload identity.
- **Network and capacity:** subnet IP exhaustion, quota gaps, network policy absence, ingress/DNS assumptions, and zone imbalance.
- **Observability and recovery:** missing Container Insights, upgrade events, alerting, node health checks, and post-maintenance validation.

## Evidence labels

Use sampled evidence, repo evidence, user-provided sanitized evidence, documentation-based, or inference. Documentation alone never proves the user's live Azure environment.
